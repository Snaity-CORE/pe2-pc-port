//! Mods' textures: which texture sheets a primitive draws from.
//!
//! The game's textures are sheets (mods/sheets.rs): columns of its
//! `.pe2img` pictures, loaded into VRAM in pieces of 32 rows. Each piece an
//! image transfer loads is recognized by its hash as a piece of a sheet of
//! the disc's catalog (pieces found in several sheets, blank ones, by their
//! neighbours above or below), so the VRAM knows which sheet each of its
//! pieces holds, and where its top is. A primitive draws from up to four
//! sheets, one per 64-halfword column of its texture page; it is replaced
//! sheet by sheet, by the mods' pictures of those sheets (with its
//! palette, or any).
//!
//! Only memory image transfers loaded counts: each row of each column is
//! marked loaded by a transfer covering it and unmarked by any other write
//! (drawing, copies, fills), so palettes and drawing next to textures
//! leave them be, and a primitive is replaced only where all it reads was
//! loaded.
//!
//! Textures the host makes while the game runs (dialogue glyphs drawn from
//! a mod's font, mods/capfont.rs) are not in the catalog: the host gives
//! each its high-resolution picture as it loads it (`host`), and the rows
//! that image transfer loads are known as that hosted sheet until written
//! over.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::raster::{DrawState, Shading, Vram};
use crate::mods::indexed::Indexed;
use crate::mods::sheets::{self, PIECE_ROWS};

/// A `Part`'s sheet with this bit is a hosted one (its id below it), not a
/// catalog index.
pub const HOSTED: u32 = 1 << 31;

/// Hosted textures kept: the most recent ones (each a page's worth of
/// high-resolution indices).
const HOSTED_KEPT: usize = 8;

/// A texture the host made: the image transfer that places it (where, and
/// its pixels' hash), and its picture.
struct Hosted {
    id: u32,
    at: (i32, i32, i32, i32),
    hash: u64,
    indexed: Arc<Indexed>,
}

fn registry() -> &'static Mutex<(u32, VecDeque<Hosted>)> {
    static REGISTRY: Mutex<(u32, VecDeque<Hosted>)> = Mutex::new((0, VecDeque::new()));
    &REGISTRY
}

fn pixels_hash(pixels: &[u16]) -> u64 {
    pixels.iter().fold(FNV_OFFSET, |h, &p| fnv(h, p))
}

/// The host is about to load `pixels` at `x, y, w, h` (a whole 64-halfword
/// column, 4-bit texels), whose picture at a higher resolution is
/// `indexed`: what that transfer loads draws it.
pub fn host(x: i32, y: i32, w: i32, h: i32, pixels: &[u16], indexed: Arc<Indexed>) {
    let hash = pixels_hash(pixels);
    let mut r = registry().lock().unwrap();
    if r.1.iter().any(|e| e.at == (x, y, w, h) && e.hash == hash) {
        return;
    }
    r.0 = (r.0 + 1) & !HOSTED;
    let id = r.0;
    r.1.push_back(Hosted { id, at: (x, y, w, h), hash, indexed });
    while r.1.len() > HOSTED_KEPT {
        r.1.pop_front();
    }
}

/// The picture of hosted sheet `id`, while kept.
pub fn hosted(id: u32) -> Option<Arc<Indexed>> {
    registry().lock().unwrap().1.iter().find(|e| e.id == id).map(|e| e.indexed.clone())
}

/// Columns of 64 halfwords; rows; rows of pieces.
const COLUMNS: i32 = 16;
const ROWS: i32 = 512;
const PIECES: i32 = ROWS / PIECE_ROWS as i32;

/// Image transfers at least this large are screen images (backgrounds,
/// pictures), which mods replace whole (mods::background).
const SCREEN: (i32, i32) = (256, 192);

const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;

fn fnv(h: u64, v: u16) -> u64 {
    let h = (h ^ u64::from(v & 0xFF)).wrapping_mul(0x0100_0000_01B3);
    (h ^ u64::from(v >> 8)).wrapping_mul(0x0100_0000_01B3)
}

