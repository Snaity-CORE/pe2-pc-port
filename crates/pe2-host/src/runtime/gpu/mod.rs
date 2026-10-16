//! The GPU: GP0 command processing on a software VRAM, the libgpu calls that
//! drive it (ordering tables, image transfers, drawing/display
//! environments), and the display state the host window presents.
//!
//! Drawing is synchronous: a command list sent with DrawOTag is fully drawn
//! when the call returns, so DrawSync never has to wait.
//!
//! With `PE2_SCALE=2..8`, everything is also drawn into a higher-resolution
//! true-colour layer (hires.rs), which the window shows instead.

pub mod hires;
pub mod raster;

use std::sync::Mutex;

use raster::{DrawState, Shading, VRAM_HEIGHT, VRAM_WIDTH, Vertex, Vram};

use super::{irq, trace};

/// What the TV shows: a region of VRAM.
#[derive(Clone, Copy, Debug)]
pub struct Display {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rgb24: bool,
    pub enabled: bool,
}

pub struct Gpu {
    pub vram: Vram,
    pub state: DrawState,
    pub display: Display,
    /// Incremented by each command list, so the presenter can tell new
    /// frames from repeats.
    pub frame: u64,
    /// PSX address of the packet being run (for traces).
    pub cur_packet: u32,
    /// Log every command of the current list (PE2_GPUDUMP or F12).
    pub dump: bool,
    dump_out: Option<std::io::BufWriter<std::fs::File>>,
    /// The higher-resolution layer, when PE2_SCALE asks for one.
    pub hires: Option<hires::Hires>,
}

/// The internal resolution, a multiple of the console's (1-8): PE2_SCALE,
/// else pe2.toml's [video] scale, else 1.
fn scale() -> i32 {
    std::env::var("PE2_SCALE")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .or(crate::config::get().video.scale)
        .unwrap_or(1)
        .clamp(1, 8)
}

pub static GPU: Mutex<Option<Gpu>> = Mutex::new(None);

pub fn start() {
    let scale = scale();
    if scale > 1 {
        eprintln!("pe2: internal resolution {scale}x ({}x{})", 320 * scale, 240 * scale);
    }
    *GPU.lock().unwrap() = Some(Gpu {
        vram: Vram::default(),
        state: DrawState::default(),
        display: Display { x: 0, y: 0, width: 320, height: 240, rgb24: false, enabled: false },
        frame: 0,
        cur_packet: 0,
        dump: false,
        dump_out: None,
        hires: (scale > 1).then(|| hires::Hires::new(scale)),
    });
}

fn with_gpu<R>(f: impl FnOnce(&mut Gpu) -> R) -> R {
    let mut g = GPU.lock().unwrap();
    f(g.as_mut().expect("gpu not started"))
}

/// A PSX 24-bit bus address (as stored in ordering-table links) as a host
/// pointer into KSEG0.
fn link_ptr(addr24: u32) -> *const u32 {
    (0x8000_0000 | (addr24 & 0x00FF_FFFF)) as *const u32
}

fn sext11(v: u32) -> i32 {
    ((v << 21) as i32) >> 21
}

fn color(word: u32) -> (i32, i32, i32) {
    ((word & 0xFF) as i32, ((word >> 8) & 0xFF) as i32, ((word >> 16) & 0xFF) as i32)
}

fn xy(word: u32) -> (i32, i32) {
    (sext11(word), sext11(word >> 16))
}

