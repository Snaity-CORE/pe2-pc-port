//! Hardware registers seen by interpreted library code.
//!
//! Some libraries are thin layers over a device (libpress over the MDEC and
//! DMA). For those, the original code runs in the interpreter and its
//! register accesses come here, to device models. Unknown registers are
//! reported once and read as zero.
//!
//! DMA transfers complete immediately; their completion interrupt (the
//! callback registered with DMACallback) is raised shortly after.

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
            None
        }
        MDEC1 => {
            if v & (1 << 31) != 0 {
                h.mdec.reset();
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
                        return Some(h.run_dma(index));
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
    if let Some(channel) = completed {
        dma_complete(channel);
    }
}

impl Hw {
    /// Runs DMA `index` to completion; returns the channel.
    fn run_dma(&mut self, index: usize) -> usize {
        let ch = self.dma[index];
        let block = ch.bcr & 0xFFFF;
        let words = match (ch.chcr >> 9) & 3 {
            0 => if block == 0 { 0x1_0000 } else { block },
            _ => block.max(1) * (ch.bcr >> 16).max(1),
        };
        let to_device = ch.chcr & 1 != 0;
        let step: i32 = if ch.chcr & 2 != 0 { -4 } else { 4 };
        let mut addr = ch.madr;
        if trace("dma") {
            eprintln!("[dma] channel {index} {} {words} words at 0x{addr:06X}", if to_device { "from RAM" } else { "to RAM" });
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

fn dma_complete(channel: usize) {
    let cb = DMA_CALLBACKS[channel].load(Ordering::SeqCst);
    if cb == 0 {
        return;
    }
    irq::schedule(Instant::now() + Duration::from_micros(100), Box::new(move || {
        call_psx(cb, &[]);
    }));
}

/// DMACallback(channel, func): `func` runs when a transfer on `channel`
/// completes. Returns the previous callback.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DMACallback(channel: i32, func: u32) -> u32 {
    let Some(slot) = DMA_CALLBACKS.get(channel as usize) else { return 0 };
    slot.swap(func, Ordering::SeqCst)
}
