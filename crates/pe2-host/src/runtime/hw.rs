//! Hardware registers seen by interpreted library code.
//!
//! Some libraries are thin layers over a device (libpress over the MDEC and
//! DMA). For those, the original code runs in the interpreter and its
//! register accesses come here, to device models. Unknown registers are
//! reported once and read as zero.
//!
//! DMA transfers complete immediately; their completion interrupt (the
//! callback registered with DMACallback) is raised shortly after, or, for
//! the MDEC's output, once the console's MDEC would have decoded what was
//! read: the game's pacing around image decodes (scene camera switches)
//! counts on that time.

use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use super::mdec::Mdec;
use super::{call_psx, irq, trace};

const MDEC0: u32 = 0x1F80_1820;
const MDEC1: u32 = 0x1F80_1824;
const DMA_BASE: u32 = 0x1F80_1080;
const DPCR: u32 = 0x1F80_10F0;
const DICR: u32 = 0x1F80_10F4;

#[derive(Default, Clone, Copy)]
struct DmaChannel {
    madr: u32,
    bcr: u32,
    chcr: u32,
}

#[derive(Default)]
struct Hw {
    mdec: Mdec,
    /// DMA 1 (the MDEC's output) was started before the MDEC had the data:
    /// it runs once a decode provides it, as the hardware's request line
    /// holds the transfer.
    out_waiting: bool,
    dma: [DmaChannel; 7],
    dpcr: u32,
    dicr: u32,
}

static HW: Mutex<Option<Hw>> = Mutex::new(None);
static DMA_CALLBACKS: [AtomicU32; 7] = [const { AtomicU32::new(0) }; 7];
static REPORTED: Mutex<Option<HashSet<u32>>> = Mutex::new(None);

fn with_hw<R>(f: impl FnOnce(&mut Hw) -> R) -> R {
    let mut h = HW.lock().unwrap();
    f(h.get_or_insert_with(Hw::default))
}

fn report(addr: u32, write: bool) {
    let mut seen = REPORTED.lock().unwrap();
    if seen.get_or_insert_with(HashSet::new).insert(addr) {
        eprintln!("[hw] unhandled {} of register 0x{addr:08X}", if write { "write" } else { "read" });
    }
}

fn ram_word(addr: u32) -> *mut u32 {
    (0x8000_0000 | (addr & 0x00FF_FFFC)) as *mut u32
}

/// The MDEC's loaded tables (luma and chroma quantization, scale), for tests.
pub fn mdec_tables() -> ([u8; 64], [u8; 64], [i16; 64]) {
    let _g = irq::guard();
    with_hw(|h| h.mdec.tables())
}

pub fn read32(addr: u32) -> u32 {
    let _g = irq::guard();
    if trace("hw") {
        eprintln!("[hw] read 0x{addr:08X}");
    }
    with_hw(|h| match addr {
        MDEC0 => h.mdec.output.pop_front().unwrap_or(0),
        MDEC1 => h.mdec.status(),
        DPCR => h.dpcr,
        DICR => h.dicr,
        a if (DMA_BASE..DMA_BASE + 0x70).contains(&a) => {
            let ch = &h.dma[((a - DMA_BASE) / 0x10) as usize];
            match a & 0xF {
                0x0 => ch.madr,
                0x4 => ch.bcr,
                0x8 => ch.chcr,
                _ => 0,
            }
        }
        a => {
            report(a, false);
            0
        }
    })
}

pub fn write32(addr: u32, v: u32) {
    let _g = irq::guard();
    if trace("hw") {
        eprintln!("[hw] write 0x{addr:08X} = 0x{v:08X}");
    }
    let completed = with_hw(|h| match addr {
        MDEC0 => {
            h.mdec.write_word(v);
            h.run_waiting_output()
        }
        MDEC1 => {
            if v & (1 << 31) != 0 {
                h.mdec.reset();
                // A reset abandons what the MDEC was decoding: no
                // completion comes for it (skipping a movie mid-frame).
                MDEC_RESETS.fetch_add(1, Ordering::SeqCst);
                *MDEC_DONE_AT.lock().unwrap() = None;
                h.out_waiting = false;
            }
            h.mdec.enable_in = v & (1 << 30) != 0;
            h.mdec.enable_out = v & (1 << 29) != 0;
            None
        }
        DPCR => {
            h.dpcr = v;
            None
        }
        DICR => {
            // Writing 1 to a flag bit acknowledges it.
            h.dicr = (h.dicr & !(v & 0x7F00_0000)) | (v & 0x00FF_FFFF);
            None
        }
        a if (DMA_BASE..DMA_BASE + 0x70).contains(&a) => {
            let index = ((a - DMA_BASE) / 0x10) as usize;
            match a & 0xF {
                0x0 => h.dma[index].madr = v & 0x00FF_FFFF,
                0x4 => h.dma[index].bcr = v,
                0x8 => {
                    h.dma[index].chcr = v;
                    if v & (1 << 24) != 0 {
                        if index == 1 {
                            h.out_waiting = true;
                            return h.run_waiting_output();
                        }
                        let channel = h.run_dma(index);
                        // The input may complete a decode a waiting output
                        // transfer needs; its completion comes after.
                        if let Some(out) = h.run_waiting_output() {
                            dma_complete(channel, 0);
                            return Some(out);
                        }
                        return Some((channel, 0));
                    }
                }
                _ => {}
            }
            None
        }
        a => {
            report(a, true);
            None
        }
    });
    if let Some((channel, macroblocks)) = completed {
        dma_complete(channel, macroblocks);
    }
}