impl Gpu {
    /// Executes one GP0 command; returns how many words it used (at least 1).
    fn command(&mut self, words: &[u32]) -> usize {
        if self.dump {
            self.dump_command(words);
        }
        let w0 = words[0];
        let op = w0 >> 24;
        let st = &mut self.state;
        match op {
            0x00 | 0x01 | 0x03..=0x1E => 1,
            0x02 => {
                if words.len() < 3 {
                    return words.len();
                }
                let (x, y) = ((words[1] & 0xFFFF) as i32, (words[1] >> 16) as i32);
                let (w, h) = ((words[2] & 0xFFFF) as i32, (words[2] >> 16) as i32);
                self.vram.fill(x, y, w, h, w0 & 0xFF_FFFF);
                if let Some(hr) = self.hires.as_mut() {
                    hr.fill_command(x, y, w, h, w0 & 0xFF_FFFF);
                }
                vtrace(format_args!("fill ({x},{y}) {w}x{h}"));
                3
            }
            0x1F => 1,
            0x20..=0x3F => self.polygon(words),
            0x40..=0x5F => self.lines(words),
            0x60..=0x7F => self.rect(words),
            0x80..=0x9F => {
                if words.len() < 4 {
                    return words.len();
                }
                let (sx, sy) = ((words[1] & 0x3FF) as i32, ((words[1] >> 16) & 0x1FF) as i32);
                let (dx, dy) = ((words[2] & 0x3FF) as i32, ((words[2] >> 16) & 0x1FF) as i32);
                let w = (((words[3] & 0xFFFF).wrapping_sub(1) & 0x3FF) + 1) as i32;
                let h = ((((words[3] >> 16).wrapping_sub(1)) & 0x1FF) + 1) as i32;
                let st = self.state;
                self.vram.copy(sx, sy, dx, dy, w, h, &st);
                if let Some(hr) = self.hires.as_mut() {
                    hr.copy(sx, sy, dx, dy, w, h, &st);
                }
                vtrace(format_args!("copy ({sx},{sy}) {w}x{h} -> ({dx},{dy})"));
                4
            }
            0xA0..=0xBF => {
                // CPU -> VRAM inside a command list: pixel data follows.
                if words.len() < 3 {
                    return words.len();
                }
                let (x, y) = ((words[1] & 0x3FF) as i32, ((words[1] >> 16) & 0x1FF) as i32);
                let w = (((words[2] & 0xFFFF).wrapping_sub(1) & 0x3FF) + 1) as i32;
                let h = ((((words[2] >> 16).wrapping_sub(1)) & 0x1FF) + 1) as i32;
                let count = ((w * h + 1) / 2) as usize;
                let data = &words[3..(3 + count).min(words.len())];
                let pixels = data.iter().flat_map(|w| [*w as u16, (*w >> 16) as u16]);
                self.load_image(x, y, w, h, pixels);
                vtrace(format_args!("packet load ({x},{y}) {w}x{h}"));
                3 + count
            }
            0xC0..=0xDF => 3,
            0xE1 => {
                st.set_texpage(w0);
                st.dither = w0 & (1 << 9) != 0;
                st.tex_disable = w0 & (1 << 11) != 0;
                st.rect_flip_x = w0 & (1 << 12) != 0;
                st.rect_flip_y = w0 & (1 << 13) != 0;
                1
            }
            0xE2 => {
                st.win_mask_x = (w0 & 0x1F) as i32;
                st.win_mask_y = ((w0 >> 5) & 0x1F) as i32;
                st.win_off_x = ((w0 >> 10) & 0x1F) as i32;
                st.win_off_y = ((w0 >> 15) & 0x1F) as i32;
                1
            }
            0xE3 => {
                st.area_x1 = (w0 & 0x3FF) as i32;
                st.area_y1 = ((w0 >> 10) & 0x1FF) as i32;
                vtrace(format_args!("draw area from ({},{})", st.area_x1, st.area_y1));
                1
            }
            0xE4 => {
                st.area_x2 = (w0 & 0x3FF) as i32;
                st.area_y2 = ((w0 >> 10) & 0x1FF) as i32;
                vtrace(format_args!("draw area to ({},{})", st.area_x2, st.area_y2));
                1
            }
            0xE5 => {
                st.offset_x = sext11(w0);
                st.offset_y = sext11(w0 >> 11);
                vtrace(format_args!("draw offset ({},{})", st.offset_x, st.offset_y));
                1
            }
            0xE6 => {
                st.set_mask = w0 & 1 != 0;
                st.check_mask = w0 & 2 != 0;
                1
            }
            _ => 1,
        }
    }

