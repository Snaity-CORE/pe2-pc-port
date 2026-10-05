//! Software rasterizer for the PlayStation GPU's drawing commands, written
//! from the public hardware documentation (psx-spx): 15-bit VRAM, flat and
//! gouraud polygons, 4/8/15-bit textures with CLUTs and texture windows,
//! semi-transparency, dithering, mask bits and the drawing area.

pub const VRAM_WIDTH: usize = 1024;
pub const VRAM_HEIGHT: usize = 512;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    /// 8-bit colour channels.
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub u: i32,
    pub v: i32,
}

/// How a primitive's pixels are produced.
#[derive(Clone, Copy, Debug)]
pub struct Shading {
    pub textured: bool,
    /// Texture colour used as is (no modulation by the vertex colour).
    pub raw_texture: bool,
    pub gouraud: bool,
    pub semi_transparent: bool,
    pub clut_x: i32,
    pub clut_y: i32,
}

/// The drawing state set by the E1-E6 commands.
#[derive(Clone, Copy, Debug)]
pub struct DrawState {
    pub tex_x: i32,
    pub tex_y: i32,
    /// 0: B/2+F/2, 1: B+F, 2: B-F, 3: B+F/4.
    pub semi_mode: u32,
    /// 0: 4-bit, 1: 8-bit, 2/3: 15-bit.
    pub tex_depth: u32,
    pub dither: bool,
    pub tex_disable: bool,
    pub rect_flip_x: bool,
    pub rect_flip_y: bool,
    pub win_mask_x: i32,
    pub win_mask_y: i32,
    pub win_off_x: i32,
    pub win_off_y: i32,
    pub area_x1: i32,
    pub area_y1: i32,
    pub area_x2: i32,
    pub area_y2: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub set_mask: bool,
    pub check_mask: bool,
}

impl Default for DrawState {
    fn default() -> Self {
        DrawState {
            tex_x: 0,
            tex_y: 0,
            semi_mode: 0,
            tex_depth: 0,
            dither: false,
            tex_disable: false,
            rect_flip_x: false,
            rect_flip_y: false,
            win_mask_x: 0,
            win_mask_y: 0,
            win_off_x: 0,
            win_off_y: 0,
            area_x1: 0,
            area_y1: 0,
            area_x2: 0,
            area_y2: 0,
            offset_x: 0,
            offset_y: 0,
            set_mask: false,
            check_mask: false,
        }
    }
}

impl DrawState {
    /// Applies a texpage value (E1 command, or the texpage halfword of a
    /// textured polygon).
    pub fn set_texpage(&mut self, v: u32) {
        self.tex_x = ((v & 0xF) * 64) as i32;
        self.tex_y = (((v >> 4) & 1) * 256) as i32;
        self.semi_mode = (v >> 5) & 3;
        self.tex_depth = (v >> 7) & 3;
    }
}

const DITHER: [[i32; 4]; 4] = [[-4, 0, -3, 1], [2, -2, 3, -1], [-3, 1, -4, 0], [3, -1, 2, -2]];

pub struct Vram {
    pub pixels: Box<[u16]>,
}

impl Default for Vram {
    fn default() -> Self {
        Vram { pixels: vec![0u16; VRAM_WIDTH * VRAM_HEIGHT].into_boxed_slice() }
    }
}

impl Vram {
    #[inline]
    pub fn get(&self, x: i32, y: i32) -> u16 {
        self.pixels[((y as usize) & (VRAM_HEIGHT - 1)) * VRAM_WIDTH + ((x as usize) & (VRAM_WIDTH - 1))]
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, v: u16) {
        self.pixels[((y as usize) & (VRAM_HEIGHT - 1)) * VRAM_WIDTH + ((x as usize) & (VRAM_WIDTH - 1))] = v;
    }

