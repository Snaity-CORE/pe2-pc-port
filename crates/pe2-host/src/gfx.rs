//! The graphics card (wgpu): one device shared by the window, which
//! presents the picture, and the renderer.
//!
//! `[video] renderer = "software"` (or `PE2_RENDERER=software`) keeps
//! everything on the CPU, as does a failure to find a usable adapter.

use std::sync::OnceLock;

pub struct Gfx {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// Whether the hardware renderer is wanted (the default).
pub fn wanted() -> bool {
    let choice = std::env::var("PE2_RENDERER").ok().or_else(|| crate::config::get().video.renderer.clone());
    match choice.as_deref().map(str::trim) {
        None | Some("hardware") => true,
        Some("software") => false,
        Some(other) => {
            eprintln!("pe2: unknown renderer {other:?} (hardware or software); using hardware");
            true
        }
    }
}

static GFX: OnceLock<Option<Gfx>> = OnceLock::new();

/// The device, opened on first use; None with the software renderer or
/// when no adapter works.
pub fn get() -> Option<&'static Gfx> {
    GFX.get_or_init(|| if wanted() { open() } else { None }).as_ref()
}

fn open() -> Option<Gfx> {
    let instance = wgpu::Instance::default();
    let options =
        wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, ..Default::default() };
    let adapter = match pollster::block_on(instance.request_adapter(&options)) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("pe2: no graphics adapter ({err}); using the software renderer");
            return None;
        }
    };
    let info = adapter.get_info();
    let (device, queue) = match pollster::block_on(
        adapter.request_device(&wgpu::DeviceDescriptor { label: Some("pe2"), ..Default::default() }),
    ) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("pe2: {}: {err}; using the software renderer", info.name);
            return None;
        }
    };
    eprintln!("pe2: video: graphics card {} ({:?})", info.name, info.backend);
    // Report errors instead of the default panic: on the interrupt thread
    // a panic would stop the game.
    device.on_uncaptured_error(std::sync::Arc::new(|err| {
        static REPORTED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if REPORTED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 10 {
            // In full: its first line alone ("Validation Error") says little.
            let text = format!("{err:?}");
            eprintln!("pe2: graphics error: {text}");
        }
    }));
    Some(Gfx { instance, adapter, device, queue })
}
