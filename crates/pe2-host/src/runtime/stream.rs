//! CD streaming (libcd CdRead2 + the St* ring functions), used for STR
//! movies: sectors are read continuously into a ring buffer in PSX RAM and
//! assembled into whole frames for the MDEC.
//!
//! Ring layout (the game sizes it as `sectors` * 2048 bytes): frame data,
//! 2016 payload bytes per sector, kept contiguous per frame in the first
//! `sectors` * 2016 bytes; each frame's 32-byte header (StHEADER) in a slot
//! of the last `sectors` * 32 bytes.

use std::collections::VecDeque;
use std::sync::Mutex;

use super::{irq, trace};

const PAYLOAD: u32 = 2016;
const HEADER: u32 = 32;
const STR_ID: u16 = 0x0160;

struct Frame {
    number: u32,
    data_off: u32,
    len: u32,
    header_slot: u32,
    sectors: u16,
    received: u64,
    complete: bool,
    handed_out: bool,
    dropped: bool,
}

#[derive(Default)]
struct Ring {
    base: u32,
    sectors: u32,
    active: bool,
    frames: VecDeque<Frame>,
    next_slot: u32,
    back_frame: u32,
    back_loc: [u8; 4],
}

static RING: Mutex<Option<Ring>> = Mutex::new(None);

fn with_ring<R>(f: impl FnOnce(&mut Ring) -> R) -> R {
    let mut r = RING.lock().unwrap();
    f(r.get_or_insert_with(Ring::default))
}

impl Ring {
    fn data_cap(&self) -> u32 {
        self.sectors * PAYLOAD
    }

    /// Finds room for `len` contiguous bytes after the live frames. Frames
    /// are allocated in order around the ring, so the live data runs from
    /// the oldest frame's start to the newest frame's end, possibly wrapped.
    fn allocate(&self, len: u32) -> Option<u32> {
        let cap = self.data_cap();
        if len > cap {
            return None;
        }
        let mut live = self.frames.iter().filter(|f| !f.dropped);
        let Some(first) = live.next() else { return Some(0) };
        let last = self.frames.iter().rev().find(|f| !f.dropped).unwrap();
        let (tail, head) = (first.data_off, last.data_off + last.len);
        let wrapped = last.data_off < first.data_off;
        if !wrapped {
            // Live: [tail, head). Free: [head, cap) then [0, tail).
            if head + len <= cap {
                Some(head)
            } else if len <= tail {
                Some(0)
            } else {
                None
            }
        } else if head + len <= tail {
            // Live: [tail, cap) and [0, head). Free: [head, tail).
            Some(head)
        } else {
            None
        }
    }

