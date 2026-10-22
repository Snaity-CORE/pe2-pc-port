//! The GPU: GP0 command processing on a software VRAM, the libgpu calls that
//! drive it (ordering tables, image transfers, drawing/display
//! environments), and the display state the host window presents.
//!
//! Drawing is synchronous: a command list sent with DrawOTag is fully drawn
//! when the call returns, so DrawSync never has to wait.
//!
//! With `PE2_SCALE=2..8`, everything is also drawn into a higher-resolution
//! true-colour layer, which the window shows instead: on the graphics card
//! (hw.rs) or, with the software renderer, on the CPU (hires.rs).

mod atlas;
pub mod hires;
pub mod hw;
mod interp;
pub mod layer;
pub mod raster;
mod textures;

pub use textures::host as host_texture;

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
    /// PSX address of the command being run (0 when it is not in RAM), so
    /// vertices can be matched with their exact positions.
    cmd_addr: u32,
    /// The first word of the command being run (traces).
    cmd_word0: u32,
    /// Log every command of the current list (PE2_GPUDUMP or F12).
    pub dump: bool,
    dump_out: Option<std::io::BufWriter<std::fs::File>>,
    /// The higher-resolution layer, when PE2_SCALE asks for one.
    pub layer: Option<layer::Layer>,
    /// Bumped by every access from the game; `latched` is its value when
    /// the window's picture was last taken.
    generation: u64,
    latched: u64,
    /// Frames in between (interp.rs), when drawn.
    interp: Option<interp::Interp>,
    /// The screen images a mod's pictures last replaced, by where they
    /// went (`Shown`).
    shown: Vec<Shown>,
    /// The last screen images read back (their hash and pixels, most
    /// recent last), and screen images loaded that are the grey negative of
    /// one of them (by hash: that one's hash).
    read_back: Vec<(u64, Vec<u16>)>,
    negatives: Vec<(u64, u64)>,
}

/// A screen image a mod's picture replaced: where it went, its hash and
/// its pixels. The game changes some such images a little before showing
/// them again (a light on, a strip of the room captured over it); those
/// keep the picture, with what changed drawn over it.
struct Shown {
    rect: (i32, i32, i32, i32),
    hash: u64,
    pixels: Vec<u16>,
}

impl Shown {
    /// The tiles (x, y, w, h within the image) where `pixels` differ from
    /// this image's, if few enough of them do for the picture to stay.
    fn changed(&self, pixels: &[u16]) -> Option<Vec<(i32, i32, i32, i32)>> {
        const TILE: i32 = 16;
        let (_, _, w, h) = self.rect;
        if pixels.len() != self.pixels.len() {
            return None;
        }
        let mut tiles = Vec::new();
        for ty in (0..h).step_by(TILE as usize) {
            for tx in (0..w).step_by(TILE as usize) {
                let (tw, th) = (TILE.min(w - tx), TILE.min(h - ty));
                let differs = (ty..ty + th).any(|y| {
                    let row = (y * w + tx) as usize..(y * w + tx + tw) as usize;
                    pixels[row.clone()] != self.pixels[row]
                });
                if differs {
                    tiles.push((tx, ty, tw, th));
                }
            }
        }
        let all = ((w + TILE - 1) / TILE) * ((h + TILE - 1) / TILE);
        (tiles.len() as i32 * 4 <= all).then_some(tiles)
    }
}

/// PE2_TRACE=pgxp: every few seconds, polygon vertices with and without an
/// exact position, by 64 KB region of their packets and GPU command.
fn pgxp_trace(packet: u32, word0: u32, exact: bool) {
    if !trace("pgxp") {
        return;
    }
    use std::collections::BTreeMap;
    /// Vertices with and without an exact position, by packet region and
    /// command, since the last report.
    type Counts = BTreeMap<(u32, u32), (u64, u64)>;
    static COUNTS: std::sync::Mutex<Option<(Counts, std::time::Instant)>> = std::sync::Mutex::new(None);
    let mut guard = COUNTS.lock().unwrap();
    let (map, since) = guard.get_or_insert_with(|| (BTreeMap::new(), std::time::Instant::now()));
    let e = map.entry((packet & 0xFFFF_0000, word0 >> 24)).or_insert((0, 0));
    if exact {
        e.0 += 1;
    } else {
        e.1 += 1;
    }
    if since.elapsed().as_secs() >= 5 {
        for ((region, op), (hit, miss)) in map.iter() {
            eprintln!("[pgxp] packets 0x{region:08X}.. op 0x{op:02X}: {hit} exact, {miss} whole");
        }
        map.clear();
        *since = std::time::Instant::now();
    }
}

/// PE2_TRACE=cpu: polygon vertices drawn, and those with exact positions.
pub static POLY_VERTICES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static EXACT_VERTICES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Whether the higher-resolution layer uses exact vertex positions:
/// PE2_SUBPIXEL=0/1, else pe2.toml's [video] subpixel, else on.
fn subpixel() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("PE2_SUBPIXEL")
            .ok()
            .map(|v| v.trim() != "0")
            .or(crate::config::get().video.subpixel)
            .unwrap_or(true)
    })
}

