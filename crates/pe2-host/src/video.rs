//! The host window: shows the GPU's display area and turns keyboard input
//! into controller state. It runs its own event loop on a separate thread;
//! the game keeps the main thread.

use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Window, WindowId};

use crate::runtime::gpu;

/// Buttons held on controller 1, PSX bit layout (1 = pressed).
pub static PAD1_BUTTONS: AtomicU16 = AtomicU16::new(0);

pub mod button {
    pub const SELECT: u16 = 1 << 0;
    pub const L3: u16 = 1 << 1;
    pub const R3: u16 = 1 << 2;
    pub const START: u16 = 1 << 3;
    pub const UP: u16 = 1 << 4;
    pub const RIGHT: u16 = 1 << 5;
    pub const DOWN: u16 = 1 << 6;
    pub const LEFT: u16 = 1 << 7;
    pub const L2: u16 = 1 << 8;
    pub const R2: u16 = 1 << 9;
    pub const L1: u16 = 1 << 10;
    pub const R1: u16 = 1 << 11;
    pub const TRIANGLE: u16 = 1 << 12;
    pub const CIRCLE: u16 = 1 << 13;
    pub const CROSS: u16 = 1 << 14;
    pub const SQUARE: u16 = 1 << 15;
}

/// The buttons a key presses ([input.keyboard] in pe2.toml; see
/// controls.rs).
fn key_to_button(key: KeyCode) -> Option<u16> {
    static MAP: std::sync::OnceLock<Vec<(KeyCode, u16)>> = std::sync::OnceLock::new();
    let bits = MAP.get_or_init(crate::controls::keyboard).iter().filter(|(k, _)| *k == key).fold(0, |a, (_, b)| a | b);
    (bits != 0).then_some(bits)
}

/// Redraws without a vertical blank asking for one (paused, or before the
/// game starts): often enough for resizing and the title.
const FRAME: Duration = Duration::from_millis(100);

/// Wakes the window's event loop from other threads.
static PROXY: std::sync::OnceLock<std::sync::Mutex<winit::event_loop::EventLoopProxy<()>>> = std::sync::OnceLock::new();

/// Each vertical blank, after the picture is latched: the window shows it
/// once, at the blank's pace. Drawing on a timer of its own instead drifts
/// against the blanks, showing some pictures twice and skipping others,
/// which shows at 60 distinct pictures a second (frame interpolation) as
/// hitches and jumps.
pub fn vblank() {
    if let Some(proxy) = PROXY.get() {
        let _ = proxy.lock().unwrap().send_event(());
    }
}

/// True when PE2_INPUT scripts the controller (unattended runs).
fn scripted() -> bool {
    std::env::var_os("PE2_INPUT").is_some()
}

struct App {
    window: Option<Arc<Window>>,
    /// What draws the picture into the window, on the graphics card.
    hardware: Option<crate::present::Presenter>,
    next_redraw: Instant,
    /// Alt is held (Alt+Enter toggles full screen).
    alt: bool,
    /// The title's frame rate: when it was last updated, and the frame
    /// count then.
    title_at: Instant,
    title_frames: u64,
}

/// Borderless full screen on the window's monitor, or a window.
fn fullscreen_mode(on: bool) -> Option<winit::window::Fullscreen> {
    on.then_some(winit::window::Fullscreen::Borderless(None))
}

