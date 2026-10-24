//! Vertical blank timing (libetc VSync / VSyncCallback) and the game's
//! interrupt masking (libapi EnterCriticalSection / ExitCriticalSection).

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::collections::VecDeque;
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
    /// When the game's own part of the latest vertical blank ended (its
    /// callback): where the console's VSync, waiting on that blank, returns.
    resumed_at: Instant,
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
/// Waiting VSync calls so far (PE2_TRACE=scene reports them per scene gap).
static WAITS: AtomicU32 = AtomicU32::new(0);
/// The latest waiting VSync calls, for the scene gap report: when each
/// came, its mode, the blank it came at and the one it returned at, and
/// what VSync(1) said on entry.
struct Wait {
    at: Instant,
    mode: i32,
    entered: u32,
    returned: u32,
    lines: i32,
}
static RECENT: Mutex<VecDeque<Wait>> = Mutex::new(VecDeque::new());

pub fn start() {
    let now = Instant::now();
    EPOCH.get_or_init(|| now);
    *TIMING.lock().unwrap() = Some(Timing { resumed_at: now, last_call: 0 });
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
            super::replay::vblank();
            super::stubs::refresh_pads();
            super::scan_images();
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
            if let Some(t) = TIMING.lock().unwrap().as_mut() {
                t.resumed_at = Instant::now();
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
    if super::trace("vsyncmode") {
        static LAST: AtomicU32 = AtomicU32::new(u32::MAX);
        if LAST.swap(mode as u32, Ordering::SeqCst) != mode as u32 {
            eprintln!("[vsync {:8.3}] waits with mode {mode}", super::spu::stamp());
        }
    }
    let last_call_at = last_call_at();
    let last_call = TIMING.lock().unwrap().as_ref().expect("vsync not started").last_call;
    let now = VBLANKS.load(Ordering::SeqCst);
    let target = if mode == 0 { now + 1 } else { last_call.wrapping_add(mode as u32).max(now + 1) };
    WAITS.fetch_add(1, Ordering::SeqCst);
    let entered = Instant::now();
    let lines_in = hsyncs_since(last_call_at);
    irq::wait_until(|| VBLANKS.load(Ordering::SeqCst).wrapping_sub(target) as i32 >= 0);
    if super::trace("vsyncwait") && mode <= 4 && entered.elapsed() > FIELD * 3 {
        eprintln!(
            "[vsync {:8.3}] VSync({mode}) waited {} ms for blank {target} (entered at {now}, back at {})",
            super::spu::stamp(),
            entered.elapsed().as_millis(),
            VBLANKS.load(Ordering::SeqCst)
        );
    }
    super::scan_images();
    if super::trace("spu") && last_call_at.elapsed() > FIELD * 6 {
        eprintln!("[spu {:8.3}] game frame took {} ms", super::spu::stamp(), last_call_at.elapsed().as_millis());
    }
    if super::trace("hang") && last_call_at.elapsed() > FIELD * 6 {
        super::profile::report(last_call_at);
    }
    let ret = hsyncs_since(last_call_at) & 0xFFFF;
    // Lines are counted from where the console's VSync returns: right after
    // the blank's interrupt handler, the game's callback included (the game
    // adds the callback's own time back itself). Not from when this thread
    // got going again, which can be milliseconds later (the port's handler
    // then hands the frame to the graphics card): counted from then, a
    // scene's paced wait (VSync(1)) runs past the next blank and every frame
    // takes one blank more. Nor from the blank itself: the callback's time
    // would count twice and paced scenes would run ahead of their sound.
    let blank_at = {
        let mut timing = TIMING.lock().unwrap();
        let t = timing.as_mut().unwrap();
        t.last_call = VBLANKS.load(Ordering::SeqCst);
        t.resumed_at
    };
    mark_call(blank_at);
    if super::trace("scene") {
        let mut recent = RECENT.lock().unwrap();
        if recent.len() == 16 {
            recent.pop_front();
        }
        recent.push_back(Wait { at: entered, mode, entered: now, returned: VBLANKS.load(Ordering::SeqCst), lines: lines_in });
    }
    ret
}

/// PE2_TRACE=scene: a paced scene's frame (gamemain.c in the fork): the
/// scanlines it has accumulated since its sound started, its deadline, and
/// what this frame measured.
#[unsafe(no_mangle)]
pub extern "C" fn pc_trace_scene_timing(accumulated: u32, deadline: u32, elapsed: i32) {
    if super::trace("scene") {
        // Not interrupted while writing: a handler writing too would wait
        // for the log forever.
        let _g = irq::guard();
        // A pause between two paced frames: the scene's clock does not count
        // it. How many blanks and waiting VSync calls it took, and (with
        // PE2_TRACE=hang) where the game spent it.
        static PREVIOUS: Mutex<Option<(Instant, u32, u32)>> = Mutex::new(None);
        let now = (Instant::now(), VBLANKS.load(Ordering::SeqCst), WAITS.load(Ordering::SeqCst));
        if let Some((at, blanks, waits)) = PREVIOUS.lock().unwrap().replace(now)
            && now.0 - at > FIELD * 5
        {
            eprintln!(
                "[scene {:8.3}] gap of {} ms: {} blanks, {} waiting VSync calls",
                super::spu::stamp(),
                (now.0 - at).as_millis(),
                now.1.wrapping_sub(blanks),
                now.2.wrapping_sub(waits)
            );
            for w in RECENT.lock().unwrap().iter().filter(|w| w.at >= at) {
                eprintln!(
                    "[scene]   +{:3} ms VSync({}) at blank {} after {} lines, back at blank {}",
                    (w.at - at).as_millis(),
                    w.mode,
                    w.entered,
                    w.lines,
                    w.returned
                );
            }
            if super::trace("hang") {
                super::profile::report(at);
            }
        }
        eprintln!(
            "[scene {:8.3}] {accumulated} lines ({:.3} s), deadline {deadline}, frame {elapsed}",
            super::spu::stamp(),
            f64::from(accumulated) / HSYNC_HZ
        );
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
