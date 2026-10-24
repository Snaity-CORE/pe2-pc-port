//! Interrupt delivery for the game thread.
//!
//! On the PlayStation, callbacks (VSync, CD data ready, command complete...)
//! run in interrupt context: the main program stops at an arbitrary point, the
//! handler runs, and the program resumes. The game is written for that, using
//! EnterCriticalSection where it must not be interrupted.
//!
//! This module reproduces it with an IRQ thread. Events are scheduled on a
//! timeline; when one is due, the IRQ thread suspends the game thread, checks
//! that interrupts are enabled, runs the handler on its own stack and resumes
//! the game thread. Interrupts are held off while:
//!
//! * the game has called EnterCriticalSection (`set_game_masked`), or
//! * the game thread is inside a runtime function (`Guard`), so it is never
//!   suspended while holding a runtime lock or a CRT lock.
//!
//! Runtime functions that block until an interrupt happens (VSync, CdSync)
//! use `wait_until`, which lets interrupts in while waiting.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::{DUPLICATE_SAME_ACCESS, DuplicateHandle, HANDLE};
use windows_sys::Win32::Media::timeBeginPeriod;
use windows_sys::Win32::System::Diagnostics::Debug::{CONTEXT, CONTEXT_CONTROL_X86, GetThreadContext};
use windows_sys::Win32::System::Threading::{
    CreateEventW, GetCurrentProcess, GetCurrentThread, GetCurrentThreadId, INFINITE, ResumeThread, SetEvent,
    SuspendThread, WaitForSingleObject,
};

/// Something that happens at a point in time on the IRQ thread.
pub type Handler = Box<dyn FnOnce() + Send>;

struct Scheduled {
    at: Instant,
    seq: u64,
    handler: Handler,
}

impl PartialEq for Scheduled {
    fn eq(&self, other: &Self) -> bool {
        (self.at, self.seq) == (other.at, other.seq)
    }
}
impl Eq for Scheduled {}
impl PartialOrd for Scheduled {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Scheduled {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.at, self.seq).cmp(&(other.at, other.seq))
    }
}

struct Irq {
    game_thread: SendHandle,
    queue: Mutex<BinaryHeap<Reverse<Scheduled>>>,
    /// Wakes the IRQ thread when an earlier event is scheduled.
    wake: SendHandle,
    /// Signalled after every delivered interrupt, for `wait_until`.
    delivered: SendHandle,
}

#[derive(Clone, Copy)]
struct SendHandle(HANDLE);
// SAFETY: Win32 handles may be used from any thread.
unsafe impl Send for SendHandle {}
unsafe impl Sync for SendHandle {}

static IRQ: OnceLock<Irq> = OnceLock::new();

unsafe extern "C" {
    fn pcgte_save(out: *mut u32);
    fn pcgte_restore(input: *const u32);
}
static GUARD_DEPTH: AtomicU32 = AtomicU32::new(0);
static GAME_MASKED: AtomicBool = AtomicBool::new(false);
static IN_HANDLER: AtomicBool = AtomicBool::new(false);
/// True while the game thread waits in `wait_until`.
static WAITING: AtomicBool = AtomicBool::new(false);
/// True while an interrupt is due but held off. Releasing the last guard
/// then gives way to it: a game polling a runtime function in a tight loop
/// is otherwise outside a guard only for a few instructions at a time, and
/// interrupts would starve.
static PENDING: AtomicBool = AtomicBool::new(false);
static SEQ: AtomicU64 = AtomicU64::new(0);

/// Starts interrupt delivery. Must be called on the game thread.
pub fn start() {
    // SAFETY: plain Win32 calls; the duplicated handle is a real handle to
    // the calling (game) thread, valid for the life of the process.
    let irq = unsafe {
        timeBeginPeriod(1);
        let mut game: HANDLE = std::ptr::null_mut();
        let ok = DuplicateHandle(
            GetCurrentProcess(),
            GetCurrentThread(),
            GetCurrentProcess(),
            &mut game,
            0,
            0,
            DUPLICATE_SAME_ACCESS,
        );
        assert!(ok != 0, "DuplicateHandle failed");
        GAME_THREAD_ID.store(GetCurrentThreadId(), Ordering::SeqCst);
        Irq {
            game_thread: SendHandle(game),
            queue: Mutex::new(BinaryHeap::new()),
            wake: SendHandle(CreateEventW(std::ptr::null(), 0, 0, std::ptr::null())),
            // Auto-reset: only the game thread waits on it.
            delivered: SendHandle(CreateEventW(std::ptr::null(), 0, 0, std::ptr::null())),
        }
    };
    let game_thread = irq.game_thread.0;
    if IRQ.set(irq).is_err() {
        return;
    }
    if super::trace("hang") || super::trace("prof") {
        super::profile::start(game_thread);
    }
    if super::trace("cpu") {
        super::profile::start_cpu_monitor();
    }
    let handle = std::thread::Builder::new()
        .name("psx-irq".into())
        .stack_size(4 << 20)
        .spawn(irq_thread)
        .expect("spawn IRQ thread");
    // Tools that keep using the thread's handle need it left open.
    use std::os::windows::io::AsRawHandle;
    let raw = handle.as_raw_handle() as HANDLE;
    if super::trace("prof") {
        super::profile::start_irq(raw);
    }
    if super::watch::enabled() {
        super::watch::arm(raw);
    }
    if super::trace("prof") || super::watch::enabled() {
        std::mem::forget(handle);
    }
}

