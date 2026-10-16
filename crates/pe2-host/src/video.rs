//! The host window: shows the GPU's display area and turns keyboard input
//! into controller state. It runs its own event loop on a separate thread;
//! the game keeps the main thread.

use std::num::NonZeroU32;
use std::rc::Rc;
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

fn key_to_button(key: KeyCode) -> Option<u16> {
    use button::*;
    Some(match key {
        KeyCode::ArrowUp => UP,
        KeyCode::ArrowDown => DOWN,
        KeyCode::ArrowLeft => LEFT,
        KeyCode::ArrowRight => RIGHT,
        KeyCode::KeyZ => CROSS,
        KeyCode::KeyX => CIRCLE,
        KeyCode::KeyA => SQUARE,
        KeyCode::KeyS => TRIANGLE,
        KeyCode::Enter => START,
        KeyCode::Backspace => SELECT,
        KeyCode::KeyQ => L1,
        KeyCode::KeyW => R1,
        KeyCode::KeyE => L2,
        KeyCode::KeyR => R2,
        _ => return None,
    })
}

const FRAME: Duration = Duration::from_micros(16_683);

/// True when PE2_INPUT scripts the controller (unattended runs).
fn scripted() -> bool {
    std::env::var_os("PE2_INPUT").is_some()
}

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    frame: Vec<u32>,
    /// Source column of each window column, for the current scale.
    columns: Vec<usize>,
    columns_for: (usize, usize),
    next_redraw: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Parasite Eve II")
            .with_inner_size(winit::dpi::LogicalSize::new(960, 720))
            // Scripted (unattended) runs must not steal focus from whatever
            // the user is typing in.
            .with_active(!scripted());
        let window = Rc::new(event_loop.create_window(attrs).expect("create window"));
        let context = softbuffer::Context::new(window.clone()).expect("softbuffer context");
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).expect("softbuffer surface"));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                std::process::exit(0);
            }
            // In scripted runs only PE2_INPUT drives the pad: stray keys
            // (even Esc) would change the run being measured.
            WindowEvent::KeyboardInput { .. } if scripted() => {}
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    if code == KeyCode::F12 && event.state == ElementState::Pressed && !event.repeat {
                        crate::runtime::gpu::DUMP_REQUEST.store(true, Ordering::SeqCst);
                    }
                    if code == KeyCode::F10 && event.state == ElementState::Pressed && !event.repeat {
                        crate::runtime::cdrom::SWAP_REQUEST.store(true, Ordering::SeqCst);
                    }
                    if code == KeyCode::F11 && event.state == ElementState::Pressed && !event.repeat {
                        eprintln!("[mark {:8.3}] F11 pressed", crate::runtime::spu::stamp());
                    }
                    if code == KeyCode::Escape {
                        std::process::exit(0);
                    }
                    if let Some(bit) = key_to_button(code) {
                        match event.state {
                            ElementState::Pressed => PAD1_BUTTONS.fetch_or(bit, Ordering::SeqCst),
                            ElementState::Released => PAD1_BUTTONS.fetch_and(!bit, Ordering::SeqCst),
                        };
                    }
                }
            }
            WindowEvent::RedrawRequested => self.present(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_redraw {
            if let Some(w) = &self.window {
                w.request_redraw();
            }
            self.next_redraw = now + FRAME;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_redraw));
    }
}

impl App {
    fn present(&mut self) {
        let (Some(window), Some(surface)) = (&self.window, &mut self.surface) else { return };
        let size = window.inner_size();
        let (Some(ww), Some(wh)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else { return };
        let Some((fw, fh)) = gpu::snapshot(&mut self.frame) else { return };
        if surface.resize(ww, wh).is_err() {
            return;
        }
        let Ok(mut buf) = surface.buffer_mut() else { return };
        let (ww, wh) = (ww.get() as usize, wh.get() as usize);

        // Fit a 4:3 picture into the window, letterboxed.
        let (pw, ph) = if ww * 3 > wh * 4 { (wh * 4 / 3, wh) } else { (ww, ww * 3 / 4) };
        let (ox, oy) = ((ww - pw) / 2, (wh - ph) / 2);
        if self.columns_for != (fw, pw) {
            self.columns = (0..pw).map(|x| x * fw / pw).collect();
            self.columns_for = (fw, pw);
        }
        buf[..oy * ww].fill(0);
        buf[(oy + ph) * ww..].fill(0);
        let mut previous_sy = usize::MAX;
        for y in 0..ph {
            let line = (oy + y) * ww;
            buf[line..line + ox].fill(0);
            buf[line + ox + pw..line + ww].fill(0);
            let sy = y * fh / ph;
            if sy == previous_sy {
                // Same source row as the line above.
                buf.copy_within(line - ww + ox..line - ww + ox + pw, line + ox);
                continue;
            }
            previous_sy = sy;
            let src = &self.frame[sy * fw..(sy + 1) * fw];
            for (px, &sx) in buf[line + ox..line + ox + pw].iter_mut().zip(&self.columns) {
                *px = src[sx];
            }
        }
        let _ = buf.present();
    }
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
        for n in 0.. {
            std::thread::sleep(every);
            let Some((w, h)) = gpu::snapshot(&mut frame) else { continue };
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
    let mut events: Vec<(f64, u16, bool)> = Vec::new();
    for item in script.split(',') {
        let Some((name, at)) = item.trim().split_once('@') else { continue };
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
            events.push((secs, bit, true));
            events.push((secs + hold, bit, false));
        }
    }
    events.sort_by(|a, b| a.0.total_cmp(&b.0));
    let t0 = Instant::now();
    std::thread::spawn(move || {
        for (secs, bit, press) in events {
            let at = t0 + Duration::from_secs_f64(secs);
            std::thread::sleep(at.saturating_duration_since(Instant::now()));
            if press {
                PAD1_BUTTONS.fetch_or(bit, Ordering::SeqCst);
            } else {
                PAD1_BUTTONS.fetch_and(!bit, Ordering::SeqCst);
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
            let mut app = App {
                window: None,
                surface: None,
                frame: Vec::new(),
                columns: Vec::new(),
                columns_for: (0, 0),
                next_redraw: Instant::now(),
            };
            let _ = event_loop.run_app(&mut app);
        })
        .expect("spawn window thread");
}
