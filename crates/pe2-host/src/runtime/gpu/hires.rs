//! Higher internal resolution (`PE2_SCALE=2..8`): a second framebuffer,
//! `scale` times the VRAM's size in each direction, that every drawing
//! operation also renders into and that the window shows.
//!
//! The native VRAM stays exactly as on the console: the game reads it back,
//! and textures and CLUTs are sampled from it. This layer only changes what
//! is seen. Its pixels are true colour, 8 bits per channel in the console's
//! scale (a 5-bit channel times 8) with no dithering, plus the mask bit;
//! uploads from the CPU are replicated, and primitives are rasterized at
//! the higher resolution.

use super::raster::{DrawState, Shading, VRAM_HEIGHT, VRAM_WIDTH, Vertex, Vram};

/// The mask bit (bit 15 of a VRAM pixel).
const MASK: u32 = 1 << 24;

pub struct Hires {
    pub scale: i32,
    pub width: usize,
    pub height: usize,
    /// 0x0MRRGGBB: mask bit, then 8-bit channels (5-bit channel * 8).
    pub pixels: Vec<u32>,
}

/// A 15-bit VRAM pixel in this layer's format.
#[inline]
pub fn from15(px: u16) -> u32 {
    let p = u32::from(px);
    ((p & 0x1F) << 19) | (((p >> 5) & 0x1F) << 11) | (((p >> 10) & 0x1F) << 3) | if p & 0x8000 != 0 { MASK } else { 0 }
}

#[inline]
fn channels(p: u32) -> (i32, i32, i32) {
    (((p >> 16) & 0xFF) as i32, ((p >> 8) & 0xFF) as i32, (p & 0xFF) as i32)
}

#[inline]
fn pack(r: i32, g: i32, b: i32) -> u32 {
    ((r.clamp(0, 255) as u32) << 16) | ((g.clamp(0, 255) as u32) << 8) | b.clamp(0, 255) as u32
}

impl Hires {
    pub fn new(scale: i32) -> Hires {
        let (width, height) = (VRAM_WIDTH * scale as usize, VRAM_HEIGHT * scale as usize);
        Hires { scale, width, height, pixels: vec![0; width * height] }
    }

    #[inline]
    fn index(&self, x: i32, y: i32) -> usize {
        y.rem_euclid(self.height as i32) as usize * self.width + x.rem_euclid(self.width as i32) as usize
    }

    /// Sets the `scale` x `scale` block of native pixel (x, y).
    fn set_block(&mut self, x: i32, y: i32, value: u32, st: Option<&DrawState>) {
        let s = self.scale;
        for dy in 0..s {
            for dx in 0..s {
                let i = self.index(x * s + dx, y * s + dy);
                if let Some(st) = st {
                    if st.check_mask && self.pixels[i] & MASK != 0 {
                        continue;
                    }
                    self.pixels[i] = value | if st.set_mask { MASK } else { 0 };
                } else {
                    self.pixels[i] = value;
                }
            }
        }
    }

    /// A native pixel written by the CPU (image transfers), with the mask
    /// settings when the transfer honours them.
    pub fn upload(&mut self, x: i32, y: i32, px: u16, st: Option<&DrawState>) {
        self.set_block(x, y, from15(px), st);
    }

    /// GP0(02h) fill, with the native command's coordinate rounding.
    pub fn fill_command(&mut self, x: i32, y: i32, w: i32, h: i32, rgb: u32) {
        let w = ((w & 0x3FF) + 0xF) & !0xF;
        self.fill(x & 0x3F0, y & 0x1FF, w, h & 0x1FF, rgb);
    }

    /// An image transfer that ignores the mask settings (LoadImage): `w` x
    /// `h` native pixels, each replicated over its block.
    pub fn upload_rect(&mut self, x: i32, y: i32, w: i32, h: i32, pixels: &[u16]) {
        let s = self.scale as usize;
        let w = w.max(0) as usize;
        let mut line = Vec::with_capacity(w * s);
        for row in 0..h.max(0) as usize {
            let Some(src) = pixels.get(row * w..(row + 1) * w) else { return };
            line.clear();
            for &px in src {
                let v = from15(px);
                line.extend(std::iter::repeat_n(v, s));
            }
            for k in 0..s {
                let mut done = 0;
                for (start, n) in self.segments((y + row as i32) * self.scale + k as i32, x * self.scale, line.len()) {
                    self.pixels[start..start + n].copy_from_slice(&line[done..done + n]);
                    done += n;
                }
            }
        }
    }