/// Runs `handler` in interrupt context at `at`.
pub fn schedule(at: Instant, handler: Handler) {
    let irq = IRQ.get().expect("irq::start not called");
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let mut q = irq.queue.lock().unwrap();
    let earliest = q.peek().is_none_or(|Reverse(s)| at < s.at);
    q.push(Reverse(Scheduled { at, seq, handler }));
    drop(q);
    if earliest {
        // SAFETY: valid event handle.
        unsafe { SetEvent(irq.wake.0) };
    }
}

/// Held by runtime functions while they run on the game thread: interrupts
/// wait until it is dropped.
pub struct Guard(());

pub fn guard() -> Guard {
    GUARD_DEPTH.fetch_add(1, Ordering::SeqCst);
    Guard(())
}

impl Guard {
    /// Re-materializes a guard that was `mem::forget`-ten by `pc_irq_enter`,
    /// so dropping it releases that hold.
    pub fn forgotten() -> Guard {
        Guard(())
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if GUARD_DEPTH.fetch_sub(1, Ordering::SeqCst) == 1
            && PENDING.load(Ordering::SeqCst)
            && !in_handler()
            && !GAME_MASKED.load(Ordering::SeqCst)
        {
            let since = Instant::now();
            while PENDING.load(Ordering::SeqCst) && since.elapsed() < Duration::from_millis(2) {
                std::thread::yield_now();
            }
        }
    }
}

/// The game's EnterCriticalSection / ExitCriticalSection.
pub fn set_game_masked(masked: bool) -> bool {
    GAME_MASKED.swap(masked, Ordering::SeqCst)
}

/// True while an interrupt handler runs (callbacks must not block).
pub fn in_handler() -> bool {
    IN_HANDLER.load(Ordering::SeqCst)
}

/// The game thread's id (0 before `start`).
static GAME_THREAD_ID: AtomicU32 = AtomicU32::new(0);

/// The console's CPU would take `cycles` (at 33.8688 MHz) for library work
/// the port does at once: the game thread waits that long, with interrupts
/// coming meanwhile as they would on the console. Only where the game's
/// pacing counts on that time (see the callers); elsewhere (other threads,
/// interrupt handlers) it returns at once.
#[unsafe(no_mangle)]
pub extern "C" fn pc_console_cpu_time(cycles: u32) {
    // SAFETY: plain query.
    if cycles == 0 || unsafe { GetCurrentThreadId() } != GAME_THREAD_ID.load(Ordering::SeqCst) || in_handler() {
        return;
    }
    let until = Instant::now() + Duration::from_nanos(u64::from(cycles) * 1_000_000_000 / 33_868_800);
    let _g = guard();
    let from = Instant::now();
    wait_until(|| Instant::now() >= until);
    if super::trace("cputime") {
        eprintln!(
            "[cputime {:8.3}] {cycles} cycles: {} us, waited {} us",
            super::spu::stamp(),
            u64::from(cycles) * 1_000_000 / 33_868_800,
            from.elapsed().as_micros()
        );
    }
}

/// Blocks the game thread until `cond` holds, letting interrupts in while
/// waiting. Called with at least one `Guard` held; all holds are released
/// meanwhile and restored after.
#[track_caller]
pub fn wait_until(mut cond: impl FnMut() -> bool) {
    let irq = IRQ.get().expect("irq::start not called");
    if in_handler() {
        // A callback waiting for another interrupt would never return; the
        // console would hang the same way. Do not deadlock the IRQ thread.
        return;
    }
    let since = std::time::Instant::now();
    loop {
        if cond() {
            if super::trace("waits") && since.elapsed().as_millis() > 100 {
                eprintln!(
                    "[wait {:8.3}] {} ms in {}",
                    super::spu::stamp(),
                    since.elapsed().as_millis(),
                    std::panic::Location::caller()
                );
            }
            return;
        }
        // Release every hold on this thread (runtime calls can nest, e.g.
        // interpreted code calling game code that calls VSync); none of
        // them keeps a lock across a wait.
        let saved = GUARD_DEPTH.swap(0, Ordering::SeqCst);
        WAITING.store(true, Ordering::SeqCst);
        // SAFETY: valid event handle. The short timeout re-checks `cond`
        // even if a delivery's signal was consumed by an earlier wait.
        unsafe {
            WaitForSingleObject(irq.delivered.0, 2);
        }
        WAITING.store(false, Ordering::SeqCst);
        GUARD_DEPTH.fetch_add(saved, Ordering::SeqCst);
    }
}

/// PE2_SYNC_IRQ=1 (debugging): deliver interrupts only while the game
/// waits for one (VSync, CdSync...), never at an arbitrary instruction.
fn sync_irq() -> bool {
    static SYNC: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *SYNC.get_or_init(|| std::env::var_os("PE2_SYNC_IRQ").is_some())
}

