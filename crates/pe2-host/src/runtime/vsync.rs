//! Vertical blank timing (libetc VSync / VSyncCallback) and the game's
//! interrupt masking (libapi EnterCriticalSection / ExitCriticalSection).

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::{call_psx, irq};

/// NTSC field rate: 59.94 Hz.
const FIELD: Duration = Duration::from_nanos(16_683_350);
/// NTSC horizontal line rate.
const HSYNC_HZ: f64 = 15_734.264;

static VBLANKS: AtomicU32 = AtomicU32::new(0);
static CALLBACK: AtomicU32 = AtomicU32::new(0);

struct Timing {
    last_vblank_at: Instant,
    /// VBLANKS at the previous VSync(n >= 0) call.
    last_call: u32,
    last_call_at: Instant,
}

static TIMING: Mutex<Option<Timing>> = Mutex::new(None);

pub fn start() {
    let now = Instant::now();
    *TIMING.lock().unwrap() = Some(Timing { last_vblank_at: now, last_call: 0, last_call_at: now });
    schedule_vblank(now + FIELD);
}

fn schedule_vblank(at: Instant) {
    irq::schedule(at, Box::new(move || {
        VBLANKS.fetch_add(1, Ordering::SeqCst);
        super::stubs::refresh_pads();
        super::scan_images();
        if let Some(t) = TIMING.lock().unwrap().as_mut() {
            t.last_vblank_at = at;
        }
        // Keep the field rate regardless of delivery latency.
        let mut next = at + FIELD;
        let now = Instant::now();
        if next < now {
            next = now + FIELD; // fell far behind (debugger, stall): resync
        }
        schedule_vblank(next);

        let cb = CALLBACK.load(Ordering::SeqCst);
        if cb != 0 {
            call_psx(cb, &[]);
        }
    }));
}

fn hsyncs_since(at: Instant) -> i32 {
    (at.elapsed().as_secs_f64() * HSYNC_HZ) as i32
}

/// VSync(mode): mode 0 waits for the next vertical blank; n > 1 waits until
/// n blanks have passed since the previous call; -1 returns the blank
/// counter. Mode 1 does not wait and, like the waiting modes, returns the
/// horizontal lines counted since the previous waiting call, as a 16-bit
/// value (libetc reads them from the free-running hblank root counter).
/// Callers time a frame with differences of these values, so they must not
/// reset at each vertical blank.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_VSync(mode: i32) -> i32 {
    let _g = irq::guard();
    match mode {
        m if m < 0 => VBLANKS.load(Ordering::SeqCst) as i32,
        1 => {
            let at = TIMING.lock().unwrap().as_ref().map(|t| t.last_call_at);
            at.map_or(0, |at| hsyncs_since(at) & 0xFFFF)
        }
        n => {
            let (last_call, last_call_at) = {
                let t = TIMING.lock().unwrap();
                let t = t.as_ref().expect("vsync not started");
                (t.last_call, t.last_call_at)
            };
            let now = VBLANKS.load(Ordering::SeqCst);
            let target = if n == 0 { now + 1 } else { last_call.wrapping_add(n as u32).max(now + 1) };
            irq::wait_until(|| VBLANKS.load(Ordering::SeqCst).wrapping_sub(target) as i32 >= 0);
            super::scan_images();
            let ret = hsyncs_since(last_call_at) & 0xFFFF;
            let mut t = TIMING.lock().unwrap();
            let t = t.as_mut().unwrap();
            t.last_call = VBLANKS.load(Ordering::SeqCst);
            t.last_call_at = Instant::now();
            ret
        }
    }
}

/// VSyncCallback(func): runs `func` at every vertical blank (0 removes it).
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_VSyncCallback(func: u32) -> i32 {
    let _g = irq::guard();
    CALLBACK.store(func, Ordering::SeqCst);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_EnterCriticalSection() -> i32 {
    irq::set_game_masked(true);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ExitCriticalSection() {
    irq::set_game_masked(false);
}