impl Hw {
    /// Runs DMA 1 if it waits and the MDEC has its data now; returns its
    /// channel and the macroblocks it carries.
    fn run_waiting_output(&mut self) -> Option<(usize, u32)> {
        let words = self.dma_words(1);
        if !self.out_waiting || (self.mdec.output.len() as u32) < words {
            return None;
        }
        self.out_waiting = false;
        let macroblocks = words.div_ceil(self.mdec.words_per_macroblock());
        Some((self.run_dma(1), macroblocks))
    }

    /// Words DMA `index` moves as set up.
    fn dma_words(&self, index: usize) -> u32 {
        let ch = self.dma[index];
        let block = ch.bcr & 0xFFFF;
        match (ch.chcr >> 9) & 3 {
            0 => {
                if block == 0 {
                    0x1_0000
                } else {
                    block
                }
            }
            _ => block.max(1) * (ch.bcr >> 16).max(1),
        }
    }

    /// Runs DMA `index` to completion; returns the channel.
    fn run_dma(&mut self, index: usize) -> usize {
        let ch = self.dma[index];
        let words = self.dma_words(index);
        let to_device = ch.chcr & 1 != 0;
        let step: i32 = if ch.chcr & 2 != 0 { -4 } else { 4 };
        let mut addr = ch.madr;
        if trace("dma") {
            eprintln!(
                "[dma] channel {index} {} {words} words at 0x{addr:06X}",
                if to_device { "from RAM" } else { "to RAM" }
            );
        }
        for _ in 0..words {
            let p = ram_word(addr);
            match (index, to_device) {
                // SAFETY: DMA addresses are PSX RAM.
                (0, true) => self.mdec.write_word(unsafe { p.read() }),
                (1, false) => {
                    let w = match self.mdec.output.pop_front() {
                        Some(w) => w,
                        None => {
                            static UNDERRUNS: AtomicU32 = AtomicU32::new(0);
                            if UNDERRUNS.fetch_add(1, Ordering::Relaxed) < 5 {
                                eprintln!("[mdec] output underrun: DMA 1 read past the decoded data");
                            }
                            0
                        }
                    };
                    // SAFETY: as above.
                    unsafe { p.write(w) };
                }
                _ => {
                    report(DMA_BASE + 0x10 * index as u32 + 8, true);
                    break;
                }
            }
            addr = addr.wrapping_add(step as u32) & 0x00FF_FFFF;
        }
        self.dma[index].madr = addr;
        self.dma[index].chcr &= !(1 << 24);
        self.dicr |= 1 << (24 + index);
        index
    }
}

/// The console's MDEC: 448 CPU cycles an 8x8 block, six blocks a
/// macroblock (as DuckStation times it).
const MDEC_MACROBLOCK: Duration = Duration::from_nanos(2688 * 1_000_000_000 / 33_868_800);

/// When the MDEC is done with what it was given so far.
static MDEC_DONE_AT: Mutex<Option<Instant>> = Mutex::new(None);
/// MDEC resets so far: a completion due from before the latest one is dropped.
static MDEC_RESETS: AtomicU32 = AtomicU32::new(0);

/// Raises `channel`'s completion interrupt; for the MDEC's output, once its
/// `macroblocks` are decoded, after what it was decoding before.
fn dma_complete(channel: usize, macroblocks: u32) {
    let mut at = Instant::now() + Duration::from_micros(100);
    if channel == 1 && macroblocks > 0 {
        let mut done = MDEC_DONE_AT.lock().unwrap();
        let start = done.map_or(Instant::now(), |d| d.max(Instant::now()));
        at = start + MDEC_MACROBLOCK * macroblocks;
        *done = Some(at);
    }
    let cb = DMA_CALLBACKS[channel].load(Ordering::SeqCst);
    if cb == 0 {
        return;
    }
    let resets = MDEC_RESETS.load(Ordering::SeqCst);
    irq::schedule(
        at,
        Box::new(move || {
            if channel <= 1 && MDEC_RESETS.load(Ordering::SeqCst) != resets {
                return;
            }
            call_psx(cb, &[]);
        }),
    );
}

/// DMACallback(channel, func): `func` runs when a transfer on `channel`
/// completes. Returns the previous callback.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DMACallback(channel: i32, func: u32) -> u32 {
    let Some(slot) = DMA_CALLBACKS.get(channel as usize) else { return 0 };
    slot.swap(func, Ordering::SeqCst)
}