    fn polygon(&mut self, words: &[u32]) -> usize {
        let op = words[0] >> 24;
        let gouraud = op & 0x10 != 0;
        let quad = op & 0x08 != 0;
        let textured = op & 0x04 != 0;
        let n = if quad { 4 } else { 3 };
        let per_vertex = 1 + usize::from(textured) + usize::from(gouraud);
        let len = 1 + n * per_vertex - usize::from(gouraud);
        if words.len() < len {
            return words.len();
        }
        let mut st = self.state;
        let mut sh = Shading {
            textured: textured && !st.tex_disable,
            raw_texture: op & 0x01 != 0,
            gouraud,
            semi_transparent: op & 0x02 != 0,
            clut_x: 0,
            clut_y: 0,
        };
        let mut verts = [Vertex::default(); 4];
        let mut i = 0;
        let mut rgb = color(words[0]);
        for (k, v) in verts.iter_mut().enumerate().take(n) {
            if gouraud && k > 0 {
                rgb = color(words[i]);
                i += 1;
            } else if k == 0 {
                i += 1;
            }
            let (x, y) = xy(words[i]);
            i += 1;
            *v = Vertex { x: x + st.offset_x, y: y + st.offset_y, r: rgb.0, g: rgb.1, b: rgb.2, u: 0, v: 0 };
            if textured {
                let t = words[i];
                i += 1;
                v.u = (t & 0xFF) as i32;
                v.v = ((t >> 8) & 0xFF) as i32;
                match k {
                    0 => {
                        sh.clut_x = (((t >> 16) & 0x3F) * 16) as i32;
                        sh.clut_y = ((t >> 22) & 0x1FF) as i32;
                    }
                    1 => {
                        let tp = t >> 16;
                        self.state.set_texpage(tp);
                        self.state.tex_disable = tp & (1 << 11) != 0;
                        st = self.state;
                        sh.textured = !st.tex_disable;
                    }
                    _ => {}
                }
            }
        }
        if !gouraud {
            for v in verts.iter_mut() {
                (v.r, v.g, v.b) = (rgb.0, rgb.1, rgb.2);
            }
        }
        if trace("gpubig") {
            let xs = verts[..n].iter().map(|v| v.x);
            let ys = verts[..n].iter().map(|v| v.y);
            let (w, h) = (xs.clone().max().unwrap() - xs.min().unwrap(), ys.clone().max().unwrap() - ys.min().unwrap());
            if w > 400 || h > 300 {
                let coords: Vec<_> = verts[..n].iter().map(|v| (v.x - st.offset_x, v.y - st.offset_y)).collect();
                eprintln!("[gpubig] packet 0x{:08X} op 0x{op:02X} {w}x{h} verts {coords:?} words {:08X?}", self.cur_packet, &words[..len]);
            }
        }
        self.vram.triangle(&st, &sh, [verts[0], verts[1], verts[2]]);
        if quad {
            self.vram.triangle(&st, &sh, [verts[1], verts[2], verts[3]]);
        }
        if let Some(hr) = self.hires.as_mut() {
            hr.triangle(&self.vram, &st, &sh, [verts[0], verts[1], verts[2]]);
            if quad {
                hr.triangle(&self.vram, &st, &sh, [verts[1], verts[2], verts[3]]);
            }
        }
        len
    }

    fn lines(&mut self, words: &[u32]) -> usize {
        let op = words[0] >> 24;
        let gouraud = op & 0x10 != 0;
        let poly = op & 0x08 != 0;
        let st = self.state;
        let sh = Shading {
            textured: false,
            raw_texture: false,
            gouraud,
            semi_transparent: op & 0x02 != 0,
            clut_x: 0,
            clut_y: 0,
        };
        let mut i = 1;
        let mut rgb = color(words[0]);
        let mut prev: Option<Vertex> = None;
        loop {
            if poly && i < words.len() && words[i] & 0xF000_F000 == 0x5000_5000 && prev.is_some() {
                return i + 1; // terminator
            }
            if gouraud && prev.is_some() {
                if i >= words.len() {
                    return words.len();
                }
                rgb = color(words[i]);
                i += 1;
            }
            if i >= words.len() {
                return words.len();
            }
            let (x, y) = xy(words[i]);
            i += 1;
            let v = Vertex { x: x + st.offset_x, y: y + st.offset_y, r: rgb.0, g: rgb.1, b: rgb.2, u: 0, v: 0 };
            if let Some(p) = prev {
                self.vram.line(&st, &sh, p, v);
                if let Some(hr) = self.hires.as_mut() {
                    hr.line(&self.vram, &st, &sh, p, v);
                }
                if !poly {
                    return i;
                }
            }
            prev = Some(v);
        }
    }