/// Full screen on or off, hiding the mouse cursor in it unless
/// `hide_cursor = false`.
fn set_fullscreen(w: &Window, on: bool) {
    w.set_fullscreen(fullscreen_mode(on));
    w.set_cursor_visible(!(on && crate::config::get().video.hide_cursor.unwrap_or(true)));
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Parasite Eve II")
            .with_window_icon(
                crate::icon::rgba().and_then(|(px, side)| winit::window::Icon::from_rgba(px, side, side).ok()),
            )
            .with_inner_size(winit::dpi::LogicalSize::new(960, 720))
            // Scripted (unattended) runs must not steal focus from whatever
            // the user is typing in.
            .with_active(!scripted());
        let window = Arc::new(event_loop.create_window(attrs).expect("create window"));
        set_fullscreen(&window, crate::config::get().video.fullscreen.unwrap_or(false) && !scripted());
        self.hardware = crate::gfx::get().and_then(|gfx| crate::present::Presenter::new(gfx, window.clone()));
        if self.hardware.is_none() {
            eprintln!("pe2: video: cannot draw into the window");
        }
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                crate::logfile::exit(0);
            }
            // In scripted runs only PE2_INPUT drives the pad: stray keys
            // (even Esc) would change the run being measured.
            WindowEvent::KeyboardInput { .. } if scripted() => {}
            WindowEvent::ModifiersChanged(modifiers) => self.alt = modifiers.state().alt_key(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    // Alt+Enter: full screen on or off (Enter is also Start:
                    // not passed to the pad while Alt is held).
                    if self.alt && matches!(code, KeyCode::Enter | KeyCode::NumpadEnter) {
                        if event.state == ElementState::Pressed
                            && !event.repeat
                            && let Some(w) = &self.window
                        {
                            set_fullscreen(w, w.fullscreen().is_none());
                        }
                        return;
                    }
                    if code == KeyCode::F12 && event.state == ElementState::Pressed && !event.repeat {
                        crate::runtime::gpu::DUMP_REQUEST.store(true, Ordering::SeqCst);
                    }
                    if code == KeyCode::F10 && event.state == ElementState::Pressed && !event.repeat {
                        crate::runtime::cdrom::SWAP_REQUEST.store(true, Ordering::SeqCst);
                    }
                    if code == KeyCode::F9 && event.state == ElementState::Pressed && !event.repeat {
                        save_screenshot();
                    }
                    if code == KeyCode::KeyP && event.state == ElementState::Pressed && !event.repeat {
                        crate::runtime::irq::set_paused(!crate::runtime::irq::PAUSED.load(Ordering::SeqCst));
                        self.update_title(Instant::now());
                    }
                    if code == KeyCode::F11 && event.state == ElementState::Pressed && !event.repeat {
                        eprintln!("[mark {:8.3}] F11 pressed", crate::runtime::spu::stamp());
                    }
                    if code == KeyCode::Escape {
                        if event.state == ElementState::Pressed && !event.repeat && self.confirm_quit() {
                            crate::logfile::exit(0);
                        }
                        return;
                    }
                    if let Some(bit) = key_to_button(code) {
                        match event.state {
                            ElementState::Pressed => PAD1_BUTTONS.fetch_or(bit, Ordering::SeqCst),
                            ElementState::Released => PAD1_BUTTONS.fetch_and(!bit, Ordering::SeqCst),
                        };
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(hw) = self.hardware.as_mut() {
                    hw.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(hw) = self.hardware.as_mut() {
                    hw.present();
                }
            }
            _ => {}
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: ()) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
        present_trace();
        self.next_redraw = Instant::now() + FRAME;
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_redraw {
            if let Some(w) = &self.window {
                w.request_redraw();
            }
            self.next_redraw = now + FRAME;
        }
        if crate::runtime::replay::take_finished() && std::env::var("PE2_REPLAY_PAUSE").as_deref() != Ok("0") {
            // Where the recording ended: held there, with a picture of it.
            eprintln!("pe2: replay: paused (P resumes, with the controls)");
            crate::runtime::irq::set_paused(true);
            save_screenshot();
            self.update_title(now);
        }
        if now.duration_since(self.title_at) >= Duration::from_secs(1) {
            self.update_title(now);
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_redraw));
    }
}

/// PE2_TRACE=present: every 5 s, the vertical blanks the window was woken
/// for and those that came more than 25 ms after the previous one (hitches).
fn present_trace() {
    if !crate::runtime::trace("present") {
        return;
    }
    /// Since when, the last blank, blanks, late ones and the longest gap.
    type Stats = (Instant, Instant, u32, u32, Duration);
    static STATE: std::sync::Mutex<Option<Stats>> = std::sync::Mutex::new(None);
    let now = Instant::now();
    let mut state = STATE.lock().unwrap();
    let (since, last, count, late, worst) = state.get_or_insert((now, now, 0, 0, Duration::ZERO));
    let gap = now - *last;
    *last = now;
    *count += 1;
    if gap > Duration::from_millis(25) {
        *late += 1;
    }
    *worst = (*worst).max(gap);
    if now - *since >= Duration::from_secs(5) {
        eprintln!(
            "[present] {count} blanks in {:.1} s, {late} late, longest gap {:.1} ms",
            (now - *since).as_secs_f64(),
            worst.as_secs_f64() * 1000.0
        );
        *state = Some((now, now, 0, 0, Duration::ZERO));
    }
}

