//! The PSX runtime: the console services the game calls (`pcrt_*`), on top
//! of interrupt delivery to the game thread.

pub mod cdrom;
pub mod gpu;
pub mod hw;
pub mod irq;
pub mod libpress;
pub mod lowmem;
pub mod mdec;
pub mod memcard;
pub mod mips;
pub mod profile;
pub mod replay;
pub mod selftest;
pub mod spu;
pub mod stream;
pub mod stubs;
pub mod vsync;
pub mod watch;

unsafe extern "C" {
    fn pc_thunk_at(addr: u32) -> i32;
}

/// Calls code at a PSX address (a callback the game or a library
/// registered) with up to four word arguments. Game functions run natively
/// through their thunk; library code without one runs in the interpreter.
pub fn call_psx(addr: u32, args: &[u32]) -> u32 {
    let mut a = [0u32; 8];
    a[..args.len()].copy_from_slice(args);
    // SAFETY: plain query of the thunk bitmap.
    if unsafe { pc_thunk_at(addr) } != 0 {
        type Native = extern "C" fn(u32, u32, u32, u32) -> u32;
        // SAFETY: a thunk at `addr` jumps to a native body; extra cdecl
        // arguments are ignored by callees that take fewer.
        let f: Native = unsafe { std::mem::transmute(addr as usize) };
        f(a[0], a[1], a[2], a[3])
    } else {
        // SAFETY: `a` holds eight words; `addr` is library MIPS code.
        unsafe { mips::pc_mips_call(addr, a.as_ptr()) }
    }
}

/// True when `PE2_TRACE` (comma-separated) enables `topic`.
pub fn trace(topic: &str) -> bool {
    static TOPICS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    TOPICS
        .get_or_init(|| {
            std::env::var("PE2_TRACE").map(|v| v.split(',').map(|t| t.trim().to_string()).collect()).unwrap_or_default()
        })
        .iter()
        .any(|t| t == topic || t == "all")
}

unsafe extern "C" {
    fn pc_scan_images() -> i32;
    static mut pc_trace_images: i32;
}

/// Installs thunks for overlays the game has loaded since the last scan.
pub fn scan_images() {
    // SAFETY: runs on the game thread inside runtime code, or on the IRQ
    // thread while the game thread is suspended; never concurrently.
    unsafe { pc_scan_images() };
}

/// Whether PE2_TRACE names `name`, for traces in the fork's C code.
#[unsafe(no_mangle)]
pub extern "C" fn pc_trace_enabled(name: *const std::ffi::c_char) -> i32 {
    // SAFETY: the C side passes a NUL-terminated string literal.
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    name.to_str().map_or(0, |n| i32::from(trace(n)))
}

/// Starts the runtime on the game thread, before the game's main().
pub fn start(discs: Vec<Box<dyn crate::disc::Disc>>) {
    // SAFETY: written once before any other thread starts.
    unsafe { pc_trace_images = i32::from(trace("images")) };
    lowmem::install();
    watch::install_handler();
    watch::start_peek();
    irq::start();
    vsync::start();
    gpu::start();
    cdrom::start(discs);
}

/// C entry points for runtime code written in C (port/src) that must not be
/// interrupted, e.g. while it holds a CRT lock.
#[unsafe(no_mangle)]
pub extern "C" fn pc_irq_enter() {
    std::mem::forget(irq::guard());
}

#[unsafe(no_mangle)]
pub extern "C" fn pc_irq_leave() {
    drop(irq::Guard::forgotten());
}

#[repr(C)]
struct PcRuntimeFn {
    psx_addr: u32,
    implementation: usize,
    stub: usize,
    name: *const std::ffi::c_char,
}

unsafe extern "C" {
    static pc_runtime_fns: [PcRuntimeFn; 0];
    static pc_runtime_fn_count: u32;
}

/// PSX address of a main-image library routine, by name.
pub fn library_address(name: &str) -> Option<u32> {
    // SAFETY: generated, immutable table of pc_runtime_fn_count entries.
    let fns = unsafe { std::slice::from_raw_parts(pc_runtime_fns.as_ptr(), pc_runtime_fn_count as usize) };
    fns.iter()
        // SAFETY: names are NUL-terminated string literals.
        .find(|f| unsafe { std::ffi::CStr::from_ptr(f.name) }.to_bytes() == name.as_bytes())
        .map(|f| f.psx_addr)
}
