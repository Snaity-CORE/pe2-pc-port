//! The console's VRAM as the CPU knows it, and the drawing state of the
//! GPU's commands (psx-spx). Drawing itself happens on the graphics card
//! (hw.rs); this VRAM holds what the game sends from the CPU (image
//! transfers, fills, copies of them), which textures and palettes come
//! from, and what the card drew once the game reads it back.

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
}
