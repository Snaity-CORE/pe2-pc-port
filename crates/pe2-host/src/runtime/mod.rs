//! The PSX runtime: the console services the game calls (`pcrt_*`), on top
//! of interrupt delivery to the game thread.

pub mod cdrom;
pub mod irq;
pub mod lowmem;
pub mod vsync;

/// Calls a game callback given as a PSX function address. The address holds
/// a thunk to the native body once its image is loaded.
///
/// # Safety
/// `addr` must be the address of a loaded game function with this signature.
pub unsafe fn psx_fn<F: Copy>(addr: u32) -> F {
    debug_assert_eq!(std::mem::size_of::<F>(), std::mem::size_of::<usize>());
    // SAFETY: function pointers and usize have the same size on this target.
    unsafe { std::mem::transmute_copy(&(addr as usize)) }
}

/// True when `PE2_TRACE` (comma-separated) enables `topic`.
pub fn trace(topic: &str) -> bool {
    static TOPICS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    TOPICS
        .get_or_init(|| {
            std::env::var("PE2_TRACE")
                .map(|v| v.split(',').map(|t| t.trim().to_string()).collect())
                .unwrap_or_default()
        })
        .iter()
        .any(|t| t == topic || t == "all")
}

/// Starts the runtime on the game thread, before the game's main().
pub fn start(discs: Vec<Box<dyn crate::disc::Disc>>) {
    lowmem::install();
    irq::start();
    vsync::start();
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