    /// A rectangle of native pixels set to one colour (ClearImage).
    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, rgb: u32) {
        // 24-bit colour in this layer's 5-bit-times-8 scale.
        let value = ((rgb & 0xFF) << 16) | (rgb & 0xFF00) | ((rgb >> 16) & 0xFF);
        let s = self.scale;
        for yy in y * s..(y + h) * s {
            for (start, len) in self.segments(yy, x * s, (w * s).max(0) as usize) {
                self.pixels[start..start + len].fill(value);
            }
        }
    }

    /// Up to two runs of pixel indices (wrapping at the right edge) for
    /// `len` pixels of row `y` from column `x`.
    fn segments(&self, y: i32, x: i32, len: usize) -> [(usize, usize); 2] {
        let len = len.min(self.width);
        let row = y.rem_euclid(self.height as i32) as usize * self.width;
        let x = x.rem_euclid(self.width as i32) as usize;
        let first = len.min(self.width - x);
        [(row + x, first), (row, len - first)]
    }

    pub fn copy(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32, st: &DrawState) {
        let s = self.scale;
        let len = ((w * s).max(0) as usize).min(self.width);
        let mut line = Vec::with_capacity(len);
        for row in 0..h * s {
            line.clear();
            for (start, n) in self.segments(sy * s + row, sx * s, len) {
                line.extend_from_slice(&self.pixels[start..start + n]);
            }
            let mut k = 0;
            for (start, n) in self.segments(dy * s + row, dx * s, len) {
                let dst = &mut self.pixels[start..start + n];
                for (d, &px) in dst.iter_mut().zip(&line[k..k + n]) {
                    if st.check_mask && *d & MASK != 0 {
                        continue;
                    }
                    *d = (px & !MASK) | if st.set_mask { MASK } else { px & MASK };
                }
                k += n;
            }
        }
    }

    /// Shades and writes one pixel of this layer. `r g b` are 8-bit vertex
    /// colours; (u, v) the texel.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn plot(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, i: usize, r: i32, g: i32, b: i32, u: i32, v: i32) {
        let dst = self.pixels[i];
        if st.check_mask && dst & MASK != 0 {
            return;
        }
        let (mut cr, mut cg, mut cb);
        let mut mask = 0;
        let mut blend = sh.semi_transparent;
        if sh.textured {
            let t = vram.texel(st, sh, u, v);
            if t == 0 {
                return;
            }
            if t & 0x8000 != 0 {
                mask = MASK;
            }
            blend = blend && mask != 0;
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
        (cr, cg, cb) = (cr.clamp(0, 255), cg.clamp(0, 255), cb.clamp(0, 255));
        if blend {
            let (br, bg, bb) = channels(dst);
            let mix = |b: i32, f: i32| -> i32 {
                match st.semi_mode {
                    0 => (b + f) >> 1,
                    1 => b + f,
                    2 => b - f,
                    _ => b + (f >> 2),
                }
            };
            (cr, cg, cb) = (mix(br, cr), mix(bg, cg), mix(bb, cb));
        }
        self.pixels[i] = pack(cr, cg, cb) | mask | if st.set_mask { MASK } else { 0 };
    }

    /// The drawing area in this layer's pixels, inclusive.
    fn area(&self, st: &DrawState) -> (i32, i32, i32, i32) {
        let s = self.scale;
        (st.area_x1 * s, st.area_y1 * s, (st.area_x2 + 1) * s - 1, (st.area_y2 + 1) * s - 1)
    }

    /// A triangle in native coordinates (the native rasterizer has already
    /// rejected oversized ones), rasterized at this layer's resolution.
    pub fn triangle(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, v: [Vertex; 3]) {
        // The GPU skips polygons larger than 1023x511.
        let (min_x, max_x) = (v[0].x.min(v[1].x).min(v[2].x), v[0].x.max(v[1].x).max(v[2].x));
        let (min_y, max_y) = (v[0].y.min(v[1].y).min(v[2].y), v[0].y.max(v[1].y).max(v[2].y));
        if max_x - min_x > 1023 || max_y - min_y > 511 {
            return;
        }
        let s = self.scale;
        let p = |q: Vertex| (q.x * s, q.y * s);
        let (a, mut b, mut c) = (v[0], v[1], v[2]);
        let edge = |p0: (i32, i32), p1: (i32, i32), x: i32, y: i32| -> i64 {
            i64::from(p1.0 - p0.0) * i64::from(y - p0.1) - i64::from(p1.1 - p0.1) * i64::from(x - p0.0)
        };
        let mut area = edge(p(a), p(b), p(c).0, p(c).1);
        if area == 0 {
            return;
        }
        if area < 0 {
            std::mem::swap(&mut b, &mut c);
            area = -area;
        }
        let (pa, pb, pc) = (p(a), p(b), p(c));
        let top_left = |p0: (i32, i32), p1: (i32, i32)| -> bool { (p1.1 < p0.1) || (p1.1 == p0.1 && p1.0 > p0.0) };
        let (tl0, tl1, tl2) = (top_left(pb, pc), top_left(pc, pa), top_left(pa, pb));

        let (ax1, ay1, ax2, ay2) = self.area(st);
        let x0 = pa.0.min(pb.0).min(pc.0).max(ax1).max(0);
        let x1 = pa.0.max(pb.0).max(pc.0).min(ax2).min(self.width as i32 - 1);
        let y0 = pa.1.min(pb.1).min(pc.1).max(ay1).max(0);
        let y1 = pa.1.max(pb.1).max(pc.1).min(ay2).min(self.height as i32 - 1);
        if x0 > x1 || y0 > y1 {
            return;
        }

        // Edge functions are linear in x: each row's covered span follows
        // from the three of them, with the same top-left rule (an edge's
        // own pixels count only on top and left edges).
        let (e0x, e1x, e2x) = (-i64::from(pc.1 - pb.1), -i64::from(pa.1 - pc.1), -i64::from(pb.1 - pa.1));
        let (t0, t1, t2) = (i64::from(!tl0), i64::from(!tl1), i64::from(!tl2));
        // Attributes as planes: value at a row's first pixel, then a step.
        let area_f = area as f64;
        let plane = |fa: i32, fb: i32, fc: i32| {
            let step = (e0x as f64 * f64::from(fa) + e1x as f64 * f64::from(fb) + e2x as f64 * f64::from(fc)) / area_f;
            (f64::from(fa), f64::from(fb), f64::from(fc), step)
        };
        let at = |k: (f64, f64, f64, f64), w0: i64, w1: i64, w2: i64| {
            (w0 as f64 * k.0 + w1 as f64 * k.1 + w2 as f64 * k.2) / area_f
        };
        let (rp, gp, bp) = (plane(a.r, b.r, c.r), plane(a.g, b.g, c.g), plane(a.b, b.b, c.b));
        let (up, vp) = (plane(a.u, b.u, c.u), plane(a.v, b.v, c.v));

        for y in y0..=y1 {
            let w0 = edge(pb, pc, x0, y);
            let w1 = edge(pc, pa, x0, y);
            let w2 = edge(pa, pb, x0, y);
            // Columns (relative to x0) where w + e * dx >= t.
            let (mut lo, mut hi) = (0i64, i64::from(x1 - x0));
            for (w, e, t) in [(w0, e0x, t0), (w1, e1x, t1), (w2, e2x, t2)] {
                if e > 0 {
                    lo = lo.max((t - w + e - 1).div_euclid(e));
                } else if e < 0 {
                    hi = hi.min((w - t).div_euclid(-e));
                } else if w < t {
                    hi = -1;
                }
            }
            if lo > hi {
                continue;
            }
            let (ws0, ws1, ws2) = (w0 + e0x * lo, w1 + e1x * lo, w2 + e2x * lo);
            let row = y as usize * self.width;
            let (mut r, mut g, mut bl) = (at(rp, ws0, ws1, ws2), at(gp, ws0, ws1, ws2), at(bp, ws0, ws1, ws2));
            let (mut u, mut vv) = (at(up, ws0, ws1, ws2), at(vp, ws0, ws1, ws2));
            for x in x0 + lo as i32..=x0 + hi as i32 {
                let (cr, cg, cb) = if sh.gouraud { (r as i32, g as i32, bl as i32) } else { (a.r, a.g, a.b) };
                let (tu, tv) = if sh.textured { (u.floor() as i32, vv.floor() as i32) } else { (0, 0) };
                self.plot(vram, st, sh, row + x as usize, cr, cg, cb, tu, tv);
                r += rp.3;
                g += gp.3;
                bl += bp.3;
                u += up.3;
                vv += vp.3;
            }
        }
    }

    /// A sprite in native coordinates; each texel covers `scale` x `scale`
    /// pixels.
    #[allow(clippy::too_many_arguments)]
    pub fn rectangle(
        &mut self,
        vram: &Vram,
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
        let s = self.scale;
        let (ax1, ay1, ax2, ay2) = self.area(st);
        for yy in (y * s).max(ay1).max(0)..((y + h) * s).min(ay2 + 1).min(self.height as i32) {
            let dy = (yy - y * s).div_euclid(s);
            let tv = if st.rect_flip_y { v - dy } else { v + dy };
            for xx in (x * s).max(ax1).max(0)..((x + w) * s).min(ax2 + 1).min(self.width as i32) {
                let dx = (xx - x * s).div_euclid(s);
                let tu = if st.rect_flip_x { u - dx } else { u + dx };
                let i = yy as usize * self.width + xx as usize;
                self.plot(vram, st, sh, i, color.0, color.1, color.2, tu, tv);
            }
        }
    }

    /// A line in native coordinates: the native rasterizer's pixels, each
    /// a `scale` x `scale` block.
    pub fn line(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, a: Vertex, b: Vertex) {
        let s = self.scale;
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        if dx.abs() > 1023 || dy.abs() > 511 {
            return;
        }
        let steps = dx.abs().max(dy.abs());
        let (ax1, ay1, ax2, ay2) = self.area(st);
        for i in 0..=steps {
            let t = |p: i32, q: i32| if steps == 0 { p } else { p + (q - p) * i / steps };
            let (x, y) = (t(a.x, b.x), t(a.y, b.y));
            let (r, g, bl) = if sh.gouraud { (t(a.r, b.r), t(a.g, b.g), t(a.b, b.b)) } else { (a.r, a.g, a.b) };
            for py in (y * s).max(ay1).max(0)..((y + 1) * s).min(ay2 + 1).min(self.height as i32) {
                for px in (x * s).max(ax1).max(0)..((x + 1) * s).min(ax2 + 1).min(self.width as i32) {
                    let idx = py as usize * self.width + px as usize;
                    self.plot(vram, st, sh, idx, r, g, bl, 0, 0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// At scale 1 this layer covers exactly the native rasterizer's pixels.
    #[test]
    fn coverage_matches_native_at_scale_1() {
        let st = DrawState { area_x1: 3, area_y1: 5, area_x2: 200, area_y2: 150, ..DrawState::default() };
        let sh = Shading { textured: false, raw_texture: false, gouraud: false, semi_transparent: false, clut_x: 0, clut_y: 0 };
        let mut seed = 0x1234_5678u32;
        let mut rnd = |n: i32| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed % n as u32) as i32 - 20
        };
        for _ in 0..3000 {
            let mut native = Vram::default();
            let mut layer = Hires::new(1);
            let mut vert = || Vertex { x: rnd(240), y: rnd(190), r: 255, g: 255, b: 255, ..Vertex::default() };
            let v = [vert(), vert(), vert()];
            native.triangle(&st, &sh, v);
            layer.triangle(&native, &st, &sh, v);
            for y in 0..200 {
                for x in 0..260 {
                    let n = native.get(x, y) != 0;
                    let h = layer.pixels[y as usize * layer.width + x as usize] != 0;
                    assert_eq!(n, h, "pixel ({x},{y}) of {v:?}");
                }
            }
        }
    }
}
