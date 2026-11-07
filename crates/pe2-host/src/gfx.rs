//! The graphics card (wgpu): one device shared by the window, which
//! presents the picture, and the renderer, through Vulkan or OpenGL
//! ([video] graphics_api).
//!
//! Not Direct3D 12: in a 32-bit process wgpu's pipeline descriptions are
//! laid out with 8-byte alignment where Direct3D reads 4-byte, and every
//! pipeline fails ("invalid parameter"). With it went the system's
//! software adapter (WARP).

use std::sync::OnceLock;

pub struct Gfx {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

static GFX: OnceLock<Option<Gfx>> = OnceLock::new();

/// The device, opened on first use; None when no adapter works.
pub fn get() -> Option<&'static Gfx> {
    GFX.get_or_init(open).as_ref()
}

/// The APIs [video] graphics_api can name, with their wgpu backends.
pub const APIS: [(&str, wgpu::Backends); 2] = [("vulkan", wgpu::Backends::VULKAN), ("opengl", wgpu::Backends::GL)];

/// Both, Vulkan first ("auto").
const AUTO: wgpu::Backends = wgpu::Backends::VULKAN.union(wgpu::Backends::GL);

/// The backends [video] graphics_api allows.
fn backends() -> wgpu::Backends {
    let name = crate::config::get().video.graphics_api.as_deref().unwrap_or("auto");
    match APIS.iter().find(|(n, _)| *n == name) {
        Some(&(_, b)) => b,
        None => {
            if name != "auto" {
                eprintln!("pe2: video: unknown graphics_api \"{name}\" (auto, vulkan or opengl); using auto");
            }
            AUTO
        }
    }
}

/// Left in the data folder by a crash while Vulkan was opening (crash.c),
/// naming the module it happened in: with graphics_api "auto", the next
/// starts use OpenGL. A driver or an overlay that hooks into Vulkan (a
/// recorder's, a statistics display's) can crash there, before anything
/// shows; the game then started only once the player had found the setting.
const VULKAN_CRASHED: &str = "vulkan-crashed.txt";

unsafe extern "C" {
    static mut pc_gfx_crash_marker: [u16; 260];
}

/// While Vulkan opens (until the window first shows a picture): a crash
/// leaves VULKAN_CRASHED.
fn watch_crash(on: bool) {
    use std::os::windows::ffi::OsStrExt;
    let path: Vec<u16> = crate::paths::user_file(VULKAN_CRASHED).as_os_str().encode_wide().collect();
    // SAFETY: read by the crash handler only; written here, before and after.
    let marker = unsafe { &mut *std::ptr::addr_of_mut!(pc_gfx_crash_marker) };
    marker.fill(0);
    if on && path.len() < marker.len() {
        marker[..path.len()].copy_from_slice(&path);
    }
}

/// The window showed its first picture: Vulkan opened fine.
pub fn started() {
    watch_crash(false);
    if get().is_some_and(|g| g.adapter.get_info().backend == wgpu::Backend::Vulkan) {
        let _ = std::fs::remove_file(crate::paths::user_file(VULKAN_CRASHED));
    }
}

fn instance(backends: wgpu::Backends) -> wgpu::Instance {
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = backends;
    wgpu::Instance::new(desc)
}

fn open() -> Option<Gfx> {
    let mut chosen = backends();
    if chosen == AUTO
        && let Ok(module) = std::fs::read_to_string(crate::paths::user_file(VULKAN_CRASHED))
    {
        eprintln!(
            "pe2: video: Vulkan crashed while starting last time (in {}); using OpenGL. Set graphics_api = \"vulkan\" in pe2.toml to try it again.",
            module.trim()
        );
        chosen = wgpu::Backends::GL;
    }
    if chosen.contains(wgpu::Backends::VULKAN) {
        watch_crash(true);
    }
    let mut instance = instance(chosen);
    let options = |fallback| wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: fallback,
        ..Default::default()
    };
    let mut found = pollster::block_on(instance.request_adapter(&options(false)));
    if let Err(err) = &found
        && chosen != AUTO
    {
        // The chosen API is missing (no driver for it): the other one.
        eprintln!("pe2: video: the chosen graphics API is not available ({err}); using auto");
        instance = self::instance(AUTO);
        found = pollster::block_on(instance.request_adapter(&options(false)));
    }
    let adapter = match found {
        Ok(a) => a,
        Err(err) => match pollster::block_on(instance.request_adapter(&options(true))) {
            Ok(a) => {
                eprintln!("pe2: no graphics adapter ({err}); drawing with the software adapter");
                a
            }
            Err(err) => {
                eprintln!("pe2: no graphics adapter ({err})");
                return None;
            }
        },
    };
    let info = adapter.get_info();
    let (device, queue) = match pollster::block_on(
        adapter.request_device(&wgpu::DeviceDescriptor { label: Some("pe2"), ..Default::default() }),
    ) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("pe2: {}: {err}", info.name);
            return None;
        }
    };
    eprintln!("pe2: video: graphics card {} ({:?})", info.name, info.backend);
    if info.backend != wgpu::Backend::Vulkan {
        watch_crash(false);
    }
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
