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
    /// The exact screen position, when the game's projection left one
    /// (pgxp.c); only the higher-resolution layer uses it.
    pub exact: Option<(f32, f32)>,
    /// The vertex's depth for perspective-correct interpolation, when every
    /// vertex of its polygon has one (0: none).
    pub w: f32,
    /// The model vertex this one projects, the same from frame to frame
    /// (pgxp.c; 0: unknown): frames in between are drawn from it.
    pub key: u32,
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
    /// A textured polygon's texel rectangle (u0, v0, u1, v1), inclusive:
    /// the texels the console's interpolation can reach. Only the
    /// higher-resolution layer uses it (see hw.rs).
    pub uv_limits: Option<(i32, i32, i32, i32)>,
}

/// The drawing state set by the E1-E6 commands.
#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone)]
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

    /// Writes a `w` x `h` image at (x, y), row by row, wrapping at the
    /// VRAM's edges (image transfers ignore the mask settings).
    pub fn write_rect(&mut self, x: i32, y: i32, w: i32, h: i32, pixels: &[u16]) {
        let (w, h) = (w.max(0) as usize, h.max(0) as usize);
        let x0 = (x as usize) & (VRAM_WIDTH - 1);
        for (row, line) in pixels.chunks(w.max(1)).take(h).enumerate() {
            let vy = ((y as usize).wrapping_add(row)) & (VRAM_HEIGHT - 1);
            if x0 + line.len() <= VRAM_WIDTH {
                self.pixels[vy * VRAM_WIDTH + x0..][..line.len()].copy_from_slice(line);
            } else {
                for (i, &px) in line.iter().enumerate() {
                    self.set(x + i as i32, vy as i32, px);
                }
            }
        }
    }

    /// Reads a `w` x `h` image at (x, y) into `out`, wrapping at the VRAM's
    /// edges.
    pub fn read_rect(&self, x: i32, y: i32, w: i32, h: i32, out: &mut [u16]) {
        let (w, h) = (w.max(0) as usize, h.max(0) as usize);
        let x0 = (x as usize) & (VRAM_WIDTH - 1);
        for (row, line) in out.chunks_mut(w.max(1)).take(h).enumerate() {
            let vy = ((y as usize).wrapping_add(row)) & (VRAM_HEIGHT - 1);
            if x0 + line.len() <= VRAM_WIDTH {
                line.copy_from_slice(&self.pixels[vy * VRAM_WIDTH + x0..][..line.len()]);
            } else {
                for (i, px) in line.iter_mut().enumerate() {
                    *px = self.get(x + i as i32, vy as i32);
                }
            }
        }
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

    #[allow(clippy::too_many_arguments)]
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

    pub fn texel(&self, st: &DrawState, sh: &Shading, u: i32, v: i32) -> u16 {
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
    fn plot(
        &mut self,
        st: &DrawState,
        sh: &Shading,
        x: i32,
        y: i32,
        r: i32,
        g: i32,
        b: i32,
        u: i32,
        v: i32,
        dither: bool,
    ) {
        if x < st.area_x1 || x > st.area_x2 || y < st.area_y1 || y > st.area_y2 {
            return;
        }
        if sh.textured {
            self.shade::<true>(st, sh, x, y, r, g, b, u, v, dither);
        } else {
            self.shade::<false>(st, sh, x, y, r, g, b, u, v, dither);
        }
    }

    /// `plot` for a pixel known to be inside the drawing area, built once
    /// for textured and once for untextured primitives.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    fn shade<const TEX: bool>(
        &mut self,
        st: &DrawState,
        sh: &Shading,
        x: i32,
        y: i32,
        r: i32,
        g: i32,
        b: i32,
        u: i32,
        v: i32,
        dither: bool,
    ) {
        let dst = self.get(x, y);
        if st.check_mask && dst & 0x8000 != 0 {
            return;
        }

        let (mut cr, mut cg, mut cb);
        let mut mask_bit = 0u16;
        let mut blend = sh.semi_transparent;
        if TEX {
            let t = self.texel(st, sh, u, v);
            if t == 0 {
                return; // fully transparent texel
            }
            mask_bit = t & 0x8000;
            blend = blend && mask_bit != 0;
            let (tr, tg, tb) =
                (i32::from(t & 0x1F) << 3, i32::from((t >> 5) & 0x1F) << 3, i32::from((t >> 10) & 0x1F) << 3);
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
        let out =
            (r5 as u16) | ((g5 as u16) << 5) | ((b5 as u16) << 10) | mask_bit | if st.set_mask { 0x8000 } else { 0 };
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

        // Edge functions are linear in x: each row's covered span follows
        // from the three of them. Top-left rule: pixels on a right or
        // bottom edge are out, so such an edge needs w >= 1.
        let edges = [(b, c), (c, a), (a, b)].map(|(p, q)| (-(q.y - p.y), i32::from(!top_left(p, q))));
        for y in y0..=y1 {
            let (mut lo, mut hi) = (x0, x1);
            for (k, &(e, t)) in edges.iter().enumerate() {
                let (p, q) = [(b, c), (c, a), (a, b)][k];
                let w = edge(p, q, x0, y);
                if e > 0 {
                    lo = lo.max(x0 + (t - w + e - 1).div_euclid(e));
                } else if e < 0 {
                    hi = hi.min(x0 + (w - t).div_euclid(-e));
                } else if w < t {
                    hi = lo - 1;
                }
            }
            if lo > hi {
                continue;
            }
            match (sh.textured, sh.gouraud) {
                (true, true) => self.span::<true, true>(st, sh, [a, b, c], area, &edges, y, lo, hi, dither),
                (true, false) => self.span::<true, false>(st, sh, [a, b, c], area, &edges, y, lo, hi, dither),
                (false, true) => self.span::<false, true>(st, sh, [a, b, c], area, &edges, y, lo, hi, dither),
                (false, false) => self.span::<false, false>(st, sh, [a, b, c], area, &edges, y, lo, hi, dither),
            }
        }
    }

    /// Draws one row of a triangle, x from `lo` to `hi` (inside the drawing
    /// area), built once per shading kind so the pixel loop carries only
    /// what that kind needs.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    fn span<const TEX: bool, const GOURAUD: bool>(
        &mut self,
        st: &DrawState,
        sh: &Shading,
        [a, b, c]: [Vertex; 3],
        area: i32,
        edges: &[(i32, i32); 3],
        y: i32,
        lo: i32,
        hi: i32,
        dither: bool,
    ) {
        // Each attribute is (w0*fa + w1*fb + w2*fc) / area: a linear
        // numerator, never negative inside (weights and values are not),
        // so its truncated quotient steps along x with a remainder,
        // exactly, without dividing per pixel.
        let (w0, w1, w2) = (edge(b, c, lo, y), edge(c, a, lo, y), edge(a, b, lo, y));
        let (s0, s1, s2) = (edges[0].0, edges[1].0, edges[2].0);
        let start = |fa: i32, fb: i32, fc: i32| {
            let n = i64::from(w0) * i64::from(fa) + i64::from(w1) * i64::from(fb) + i64::from(w2) * i64::from(fc);
            let d = i64::from(s0) * i64::from(fa) + i64::from(s1) * i64::from(fb) + i64::from(s2) * i64::from(fc);
            Step::new(n, d, i64::from(area))
        };
        let mut attrs = [Step::default(); 5];
        if GOURAUD {
            attrs[0] = start(a.r, b.r, c.r);
            attrs[1] = start(a.g, b.g, c.g);
            attrs[2] = start(a.b, b.b, c.b);
        }
        if TEX {
            attrs[3] = start(a.u, b.u, c.u);
            attrs[4] = start(a.v, b.v, c.v);
        }
        for x in lo..=hi {
            let (r, g, bl) =
                if GOURAUD { (attrs[0].q as i32, attrs[1].q as i32, attrs[2].q as i32) } else { (a.r, a.g, a.b) };
            let (u, vv) = if TEX { (attrs[3].q as i32, attrs[4].q as i32) } else { (0, 0) };
            self.shade::<TEX>(st, sh, x, y, r, g, bl, u, vv, dither);
            if GOURAUD {
                attrs[0].advance();
                attrs[1].advance();
                attrs[2].advance();
            }
            if TEX {
                attrs[3].advance();
                attrs[4].advance();
            }
        }
    }

    /// GP0(60h-7Fh) rectangle / sprite at (x, y), `w` x `h`, with texture
    /// origin (u, v).
    #[allow(clippy::too_many_arguments)]
    pub fn rectangle(
        &mut self,
        st: &DrawState,
        sh: &Shading,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: (i32, i32, i32),
        u: i32,
        v: i32,
    ) {
        let (dx0, dx1) = ((st.area_x1 - x).max(0), (st.area_x2 - x + 1).min(w));
        let (dy0, dy1) = ((st.area_y1 - y).max(0), (st.area_y2 - y + 1).min(h));
        for dy in dy0..dy1 {
            let tv = if st.rect_flip_y { v - dy } else { v + dy };
            for dx in dx0..dx1 {
                let tu = if st.rect_flip_x { u - dx } else { u + dx };
                if sh.textured {
                    self.shade::<true>(st, sh, x + dx, y + dy, color.0, color.1, color.2, tu, tv, false);
                } else {
                    self.shade::<false>(st, sh, x + dx, y + dy, color.0, color.1, color.2, tu, tv, false);
                }
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

/// floor(n / d) as n goes up by `step` at a time (d > 0): the quotient,
/// and the remainder in 0..d.
#[derive(Clone, Copy, Default)]
struct Step {
    q: i64,
    r: i64,
    dq: i64,
    dr: i64,
    d: i64,
}

impl Step {
    fn new(n: i64, step: i64, d: i64) -> Step {
        Step { q: n.div_euclid(d), r: n.rem_euclid(d), dq: step.div_euclid(d), dr: step.rem_euclid(d), d }
    }

    #[inline]
    fn advance(&mut self) {
        self.q += self.dq;
        self.r += self.dr;
        if self.r >= self.d {
            self.r -= self.d;
            self.q += 1;
        }
    }
}

#[inline]
fn edge(a: Vertex, b: Vertex, x: i32, y: i32) -> i32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}

/// Whether pixels exactly on edge a->b belong to the triangle. The GPU draws
/// a polygon's top and left edges and leaves out its bottom and right ones.
/// With the triangle oriented so that `edge` is positive inside, the
/// interior lies to the right of a "left" edge (dE/dx > 0, so b.y < a.y) and
/// below a horizontal "top" edge (dE/dy > 0, so b.x > a.x).
#[inline]
fn top_left(a: Vertex, b: Vertex) -> bool {
    (a.y == b.y && b.x > a.x) || b.y < a.y
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

#[cfg(test)]
mod tests {
    use super::*;

    fn flat() -> Shading {
        Shading {
            textured: false,
            raw_texture: false,
            gouraud: false,
            semi_transparent: false,
            clut_x: 0,
            clut_y: 0,
            uv_limits: None,
        }
    }

    fn state() -> DrawState {
        DrawState { area_x2: 1023, area_y2: 511, ..DrawState::default() }
    }

    fn v(x: i32, y: i32) -> Vertex {
        Vertex { x, y, r: 255, g: 255, b: 255, ..Vertex::default() }
    }

    /// The rasterizer before span stepping: every pixel of the bounding box
    /// tested, the same per-pixel arithmetic.
    fn reference_triangle(vram: &mut Vram, st: &DrawState, sh: &Shading, v: [Vertex; 3]) {
        let [a, b, c] = v;
        let (min_x, max_x) = (a.x.min(b.x).min(c.x), a.x.max(b.x).max(c.x));
        let (min_y, max_y) = (a.y.min(b.y).min(c.y), a.y.max(b.y).max(c.y));
        if max_x - min_x > 1023 || max_y - min_y > 511 {
            return;
        }
        let area = edge(a, b, c.x, c.y);
        if area == 0 {
            return;
        }
        let (b, c, area) = if area < 0 { (c, b, -area) } else { (b, c, area) };
        let dither = sh.gouraud || (sh.textured && !sh.raw_texture);
        for y in min_y.max(st.area_y1)..=max_y.min(st.area_y2) {
            for x in min_x.max(st.area_x1)..=max_x.min(st.area_x2) {
                let (w0, w1, w2) = (edge(b, c, x, y), edge(c, a, x, y), edge(a, b, x, y));
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
                vram.plot(st, sh, x, y, r, g, bl, u, vv, dither);
            }
        }
    }

    /// Timing of textured gouraud triangles: `cargo test --release -p
    /// pe2-host raster_speed -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn raster_speed() {
        let st = DrawState { area_x2: 1023, area_y2: 511, dither: true, tex_depth: 2, ..DrawState::default() };
        let sh = Shading { gouraud: true, textured: true, ..flat() };
        let mut seed = 0x1357_9BDFu32;
        let mut rnd = |n: i32| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed % n as u32) as i32
        };
        let tris: Vec<[Vertex; 3]> = (0..20_000)
            .map(|_| {
                let (cx, cy) = (rnd(300), rnd(220));
                std::array::from_fn(|_| Vertex {
                    x: cx + rnd(60),
                    y: cy + rnd(60),
                    r: rnd(256),
                    g: rnd(256),
                    b: rnd(256),
                    u: rnd(256),
                    v: rnd(256),
                    ..Vertex::default()
                })
            })
            .collect();
        let mut vram = Vram::default();
        let t = std::time::Instant::now();
        for v in &tris {
            vram.triangle(&st, &sh, *v);
        }
        eprintln!("raster_speed: {:?} for {} triangles", t.elapsed(), tris.len());
    }

    /// Span stepping draws exactly the pixels, and values, of testing every
    /// pixel of the bounding box.
    #[test]
    fn spans_match_reference() {
        let st = DrawState { area_x1: 3, area_y1: 5, area_x2: 200, area_y2: 150, dither: true, ..DrawState::default() };
        let sh = Shading { gouraud: true, ..flat() };
        let mut seed = 0x9E37_79B9u32;
        let mut rnd = |n: i32| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed % n as u32) as i32
        };
        // A texture page of non-transparent texels, for the textured runs.
        let mut textured_vram = Vram::default();
        for (i, p) in textured_vram.pixels.iter_mut().enumerate() {
            *p = (i as u16).wrapping_mul(40503) | 1;
        }
        let tex_st = DrawState { tex_x: 512, tex_y: 0, tex_depth: 2, ..st };
        for k in 0..6000 {
            let mut vert = || Vertex {
                x: rnd(240) - 20,
                y: rnd(190) - 20,
                r: rnd(256),
                g: rnd(256),
                b: rnd(256),
                u: rnd(256),
                v: rnd(256),
                ..Vertex::default()
            };
            let v = [vert(), vert(), vert()];
            let (st, sh, base) = match k % 3 {
                0 => (st, sh, Vram::default()),
                1 => (tex_st, Shading { textured: true, ..sh }, textured_vram.clone()),
                _ => (tex_st, Shading { textured: true, gouraud: false, ..sh }, textured_vram.clone()),
            };
            let (mut got, mut want) = (base.clone(), base);
            got.triangle(&st, &sh, v);
            reference_triangle(&mut want, &st, &sh, v);
            assert!(got.pixels == want.pixels, "{v:?}");
        }
    }

    /// Rectangles clipped to the drawing area up front draw what testing
    /// every pixel against it draws.
    #[test]
    fn rectangles_match_reference() {
        let st = DrawState {
            area_x1: 3,
            area_y1: 5,
            area_x2: 200,
            area_y2: 150,
            tex_depth: 2,
            tex_x: 512,
            ..DrawState::default()
        };
        let mut base = Vram::default();
        for (i, p) in base.pixels.iter_mut().enumerate() {
            *p = (i as u16).wrapping_mul(40503) | 1;
        }
        for (k, &(x, y, w, h)) in
            [(-10, -10, 40, 30), (190, 140, 30, 30), (50, 60, 16, 16), (0, 0, 256, 256), (201, 10, 5, 5)]
                .iter()
                .enumerate()
        {
            let sh = Shading { textured: k % 2 == 0, raw_texture: true, ..flat() };
            let st = DrawState { rect_flip_x: k == 2, ..st };
            let (mut got, mut want) = (base.clone(), base.clone());
            got.rectangle(&st, &sh, x, y, w, h, (100, 150, 200), 7, 9);
            for dy in 0..h {
                for dx in 0..w {
                    let tu = if st.rect_flip_x { 7 - dx } else { 7 + dx };
                    want.plot(&st, &sh, x + dx, y + dy, 100, 150, 200, tu, 9 + dy, false);
                }
            }
            assert!(got.pixels == want.pixels, "{x} {y} {w} {h}");
        }
    }

    /// Row-by-row image writes and reads match pixel-by-pixel ones,
    /// wrapping at the VRAM's edges included.
    #[test]
    fn rect_transfers_match_pixels() {
        for &(x, y, w, h) in &[(0, 0, 320, 240), (1000, 500, 40, 20), (-8, 3, 16, 4), (700, 100, 1, 1), (5, 5, 0, 3)] {
            let pixels: Vec<u16> = (0..w * h).map(|i| (i as u16).wrapping_mul(7919) | 1).collect();
            let (mut got, mut want) = (Vram::default(), Vram::default());
            got.write_rect(x, y, w, h, &pixels);
            for (i, &px) in pixels.iter().enumerate() {
                want.set(x + i as i32 % w, y + i as i32 / w, px);
            }
            assert!(got.pixels == want.pixels, "write {x},{y} {w}x{h}");
            let mut back = vec![0u16; (w * h) as usize];
            got.read_rect(x, y, w, h, &mut back);
            assert_eq!(back, pixels, "read {x},{y} {w}x{h}");
        }
    }

    /// The GPU leaves out a polygon's right column and bottom row: a quad
    /// from (10,10) to (14,14) covers exactly x 10..=13, y 10..=13.
    #[test]
    fn quad_excludes_right_and_bottom_edges() {
        let (st, sh) = (state(), flat());
        let quads = [[v(10, 10), v(14, 10), v(10, 14), v(14, 14)], [v(14, 14), v(10, 14), v(14, 10), v(10, 10)]];
        for q in quads {
            let mut vram = Vram::default();
            vram.triangle(&st, &sh, [q[0], q[1], q[2]]);
            vram.triangle(&st, &sh, [q[1], q[2], q[3]]);
            for y in 8..17 {
                for x in 8..17 {
                    let inside = (10..14).contains(&x) && (10..14).contains(&y);
                    assert_eq!(vram.get(x, y) != 0, inside, "pixel ({x},{y}) for quad {q:?}");
                }
            }
        }
    }
}
