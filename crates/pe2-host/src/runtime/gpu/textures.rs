//! Mods' textures: which texture a primitive draws from.
//!
//! On the PlayStation a texture is texture memory read through a palette.
//! A texture here is a texture page (256x256 texels: 64, 128 or 256
//! halfwords wide for 4-, 8- and 15-bit texels) as image transfers loaded
//! it, drawn with one palette: known by a hash of the page's loaded
//! contents and one of the palette, what mods replace (docs/MODDING.md) and
//! dumping saves. Only memory image transfers loaded counts: each row of
//! each 64-halfword column is marked loaded by a transfer covering it and
//! unmarked by any other write (drawing, copies, fills), so palettes and
//! drawing next to textures leave them be, and a primitive is replaced
//! only when all it reads was loaded.
//!
//! A page often holds many textures, each drawn with its own palette (or
//! texel depth): seen through another palette, they are noise. A texture's
//! dump shows its whole page but for the texture memory primitives drew
//! with another palette and never with its own, and is saved again as
//! drawing tells more.

use super::raster::{DrawState, Shading, Vram};
use crate::mods::TextureKey;

/// Columns of 64 halfwords; rows.
const COLUMNS: i32 = 16;
const ROWS: i32 = 512;

/// Image transfers at least this large are screen images (backgrounds,
/// pictures), which mods replace whole (mods::background).
const SCREEN: (i32, i32) = (256, 192);

const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;

fn fnv(h: u64, v: u16) -> u64 {
    let h = (h ^ u64::from(v & 0xFF)).wrapping_mul(0x0100_0000_01B3);
    (h ^ u64::from(v >> 8)).wrapping_mul(0x0100_0000_01B3)
}

/// Where a primitive's texture is: its key, and its page in texels.
#[derive(Clone, Copy)]
pub struct Found {
    pub key: TextureKey,
    pub origin_x: i32,
    pub origin_y: i32,
    pub width: i32,
    pub height: i32,
}

/// A texture being dumped: where its page is (x, y in halfwords, and
/// width), the halfwords of it drawn with it so far (a bit each, 256 rows
/// of 4 words), whether what is known about it grew since it was saved, and
/// when it was.
struct Dump {
    page: (i32, i32, i32),
    drawn: Vec<u64>,
    grew: bool,
    saved: Option<std::time::Instant>,
}

impl Dump {
    fn drew(&self, x: i32, y: i32) -> bool {
        let (hx, hy) = ((x - self.page.0) as usize, (y - self.page.1) as usize);
        self.drawn[hy * 4 + hx / 64] & (1 << (hx % 64)) != 0
    }
}

/// A growing dump is saved again at most this often.
const RESAVE: std::time::Duration = std::time::Duration::from_secs(1);

pub struct Tracker {
    /// Per row, a bit per column: loaded by an image transfer.
    loaded: Vec<u16>,
    /// Textures being dumped, and the VRAM halfwords primitives drew any
    /// texture from (a bit each).
    dumps: std::collections::HashMap<TextureKey, Dump>,
    claimed: Vec<u64>,
    /// Page hashes by place and width in columns, while unchanged.
    pages: Vec<(i32, i32, i32, u64)>,
    /// Palette hashes by place and size, while unchanged.
    palettes: Vec<(i32, i32, i32, u64)>,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            loaded: vec![0; ROWS as usize],
            dumps: Default::default(),
            claimed: Vec::new(),
            pages: Vec::new(),
            palettes: Vec::new(),
        }
    }
}

/// The rows `y..y + h` of VRAM (wrapping at its bottom).
fn rows(y: i32, h: i32) -> impl Iterator<Item = usize> {
    (y..y + h.min(ROWS)).map(|r| r.rem_euclid(ROWS) as usize)
}

/// Bits for columns `c0..=c1`.
fn column_bits(c0: i32, c1: i32) -> u16 {
    ((1u32 << (c1 + 1)) - (1u32 << c0)) as u16
}