impl App {
    /// Esc: asks before quitting ([general] confirm_exit, default on). Cancel
    /// is the default button, so Enter or a second Esc keep playing.
    fn confirm_quit(&self) -> bool {
        if !crate::config::get().general.confirm_exit.unwrap_or(true) {
            return true;
        }
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            IDOK, MB_DEFBUTTON2, MB_ICONQUESTION, MB_OKCANCEL, MB_SETFOREGROUND, MessageBoxW,
        };
        use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let hwnd = self
            .window
            .as_ref()
            .and_then(|w| w.window_handle().ok())
            .and_then(|h| match h.as_raw() {
                RawWindowHandle::Win32(h) => Some(h.hwnd.get() as *mut core::ffi::c_void),
                _ => None,
            })
            .unwrap_or(std::ptr::null_mut());
        let wide = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
        let (text, caption) = (wide("Quit Parasite Eve II?"), wide("Parasite Eve II"));
        // SAFETY: valid NUL-terminated strings and this thread's own window.
        let answer = unsafe {
            MessageBoxW(
                hwnd,
                text.as_ptr(),
                caption.as_ptr(),
                MB_OKCANCEL | MB_ICONQUESTION | MB_DEFBUTTON2 | MB_SETFOREGROUND,
            )
        };
        // Keys released while the box was open never reached the window.
        PAD1_BUTTONS.store(0, Ordering::SeqCst);
        answer == IDOK
    }

    /// "Parasite Eve II — 30 fps — 1280×960 — Vulkan": frames the game drew
    /// in the last second, the picture's size and the renderer.
    fn update_title(&mut self, now: Instant) {
        let frames = gpu::FRAMES.load(Ordering::Relaxed);
        let fps = (frames - self.title_frames) as f64 / now.duration_since(self.title_at).as_secs_f64();
        (self.title_at, self.title_frames) = (now, frames);
        let Some(window) = &self.window else { return };
        let (w, h) = gpu::picture_size();
        let renderer =
            crate::gfx::get().map_or_else(String::new, |gfx| format!("{:?}", gfx.adapter.get_info().backend));
        if crate::runtime::irq::PAUSED.load(Ordering::SeqCst) {
            window.set_title(&format!("Parasite Eve II — Paused (P) — {w}×{h} — {renderer}"));
        } else {
            window.set_title(&format!("Parasite Eve II — {fps:.0} fps — {w}×{h} — {renderer}"));
        }
    }
}

/// Writes 0RGB pixels as a PNG.
fn write_png(path: &std::path::Path, w: usize, h: usize, frame: &[u32]) -> std::io::Result<()> {
    let rgb: Vec<u8> = frame.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8]).collect();
    let file = std::fs::File::create(path)?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().map_err(std::io::Error::other)?;
    writer.write_image_data(&rgb).map_err(std::io::Error::other)
}

/// F9: saves the picture, at the internal resolution, as
/// screenshots/pe2_YYYY-MM-DD_HH-MM-SS.png in the folder of the player's
/// files (paths.rs; on a thread:
/// reading it back from the graphics card waits for it).
fn save_screenshot() {
    std::thread::spawn(|| {
        let mut frame = Vec::new();
        let Some((w, h)) = gpu::snapshot(&mut frame) else { return };
        let dir = crate::paths::user_file("screenshots");
        let _ = std::fs::create_dir_all(&dir);
        // SAFETY: fills a plain struct.
        let t = unsafe {
            let mut t = std::mem::zeroed();
            windows_sys::Win32::System::SystemInformation::GetLocalTime(&mut t);
            t
        };
        let stem = format!(
            "pe2_{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
            t.wYear, t.wMonth, t.wDay, t.wHour, t.wMinute, t.wSecond
        );
        let mut path = dir.join(format!("{stem}.png"));
        for n in 2.. {
            if !path.exists() {
                break;
            }
            path = dir.join(format!("{stem}_{n}.png"));
        }
        match write_png(&path, w, h, &frame) {
            Ok(()) => eprintln!("pe2: screenshot {}", path.display()),
            Err(e) => eprintln!("pe2: screenshot {}: {e}", path.display()),
        }
    });
}

