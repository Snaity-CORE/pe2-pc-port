//! Vertical blank timing (libetc VSync / VSyncCallback) and the game's
//! interrupt masking (libapi EnterCriticalSection / ExitCriticalSection).

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
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
}

static TIMING: Mutex<Option<Timing>> = Mutex::new(None);
/// Time base for LAST_CALL_NS.
static EPOCH: OnceLock<Instant> = OnceLock::new();
/// When the previous waiting VSync call returned, in nanoseconds since
/// EPOCH. An atomic rather than part of TIMING: VSync(1) reads it without a
/// lock, so it needs no guard and interrupts get in while the game polls it.
static LAST_CALL_NS: AtomicU64 = AtomicU64::new(0);

pub fn start() {
    let now = Instant::now();
    EPOCH.get_or_init(|| now);
    *TIMING.lock().unwrap() = Some(Timing { last_vblank_at: now, last_call: 0 });
    schedule_vblank(now + FIELD);
}

fn last_call_at() -> Instant {
    *EPOCH.get().expect("vsync not started") + Duration::from_nanos(LAST_CALL_NS.load(Ordering::SeqCst))
}

fn mark_call(at: Instant) {
    let since = at.duration_since(*EPOCH.get().expect("vsync not started"));
    LAST_CALL_NS.store(since.as_nanos() as u64, Ordering::SeqCst);
}

fn schedule_vblank(at: Instant) {
    irq::schedule(
        at,
        Box::new(move || {
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
            if super::trace("spu") && now - at > FIELD * 3 {
                eprintln!("[spu {:8.3}] vblank delivered {} ms late", super::spu::stamp(), (now - at).as_millis());
            }
            schedule_vblank(next);

            // Mods' cheats and patches, before the game's handler sees memory.
            crate::mods::apply_patches();
            crate::mods::music::poll();
            let cb = CALLBACK.load(Ordering::SeqCst);
            if cb != 0 {
                call_psx(cb, &[]);
            }
            // The game flips and draws in its callback: the frame is whole now.
            super::gpu::latch();
            crate::video::vblank();
        }),
    );
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
///
/// Modes -1 and 1 take no lock and no guard: games poll them in tight
/// loops (scene pacing busy-waits on VSync(1)), and interrupts must keep
/// coming during those loops as they do on the console.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_VSync(mode: i32) -> i32 {
    match mode {
        m if m < 0 => return VBLANKS.load(Ordering::SeqCst) as i32,
        1 => return hsyncs_since(last_call_at()) & 0xFFFF,
        _ => {}
    }
    let _g = irq::guard();
    let last_call_at = last_call_at();
    let last_call = TIMING.lock().unwrap().as_ref().expect("vsync not started").last_call;
    let now = VBLANKS.load(Ordering::SeqCst);
    let target = if mode == 0 { now + 1 } else { last_call.wrapping_add(mode as u32).max(now + 1) };
    irq::wait_until(|| VBLANKS.load(Ordering::SeqCst).wrapping_sub(target) as i32 >= 0);
    super::scan_images();
    if super::trace("spu") && last_call_at.elapsed() > FIELD * 6 {
        eprintln!("[spu {:8.3}] game frame took {} ms", super::spu::stamp(), last_call_at.elapsed().as_millis());
    }
    if super::trace("hang") && last_call_at.elapsed() > FIELD * 6 {
        super::profile::report(last_call_at);
    }
    let ret = hsyncs_since(last_call_at) & 0xFFFF;
    TIMING.lock().unwrap().as_mut().unwrap().last_call = VBLANKS.load(Ordering::SeqCst);
    mark_call(Instant::now());
    ret
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