impl Tracker {
    /// The native VRAM changed at `x, y, w, h`: what was loaded there is
    /// gone.
    pub fn write(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w <= 0 || h <= 0 {
            return;
        }
        let bits = column_bits((x / 64).clamp(0, COLUMNS - 1), ((x + w - 1) / 64).clamp(0, COLUMNS - 1));
        for r in rows(y, h) {
            self.loaded[r] &= !bits;
        }
        if !self.claimed.is_empty() {
            for r in rows(y, h) {
                for c in x.max(0)..(x + w).min(1024) {
                    let i = r * 1024 + c as usize;
                    self.claimed[i / 64] &= !(1 << (i % 64));
                }
            }
        }
        let hits = |px: i32, py: i32, pw: i32, ph: i32| px < x + w && x < px + pw && py < y + h && y < py + ph;
        self.pages.retain(|&(px, py, cols, _)| !hits(px, py, cols * 64, 256));
        self.palettes.retain(|&(px, py, n, _)| !hits(px, py, n, 1));
    }

    /// An image transfer to `x, y, w, h` (after `write`): the columns it
    /// covers whole are loaded.
    pub fn upload(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w >= SCREEN.0 && h >= SCREEN.1 {
            return;
        }
        let (c0, c1) = ((x + 63) / 64, (x + w) / 64);
        if c1 <= c0 || c0 < 0 || c1 > COLUMNS {
            return;
        }
        let bits = column_bits(c0, c1 - 1);
        for r in rows(y, h) {
            self.loaded[r] |= bits;
        }
    }

    /// The texture a textured primitive draws from, if all it reads was
    /// loaded; `uv` is its texel rectangle (u0, v0, u1, v1), inclusive.
    pub fn find(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, uv: (i32, i32, i32, i32)) -> Option<Found> {
        // A texture window repeats part of the page: not one texture.
        if st.win_mask_x != 0 || st.win_mask_y != 0 {
            return None;
        }
        let depth = st.tex_depth.min(2);
        let per = [4, 2, 1][depth as usize];
        let cols = [1, 2, 4][depth as usize];
        let (u0, v0, u1, v1) = (uv.0.clamp(0, 255), uv.1.clamp(0, 255), uv.2.clamp(0, 255), uv.3.clamp(0, 255));
        let (px, py) = (st.tex_x, st.tex_y);
        if px % 64 != 0 || px / 64 + cols > COLUMNS || py + 256 > ROWS {
            return None;
        }
        let bits = column_bits((px + u0 / per) / 64, (px + u1 / per) / 64);
        if rows(py + v0, v1 - v0 + 1).any(|r| self.loaded[r] & bits != bits) {
            return None;
        }
        let hash = self.page(vram, px, py, cols);
        let palette =
            if depth < 2 { self.palette(vram, sh.clut_x, sh.clut_y, if depth == 0 { 16 } else { 256 }) } else { 0 };
        let found = Found { key: (hash, palette), origin_x: px * per, origin_y: py, width: 256, height: 256 };
        if crate::mods::dumping() {
            self.dump(vram, found.key, (u0 / per, v0, u1 / per, v1), (px, py, depth), (sh.clut_x, sh.clut_y));
        }
        Some(found)
    }

