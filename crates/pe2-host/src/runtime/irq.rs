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

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::{DUPLICATE_SAME_ACCESS, DuplicateHandle, HANDLE};
use windows_sys::Win32::Media::timeBeginPeriod;
use windows_sys::Win32::System::Diagnostics::Debug::{CONTEXT, CONTEXT_CONTROL_X86, GetThreadContext};
use windows_sys::Win32::System::Threading::{
    CreateEventW, GetCurrentProcess, GetCurrentThread, INFINITE, ResumeThread, SetEvent,
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
static GUARD_DEPTH: AtomicU32 = AtomicU32::new(0);
static GAME_MASKED: AtomicBool = AtomicBool::new(false);
static IN_HANDLER: AtomicBool = AtomicBool::new(false);
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
        Irq {
            game_thread: SendHandle(game),
            queue: Mutex::new(BinaryHeap::new()),
            wake: SendHandle(CreateEventW(std::ptr::null(), 0, 0, std::ptr::null())),
            // Auto-reset: only the game thread waits on it.
            delivered: SendHandle(CreateEventW(std::ptr::null(), 0, 0, std::ptr::null())),
        }
    };
    if IRQ.set(irq).is_err() {
        return;
    }
    std::thread::Builder::new()
        .name("psx-irq".into())
        .stack_size(4 << 20)
        .spawn(irq_thread)
        .expect("spawn IRQ thread");
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
        GUARD_DEPTH.fetch_sub(1, Ordering::SeqCst);
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

/// Blocks the game thread until `cond` holds, letting interrupts in while
/// waiting. Called with at least one `Guard` held; all holds are released
/// meanwhile and restored after.
pub fn wait_until(mut cond: impl FnMut() -> bool) {
    let irq = IRQ.get().expect("irq::start not called");
    if in_handler() {
        // A callback waiting for another interrupt would never return; the
        // console would hang the same way. Do not deadlock the IRQ thread.
        return;
    }
    loop {
        if cond() {
            return;
        }
        // Release every hold on this thread (runtime calls can nest, e.g.
        // interpreted code calling game code that calls VSync); none of
        // them keeps a lock across a wait.
        let saved = GUARD_DEPTH.swap(0, Ordering::SeqCst);
        // SAFETY: valid event handle. The short timeout re-checks `cond`
        // even if a delivery's signal was consumed by an earlier wait.
        unsafe {
            WaitForSingleObject(irq.delivered.0, 2);
        }
        GUARD_DEPTH.fetch_add(saved, Ordering::SeqCst);
    }
}

fn interrupts_enabled() -> bool {
    GUARD_DEPTH.load(Ordering::SeqCst) == 0 && !GAME_MASKED.load(Ordering::SeqCst)
}

fn irq_thread() {
    let irq = IRQ.get().unwrap();
    loop {
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

fn deliver(irq: &Irq, handler: Handler) {
    let game = irq.game_thread.0;
    loop {
        if !interrupts_enabled() {
            std::thread::sleep(Duration::from_micros(100));
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
            break;
        }
        // SAFETY: balances the SuspendThread above.
        unsafe { ResumeThread(game) };
    }

    IN_HANDLER.store(true, Ordering::SeqCst);
    handler();
    IN_HANDLER.store(false, Ordering::SeqCst);

    // SAFETY: balances the SuspendThread in the loop; valid event handle.
    unsafe {
        ResumeThread(game);
        SetEvent(irq.delivered.0);
    }
}