    /// GP0(02h) fill: ignores the drawing area and mask settings; x and
    /// width are in 16-pixel units.
    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, rgb: u32) {
        let c = rgb24_to_15(rgb);
        let x = x & 0x3F0;
        let y = y & 0x1FF;
        let w = ((w & 0x3FF) + 0xF) & !0xF;
        let h = h & 0x1FF;
        for dy in 0..h {
            for dx in 0..w {
                self.set(x + dx, y + dy, c);
            }
        }
    }

    pub fn copy(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32, st: &DrawState) {
        for row in 0..h {
            // Copy row by row through a buffer so overlapping copies behave.
            let line: Vec<u16> = (0..w).map(|col| self.get(sx + col, sy + row)).collect();
            for (col, px) in line.into_iter().enumerate() {
                let (x, y) = (dx + col as i32, dy + row);
                if st.check_mask && self.get(x, y) & 0x8000 != 0 {
                    continue;
                }
                self.set(x, y, px | if st.set_mask { 0x8000 } else { 0 });
            }
        }
    }

    fn texel(&self, st: &DrawState, sh: &Shading, u: i32, v: i32) -> u16 {
        let u = (u & !(st.win_mask_x * 8)) | ((st.win_off_x & st.win_mask_x) * 8);
        let v = (v & !(st.win_mask_y * 8)) | ((st.win_off_y & st.win_mask_y) * 8);
        let (u, v) = (u & 0xFF, v & 0xFF);
        match st.tex_depth {
            0 => {
                let word = self.get(st.tex_x + u / 4, st.tex_y + v);
                let index = (word >> ((u & 3) * 4)) & 0xF;
                self.get(sh.clut_x + i32::from(index), sh.clut_y)
            }
            1 => {
                let word = self.get(st.tex_x + u / 2, st.tex_y + v);
                let index = (word >> ((u & 1) * 8)) & 0xFF;
                self.get(sh.clut_x + i32::from(index), sh.clut_y)
            }
            _ => self.get(st.tex_x + u, st.tex_y + v),
        }
    }

    /// Shades and writes one pixel. `r g b` are 8-bit vertex colours.
    #[allow(clippy::too_many_arguments)]
    fn plot(&mut self, st: &DrawState, sh: &Shading, x: i32, y: i32, r: i32, g: i32, b: i32, u: i32, v: i32, dither: bool) {
        if x < st.area_x1 || x > st.area_x2 || y < st.area_y1 || y > st.area_y2 {
            return;
        }
        let dst = self.get(x, y);
        if st.check_mask && dst & 0x8000 != 0 {
            return;
        }

        let (mut cr, mut cg, mut cb);
        let mut mask_bit = 0u16;
        let mut blend = sh.semi_transparent;
        if sh.textured {
            let t = self.texel(st, sh, u, v);
            if t == 0 {
                return; // fully transparent texel
            }
            mask_bit = t & 0x8000;
            blend = blend && mask_bit != 0;
            let (tr, tg, tb) = (i32::from(t & 0x1F) << 3, i32::from((t >> 5) & 0x1F) << 3, i32::from((t >> 10) & 0x1F) << 3);
            if sh.raw_texture {
                (cr, cg, cb) = (tr, tg, tb);
            } else {
                cr = (tr * r) >> 7;
                cg = (tg * g) >> 7;
                cb = (tb * b) >> 7;
            }
        } else {
            (cr, cg, cb) = (r, g, b);
        }

        if dither && st.dither {
            let d = DITHER[(y & 3) as usize][(x & 3) as usize];
            cr += d;
            cg += d;
            cb += d;
        }
        let (mut r5, mut g5, mut b5) = (clamp5(cr), clamp5(cg), clamp5(cb));

        if blend {
            let (br, bg, bb) = (i32::from(dst & 0x1F), i32::from((dst >> 5) & 0x1F), i32::from((dst >> 10) & 0x1F));
            let mix = |b: i32, f: i32| -> i32 {
                match st.semi_mode {
                    0 => (b + f) >> 1,
                    1 => (b + f).min(31),
                    2 => (b - f).max(0),
                    _ => (b + (f >> 2)).min(31),
                }
            };
            r5 = mix(br, r5);
            g5 = mix(bg, g5);
            b5 = mix(bb, b5);
        }
        let out = (r5 as u16) | ((g5 as u16) << 5) | ((b5 as u16) << 10) | mask_bit | if st.set_mask { 0x8000 } else { 0 };
        self.set(x, y, out);
    }

    pub fn triangle(&mut self, st: &DrawState, sh: &Shading, v: [Vertex; 3]) {
        let [a, b, c] = v;
        // The GPU skips polygons larger than 1023x511.
        let (min_x, max_x) = (a.x.min(b.x).min(c.x), a.x.max(b.x).max(c.x));
        let (min_y, max_y) = (a.y.min(b.y).min(c.y), a.y.max(b.y).max(c.y));
        if max_x - min_x > 1023 || max_y - min_y > 511 {
            return;
        }
        let area = edge(a, b, c.x, c.y);
        if area == 0 {
            return;
        }
        // Orient counter-clockwise so inside means all edges >= 0.
        let (b, c, area) = if area < 0 { (c, b, -area) } else { (b, c, area) };

        let x0 = min_x.max(st.area_x1);
        let x1 = max_x.min(st.area_x2);
        let y0 = min_y.max(st.area_y1);
        let y1 = max_y.min(st.area_y2);
        let dither = sh.gouraud || (sh.textured && !sh.raw_texture);

        for y in y0..=y1 {
            for x in x0..=x1 {
                let w0 = edge(b, c, x, y);
                let w1 = edge(c, a, x, y);
                let w2 = edge(a, b, x, y);
                // Top-left rule: pixels on a right or bottom edge are out.
                if w0 < 0 || w1 < 0 || w2 < 0 {
                    continue;
                }
                if (w0 == 0 && !top_left(b, c)) || (w1 == 0 && !top_left(c, a)) || (w2 == 0 && !top_left(a, b)) {
                    continue;
                }
                let interp = |fa: i32, fb: i32, fc: i32| -> i32 {
                    ((i64::from(w0) * i64::from(fa) + i64::from(w1) * i64::from(fb) + i64::from(w2) * i64::from(fc))
                        / i64::from(area)) as i32
                };
                let (r, g, bl) = if sh.gouraud {
                    (interp(a.r, b.r, c.r), interp(a.g, b.g, c.g), interp(a.b, b.b, c.b))
                } else {
                    (a.r, a.g, a.b)
                };
                let (u, vv) = if sh.textured { (interp(a.u, b.u, c.u), interp(a.v, b.v, c.v)) } else { (0, 0) };
                self.plot(st, sh, x, y, r, g, bl, u, vv, dither);
            }
        }
    }

    /// GP0(60h-7Fh) rectangle / sprite at (x, y), `w` x `h`, with texture
    /// origin (u, v).
    #[allow(clippy::too_many_arguments)]
    pub fn rectangle(&mut self, st: &DrawState, sh: &Shading, x: i32, y: i32, w: i32, h: i32, color: (i32, i32, i32), u: i32, v: i32) {
        for dy in 0..h {
            for dx in 0..w {
                let tu = if st.rect_flip_x { u - dx } else { u + dx };
                let tv = if st.rect_flip_y { v - dy } else { v + dy };
                self.plot(st, sh, x + dx, y + dy, color.0, color.1, color.2, tu, tv, false);
            }
        }
    }

    /// GP0(40h-5Fh) line, with gouraud interpolation along its length.
    pub fn line(&mut self, st: &DrawState, sh: &Shading, a: Vertex, b: Vertex) {
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        if dx.abs() > 1023 || dy.abs() > 511 {
            return;
        }
        let steps = dx.abs().max(dy.abs());
        for i in 0..=steps {
            let t = |p: i32, q: i32| if steps == 0 { p } else { p + (q - p) * i / steps };
            let x = t(a.x, b.x);
            let y = t(a.y, b.y);
            let (r, g, bl) = if sh.gouraud { (t(a.r, b.r), t(a.g, b.g), t(a.b, b.b)) } else { (a.r, a.g, a.b) };
            self.plot(st, sh, x, y, r, g, bl, 0, 0, sh.gouraud);
        }
    }
}

#[inline]
fn edge(a: Vertex, b: Vertex, x: i32, y: i32) -> i32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}

/// For a counter-clockwise triangle (in screen space, y down), an edge is
/// "top" when horizontal and going left, "left" when going down.
#[inline]
fn top_left(a: Vertex, b: Vertex) -> bool {
    (a.y == b.y && b.x < a.x) || b.y > a.y
}

#[inline]
fn clamp5(c: i32) -> i32 {
    (c.clamp(0, 255)) >> 3
}

pub fn rgb24_to_15(rgb: u32) -> u16 {
    let r = (rgb & 0xFF) >> 3;
    let g = ((rgb >> 8) & 0xFF) >> 3;
    let b = ((rgb >> 16) & 0xFF) >> 3;
    (r | (g << 5) | (b << 10)) as u16
}