/// With `PE2_SCREENSHOT=<dir>`, saves the displayed picture as
/// `<dir>/frame_NNN.png` every two seconds (debugging aid).
/// `PE2_SCREENSHOT_EVERY=<ms>` changes the interval and
/// `PE2_SCREENSHOT_AFTER=<s>` delays the first one (bursts, for flicker).
fn start_screenshots() {
    let Some(dir) = std::env::var_os("PE2_SCREENSHOT") else { return };
    let dir = std::path::PathBuf::from(dir);
    let _ = std::fs::create_dir_all(&dir);
    let number = |name: &str| std::env::var(name).ok().and_then(|v| v.trim().parse::<f64>().ok());
    let every = Duration::from_secs_f64(number("PE2_SCREENSHOT_EVERY").map_or(2.0, |ms| ms / 1000.0));
    let after = Duration::from_secs_f64(number("PE2_SCREENSHOT_AFTER").unwrap_or(0.0));
    std::thread::spawn(move || {
        let mut frame = Vec::new();
        std::thread::sleep(after);
        for n in 0..u32::MAX {
            std::thread::sleep(every);
            let Some((w, h)) = gpu::snapshot(&mut frame) else { continue };
            // When it was taken, on the time base of the sound trace.
            eprintln!("[shot {:8.3}] frame_{n:03}", crate::runtime::spu::stamp());
            let rgb: Vec<u8> = frame.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8]).collect();
            let path = dir.join(format!("frame_{n:03}.png"));
            let Ok(file) = std::fs::File::create(&path) else { continue };
            let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
            enc.set_color(png::ColorType::Rgb);
            enc.set_depth(png::BitDepth::Eight);
            if let Ok(mut writer) = enc.write_header() {
                let _ = writer.write_image_data(&rgb);
            }
        }
    });
}

/// With `PE2_INPUT=button@seconds,...` (e.g. `start@30,cross@45.5`),
/// presses each button for 200 ms at that time after start (debugging aid
/// for unattended runs). Buttons: up down left right cross circle square
/// triangle start select l1 r1 l2 r2.
fn start_input_script() {
    let Ok(script) = std::env::var("PE2_INPUT") else { return };
    /// Scripted actions: buttons pressed and released, and the P and F9
    /// keys (`pause@t` toggles pause, `screenshot@t` saves one).
    enum Action {
        Press(u16),
        Release(u16),
        Pause,
        Screenshot,
    }
    let mut events: Vec<(f64, Action)> = Vec::new();
    for item in script.split(',') {
        let Some((name, at)) = item.trim().split_once('@') else { continue };
        let key = match name.to_ascii_lowercase().as_str() {
            "pause" => Some(Action::Pause),
            "screenshot" => Some(Action::Screenshot),
            _ => None,
        };
        if let Some(action) = key {
            if let Ok(secs) = at.parse::<f64>() {
                events.push((secs, action));
            }
            continue;
        }
        let bit = match name.to_ascii_lowercase().as_str() {
            "up" => button::UP,
            "down" => button::DOWN,
            "left" => button::LEFT,
            "right" => button::RIGHT,
            "cross" => button::CROSS,
            "circle" => button::CIRCLE,
            "square" => button::SQUARE,
            "triangle" => button::TRIANGLE,
            "start" => button::START,
            "select" => button::SELECT,
            "l1" => button::L1,
            "r1" => button::R1,
            "l2" => button::L2,
            "r2" => button::R2,
            other => {
                eprintln!("pe2: PE2_INPUT: unknown button {other}");
                continue;
            }
        };
        // `button@start` taps for 200 ms; `button@start+seconds` holds.
        let (start, hold) = match at.split_once('+') {
            Some((a, h)) => (a.parse::<f64>(), h.parse::<f64>().unwrap_or(0.2)),
            None => (at.parse::<f64>(), 0.2),
        };
        if let Ok(secs) = start {
            events.push((secs, Action::Press(bit)));
            events.push((secs + hold, Action::Release(bit)));
        }
    }
    events.sort_by(|a, b| a.0.total_cmp(&b.0));
    let t0 = Instant::now();
    std::thread::spawn(move || {
        for (secs, action) in events {
            let at = t0 + Duration::from_secs_f64(secs);
            std::thread::sleep(at.saturating_duration_since(Instant::now()));
            match action {
                Action::Press(bit) => _ = PAD1_BUTTONS.fetch_or(bit, Ordering::SeqCst),
                Action::Release(bit) => _ = PAD1_BUTTONS.fetch_and(!bit, Ordering::SeqCst),
                Action::Pause => crate::runtime::irq::set_paused(!crate::runtime::irq::PAUSED.load(Ordering::SeqCst)),
                Action::Screenshot => save_screenshot(),
            }
        }
    });
}

/// Opens the window on its own thread.
pub fn start() {
    start_screenshots();
    start_input_script();
    std::thread::Builder::new()
        .name("window".into())
        .spawn(|| {
            let event_loop = EventLoop::builder().with_any_thread(true).build().expect("event loop");
            let _ = PROXY.set(std::sync::Mutex::new(event_loop.create_proxy()));
            let mut app = App {
                window: None,
                hardware: None,
                alt: false,
                title_at: Instant::now(),
                title_frames: 0,
                next_redraw: Instant::now(),
            };
            let _ = event_loop.run_app(&mut app);
        })
        .expect("spawn window thread");
}