    /// Notes the halfwords `area` (x0, y0, x1, y1 in the page, inclusive)
    /// drawn with texture `key`, and saves its dump if what is known about
    /// it grew.
    fn dump(
        &mut self,
        vram: &Vram,
        key: TextureKey,
        area: (i32, i32, i32, i32),
        (px, py, depth): (i32, i32, u32),
        (cx, cy): (i32, i32),
    ) {
        if self.claimed.is_empty() {
            self.claimed = vec![0; 1024 * 512 / 64];
        }
        let cols = [1, 2, 4][depth as usize];
        let d = self.dumps.entry(key).or_insert_with(|| Dump {
            page: (px, py, cols * 64),
            drawn: vec![0; 256 * 4],
            grew: true,
            saved: None,
        });
        let (x0, y0, x1, y1) = area;
        let mut claimed_more = false;
        for v in y0..=y1 {
            for hx in x0..=x1 {
                let own = v as usize * 4 + hx as usize / 64;
                if d.drawn[own] & (1 << (hx % 64)) == 0 {
                    d.drawn[own] |= 1 << (hx % 64);
                    d.grew = true;
                }
                let i = (py + v) as usize * 1024 + (px + hx) as usize;
                if self.claimed[i / 64] & (1 << (i % 64)) == 0 {
                    self.claimed[i / 64] |= 1 << (i % 64);
                    claimed_more = true;
                }
            }
        }
        // Others on the same memory now know more of what is not theirs.
        if claimed_more {
            let (ax0, ay0, ax1, ay1) = (px + x0, py + y0, px + x1, py + y1);
            for (k, other) in self.dumps.iter_mut() {
                let (ox, oy, ow) = other.page;
                if *k != key && ox <= ax1 && ax0 < ox + ow && oy <= ay1 && ay0 < oy + 256 {
                    other.grew = true;
                }
            }
        }
        let d = &mut self.dumps.get_mut(&key).unwrap();
        if !d.grew || d.saved.is_some_and(|t| t.elapsed() < RESAVE) {
            return;
        }
        d.grew = false;
        d.saved = Some(std::time::Instant::now());
        let d = &self.dumps[&key];
        let per = [4, 2, 1][depth as usize];
        let mut rgba = decode(vram, &self.loaded, px, py, depth, cx, cy);
        for (i, texel) in rgba.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            let (x, y) = (px + (i % 256) as i32 / per, py + (i / 256) as i32);
            let at = y as usize * 1024 + x as usize;
            if self.claimed[at / 64] & (1 << (at % 64)) != 0 && !d.drew(x, y) {
                *texel = [0; 4];
            }
        }
        crate::mods::save_texture_dump(key, rgba);
    }

    /// The hash of a page's loaded contents (where they are, and what).
    fn page(&mut self, vram: &Vram, px: i32, py: i32, cols: i32) -> u64 {
        if let Some(&(.., h)) = self.pages.iter().find(|p| (p.0, p.1, p.2) == (px, py, cols)) {
            return h;
        }
        let mut h = fnv(FNV_OFFSET, cols as u16);
        for y in py..py + 256 {
            for c in 0..cols {
                let col = px / 64 + c;
                let loaded = self.loaded[y as usize] & (1 << col) != 0;
                h = fnv(h, u16::from(loaded));
                if loaded {
                    for x in col * 64..col * 64 + 64 {
                        h = fnv(h, vram.get(x, y));
                    }
                }
            }
        }
        self.pages.push((px, py, cols, h));
        h
    }

    fn palette(&mut self, vram: &Vram, x: i32, y: i32, n: i32) -> u64 {
        if let Some(&(.., h)) = self.palettes.iter().find(|p| (p.0, p.1, p.2) == (x, y, n)) {
            return h;
        }
        let h = (0..n).fold(FNV_OFFSET, |h, i| fnv(h, vram.get(x + i, y)));
        self.palettes.push((x, y, n, h));
        h
    }
}

/// A page as 256x256 RGBA texels, through a palette (`depth` 0 or 1) or as
/// they are (2); what was not loaded, and value 0 (which the console does
/// not draw), are transparent.
fn decode(vram: &Vram, loaded: &[u16], px: i32, py: i32, depth: u32, cx: i32, cy: i32) -> Vec<u8> {
    let per = [4, 2, 1][depth as usize];
    let mut out = Vec::with_capacity(256 * 256 * 4);
    for y in py..py + 256 {
        for u in 0..256 {
            let x = px + u / per;
            if loaded[y as usize] & (1 << (x / 64)) == 0 {
                out.extend_from_slice(&[0; 4]);
                continue;
            }
            let word = vram.get(x, y);
            let k = (u % per) as u16;
            let t = match depth {
                0 => vram.get(cx + i32::from((word >> (k * 4)) & 0xF), cy),
                1 => vram.get(cx + i32::from((word >> (k * 8)) & 0xFF), cy),
                _ => word,
            };
            let c = |s: u16| ((t >> s & 31) << 3) as u8;
            out.extend_from_slice(&[c(0), c(5), c(10), if t == 0 { 0 } else { 255 }]);
        }
    }
    out
}