/// Whether textures and colours are interpolated with perspective
/// correction where vertex depths are known: PE2_PERSPECTIVE=0/1, else
/// pe2.toml's [video] perspective, else on.
fn perspective() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("PE2_PERSPECTIVE")
            .ok()
            .map(|v| v.trim() != "0")
            .or(crate::config::get().video.perspective)
            .unwrap_or(true)
    })
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
        eprintln!("pe2: video: internal resolution {scale}x ({}x{})", 320 * scale, 240 * scale);
        if subpixel() {
            // SAFETY: a flag in gte.c, set before the game runs.
            unsafe { pc_pgxp_culling = 1 };
        }
    }
    let mut layer = (scale > 1).then(|| match crate::gfx::get() {
        Some(gfx) => layer::Layer::Hardware(Box::new(hw::Hw::new(gfx, scale))),
        None => layer::Layer::Software(hires::Hires::new(scale)),
    });
    let mut interp = None;
    if let Some(layer::Layer::Hardware(hw)) = layer.as_mut()
        && subpixel()
        && interp::enabled()
    {
        eprintln!("pe2: video: frame interpolation on (60 fps motion)");
        hw.keep_frames();
        interp = Some(interp::Interp::default());
    }
    *GPU.lock().unwrap() = Some(Gpu {
        vram: Vram::default(),
        state: DrawState::default(),
        display: Display { x: 0, y: 0, width: 320, height: 240, rgb24: false, enabled: false },
        frame: 0,
        cur_packet: 0,
        dump: false,
        dump_out: None,
        layer,
        generation: 0,
        latched: u64::MAX,
        interp,
        cmd_addr: 0,
        cmd_word0: 0,
        shown: Vec::new(),
        read_back: Vec::new(),
        negatives: Vec::new(),
    });
}

