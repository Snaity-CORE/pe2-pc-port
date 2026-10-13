//! Debugging aid: a sampling profiler for long game frames.
//!
//! With `PE2_TRACE=hang`, a thread samples the game thread's instruction
//! pointer (and a few return addresses found on its stack) every
//! millisecond. When a frame takes too long, `report` prints where the
//! game thread spent that time. Addresses are given for the preferred load
//! address 0x00400000; map them to functions with tools/symbolize.py
//! (--map target/i686-pc-windows-msvc/release/deps/pe2.map).

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Diagnostics::Debug::{
    CONTEXT, CONTEXT_CONTROL_X86, GetThreadContext, ReadProcessMemory,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, ResumeThread, SuspendThread};

const PREFERRED_BASE: u32 = 0x0040_0000;
/// Code addresses found on the stack, nearest first.
const CALLERS: usize = 6;

struct Sample {
    at: Instant,
    in_handler: bool,
    eip: u32,
    callers: [u32; CALLERS],
}

static SAMPLES: Mutex<VecDeque<Sample>> = Mutex::new(VecDeque::new());

#[derive(Clone, Copy)]
struct SendHandle(HANDLE);
// SAFETY: Win32 handles may be used from any thread.
unsafe impl Send for SendHandle {}

/// Starts sampling `game_thread` (a real handle to it).
pub fn start(game_thread: HANDLE) {
    let thread = SendHandle(game_thread);
    std::thread::Builder::new()
        .name("profiler".into())
        .spawn(move || sampler(thread))
        .expect("spawn profiler");
}

fn sampler(thread: SendHandle) {
    let thread = thread;
    // SAFETY: the null module name is the executable itself.
    let base = unsafe { GetModuleHandleW(std::ptr::null()) } as u32;
    // The code section, from the PE header: BaseOfCode and SizeOfCode.
    // SAFETY: the executable's headers are mapped at its base address.
    let text = unsafe {
        let pe = base + *((base + 0x3C) as *const u32);
        let opt = pe + 0x18;
        let size = *((opt + 4) as *const u32);
        let start = base + *((opt + 20) as *const u32);
        start..start + size
    };
    let rebase = |a: u32| a.wrapping_sub(base).wrapping_add(PREFERRED_BASE);
    loop {
        std::thread::sleep(Duration::from_millis(1));
        let mut stack = [0u32; 512];
        let mut ctx: CONTEXT = unsafe { std::mem::zeroed() };
        ctx.ContextFlags = CONTEXT_CONTROL_X86;
        // SAFETY: suspends the game thread only for the duration of the
        // context and stack reads; ReadProcessMemory fails cleanly past the
        // end of the stack.
        let in_handler = unsafe {
            SuspendThread(thread.0);
            GetThreadContext(thread.0, &mut ctx);
            let mut read = 0usize;
            ReadProcessMemory(
                GetCurrentProcess(),
                ctx.Esp as usize as *const _,
                stack.as_mut_ptr().cast(),
                std::mem::size_of_val(&stack),
                &mut read,
            );
            let h = super::irq::in_handler();
            ResumeThread(thread.0);
            h
        };
        let mut callers = [0u32; CALLERS];
        let mut n = 0;
        for &w in &stack {
            if n == CALLERS {
                break;
            }
            if text.contains(&w) {
                callers[n] = rebase(w);
                n += 1;
            }
        }
        let now = Instant::now();
        let mut s = SAMPLES.lock().unwrap();
        s.push_back(Sample { at: now, in_handler, eip: rebase(ctx.Eip), callers });
        while s.front().is_some_and(|f| now - f.at > Duration::from_secs(10)) {
            s.pop_front();
        }
    }
}

/// Prints where the game thread spent its time since `since`.
pub fn report(since: Instant) {
    let s = SAMPLES.lock().unwrap();
    let window: Vec<&Sample> = s.iter().filter(|x| x.at >= since).collect();
    if window.is_empty() {
        return;
    }
    let total = window.len();
    let in_irq = window.iter().filter(|x| x.in_handler).count();
    eprintln!("[hang] {total} samples, {in_irq} in interrupt handlers");
    let top = |counts: HashMap<u32, usize>, what: &str| {
        let mut v: Vec<_> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        for (addr, n) in v.into_iter().take(12) {
            eprintln!("[hang]   {what} 0x{addr:08X} {:5.1}%", 100.0 * n as f64 / total as f64);
        }
    };
    let mut eips = HashMap::new();
    let mut callers = HashMap::new();
    for x in &window {
        *eips.entry(x.eip).or_insert(0) += 1;
        // Count each caller once per sample.
        let mut seen = x.callers.to_vec();
        seen.sort_unstable();
        seen.dedup();
        for c in seen.into_iter().filter(|&c| c != 0) {
            *callers.entry(c).or_insert(0) += 1;
        }
    }
    top(eips, "at");
    top(callers, "under");
}