    fn rect(&mut self, words: &[u32]) -> usize {
        let op = words[0] >> 24;
        let textured = op & 0x04 != 0;
        let size = (op >> 3) & 3;
        let len = 2 + usize::from(textured) + usize::from(size == 0);
        if words.len() < len {
            return words.len();
        }
        let st = self.state;
        let (x, y) = xy(words[1]);
        let mut sh = Shading {
            textured: textured && !st.tex_disable,
            raw_texture: op & 0x01 != 0,
            gouraud: false,
            semi_transparent: op & 0x02 != 0,
            clut_x: 0,
            clut_y: 0,
        };
        let mut i = 2;
        let (mut u, mut v) = (0, 0);
        if textured {
            let t = words[i];
            i += 1;
            u = (t & 0xFF) as i32;
            v = ((t >> 8) & 0xFF) as i32;
            sh.clut_x = (((t >> 16) & 0x3F) * 16) as i32;
            sh.clut_y = ((t >> 22) & 0x1FF) as i32;
        }
        let (w, h) = match size {
            0 => ((words[i] & 0x3FF) as i32, ((words[i] >> 16) & 0x1FF) as i32),
            1 => (1, 1),
            2 => (8, 8),
            _ => (16, 16),
        };
        self.vram.rectangle(&st, &sh, x + st.offset_x, y + st.offset_y, w, h, color(words[0]), u, v);
        if let Some(hr) = self.hires.as_mut() {
            hr.rectangle(&self.vram, &st, &sh, x + st.offset_x, y + st.offset_y, w, h, color(words[0]), u, v);
        }
        len
    }

    /// PE2_GPUDUMP: one line per command with the state it draws with, and
    /// the 16 first CLUT entries of textured primitives.
    fn dump_command(&mut self, words: &[u32]) {
        let op = words[0] >> 24;
        let st = &self.state;
        let mut line = format!(
            "[gpudump] pkt 0x{:08X} op 0x{op:02X} words {:08X?} | tpage x{} y{} depth{} semi{}",
            self.cur_packet,
            &words[..words.len().min(12)],
            st.tex_x,
            st.tex_y,
            st.tex_depth,
            st.semi_mode
        );
        let clut_word = match op {
            0x24..=0x27 | 0x2C..=0x2F => words.get(2),
            0x34..=0x37 | 0x3C..=0x3F => words.get(2),
            0x64..=0x67 | 0x6C..=0x6F | 0x74..=0x77 | 0x7C..=0x7F => words.get(2),
            _ => None,
        };
        if let Some(t) = clut_word {
            let (cx, cy) = ((((t >> 16) & 0x3F) * 16) as i32, ((t >> 22) & 0x1FF) as i32);
            let entries: Vec<u16> = (0..16).map(|i| self.vram.get(cx + i, cy)).collect();
            line += &format!(" clut ({cx},{cy}) {entries:04X?}");
            if (0x24..=0x3F).contains(&op) {
                if let Some(tp) = words.get(if op >= 0x34 { 5 } else { 4 }) {
                    line += &format!(" polytpage 0x{:04X}", tp >> 16);
                }
            }
        }
        match self.dump_out.as_mut() {
            Some(f) => {
                use std::io::Write;
                let _ = writeln!(f, "{line}");
            }
            None => eprintln!("{line}"),
        }
    }

    /// Runs a buffer of GP0 words.
    fn run(&mut self, words: &[u32]) {
        let mut i = 0;
        while i < words.len() {
            i += self.command(&words[i..]).max(1);
        }
    }

    fn load_image(&mut self, x: i32, y: i32, w: i32, h: i32, mut pixels: impl Iterator<Item = u16>) {
        let st = self.state;
        for row in 0..h {
            for col in 0..w {
                let Some(px) = pixels.next() else { return };
                let (px_x, px_y) = (x + col, y + row);
                if let Some(hr) = self.hires.as_mut() {
                    hr.upload(px_x, px_y, px, Some(&st));
                }
                if st.check_mask && self.vram.get(px_x, px_y) & 0x8000 != 0 {
                    continue;
                }
                self.vram.set(px_x, px_y, px | if st.set_mask { 0x8000 } else { 0 });
            }
        }
    }
}

// ---- libgpu ---------------------------------------------------------------

/// RECT { short x, y, w, h }.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