fn with_gpu<R>(f: impl FnOnce(&mut Gpu) -> R) -> R {
    let mut g = GPU.lock().unwrap();
    let g = g.as_mut().expect("gpu not started");
    g.generation += 1;
    f(g)
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
    /// Brackets a change of the native VRAM, for the layer's ordering.
    fn pre(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if let Some(l) = self.layer.as_mut() {
            l.before_native_write(&self.vram, x, y, w, h);
        }
    }

    fn post(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if let Some(l) = self.layer.as_mut() {
            l.after_native_write(x, y, w, h);
        }
    }

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
                let area = (x & 0x3F0, y & 0x1FF, ((w & 0x3FF) + 0xF) & !0xF, h & 0x1FF);
                self.pre(area.0, area.1, area.2, area.3);
                self.vram.fill(x, y, w, h, w0 & 0xFF_FFFF);
                self.post(area.0, area.1, area.2, area.3);
                if let Some(hr) = self.layer.as_mut() {
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
                self.pre(dx, dy, w, h);
                self.vram.copy(sx, sy, dx, dy, w, h, &st);
                self.post(dx, dy, w, h);
                if let Some(hr) = self.layer.as_mut() {
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
            uv_limits: None,
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
            let found = self.exact(i, words[i], x, y);
            let exact = found.map(|(ex, ey, ..)| (ex + st.offset_x as f32, ey + st.offset_y as f32));
            let w = found.map_or(0.0, |(_, _, ez, _)| ez);
            let key = found.map_or(0, |(.., k)| k);
            i += 1;
            *v = Vertex {
                x: x + st.offset_x,
                y: y + st.offset_y,
                r: rgb.0,
                g: rgb.1,
                b: rgb.2,
                u: 0,
                v: 0,
                exact,
                w,
                key,
            };
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
        // The texels the polygon can sample on the console: at a higher
        // resolution, interpolation reaches past the last one (its maximum
        // coordinate is exclusive) into neighbouring textures.
        if sh.textured {
            let (u0, u1) =
                (verts[..n].iter().map(|v| v.u).min().unwrap(), verts[..n].iter().map(|v| v.u).max().unwrap());
            let (v0, v1) =
                (verts[..n].iter().map(|v| v.v).min().unwrap(), verts[..n].iter().map(|v| v.v).max().unwrap());
            let last = |lo: i32, hi: i32| if hi > lo { hi - 1 } else { hi };
            sh.uv_limits = Some((u0, v0, last(u0, u1), last(v0, v1)));
        }
        // Perspective-correct interpolation needs every vertex's depth.
        if !perspective() || verts[..n].iter().any(|v| v.w <= 0.0) {
            for v in verts.iter_mut() {
                v.w = 0.0;
            }
        }
        if trace("gpubig") {
            let xs = verts[..n].iter().map(|v| v.x);
            let ys = verts[..n].iter().map(|v| v.y);
            let (w, h) = (xs.clone().max().unwrap() - xs.min().unwrap(), ys.clone().max().unwrap() - ys.min().unwrap());
            if w > 400 || h > 300 {
                let coords: Vec<_> = verts[..n].iter().map(|v| (v.x - st.offset_x, v.y - st.offset_y)).collect();
                eprintln!(
                    "[gpubig] packet 0x{:08X} op 0x{op:02X} {w}x{h} verts {coords:?} words {:08X?}",
                    self.cur_packet,
                    &words[..len]
                );
            }
        }
        let (x0, x1) = (verts[..n].iter().map(|v| v.x).min().unwrap(), verts[..n].iter().map(|v| v.x).max().unwrap());
        let (y0, y1) = (verts[..n].iter().map(|v| v.y).min().unwrap(), verts[..n].iter().map(|v| v.y).max().unwrap());
        let area = clip_area(&st, x0, y0, x1, y1);
        self.pre(area.0, area.1, area.2, area.3);
        if self.dump {
            let list: Vec<String> = verts[..n]
                .iter()
                .map(|v| match v.exact {
                    Some((ex, ey)) => format!("({},{})~({ex:.3},{ey:.3})", v.x, v.y),
                    None => format!("({},{})", v.x, v.y),
                })
                .collect();
            let line = format!("[gpudump]   vertices {}", list.join(" "));
            match self.dump_out.as_mut() {
                Some(f) => {
                    use std::io::Write;
                    let _ = writeln!(f, "{line}");
                }
                None => eprintln!("{line}"),
            }
        }
        self.vram.triangle(&st, &sh, [verts[0], verts[1], verts[2]]);
        if quad {
            self.vram.triangle(&st, &sh, [verts[1], verts[2], verts[3]]);
        }
        self.post(area.0, area.1, area.2, area.3);
        if let Some(hr) = self.layer.as_mut() {
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
            uv_limits: None,
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
            let v =
                Vertex { x: x + st.offset_x, y: y + st.offset_y, r: rgb.0, g: rgb.1, b: rgb.2, ..Vertex::default() };
            if let Some(p) = prev {
                let area = clip_area(&st, p.x.min(v.x), p.y.min(v.y), p.x.max(v.x), p.y.max(v.y));
                self.pre(area.0, area.1, area.2, area.3);
                self.vram.line(&st, &sh, p, v);
                self.post(area.0, area.1, area.2, area.3);
                if let Some(hr) = self.layer.as_mut() {
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
            uv_limits: None,
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
        let (rx, ry) = (x + st.offset_x, y + st.offset_y);
        let area = clip_area(&st, rx, ry, rx + w - 1, ry + h - 1);
        self.pre(area.0, area.1, area.2, area.3);
        self.vram.rectangle(&st, &sh, rx, ry, w, h, color(words[0]), u, v);
        self.post(area.0, area.1, area.2, area.3);
        if let Some(hr) = self.layer.as_mut() {
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
            if (0x24..=0x3F).contains(&op)
                && let Some(tp) = words.get(if op >= 0x34 { 5 } else { 4 })
            {
                line += &format!(" polytpage 0x{:04X}", tp >> 16);
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
    /// Runs GP0 words read from PSX address `addr` (0: not from RAM).
    fn run(&mut self, words: &[u32], addr: u32) {
        let mut i = 0;
        while i < words.len() {
            self.cmd_addr = if addr != 0 { addr + 4 * i as u32 } else { 0 };
            self.cmd_word0 = words[i];
            i += self.command(&words[i..]).max(1);
        }
    }

    /// The exact position of the vertex in command word `i`, when there is
    /// one for that word (and it rounds to what the word holds), its depth
    /// and its key.
    fn exact(&self, i: usize, word: u32, x: i32, y: i32) -> Option<(f32, f32, f32, u32)> {
        if self.cmd_addr == 0 || !subpixel() || self.layer.is_none() {
            return None;
        }
        POLY_VERTICES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (mut ex, mut ey, mut ez, mut key) = (0f32, 0f32, 0f32, 0u32);
        // SAFETY: pgxp.c's lookup reads only its own table.
        let found = unsafe { pc_pgxp_lookup(self.cmd_addr + 4 * i as u32, word, &mut ex, &mut ey, &mut ez, &mut key) };
        if found == 0 || (ex - x as f32).abs() > 8.0 || (ey - y as f32).abs() > 8.0 {
            pgxp_trace(self.cur_packet, self.cmd_word0, false);
            return None;
        }
        EXACT_VERTICES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        pgxp_trace(self.cur_packet, self.cmd_word0, true);
        Some((ex, ey, ez, key))
    }

    fn load_image(&mut self, x: i32, y: i32, w: i32, h: i32, pixels: impl Iterator<Item = u16>) {
        let st = self.state;
        let pixels: Vec<u16> = pixels.take((w * h).max(0) as usize).collect();
        self.pre(x, y, w, h);
        for (i, &px) in pixels.iter().enumerate() {
            let (px_x, px_y) = (x + i as i32 % w, y + i as i32 / w);
            if st.check_mask && self.vram.get(px_x, px_y) & 0x8000 != 0 {
                continue;
            }
            self.vram.set(px_x, px_y, px | if st.set_mask { 0x8000 } else { 0 });
        }
        self.post(x, y, w, h);
        if let Some(hr) = self.layer.as_mut() {
            hr.upload(&self.vram, x, y, w, h, &pixels, Some(&st));
        }
    }
}

// ---- libgpu ---------------------------------------------------------------

/// The rectangle a primitive spanning x0..=x1, y0..=y1 can change: inside
/// the drawing area, as (x, y, w, h).
fn clip_area(st: &DrawState, x0: i32, y0: i32, x1: i32, y1: i32) -> (i32, i32, i32, i32) {
    let (x0, y0) = (x0.max(st.area_x1), y0.max(st.area_y1));
    let (x1, y1) = (x1.min(st.area_x2), y1.min(st.area_y2));
    (x0, y0, (x1 - x0 + 1).max(0), (y1 - y0 + 1).max(0))
}

/// RECT { short x, y, w, h }.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    x: i16,
    y: i16,
    w: i16,
    h: i16,
}

impl Rect {
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Rect {
        Rect { x, y, w, h }
    }
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
    fn pc_pgxp_lookup(addr: u32, word: u32, x: *mut f32, y: *mut f32, z: *mut f32, key: *mut u32) -> i32;
    fn pc_pgxp_tick();
    static mut pc_pgxp_culling: i32;
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
                g.run(&words, node as u32 + 4);
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
        vtrace(format_args!("DrawOTag 0x{:08X}: {packets} packets", p as u32));
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
    with_gpu(|g| g.run(&words, p as u32 + 4));
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ClearImage(rect: *const Rect, r: u8, g: u8, b: u8) -> i32 {
    let _g = irq::guard();
    let rc = read_rect(rect);
    vtrace(format_args!("ClearImage ({},{}) {}x{}", rc.x, rc.y, rc.w, rc.h));
    let rgb = u32::from(r) | (u32::from(g) << 8) | (u32::from(b) << 16);
    with_gpu(|gpu| {
        let (x0, y0, w, h) = (i32::from(rc.x), i32::from(rc.y), i32::from(rc.w), i32::from(rc.h));
        gpu.pre(x0, y0, w, h);
        let c = raster::rgb24_to_15(rgb);
        for y in 0..h {
            for x in 0..w {
                gpu.vram.set(x0 + x, y0 + y, c);
            }
        }
        gpu.post(x0, y0, w, h);
        if let Some(hr) = gpu.layer.as_mut() {
            hr.fill(x0, y0, w, h, rgb);
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
        let (x0, y0, w, h) = (i32::from(rc.x), i32::from(rc.y), i32::from(rc.w), i32::from(rc.h));
        g.pre(x0, y0, w, h);
        // Image transfers ignore the mask settings.
        g.vram.write_rect(x0, y0, w, h, pixels);
        g.post(x0, y0, w, h);
        // A screen image (a background, a full-screen picture: about the
        // size of the screen, where the game draws or shows) is known to
        // mods by its hash.
        let st = g.state;
        let on_screen = |ax: i32, ay: i32, aw: i32, ah: i32| x0 < ax + aw && ax < x0 + w && y0 < ay + ah && ay < y0 + h;
        let screen = w >= 256
            && h >= 192
            && crate::mods::wants_screens()
            && (on_screen(st.area_x1, st.area_y1, st.area_x2 - st.area_x1 + 1, st.area_y2 - st.area_y1 + 1)
                || on_screen(g.display.x, g.display.y, g.display.width, g.display.height));
        let hash = screen.then(|| crate::mods::screen_hash(w, h, pixels));
        let rect = (x0, y0, w, h);
        let picture = hash.and_then(|hash| crate::mods::background(hash).map(|p| (hash, p)));
        if let Some(hash) = hash {
            let has = if picture.is_some() { "a mod's picture" } else { "no picture (yet)" };
            vtrace(format_args!("screen image {hash:016x} at ({x0},{y0}) {w}x{h}: {has}"));
        }
        if let Some((hash, _)) = picture {
            g.shown.retain(|s| s.rect != rect);
            g.shown.truncate(3);
            g.shown.insert(0, Shown { rect, hash, pixels: pixels.to_vec() });
        }
        // An image the mod has no picture for, changed a little from one it
        // has: that picture, and what changed.
        let changed = match (hash, &picture) {
            (Some(_), None) => g.shown.iter().find(|s| s.rect == rect).and_then(|s| {
                let p = crate::mods::background(s.hash)?;
                Some((s.hash, p, s.changed(pixels)?))
            }),
            _ => None,
        };
        if let Some(hr) = g.layer.as_mut() {
            match (picture, changed) {
                (Some((hash, p)), _) => hr.upload_picture(&g.vram, x0, y0, w, h, pixels, hash, &p),
                (None, Some((hash, p, tiles))) => {
                    hr.upload_picture(&g.vram, x0, y0, w, h, pixels, hash, &p);
                    for (tx, ty, tw, th) in tiles {
                        let tile: Vec<u16> = (ty..ty + th)
                            .flat_map(|y| pixels[(y * w + tx) as usize..(y * w + tx + tw) as usize].iter().copied())
                            .collect();
                        hr.upload(&g.vram, x0 + tx, y0 + ty, tw, th, &tile, None);
                    }
                }
                (None, None) => {
                    // A screen image read back before (the picture frozen
                    // behind the menus): as the layer had it.
                    let screen = w >= KEPT_SCREEN.0 && h >= KEPT_SCREEN.1;
                    let hash = screen.then(|| crate::mods::screen_hash(w, h, pixels));
                    let kept = hash.is_some_and(|hash| hr.load_kept(&g.vram, x0, y0, w, h, pixels, hash));
                    // The grey negative of one read back (the screen
                    // negative effect, made on the CPU): drawn so from it.
                    let negative_of = hash.filter(|_| !kept).and_then(|hash| {
                        if let Some(&(_, of)) = g.negatives.iter().find(|n| n.0 == hash) {
                            return Some(of);
                        }
                        let of = g.read_back.iter().rev().find(|r| is_negative_of(&r.1, pixels))?.0;
                        g.negatives.push((hash, of));
                        if g.negatives.len() > READ_BACKS_KEPT * 4 {
                            g.negatives.remove(0);
                        }
                        Some(of)
                    });
                    let negative =
                        negative_of.is_some_and(|of| hr.load_kept_negative(&g.vram, x0, y0, w, h, pixels, of));
                    if kept {
                        vtrace(format_args!("LoadImage ({x0},{y0}) {w}x{h}: kept"));
                    } else if negative {
                        vtrace(format_args!("LoadImage ({x0},{y0}) {w}x{h}: kept, as its negative"));
                    } else if screen && trace("vram") {
                        if let Some((_, old)) = g.read_back.last().filter(|o| o.1.len() == pixels.len()) {
                            vtrace(format_args!("  vs last read back: {}", compare(old, pixels, w as usize)));
                        }
                        hr.upload(&g.vram, x0, y0, w, h, pixels, None);
                    } else {
                        hr.upload(&g.vram, x0, y0, w, h, pixels, None);
                    }
                }
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
    let n = (i32::from(rc.w) * i32::from(rc.h)).max(0) as usize;
    // SAFETY: the game passes room for w*h pixels.
    let out = unsafe { std::slice::from_raw_parts_mut(p, n) };
    vtrace(format_args!("StoreImage ({},{}) {}x{}", rc.x, rc.y, rc.w, rc.h));
    with_gpu(|g| {
        let (x, y, w, h) = (i32::from(rc.x), i32::from(rc.y), i32::from(rc.w), i32::from(rc.h));
        g.vram.read_rect(x, y, w, h, out);
        // A screen image read back is loaded again later (behind menus and
        // examined items): the layer keeps its higher-resolution picture.
        if w >= KEPT_SCREEN.0
            && h >= KEPT_SCREEN.1
            && let Some(hr) = g.layer.as_mut()
        {
            let hash = crate::mods::screen_hash(w, h, out);
            hr.keep_screen(&g.vram, x, y, w, h, hash);
            g.read_back.retain(|r| r.0 != hash);
            g.read_back.push((hash, out.to_vec()));
            if g.read_back.len() > READ_BACKS_KEPT {
                g.read_back.remove(0);
            }
        }
    });
    0
}

/// Screen images read back kept, as their pixels, to tell images made
/// from them.
const READ_BACKS_KEPT: usize = 2;

/// A screen negative's pixel (shared/screen_negative_filter.inc.c): the
/// grey 31 - (3R + 4G + B) / 8, without the mask bit.
fn screen_negative(px: u16) -> u16 {
    let (r, g, b) = (px & 31, (px >> 5) & 31, (px >> 10) & 31);
    let grey = 31 - ((3 * r + 4 * g + b) >> 3);
    grey | grey << 5 | grey << 10
}

/// Whether `new` is the screen negative of `old`, pixel for pixel.
fn is_negative_of(old: &[u16], new: &[u16]) -> bool {
    old.len() == new.len() && old.iter().zip(new).all(|(&o, &n)| screen_negative(o) == n)
}

/// PE2_TRACE=vram: how screen image `new` differs from `old` (both `w`
/// wide): how many pixels, where, a few of them, and whether it is `old`
/// moved up or down.
fn compare(old: &[u16], new: &[u16], w: usize) -> String {
    let diff: Vec<usize> = (0..old.len()).filter(|&i| old[i] != new[i]).collect();
    if diff.is_empty() {
        return "the same".into();
    }
    let (x0, x1) = diff.iter().fold((w, 0), |(a, b), &i| (a.min(i % w), b.max(i % w)));
    let (y0, y1) = (diff[0] / w, diff[diff.len() - 1] / w);
    let samples: Vec<String> = diff
        .iter()
        .step_by((diff.len() / 6).max(1))
        .take(6)
        .map(|&i| format!("{:04X}>{:04X}", old[i], new[i]))
        .collect();
    let h = old.len() / w;
    let moved = (-24i32..=24)
        .filter(|&d| d != 0)
        .map(|d| {
            let same = (0..h as i32)
                .filter(|&y| (0..h as i32).contains(&(y + d)))
                .filter(|&y| old[y as usize * w..][..w] == new[(y + d) as usize * w..][..w])
                .count();
            (same, d)
        })
        .max()
        .filter(|&(same, _)| same > h / 2)
        .map(|(same, d)| format!(", moved {d} rows ({same} rows match)"))
        .unwrap_or_default();
    let changed = old.iter().zip(new).filter(|(a, b)| a != b).count();
    format!("{changed} pixels differ in ({x0},{y0})-({x1},{y1}), e.g. {}{moved}", samples.join(" "))
}

/// Image transfers at least this large (screen images) read back are kept
/// by the layer at its resolution.
const KEPT_SCREEN: (i32, i32) = (256, 192);

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
        g.pre(x, y, i32::from(rc.w), i32::from(rc.h));
        g.vram.copy(i32::from(rc.x), i32::from(rc.y), x, y, i32::from(rc.w), i32::from(rc.h), &st);
        g.post(x, y, i32::from(rc.w), i32::from(rc.h));
        if let Some(hr) = g.layer.as_mut() {
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
    with_gpu(|g| g.run(&code, 0));
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

/// The picture the window shows: the displayed region's raw pixels as of
/// the last `latch`.
struct Front {
    width: usize,
    height: usize,
    /// The console's lines in the picture (240, or 480 interlaced).
    lines: usize,
    enabled: bool,
    /// The higher-resolution layer's pixels (when it is shown)...
    hires: Vec<u32>,
    /// ...else the VRAM's halfwords, `row_words` per row.
    vram: Vec<u16>,
    row_words: usize,
    rgb24: bool,
    /// The picture is in HW_FRONT, on the graphics card.
    on_card: bool,
    /// A mod's movie picture shown instead of the game's (mods::movies).
    movie: Option<std::sync::Arc<crate::mods::movies::Picture>>,
}

/// The picture on the graphics card (hardware layer): the displayed region
/// copied out of the layer's texture, and its size.
pub struct CardFront {
    pub texture: wgpu::Texture,
    pub width: u32,
    pub height: u32,
}

pub static HW_FRONT: Mutex<Option<CardFront>> = Mutex::new(None);

/// New pictures latched so far: frames the game drew (the window title's
/// frame rate).
pub static FRAMES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// The size of the window's picture (the internal resolution, or the
/// console's for 24-bit movies).
pub fn picture_size() -> (usize, usize) {
    let f = FRONT.lock().unwrap();
    (f.width, f.height)
}

/// The console's lines in the window's picture (240, or 480 interlaced).
pub fn picture_lines() -> usize {
    FRONT.lock().unwrap().lines
}

/// Whether the window's picture is on the graphics card.
pub fn front_on_card() -> bool {
    FRONT.lock().unwrap().on_card
}

static FRONT: Mutex<Front> = Mutex::new(Front {
    width: 0,
    height: 0,
    lines: 240,
    enabled: false,
    hires: Vec::new(),
    vram: Vec::new(),
    row_words: 0,
    rgb24: false,
    on_card: false,
    movie: None,
});

/// Takes the window's picture from the displayed region. Called at the end
/// of each vertical blank, after the game's vblank callback: the game flips
/// and draws there (reloading the background, then drawing over it), so
/// the window sees whole frames only, as a TV does, never one half drawn.
/// Skipped when nothing changed, or while the window is reading the last
/// picture (the next blank takes it). With frame interpolation, the blank
/// after a new frame shows that frame drawn halfway from the one on screen
/// (interp.rs).
pub fn latch() {
    // SAFETY: a counter in pgxp.c, also called on the interrupt thread only.
    unsafe { pc_pgxp_tick() };
    let mut guard = GPU.lock().unwrap();
    let Some(g) = guard.as_mut() else { return };
    if let Some(interp) = g.interp.as_mut() {
        interp.blank();
    }
    if g.generation == g.latched {
        // No new frame: the frame in between, if one is ready.
        let pending = g.interp.as_mut().and_then(interp::Interp::take_pending);
        if let (Some((x, y, w, h)), Some(layer::Layer::Hardware(hw))) = (pending, g.layer.as_ref())
            && let Some(between) = hw.between.as_ref()
        {
            latch_on_card(hw, between, x, y, w, h);
            FRAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        return;
    }
    let Ok(mut f) = FRONT.try_lock() else { return };
    g.latched = g.generation;
    FRAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    vtrace(format_args!("latch"));
    let d = g.display;
    // Whatever is shown, the card catches up once a frame (uploads would
    // pile up while a 24-bit movie is displayed from the VRAM).
    if let Some(layer::Layer::Hardware(hw)) = g.layer.as_mut() {
        hw.flush(&g.vram);
    }
    f.enabled = d.enabled;
    f.rgb24 = d.rgb24;
    f.lines = d.height as usize;
    f.hires.clear();
    f.vram.clear();
    f.on_card = false;
    f.movie = None;
    crate::mods::movies::tick();
    let kept = match g.layer.as_mut() {
        Some(layer::Layer::Hardware(hw)) => hw.take_frame(),
        _ => Vec::new(),
    };
    if let (Some(layer::Layer::Hardware(hw)), false) = (g.layer.as_mut(), d.rgb24) {
        let s = hw.scale as u32;
        let (w, h) = (d.width as u32 * s, d.height as u32 * s);
        (f.width, f.height) = (w as usize, h as usize);
        let shown = (d.x as u32 * s, d.y as u32 * s, w, h);
        if d.enabled {
            latch_on_card(hw, &hw.target, shown.0, shown.1, w, h);
            f.on_card = true;
        }
        // The new frame was drawn into the other framebuffer, which the
        // game shows from the blank after next: the drawing area's.
        let st = g.state;
        let drawn = (st.area_x1.max(0) as u32 * s, st.area_y1.max(0) as u32 * s, w, h);
        let region = (d.enabled && drawn != shown).then_some(drawn);
        let between = g.interp.as_mut().is_some_and(|i| i.new_frame(hw, &kept, region, shown));
        if g.interp.is_some() {
            hw.keep_vram();
        }
        if between && trace("interpcheck") {
            check_between(hw, drawn);
        }
        if between && trace("interpshots") {
            static SHOTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = SHOTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n.is_multiple_of(20) {
                let n = n / 20;
                save_card_region(&hw.target, shown, &format!("pe2-interp-{n:03}-a-shown.png"));
                save_card_region(hw.between.as_ref().unwrap(), drawn, &format!("pe2-interp-{n:03}-b-between.png"));
                save_card_region(&hw.target, drawn, &format!("pe2-interp-{n:03}-c-next.png"));
            }
        }
        return;
    }
    if let Some(interp) = g.interp.as_mut() {
        interp.new_frame_hidden();
        if let Some(layer::Layer::Hardware(hw)) = g.layer.as_mut() {
            hw.keep_vram();
        }
    }
    if let (Some(layer::Layer::Software(hr)), false) = (g.layer.as_ref(), d.rgb24) {
        let s = hr.scale as usize;
        let (w, h) = (d.width as usize * s, d.height as usize * s);
        (f.width, f.height) = (w, h);
        if !d.enabled {
            return;
        }
        for y in 0..h {
            let hy = (d.y as usize * s + y) % hr.height;
            let row = &hr.pixels[hy * hr.width..(hy + 1) * hr.width];
            let x0 = (d.x as usize * s) % hr.width;
            if x0 + w <= hr.width {
                f.hires.extend_from_slice(&row[x0..x0 + w]);
            } else {
                f.hires.extend((0..w).map(|i| row[(x0 + i) % hr.width]));
            }
        }
        return;
    }
    // A mod's movie, shown at its own size instead of the game's.
    if d.rgb24
        && d.enabled
        && let Some(p) = crate::mods::movies::picture()
    {
        (f.width, f.height) = (p.width, p.height);
        f.movie = Some(p);
        return;
    }
    let (w, h) = (d.width as usize, d.height as usize);
    (f.width, f.height) = (w, h);
    // Halfwords per row: three bytes per pixel in 24-bit mode.
    let row_words = if d.rgb24 { (w * 3).div_ceil(2) } else { w };
    f.row_words = row_words;
    if !d.enabled {
        return;
    }
    for y in 0..h {
        let vy = (d.y as usize + y) % VRAM_HEIGHT;
        let row = &g.vram.pixels[vy * VRAM_WIDTH..(vy + 1) * VRAM_WIDTH];
        let x0 = d.x as usize % VRAM_WIDTH;
        if x0 + row_words <= VRAM_WIDTH {
            f.vram.extend_from_slice(&row[x0..x0 + row_words]);
        } else {
            f.vram.extend((0..row_words).map(|i| row[(x0 + i) % VRAM_WIDTH]));
        }
    }
}

/// Copies the displayed region of the hardware layer (or of `source`, a
/// texture of the same size) into HW_FRONT.
fn latch_on_card(hw: &hw::Hw, source: &wgpu::Texture, x: u32, y: u32, w: u32, h: u32) {
    let Some(gfx) = crate::gfx::get() else { return };
    let mut front = HW_FRONT.lock().unwrap();
    if front.as_ref().is_none_or(|f| (f.width, f.height) != (w, h)) {
        let texture = gfx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("front"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        *front = Some(CardFront { texture, width: w, height: h });
    }
    let f = front.as_ref().unwrap();
    let size = hw.target.size();
    // The displayed region, clipped to the layer (no wrapping).
    let (cw, ch) = (w.min(size.width.saturating_sub(x)), h.min(size.height.saturating_sub(y)));
    if cw == 0 || ch == 0 {
        return;
    }
    let mut encoder = gfx.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_texture(
        wgpu::TexelCopyTextureInfo {
            texture: source,
            mip_level: 0,
            origin: wgpu::Origin3d { x, y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyTextureInfo {
            texture: &f.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::Extent3d { width: cw, height: ch, depth_or_array_layers: 1 },
    );
    gfx.queue.submit([encoder.finish()]);
}

/// PE2_TRACE=interpshots: saves a region of a layer texture as a PNG.
fn save_card_region(texture: &wgpu::Texture, (x, y, w, h): interp::Region, path: &str) {
    let Some(rgb) = read_card_region(texture, (x, y, w, h)) else { return };
    let Ok(file) = std::fs::File::create(path) else { return };
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w, h);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    if let Ok(mut writer) = enc.write_header() {
        let _ = writer.write_image_data(&rgb);
    }
}

/// PE2_TRACE=interpcheck (with PE2_INTERP_T=1, the frame drawn again where
/// it is): pixels of the frame in between unlike the frame's, per frame and
/// every 5 s. Any is a fault in drawing frames again.
fn check_between(hw: &hw::Hw, region: interp::Region) {
    use std::sync::Mutex;
    static TOTALS: Mutex<(u64, u64, Option<std::time::Instant>)> = Mutex::new((0, 0, None));
    let (Some(between), Some(a)) = (hw.between.as_ref(), read_card_region(&hw.target, region)) else { return };
    let Some(b) = read_card_region(between, region) else { return };
    let differ = a
        .as_chunks::<3>()
        .0
        .iter()
        .zip(b.as_chunks::<3>().0)
        .filter(|(p, q)| p.iter().zip(q.iter()).any(|(x, y)| x.abs_diff(*y) > 8))
        .count();
    let mut t = TOTALS.lock().unwrap();
    t.0 += 1;
    if differ > 0 {
        t.1 += 1;
        eprintln!("[interpcheck] frame {}: {differ} pixels differ", t.0);
    }
    let since = *t.2.get_or_insert_with(std::time::Instant::now);
    if since.elapsed().as_secs() >= 5 {
        eprintln!("[interpcheck] {} frames checked, {} differ", t.0, t.1);
        t.2 = Some(std::time::Instant::now());
    }
}

/// A region of a layer texture read back as RGB bytes (slow).
fn read_card_region(texture: &wgpu::Texture, (x, y, w, h): interp::Region) -> Option<Vec<u8>> {
    let gfx = crate::gfx::get()?;
    let row = (w * 4).next_multiple_of(256);
    let buffer = gfx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: u64::from(row * h),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gfx.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x, y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) },
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    gfx.queue.submit([encoder.finish()]);
    buffer.map_async(wgpu::MapMode::Read, .., |_| {});
    gfx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).ok()?;
    let data = buffer.get_mapped_range(..).ok()?;
    let mut rgb = Vec::with_capacity((w * h * 3) as usize);
    for line in data.chunks(row as usize).take(h as usize) {
        rgb.extend(line[..w as usize * 4].as_chunks::<4>().0.iter().flat_map(|p| [p[0], p[1], p[2]]));
    }
    Some(rgb)
}

/// Reads HW_FRONT back as 0RGB pixels (screenshots; slow).
fn read_card_front(out: &mut Vec<u32>) -> Option<(usize, usize)> {
    let gfx = crate::gfx::get()?;
    let front = HW_FRONT.lock().unwrap();
    let f = front.as_ref()?;
    let row = (f.width * 4).next_multiple_of(256);
    let buffer = gfx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: u64::from(row * f.height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gfx.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &f.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(f.height) },
        },
        wgpu::Extent3d { width: f.width, height: f.height, depth_or_array_layers: 1 },
    );
    gfx.queue.submit([encoder.finish()]);
    buffer.map_async(wgpu::MapMode::Read, .., |_| {});
    gfx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).ok()?;
    let data = buffer.get_mapped_range(..).ok()?;
    let lut = hires_channel_lut();
    out.clear();
    for y in 0..f.height as usize {
        let line = &data[y * row as usize..y * row as usize + f.width as usize * 4];
        out.extend(
            line.as_chunks::<4>()
                .0
                .iter()
                .map(|p| (lut[p[0] as usize] << 16) | (lut[p[1] as usize] << 8) | lut[p[2] as usize]),
        );
    }
    Some((f.width as usize, f.height as usize))
}

/// The window's picture (see `latch`) as 0RGB pixels.
pub fn snapshot(out: &mut Vec<u32>) -> Option<(usize, usize)> {
    let f = FRONT.lock().unwrap();
    let (w, h) = (f.width, f.height);
    if w == 0 || h == 0 {
        return None;
    }
    if f.on_card {
        drop(f);
        return read_card_front(out);
    }
    out.clear();
    out.resize(w * h, 0);
    if !f.enabled {
        return Some((w, h));
    }
    if let Some(p) = &f.movie {
        out.copy_from_slice(&p.pixels[..w * h]);
    } else if !f.hires.is_empty() {
        let lut = hires_channel_lut();
        for (px, &p) in out.iter_mut().zip(f.hires.iter()) {
            *px = (lut[((p >> 16) & 0xFF) as usize] << 16)
                | (lut[((p >> 8) & 0xFF) as usize] << 8)
                | lut[(p & 0xFF) as usize];
        }
    } else if f.rgb24 {
        let row_words = f.row_words;
        for y in 0..h {
            let row = &f.vram[y * row_words..(y + 1) * row_words];
            let byte = |i: usize| -> u32 {
                let word = row[i / 2];
                u32::from(if i.is_multiple_of(2) { word & 0xFF } else { word >> 8 })
            };
            for x in 0..w {
                let i = x * 3;
                out[y * w + x] = (byte(i) << 16) | (byte(i + 1) << 8) | byte(i + 2);
            }
        }
    } else {
        let lut = rgb15_lut();
        for (px, &v) in out.iter_mut().zip(f.vram.iter()) {
            *px = lut[usize::from(v & 0x7FFF)];
        }
    }
    Some((w, h))
}

/// The layer's channels (5-bit console scale times 8) to 0..255.
pub fn hires_channel_lut() -> &'static [u32; 256] {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The screen negative as the game makes it (pairs seen in the game:
    /// read back, then loaded).
    #[test]
    fn screen_negative_matches_the_games() {
        for (old, new) in [(0x0421, 0x7BDE), (0x18E7, 0x6739), (0xB1CF, 0x4631), (0x8CC4, 0x6F7B)] {
            assert_eq!(screen_negative(old), new, "{old:04X}");
        }
        assert!(is_negative_of(&[0x0421, 0x18E7], &[0x7BDE, 0x6739]));
        assert!(!is_negative_of(&[0x0421, 0x18E7], &[0x7BDE, 0x6738]));
    }
}
