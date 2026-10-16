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

    /// A rectangle of native pixels set to one colour (ClearImage).
    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, rgb: u32) {
        // 24-bit colour in this layer's 5-bit-times-8 scale.
        let value = ((rgb & 0xFF) << 16) | (rgb & 0xFF00) | ((rgb >> 16) & 0xFF);
        let s = self.scale;
        for yy in y * s..(y + h) * s {
            for xx in x * s..(x + w) * s {
                let i = self.index(xx, yy);
                self.pixels[i] = value;
            }
        }
    }

    pub fn copy(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32, st: &DrawState) {
        let s = self.scale;
        let mut line = Vec::with_capacity((w * s) as usize);
        for row in 0..h * s {
            line.clear();
            line.extend((0..w * s).map(|col| self.pixels[self.index(sx * s + col, sy * s + row)]));
            for (col, px) in line.iter().enumerate() {
                let i = self.index(dx * s + col as i32, dy * s + row);
                if st.check_mask && self.pixels[i] & MASK != 0 {
                    continue;
                }
                self.pixels[i] = (px & !MASK) | if st.set_mask { MASK } else { px & MASK };
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

        // Edge functions step by constants along x and y.
        let (e0x, e1x, e2x) = (-i64::from(pc.1 - pb.1), -i64::from(pa.1 - pc.1), -i64::from(pb.1 - pa.1));
        let inv = 1.0 / area as f32;
        let attr = |fa: i32, fb: i32, fc: i32| (fa as f32 * inv, fb as f32 * inv, fc as f32 * inv);
        let (ra, ga, ba) = (attr(a.r, b.r, c.r), attr(a.g, b.g, c.g), attr(a.b, b.b, c.b));
        let (ua, va) = (attr(a.u, b.u, c.u), attr(a.v, b.v, c.v));
        let mix = |w: (f32, f32, f32), k: (f32, f32, f32)| w.0 * k.0 + w.1 * k.1 + w.2 * k.2;

        for y in y0..=y1 {
            let mut w0 = edge(pb, pc, x0, y);
            let mut w1 = edge(pc, pa, x0, y);
            let mut w2 = edge(pa, pb, x0, y);
            let row = y as usize * self.width;
            for x in x0..=x1 {
                let inside = (w0 > 0 || (w0 == 0 && tl0)) && (w1 > 0 || (w1 == 0 && tl1)) && (w2 > 0 || (w2 == 0 && tl2));
                if inside {
                    let w = (w0 as f32, w1 as f32, w2 as f32);
                    let (r, g, bl) = if sh.gouraud {
                        (mix(w, ra) as i32, mix(w, ga) as i32, mix(w, ba) as i32)
                    } else {
                        (a.r, a.g, a.b)
                    };
                    let (u, vv) = if sh.textured { (mix(w, ua).floor() as i32, mix(w, va).floor() as i32) } else { (0, 0) };
                    self.plot(vram, st, sh, row + x as usize, r, g, bl, u, vv);
                }
                w0 += e0x;
                w1 += e1x;
                w2 += e2x;
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
