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
//! With dumping on, each sheet a primitive draws from is saved as
//! `dumps/textures/<sheet>.png`: the sheet in grey (its texel values; see
//! the extract), coloured where primitives drew it, with their palettes,
//! growing as more is drawn.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::raster::{DrawState, Shading, Vram};
use crate::mods::sheets::{self, PIECE_ROWS};

/// Columns of 64 halfwords; rows; rows of pieces.
const COLUMNS: i32 = 16;
const ROWS: i32 = 512;
const PIECES: i32 = ROWS / PIECE_ROWS as i32;

/// Image transfers at least this large are screen images (backgrounds,
/// pictures), which mods replace whole (mods::background).
const SCREEN: (i32, i32) = (256, 192);

/// A growing dump is saved again at most this often.
const RESAVE: Duration = Duration::from_secs(1);

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

/// A sheet being dumped: its depth, its picture so far (RGBA), whether it
/// changed since it was saved, and when it was.
struct Dump {
    depth: u32,
    width: usize,
    rgba: Vec<u8>,
    changed: bool,
    saved: Option<Instant>,
}

pub struct Tracker {
    /// Per row, a bit per column: loaded by an image transfer.
    loaded: Vec<u16>,
    /// Per piece of each column (column-major), what loaded it.
    pieces: Vec<Option<Piece>>,
    /// Palette hashes by place and size, while unchanged.
    palettes: Vec<(i32, i32, i32, u64)>,
    /// Sheets being dumped, by catalog index.
    dumps: HashMap<u32, Dump>,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            loaded: vec![0; ROWS as usize],
            pieces: vec![None; (COLUMNS * PIECES) as usize],
            palettes: Vec::new(),
            dumps: HashMap::new(),
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
                for part in &parts {
                    if SEEN.lock().unwrap().get_or_insert_with(Default::default).insert((part.sheet, palette)) {
                        let name = crate::mods::texture_name((catalog.sheets[part.sheet as usize].hash, palette));
                        eprintln!(
                            "[textures] {name}: {}-bit sheet, palette at ({}, {})",
                            [4, 8, 15][depth as usize],
                            sh.clut_x,
                            sh.clut_y
                        );
                    }
                }
            }
        }
        if crate::mods::dumping() {
            for part in &parts {
                self.dump(vram, part, depth, (u0, v0, u1, v1), (px, py), (sh.clut_x, sh.clut_y));
            }
        }
        Some(Found { depth, palette, parts })
    }

    /// Colours into sheet `part`'s dump the texels `uv` of the page at
    /// (`px`, `py`) drew with the palette at (`cx`, `cy`), and saves it if
    /// they changed it.
    fn dump(
        &mut self,
        vram: &Vram,
        part: &Part,
        depth: u32,
        uv: (i32, i32, i32, i32),
        (px, py): (i32, i32),
        (cx, cy): (i32, i32),
    ) {
        let Some(catalog) = sheets::catalog() else { return };
        let Some(sheet) = catalog.sheets.get(part.sheet as usize) else { return };
        let per = [4, 2, 1][depth as usize];
        let d = self.dumps.entry(part.sheet).or_insert_with(|| new_dump(sheet, depth));
        if d.depth != depth {
            *d = new_dump(sheet, depth);
        }
        let height = d.rgba.len() / 4 / d.width;
        // The part's texels: its column of the page.
        let first = part.column as i32 * 64 * per;
        let (u0, u1) = (uv.0.max(first), uv.2.min(first + 64 * per - 1));
        for v in uv.1..=uv.3 {
            let sy = py + v - part.top;
            if sy < 0 || sy as usize >= height {
                continue;
            }
            for u in u0..=u1 {
                let word = vram.get(px + u / per, py + v);
                let t = match depth {
                    0 => vram.get(cx + i32::from((word >> ((u % 4) * 4)) & 0xF), cy),
                    1 => vram.get(cx + i32::from((word >> ((u % 2) * 8)) & 0xFF), cy),
                    _ => word,
                };
                let c = |s: u16| ((t >> s & 31) << 3) as u8;
                let rgba = [c(0), c(5), c(10), if t == 0 { 0 } else { 255 }];
                let at = (sy as usize * d.width + (u - first) as usize) * 4;
                if d.rgba[at..at + 4] != rgba {
                    d.rgba[at..at + 4].copy_from_slice(&rgba);
                    d.changed = true;
                }
            }
        }
        if d.changed && d.saved.is_none_or(|t| t.elapsed() >= RESAVE) {
            d.changed = false;
            d.saved = Some(Instant::now());
            crate::mods::save_texture_dump((sheet.hash, 0), d.width as u32, height as u32, d.rgba.clone());
        }
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

/// A sheet's dump to start from: what an earlier session or the extract
/// saved, if of this depth's size, else the sheet in grey.
fn new_dump(sheet: &sheets::Sheet, depth: u32) -> Dump {
    let (width, grey) = sheet.grey(depth);
    let height = sheet.rows.len();
    let rgba = match crate::mods::read_texture_dump((sheet.hash, 0)) {
        Some((w, h, rgba)) if (w as usize, h as usize) == (width, height) => rgba,
        _ => grey,
    };
    Dump { depth, width, rgba, changed: false, saved: None }
}
