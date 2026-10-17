//! Debugging aid: a sampling profiler for long game frames.
//!
//! With `PE2_TRACE=hang`, a thread samples the game thread's instruction
//! pointer (and a few return addresses found on its stack) every
//! millisecond. When a frame takes too long, `report` prints where the
//! game thread spent that time. Addresses are given for the preferred load
//! address 0x00400000; map them to functions with tools/symbolize.py
//! (--map target/i686-pc-windows-msvc/release/deps/pe2.map).
//!
//! With `PE2_TRACE=prof`, every sample also goes to pe2-prof.bin (eight
//! little-endian words: flags, eip, then the callers), for a whole-session
//! profile by function: tools/profile_report.py. Flags: bit 0 in an
//! interrupt handler, bit 1 eip outside the executable (then 0).
//!
//! With `PE2_TRACE=cpu`, every 10 seconds prints the CPU time each thread
//! of the process used (by thread name, or id for unnamed ones; the game
//! runs on the main thread).

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
        .spawn(move || sampler(thread, "pe2-prof.bin", true))
        .expect("spawn profiler");
}

/// PE2_TRACE=prof: also samples the interrupt thread, where the game's
/// vblank callback (and the drawing it does) runs, into pe2-prof-irq.bin.
pub fn start_irq(irq_thread: HANDLE) {
    let thread = SendHandle(irq_thread);
    std::thread::Builder::new()
        .name("profiler-irq".into())
        .spawn(move || sampler(thread, "pe2-prof-irq.bin", false))
        .expect("spawn profiler");
}

/// Samples `thread` into `file` (with PE2_TRACE=prof); `game` keeps the
/// recent samples for the long-frame reports.
fn sampler(thread: SendHandle, file: &'static str, game: bool) {
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
    let mut out = super::trace("prof")
        .then(|| std::io::BufWriter::new(std::fs::File::create(file).expect("create profile file")));
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
        if let Some(out) = out.as_mut() {
            use std::io::Write;
            let mut record = [0u32; 2 + CALLERS];
            let inside = text.contains(&ctx.Eip);
            record[0] = in_handler as u32 | (!inside as u32) << 1;
            record[1] = if inside { rebase(ctx.Eip) } else { 0 };
            record[2..].copy_from_slice(&callers);
            for w in record {
                let _ = out.write_all(&w.to_le_bytes());
            }
            let _ = out.flush();
        }
        if !game {
            continue;
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
        v.sort_by_key(|e| std::cmp::Reverse(e.1));
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

/// Starts the `PE2_TRACE=cpu` monitor.
pub fn start_cpu_monitor() {
    std::thread::Builder::new().name("cpu-monitor".into()).spawn(cpu_monitor).expect("spawn cpu monitor");
}

fn cpu_monitor() {
    use windows_sys::Win32::Foundation::{CloseHandle, FILETIME, LocalFree};
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
    };
    use windows_sys::Win32::System::Threading::{
        GetCurrentProcessId, GetThreadDescription, GetThreadTimes, OpenThread, THREAD_QUERY_LIMITED_INFORMATION,
    };

    const PERIOD: Duration = Duration::from_secs(10);
    let ticks = |t: FILETIME| (u64::from(t.dwHighDateTime) << 32) | u64::from(t.dwLowDateTime);
    let mut previous: HashMap<u32, u64> = HashMap::new();
    let mut last = Instant::now();
    loop {
        std::thread::sleep(PERIOD);
        let now = Instant::now();
        let wall = (now - last).as_secs_f64();
        last = now;
        let mut rows = Vec::new();
        let mut current = HashMap::new();
        // SAFETY: plain Win32 queries on this process's own threads; every
        // handle opened here is closed, and the description is freed.
        unsafe {
            let pid = GetCurrentProcessId();
            let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
            let mut entry: THREADENTRY32 = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;
            let mut more = Thread32First(snap, &mut entry) != 0;
            while more {
                if entry.th32OwnerProcessID == pid {
                    let tid = entry.th32ThreadID;
                    let h = OpenThread(THREAD_QUERY_LIMITED_INFORMATION, 0, tid);
                    if !h.is_null() {
                        let mut t: [FILETIME; 4] = std::mem::zeroed();
                        if GetThreadTimes(h, &mut t[0], &mut t[1], &mut t[2], &mut t[3]) != 0 {
                            let used = ticks(t[2]) + ticks(t[3]);
                            current.insert(tid, used);
                            let mut name = format!("tid {tid}");
                            let mut desc = std::ptr::null_mut();
                            if GetThreadDescription(h, &mut desc) >= 0 && !desc.is_null() {
                                let len = (0..).take_while(|&i| *desc.add(i) != 0).count();
                                let s = String::from_utf16_lossy(std::slice::from_raw_parts(desc, len));
                                if !s.is_empty() {
                                    name = s;
                                }
                                LocalFree(desc.cast());
                            }
                            let delta = used.saturating_sub(previous.get(&tid).copied().unwrap_or(used));
                            rows.push((delta, name));
                        }
                        CloseHandle(h);
                    }
                }
                more = Thread32Next(snap, &mut entry) != 0;
            }
            CloseHandle(snap);
        }
        previous = current;
        rows.sort_by_key(|r| std::cmp::Reverse(r.0));
        let total: u64 = rows.iter().map(|r| r.0).sum();
        let pct = |t: u64| 100.0 * t as f64 / 1e7 / wall;
        let list: Vec<String> =
            rows.iter().filter(|r| pct(r.0) >= 0.5).map(|(t, n)| format!("{n} {:.1}%", pct(*t))).collect();
        eprintln!("[cpu] {:.1}% of one core: {}", pct(total), list.join(", "));
        {
            use super::irq::{BLOCKED_NS, DELIVERED, HANDLER_NS};
            use std::sync::atomic::Ordering::Relaxed;
            let ms = |ns: u64| ns as f64 / 1e6 / wall;
            eprintln!(
                "[cpu]   interrupts: {:.0}/s, waiting for the game {:.2} ms/s, handlers {:.2} ms/s",
                DELIVERED.swap(0, Relaxed) as f64 / wall,
                ms(BLOCKED_NS.swap(0, Relaxed)),
                ms(HANDLER_NS.swap(0, Relaxed))
            );
            use super::gpu::hw::{DRAWS, FLUSHES, PASSES, VERTICES, VRAM_SENT};
            use super::gpu::{EXACT_VERTICES, POLY_VERTICES};
            let (all, exact) = (POLY_VERTICES.swap(0, Relaxed), EXACT_VERTICES.swap(0, Relaxed));
            if all > 0 {
                eprintln!(
                    "[cpu]   vertices: {:.0}/s, {:.1}% exact",
                    all as f64 / wall,
                    100.0 * exact as f64 / all as f64
                );
            }
            eprintln!(
                "[cpu]   graphics card: {:.0} flushes/s, {:.0} KB/s of VRAM sent, {:.0} passes/s, {:.0} draws/s, {:.0} vertices/s",
                FLUSHES.swap(0, Relaxed) as f64 / wall,
                VRAM_SENT.swap(0, Relaxed) as f64 * 2.0 / 1024.0 / wall,
                PASSES.swap(0, Relaxed) as f64 / wall,
                DRAWS.swap(0, Relaxed) as f64 / wall,
                VERTICES.swap(0, Relaxed) as f64 / wall
            );
        }
    }
}