    fn push_sector(&mut self, user: &[u8], loc: [u8; 3]) {
        let rd16 = |o: usize| u16::from_le_bytes([user[o], user[o + 1]]);
        let rd32 = |o: usize| u32::from_le_bytes([user[o], user[o + 1], user[o + 2], user[o + 3]]);
        if rd16(0) != STR_ID {
            return;
        }
        let (index, count, number) = (rd16(4), rd16(6), rd32(8));
        if count == 0 || u32::from(count) > self.sectors || index >= count {
            return;
        }
        let same_frame = self.frames.back().is_some_and(|f| f.number == number && !f.complete);
        if !same_frame {
            // A new frame: anything unfinished before it is lost.
            if let Some(f) = self.frames.back_mut()
                && !f.complete
            {
                f.dropped = true;
            }
            self.frames.retain(|f| !(f.dropped && !f.handed_out));
            let len = u32::from(count) * PAYLOAD;
            let (data_off, dropped) = match self.allocate(len) {
                Some(off) => (off, false),
                None => (0, true),
            };
            let slot = self.next_slot % self.sectors.max(1);
            self.next_slot += 1;
            self.frames.push_back(Frame {
                number,
                data_off,
                len: if dropped { 0 } else { len },
                header_slot: slot,
                sectors: count,
                received: 0,
                complete: false,
                handed_out: false,
                dropped,
            });
        }
        let base = self.base;
        let header_base = base + self.data_cap();
        let f = self.frames.back_mut().unwrap();
        if f.dropped {
            return;
        }
        // SAFETY: the ring lies in PSX RAM, sized by the game.
        unsafe {
            let dst = (base + f.data_off + u32::from(index) * PAYLOAD) as *mut u8;
            std::ptr::copy_nonoverlapping(user[HEADER as usize..].as_ptr(), dst, PAYLOAD as usize);
            if index == 0 {
                let h = (header_base + f.header_slot * HEADER) as *mut u8;
                std::ptr::copy_nonoverlapping(user.as_ptr(), h, HEADER as usize);
                // StHEADER ends with the frame's disc location.
                std::ptr::copy_nonoverlapping(loc.as_ptr(), h.add(28), 3);
            }
        }
        f.received |= 1 << index;
        if f.received.count_ones() == u32::from(f.sectors) {
            f.complete = true;
            self.back_frame = number;
            self.back_loc = [loc[0], loc[1], loc[2], 0];
            if trace("stream") {
                eprintln!("[stream] frame {number} complete ({} sectors)", f.sectors);
            }
        }
    }
}

/// True while CdRead2 streams into the ring.
pub fn active() -> bool {
    with_ring(|r| r.active && r.sectors > 0)
}

pub fn set_active(on: bool) {
    with_ring(|r| r.active = on);
}

/// A data sector read while streaming (2048 user bytes).
pub fn push_sector(user: &[u8], loc: [u8; 3]) {
    with_ring(|r| r.push_sector(user, loc));
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StSetRing(ring: u32, sectors: u32) {
    let _g = irq::guard();
    with_ring(|r| {
        r.base = ring;
        r.sectors = sectors;
        r.frames.clear();
        r.next_slot = 0;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StClearRing() {
    let _g = irq::guard();
    with_ring(|r| {
        r.frames.clear();
        r.next_slot = 0;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StUnSetRing() {
    let _g = irq::guard();
    with_ring(|r| {
        r.frames.clear();
        r.active = false;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StSetStream(_mode: u32, _start: u32, _end: u32, _func1: u32, _func2: u32) {}

/// StGetNext(&addr, &header): the oldest complete frame not yet handed out.
/// Returns 0 when one is available, 1 otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StGetNext(addr: *mut u32, header: *mut u32) -> u32 {
    let _g = irq::guard();
    with_ring(|r| {
        let header_base = r.base + r.data_cap();
        let base = r.base;
        let Some(f) = r.frames.iter_mut().find(|f| f.complete && !f.handed_out && !f.dropped) else {
            return 1;
        };
        f.handed_out = true;
        // SAFETY: the game passes two out pointers.
        unsafe {
            *addr = base + f.data_off;
            *header = header_base + f.header_slot * HEADER;
        }
        0
    })
}

/// StFreeRing(base): releases the frame whose data starts at `base`.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StFreeRing(data: u32) -> u32 {
    let _g = irq::guard();
    with_ring(|r| {
        let base = r.base;
        match r.frames.iter().position(|f| base + f.data_off == data && f.handed_out) {
            Some(i) => {
                r.frames.remove(i);
                0
            }
            None => 1,
        }
    })
}

/// The ring is filled as sectors arrive; nothing is deferred.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StCdInterrupt() {}

/// StGetBackloc(loc): the most recently completed frame's number, and its
/// disc location in `loc`.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StGetBackloc(loc: *mut u8) -> i32 {
    let _g = irq::guard();
    with_ring(|r| {
        if !loc.is_null() {
            // SAFETY: `loc` is a 4-byte CdlLOC.
            unsafe { std::ptr::copy_nonoverlapping(r.back_loc.as_ptr(), loc, 4) };
        }
        r.back_frame as i32
    })
}