/// A piece of a VRAM column an image transfer loaded: its hash, and once
/// known, the sheet it is part of (catalog index) and the row its top is
/// at.
#[derive(Clone, Copy)]
struct Piece {
    hash: u64,
    sheet: Option<(u32, i32)>,
}

/// A sheet a primitive draws from: its page column (0 to 3), the sheet
/// (catalog index) and the VRAM row its top is at.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Part {
    pub column: usize,
    pub sheet: u32,
    pub top: i32,
}

/// What a primitive draws from: its texel depth (0: 4 bits, 1: 8, 2: 15),
/// its palette's hash (0 for 15 bits) and the sheets.
pub struct Found {
    pub depth: u32,
    pub palette: u64,
    pub parts: Vec<Part>,
}

pub struct Tracker {
    /// Per row, a bit per column: loaded by an image transfer.
    loaded: Vec<u16>,
    /// Per piece of each column (column-major), what loaded it.
    pieces: Vec<Option<Piece>>,
    /// Palette hashes by place and size, while unchanged.
    palettes: Vec<(i32, i32, i32, u64)>,
    /// Rows of a column a hosted texture was loaded to: (x, y, rows, id).
    hosted: Vec<(i32, i32, i32, u32)>,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            loaded: vec![0; ROWS as usize],
            pieces: vec![None; (COLUMNS * PIECES) as usize],
            palettes: Vec::new(),
            hosted: Vec::new(),
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
        let hits = |px: i32, py: i32, pw: i32, ph: i32| px < x + w && x < px + pw && py < y + h && y < py + ph;
        self.palettes.retain(|&(px, py, n, _)| !hits(px, py, n, 1));
        self.hosted.retain(|&(hx, hy, hh, _)| !hits(hx, hy, 64, hh));
    }

    /// An image transfer of `pixels` to `x, y, w, h` (after `write`): the
    /// columns it covers whole are loaded, and a piece of a column (64x32,
    /// aligned) is recognized.
    pub fn upload(&mut self, x: i32, y: i32, w: i32, h: i32, pixels: &[u16]) {
        let rows_per = PIECE_ROWS as i32;
        // Pieces it overlaps are no longer what loaded them.
        for c in (x / 64).max(0)..((x + w + 63) / 64).min(COLUMNS) {
            for p in (y / rows_per).max(0)..((y + h + rows_per - 1) / rows_per).min(PIECES) {
                self.pieces[(c * PIECES + p) as usize] = None;
            }
        }
        if w >= SCREEN.0 && h >= SCREEN.1 {
            return;
        }
        if w == 64 && x % 64 == 0 {
            let hash = pixels_hash(&pixels[..pixels.len().min((w * h) as usize)]);
            let r = registry().lock().unwrap();
            if let Some(e) = r.1.iter().rev().find(|e| e.at == (x, y, w, h) && e.hash == hash) {
                self.hosted.push((x, y, h, e.id));
            }
        }
        let (c0, c1) = ((x + 63) / 64, (x + w) / 64);
        if c1 > c0 && c0 >= 0 && c1 <= COLUMNS {
            let bits = column_bits(c0, c1 - 1);
            for r in rows(y, h) {
                self.loaded[r] |= bits;
            }
        }
        if (w, h) == (64, rows_per) && x % 64 == 0 && y % rows_per == 0 && pixels.len() >= 64 * PIECE_ROWS {
            let hash = sheets::piece_hash(pixels[..64 * PIECE_ROWS].iter().copied(), 0);
            self.pieces[((x / 64) * PIECES + y / rows_per) as usize] = Some(Piece { hash, sheet: None });
        }
    }

    /// The sheet piece `p` of column `c` is part of, and the row its top is
    /// at: by its hash if only one sheet has it, else by its neighbours.
    fn resolve(&mut self, c: i32, p: i32) -> Option<(u32, i32)> {
        let rows_per = PIECE_ROWS as i32;
        let piece = self.pieces[(c * PIECES + p) as usize]?;
        if piece.sheet.is_some() {
            return piece.sheet;
        }
        let catalog = sheets::catalog()?;
        let unique = |piece: Piece, p: i32| match catalog.pieces(piece.hash) {
            [(s, k)] => Some((*s, (p - i32::from(*k)) * rows_per)),
            _ => piece.sheet,
        };
        let mut found = unique(piece, p);
        // A neighbour of known sheet whose sheet has this piece where it
        // falls: pieces many sheets share (blank ones) are told apart so.
        for dir in [-1, 1] {
            let mut q = p + dir;
            while found.is_none() && (0..PIECES).contains(&q) {
                let Some(n) = self.pieces[(c * PIECES + q) as usize] else { break };
                if let Some((s, top)) = unique(n, q) {
                    let k = (p * rows_per - top) / rows_per;
                    if k >= 0 && catalog.piece_is(s, k as usize, piece.hash) {
                        found = Some((s, top));
                    }
                    break;
                }
                q += dir;
            }
        }
        if let Some(slot) = self.pieces[(c * PIECES + p) as usize].as_mut() {
            slot.sheet = found;
        }
        found
    }

    /// The sheets a textured primitive draws from, if it draws only what was
    /// loaded and its sheets are known; `uv` is its texel rectangle (u0,
    /// v0, u1, v1), inclusive.
    pub fn find(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, uv: (i32, i32, i32, i32)) -> Option<Found> {
        // A texture window repeats part of the page: not one texture.
        if st.win_mask_x != 0 || st.win_mask_y != 0 {
            return None;
        }
        let depth = st.tex_depth.min(2);
        let per = [4, 2, 1][depth as usize];
        let (u0, v0, u1, v1) = (uv.0.clamp(0, 255), uv.1.clamp(0, 255), uv.2.clamp(0, 255), uv.3.clamp(0, 255));
        let (px, py) = (st.tex_x, st.tex_y);
        let (x0, x1, y0, y1) = (px + u0 / per, px + u1 / per, py + v0, py + v1);
        if px % 64 != 0 || x1 >= COLUMNS * 64 || y1 >= ROWS {
            return None;
        }
        let bits = column_bits(x0 / 64, x1 / 64);
        if rows(y0, y1 - y0 + 1).any(|r| self.loaded[r] & bits != bits) {
            return None;
        }
        let rows_per = PIECE_ROWS as i32;
        let mut parts = Vec::new();
        for c in x0 / 64..=x1 / 64 {
            if let Some(&(_, hy, _, id)) =
                self.hosted.iter().find(|&&(hx, hy, hh, _)| hx == c * 64 && hy <= y0 && y1 < hy + hh)
            {
                parts.push(Part { column: (c - px / 64) as usize, sheet: HOSTED | id, top: hy });
                continue;
            }
            let mut at = None;
            for p in y0 / rows_per..=y1 / rows_per {
                let r = self.resolve(c, p)?;
                if at.is_some_and(|a| a != r) {
                    return None;
                }
                at = Some(r);
            }
            let (sheet, top) = at?;
            parts.push(Part { column: (c - px / 64) as usize, sheet, top });
        }
        let palette =
            if depth < 2 { self.palette(vram, sh.clut_x, sh.clut_y, if depth == 0 { 16 } else { 256 }) } else { 0 };
        if crate::runtime::trace("textures") {
            // Each sheet and palette once: the names mods give pictures.
            static SEEN: std::sync::Mutex<Option<std::collections::HashSet<(u32, u64)>>> = std::sync::Mutex::new(None);
            if let Some(catalog) = sheets::catalog() {
                for part in parts.iter().filter(|p| p.sheet & HOSTED == 0) {
                    if SEEN.lock().unwrap().get_or_insert_with(Default::default).insert((part.sheet, palette)) {
                        let name = crate::mods::texture_name((catalog.sheets[part.sheet as usize].hash, palette));
                        let sheet = &catalog.sheets[part.sheet as usize];
                        eprintln!(
                            "[textures] {name}: {}-bit sheet from {}, at ({}, {}), palette at ({}, {}), texels {u0},{v0}-{u1},{v1}",
                            [4, 8, 15][depth as usize],
                            sheet.origin,
                            px + part.column as i32 * 64,
                            part.top,
                            sh.clut_x,
                            sh.clut_y
                        );
                    }
                }
            }
        }
        Some(Found { depth, palette, parts })
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