fn read_rect(p: *const Rect) -> Rect {
    // SAFETY: the game passes a valid RECT.
    unsafe { p.read_unaligned() }
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ResetGraph(mode: i32) -> i32 {
    let _g = irq::guard();
    if mode != 1 {
        with_gpu(|g| {
            g.state = DrawState::default();
            g.display.enabled = false;
        });
        init_genv();
    }
    0
}

unsafe extern "C" {
    static pc_sym_GEnv: u32;
}

/// Sets up libgpu's GEnv the way the original ResetGraph does. Interpreted
/// libgpu routines read it: SetDrawArea clamps coordinates to the VRAM size
/// stored here, so left at zero every drawing area collapses to one pixel.
/// Layout: byte 0 GPU type, byte 1 initialized, +4/+6 VRAM width/height,
/// +0x10..+0x6C filled with 0xFF.
fn init_genv() {
    // SAFETY: generated constant; GEnv lies in the main image's data in RAM.
    unsafe {
        let base = pc_sym_GEnv as *mut u8;
        base.write(0); // type 0: 1024x512 VRAM, as on retail consoles
        base.add(1).write(1);
        base.add(4).cast::<u16>().write_unaligned(VRAM_WIDTH as u16);
        base.add(6).cast::<u16>().write_unaligned(VRAM_HEIGHT as u16);
        std::ptr::write_bytes(base.add(0x10), 0xFF, 0x5C);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SetDispMask(mask: i32) {
    let _g = irq::guard();
    with_gpu(|g| g.display.enabled = mask != 0);
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DrawSync(_mode: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_IsIdleGPU(_max: i32) -> i32 {
    0
}

/// ClearOTagR(ot, n): links ot[n-1] -> ot[n-2] -> ... -> ot[0] -> end, the
/// order DrawOTag(&ot[n-1]) walks.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ClearOTagR(ot: *mut u32, n: i32) -> *mut u32 {
    // SAFETY: the game passes an ordering table of `n` words.
    unsafe {
        for i in (1..n.max(1) as usize).rev() {
            *ot.add(i) = (ot.add(i - 1) as u32) & 0x00FF_FFFF;
        }
        *ot = 0x00FF_FFFF;
    }
    ot
}

/// ClearOTag(ot, n): the forward version, ot[0] -> ot[1] -> ... -> end.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ClearOTag(ot: *mut u32, n: i32) -> *mut u32 {
    // SAFETY: as above.
    unsafe {
        let n = n.max(1) as usize;
        for i in 0..n - 1 {
            *ot.add(i) = (ot.add(i + 1) as u32) & 0x00FF_FFFF;
        }
        *ot.add(n - 1) = 0x00FF_FFFF;
    }
    ot
}

/// PE2_TRACE=vram: VRAM transfers and drawing-area changes, timestamped.
fn vtrace(args: std::fmt::Arguments) {
    use std::sync::OnceLock;
    use std::time::Instant;
    static START: OnceLock<Instant> = OnceLock::new();
    if trace("vram") {
        let t = START.get_or_init(Instant::now).elapsed().as_secs_f64();
        eprintln!("[vram {t:8.3}] {args}");
    }
}

/// Set by F12 in the window: the next command list is dumped to
/// pe2-gpudump.txt, with the VRAM before and after it as PNGs.
pub static DUMP_REQUEST: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Writes the whole 1024x512 VRAM as a PNG (15-bit pixels shown as RGB, so
/// texture pages appear as raw data and CLUTs as rows of colours).
fn save_vram_png(vram: &Vram, path: &str) {
    let Ok(file) = std::fs::File::create(path) else { return };
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), VRAM_WIDTH as u32, VRAM_HEIGHT as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let Ok(mut w) = enc.write_header() else { return };
    let c = |v: u16| ((u32::from(v & 0x1F) * 255) / 31) as u8;
    let data: Vec<u8> = vram.pixels.iter().flat_map(|&px| [c(px), c(px >> 5), c(px >> 10)]).collect();
    let _ = w.write_image_data(&data);
}

/// PE2_GPUDUMP=<seconds>: dumps the first command list drawn after that time.
fn dump_due() -> bool {
    use std::sync::OnceLock;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};
    static AT: OnceLock<Option<Instant>> = OnceLock::new();
    static DONE: AtomicBool = AtomicBool::new(false);
    let at = AT.get_or_init(|| {
        std::env::var("PE2_GPUDUMP").ok()?.parse::<f64>().ok().map(|s| Instant::now() + Duration::from_secs_f64(s))
    });
    match at {
        Some(t) if Instant::now() >= *t => !DONE.swap(true, Ordering::SeqCst),
        _ => false,
    }
}

/// DrawOTag(p): draws every packet of the list starting at `p`.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DrawOTag(p: *const u32) {
    let _g = irq::guard();
    with_gpu(|g| {
        let requested = DUMP_REQUEST.swap(false, std::sync::atomic::Ordering::SeqCst);
        g.dump = requested || dump_due();
        if requested {
            g.dump_out = std::fs::File::create("pe2-gpudump.txt").ok().map(std::io::BufWriter::new);
            save_vram_png(&g.vram, "pe2-vram-before.png");
        }
        if g.dump {
            eprintln!("[gpudump] ---- DrawOTag 0x{:08X} ----", p as u32);
        }
        let mut node = p;
        let mut packets = 0;
        let mut words = Vec::with_capacity(16);
        loop {
            // SAFETY: ordering-table packets live in PSX RAM.
            let tag = unsafe { node.read() };
            let len = (tag >> 24) as usize;
            if len > 0 {
                words.clear();
                // SAFETY: the packet holds `len` words after its tag.
                words.extend((1..=len).map(|i| unsafe { node.add(i).read() }));
                g.cur_packet = node as u32;
                g.run(&words);
            }
            packets += 1;
            if tag & 0x00FF_FFFF == 0x00FF_FFFF || packets > 1_000_000 {
                break;
            }
            node = link_ptr(tag);
        }
        g.frame += 1;
        if g.dump_out.take().is_some() {
            save_vram_png(&g.vram, "pe2-vram.png");
            eprintln!("pe2: F12: wrote pe2-gpudump.txt, pe2-vram-before.png and pe2-vram.png");
        }
        g.dump = false;
        if trace("gpu") {
            eprintln!("[gpu] DrawOTag 0x{:08X}: {packets} packets", p as u32);
        }
    });
}

/// DrawPrim(p): draws one primitive packet.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DrawPrim(p: *const u32) {
    let _g = irq::guard();
    // SAFETY: a packet: tag then `len` words.
    let words: Vec<u32> = unsafe {
        let len = (p.read() >> 24) as usize;
        (1..=len).map(|i| p.add(i).read()).collect()
    };
    with_gpu(|g| g.run(&words));
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ClearImage(rect: *const Rect, r: u8, g: u8, b: u8) -> i32 {
    let _g = irq::guard();
    let rc = read_rect(rect);
    vtrace(format_args!("ClearImage ({},{}) {}x{}", rc.x, rc.y, rc.w, rc.h));
    let rgb = u32::from(r) | (u32::from(g) << 8) | (u32::from(b) << 16);
    with_gpu(|gpu| {
        if let Some(hr) = gpu.hires.as_mut() {
            hr.fill(i32::from(rc.x), i32::from(rc.y), i32::from(rc.w), i32::from(rc.h), rgb);
        }
        let c = raster::rgb24_to_15(rgb);
        for y in 0..i32::from(rc.h) {
            for x in 0..i32::from(rc.w) {
                gpu.vram.set(i32::from(rc.x) + x, i32::from(rc.y) + y, c);
            }
        }
    });
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_LoadImage(rect: *const Rect, p: *const u16) -> i32 {
    let _g = irq::guard();
    let rc = read_rect(rect);
    vtrace(format_args!("LoadImage ({},{}) {}x{}", rc.x, rc.y, rc.w, rc.h));
    let n = (i32::from(rc.w) * i32::from(rc.h)).max(0) as usize;
    // SAFETY: the game passes w*h pixels.
    let pixels = unsafe { std::slice::from_raw_parts(p, n) };
    with_gpu(|g| {
        // Image transfers ignore the mask settings.
        for (i, px) in pixels.iter().enumerate() {
            let (x, y) = ((i % rc.w as usize) as i32, (i / rc.w as usize) as i32);
            g.vram.set(i32::from(rc.x) + x, i32::from(rc.y) + y, *px);
            if let Some(hr) = g.hires.as_mut() {
                hr.upload(i32::from(rc.x) + x, i32::from(rc.y) + y, *px, None);
            }
        }
    });
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_LoadImage2(rect: *const Rect, p: *const u16) -> i32 {
    pcrt_LoadImage(rect, p)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StoreImage(rect: *const Rect, p: *mut u16) -> i32 {
    let _g = irq::guard();
    let rc = read_rect(rect);
    with_gpu(|g| {
        for y in 0..i32::from(rc.h) {
            for x in 0..i32::from(rc.w) {
                let px = g.vram.get(i32::from(rc.x) + x, i32::from(rc.y) + y);
                // SAFETY: the game passes room for w*h pixels.
                unsafe { *p.add((y * i32::from(rc.w) + x) as usize) = px };
            }
        }
    });
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StoreImage2(rect: *const Rect, p: *mut u16) -> i32 {
    pcrt_StoreImage(rect, p)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MoveImage(rect: *const Rect, x: i32, y: i32) -> i32 {
    let _g = irq::guard();
    let rc = read_rect(rect);
    vtrace(format_args!("MoveImage ({},{}) {}x{}", rc.x, rc.y, rc.w, rc.h));
    with_gpu(|g| {
        let st = DrawState::default();
        g.vram.copy(i32::from(rc.x), i32::from(rc.y), x, y, i32::from(rc.w), i32::from(rc.h), &st);
        if let Some(hr) = g.hires.as_mut() {
            hr.copy(i32::from(rc.x), i32::from(rc.y), x, y, i32::from(rc.w), i32::from(rc.h), &st);
        }
    });
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MoveImage2(rect: *const Rect, x: i32, y: i32) -> i32 {
    pcrt_MoveImage(rect, x, y)
}

/// DRAWENV, as laid out by libgpu.
#[repr(C)]
pub struct DrawEnv {
    clip: Rect,
    ofs: [i16; 2],
    tw: Rect,
    tpage: u16,
    dtd: u8,
    dfe: u8,
    isbg: u8,
    r0: u8,
    g0: u8,
    b0: u8,
    dr_env: [u32; 16],
}

/// PutDrawEnv(env): builds the environment's packet in env->dr_env (as
/// libgpu does) and executes it.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PutDrawEnv(env: *mut DrawEnv) -> *mut DrawEnv {
    let _g = irq::guard();
    // SAFETY: the game passes a valid DRAWENV.
    let e = unsafe { &mut *env };
    let (cx, cy, cw, ch) = (i32::from(e.clip.x), i32::from(e.clip.y), i32::from(e.clip.w), i32::from(e.clip.h));
    let mut code = vec![
        0xE300_0000 | (cx as u32 & 0x3FF) | ((cy as u32 & 0x1FF) << 10),
        0xE400_0000 | ((cx + cw - 1).max(0) as u32 & 0x3FF) | (((cy + ch - 1).max(0) as u32 & 0x1FF) << 10),
        0xE500_0000 | (e.ofs[0] as u32 & 0x7FF) | ((e.ofs[1] as u32 & 0x7FF) << 11),
        0xE100_0000 | u32::from(e.tpage & 0x9FF) | (u32::from(e.dtd != 0) << 9) | (u32::from(e.dfe != 0) << 10),
        texture_window(&e.tw),
        0xE600_0000,
    ];
    if e.isbg != 0 {
        code.push(0x0200_0000 | u32::from(e.r0) | (u32::from(e.g0) << 8) | (u32::from(e.b0) << 16));
        code.push((cx as u32 & 0xFFFF) | ((cy as u32) << 16));
        code.push((cw as u32 & 0xFFFF) | ((ch as u32) << 16));
    }
    e.dr_env[0] = ((code.len() as u32) << 24) | 0x00FF_FFFF;
    for (i, w) in code.iter().enumerate() {
        e.dr_env[1 + i] = *w;
    }
    with_gpu(|g| g.run(&code));
    env
}

fn texture_window(tw: &Rect) -> u32 {
    if tw.w == 0 || tw.h == 0 {
        return 0xE200_0000;
    }
    let mask = |len: i16| ((!(len as u32 - 1) & 0xFF) >> 3) & 0x1F;
    0xE200_0000
        | mask(tw.w)
        | (mask(tw.h) << 5)
        | (((tw.x as u32 >> 3) & 0x1F) << 10)
        | (((tw.y as u32 >> 3) & 0x1F) << 15)
}

/// DISPENV, as laid out by libgpu.
#[repr(C)]
pub struct DispEnv {
    disp: Rect,
    screen: Rect,
    isinter: u8,
    isrgb24: u8,
    pad0: u8,
    pad1: u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PutDispEnv(env: *mut DispEnv) -> *mut DispEnv {
    let _g = irq::guard();
    // SAFETY: the game passes a valid DISPENV.
    let e = unsafe { &*env };
    let w = i32::from(e.disp.w);
    let width = match w {
        w if w > 560 => 640,
        w if w > 400 => 512,
        w if w > 352 => 368,
        w if w > 280 => 320,
        _ => 256,
    };
    let height = if e.disp.h > 256 { 480 } else { 240 };
    with_gpu(|g| {
        g.display = Display {
            x: i32::from(e.disp.x),
            y: i32::from(e.disp.y),
            width,
            height,
            rgb24: e.isrgb24 != 0,
            enabled: g.display.enabled,
        };
    });
    if trace("gpu") {
        eprintln!("[gpu] PutDispEnv ({}, {}) {width}x{height} rgb24={}", e.disp.x, e.disp.y, e.isrgb24);
    }
    env
}

/// Converts the displayed region to 0RGB pixels for the host window. The
/// GPU lock is held only to copy the raw rows; conversion happens after.
pub fn snapshot(out: &mut Vec<u32>) -> Option<(usize, usize)> {
    thread_local! {
        static RAW: std::cell::RefCell<Vec<u16>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    RAW.with_borrow_mut(|raw| {
        let (w, h, rgb24) = {
            let guard = GPU.lock().unwrap();
            let g = guard.as_ref()?;
            let d = g.display;
            if let (Some(hr), true, false) = (g.hires.as_ref(), d.enabled, d.rgb24) {
                // The layer's rows, converted after the lock is released.
                let s = hr.scale as usize;
                let (w, h) = (d.width as usize * s, d.height as usize * s);
                out.clear();
                out.reserve(w * h);
                for y in 0..h {
                    let hy = (d.y as usize * s + y) % hr.height;
                    let row = &hr.pixels[hy * hr.width..(hy + 1) * hr.width];
                    let x0 = (d.x as usize * s) % hr.width;
                    if x0 + w <= hr.width {
                        out.extend_from_slice(&row[x0..x0 + w]);
                    } else {
                        out.extend((0..w).map(|i| row[(x0 + i) % hr.width]));
                    }
                }
                drop(guard);
                let lut = hires_channel_lut();
                for px in out.iter_mut() {
                    let p = *px;
                    *px = (lut[((p >> 16) & 0xFF) as usize] << 16)
                        | (lut[((p >> 8) & 0xFF) as usize] << 8)
                        | lut[(p & 0xFF) as usize];
                }
                return Some((w, h));
            }
            let (w, h) = (d.width as usize, d.height as usize);
            out.clear();
            out.resize(w * h, 0);
            if !d.enabled {
                return Some((w, h));
            }
            // Halfwords per row: three bytes per pixel in 24-bit mode.
            let row_words = if d.rgb24 { (w * 3).div_ceil(2) } else { w };
            raw.clear();
            for y in 0..h {
                let vy = (d.y as usize + y) % VRAM_HEIGHT;
                let row = &g.vram.pixels[vy * VRAM_WIDTH..(vy + 1) * VRAM_WIDTH];
                let x0 = d.x as usize % VRAM_WIDTH;
                if x0 + row_words <= VRAM_WIDTH {
                    raw.extend_from_slice(&row[x0..x0 + row_words]);
                } else {
                    raw.extend((0..row_words).map(|i| row[(x0 + i) % VRAM_WIDTH]));
                }
            }
            (w, h, d.rgb24)
        };
        let row_words = raw.len() / h.max(1);
        if rgb24 {
            for y in 0..h {
                let row = &raw[y * row_words..(y + 1) * row_words];
                let byte = |i: usize| -> u32 {
                    let word = row[i / 2];
                    u32::from(if i % 2 == 0 { word & 0xFF } else { word >> 8 })
                };
                for x in 0..w {
                    let i = x * 3;
                    out[y * w + x] = (byte(i) << 16) | (byte(i + 1) << 8) | byte(i + 2);
                }
            }
        } else {
            let lut = rgb15_lut();
            for (px, &v) in out.iter_mut().zip(raw.iter()) {
                *px = lut[usize::from(v & 0x7FFF)];
            }
        }
        Some((w, h))
    })
}

/// The layer's channels (5-bit console scale times 8) to 0..255.
fn hires_channel_lut() -> &'static [u32; 256] {
    static LUT: std::sync::OnceLock<[u32; 256]> = std::sync::OnceLock::new();
    LUT.get_or_init(|| std::array::from_fn(|c| (c as u32 * 255 / 248).min(255)))
}

/// 15-bit BGR VRAM pixels to 0RGB, each channel scaled to 0..255.
fn rgb15_lut() -> &'static [u32] {
    static LUT: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
    LUT.get_or_init(|| {
        let c = |v: u32| (v & 0x1F) * 255 / 31;
        (0..0x8000u32).map(|px| (c(px) << 16) | (c(px >> 5) << 8) | c(px >> 10)).collect()
    })
}