fn interrupts_enabled() -> bool {
    GUARD_DEPTH.load(Ordering::SeqCst) == 0
        && !GAME_MASKED.load(Ordering::SeqCst)
        && (!sync_irq() || WAITING.load(Ordering::SeqCst))
}

/// The game is paused (P): its thread suspended and no interrupt delivered.
pub static PAUSED: AtomicBool = AtomicBool::new(false);

/// Pauses or resumes the game. The game thread is suspended outright; an
/// interrupt being delivered at that moment still finishes (Windows counts
/// suspensions, so the game stays stopped after the handler resumes it),
/// and no other is delivered until the game resumes. Overdue events then
/// run at once; the vertical blank resynchronizes by itself.
pub fn set_paused(on: bool) {
    let Some(irq) = IRQ.get() else { return };
    if PAUSED.swap(on, Ordering::SeqCst) == on {
        return;
    }
    // SAFETY: the game thread's handle, valid for the process's life.
    unsafe {
        if on {
            SuspendThread(irq.game_thread.0);
        } else {
            ResumeThread(irq.game_thread.0);
        }
    }
}

fn irq_thread() {
    let irq = IRQ.get().unwrap();
    super::watch::arm(irq.game_thread.0);
    loop {
        if PAUSED.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(10));
            continue;
        }
        let due = {
            let mut q = irq.queue.lock().unwrap();
            match q.peek() {
                Some(Reverse(s)) if s.at <= Instant::now() => q.pop().map(|Reverse(s)| s),
                Some(Reverse(s)) => {
                    let wait = s.at.saturating_duration_since(Instant::now());
                    drop(q);
                    // SAFETY: valid event handle.
                    unsafe { WaitForSingleObject(irq.wake.0, wait.as_millis().max(1) as u32) };
                    continue;
                }
                None => {
                    drop(q);
                    // SAFETY: valid event handle.
                    unsafe { WaitForSingleObject(irq.wake.0, INFINITE) };
                    continue;
                }
            }
        };
        if let Some(event) = due {
            deliver(irq, event.handler);
        }
    }
}

/// PE2_TRACE=cpu: interrupts delivered, and nanoseconds spent waiting for
/// the game to enable interrupts and running handlers.
pub static DELIVERED: AtomicU64 = AtomicU64::new(0);
pub static BLOCKED_NS: AtomicU64 = AtomicU64::new(0);
pub static HANDLER_NS: AtomicU64 = AtomicU64::new(0);

fn deliver(irq: &Irq, handler: Handler) {
    let game = irq.game_thread.0;
    PENDING.store(true, Ordering::SeqCst);
    let blocked_since = Instant::now();
    let mut tries = 0u32;
    loop {
        if !interrupts_enabled() {
            // Spin briefly (the game is usually just finishing a runtime
            // call), then back off.
            tries += 1;
            if tries < 1000 {
                std::thread::yield_now();
            } else {
                std::thread::sleep(Duration::from_micros(100));
            }
            continue;
        }
        // SAFETY: `game` is a valid thread handle. GetThreadContext after
        // SuspendThread waits until the suspension has taken effect.
        unsafe {
            SuspendThread(game);
            let mut ctx: CONTEXT = std::mem::zeroed();
            ctx.ContextFlags = CONTEXT_CONTROL_X86;
            GetThreadContext(game, &mut ctx);
        }
        if interrupts_enabled() {
            PENDING.store(false, Ordering::SeqCst);
            break;
        }
        // SAFETY: balances the SuspendThread above.
        unsafe { ResumeThread(game) };
    }

    let handler_since = Instant::now();
    BLOCKED_NS.fetch_add((handler_since - blocked_since).as_nanos() as u64, Ordering::Relaxed);
    IN_HANDLER.store(true, Ordering::SeqCst);
    // The game's interrupt callbacks must not touch the GTE (the console's
    // exception handler does not save it). Check, and keep the interrupted
    // code's GTE state intact either way.
    let mut before = [0u32; 64];
    // SAFETY: plain copies of the software GTE register file.
    unsafe { pcgte_save(before.as_mut_ptr()) };
    handler();
    let mut after = [0u32; 64];
    unsafe { pcgte_save(after.as_mut_ptr()) };
    if before != after {
        static REPORTED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if REPORTED.fetch_add(1, Ordering::Relaxed) < 5 {
            let regs: Vec<usize> = (0..64).filter(|&i| before[i] != after[i]).collect();
            eprintln!("[irq] interrupt handler changed GTE registers {regs:?}; restoring");
        }
        unsafe { pcgte_restore(before.as_ptr()) };
    }
    IN_HANDLER.store(false, Ordering::SeqCst);
    HANDLER_NS.fetch_add(handler_since.elapsed().as_nanos() as u64, Ordering::Relaxed);
    DELIVERED.fetch_add(1, Ordering::Relaxed);

    // SAFETY: balances the SuspendThread in the loop; valid event handle.
    unsafe {
        ResumeThread(game);
        SetEvent(irq.delivered.0);
    }
}
