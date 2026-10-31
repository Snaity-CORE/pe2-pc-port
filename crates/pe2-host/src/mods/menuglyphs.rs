//! Letters the menus' font lacks for translations: ã õ Ã Õ (Latin-1 0xE3,
//! 0xF5, 0xC3, 0xD5), in its medium and large faces. The font has the
//! other accented letters of Latin-1 (main/textdraw.c's glyph tables run to
//! 0xFF); these four records are empty.
//!
//! Each is made from the font's own â ô Â Ô: the same record (size,
//! offsets, advance) for a cell of the same size in an empty part of the
//! font's sheet (stage0/file1, at (960, 256) in VRAM), and the same picture
//! with a tilde drawn where the circumflex was. strings.rs asks for them
//! (`ensure`) when a translated text that uses them is drawn: the records
//! are written into the game's tables, and the pictures into VRAM unless
//! the font is redrawn from a high-definition one (fontsheets.rs), which
//! draws these cells too (glyphmaps.rs lists them).

use std::sync::OnceLock;

/// The faces that have accented letters: their glyph tables (224 records
/// of 12 bytes, from ' ') and the V bias each adds; then the small face's
/// table (91 records), whose cells must be kept clear too.
const FACES: [(u32, u8); 2] = [(0x8005_EFB0, 38), (0x8005_FA30, 128)];
const SMALL: u32 = 0x8006_04B0;
const RECORDS: usize = 224;
const SMALL_RECORDS: usize = 91;

/// Each added letter, the letter with a circumflex it is made from, and
/// that letter without its accent.
const ADDED: [(u8, u8, u8); 4] = [(0xE3, 0xE2, b'a'), (0xF5, 0xF4, b'o'), (0xC3, 0xC2, b'A'), (0xD5, 0xD4, b'O')];

/// Whether `code` is one of the added letters.
pub fn is_added(code: u8) -> bool {
    ADDED.iter().any(|a| a.0 == code)
}

/// The sheet's rows past which the font is not looked for room: the last
/// ones hold palettes (the menus' at (976, 511)).
const ROWS: usize = 250;

/// The font's cell indices: the page around glyphs, the ring around their
/// strokes, the fill, and the soft corner between ring and page.
const PAGE: u8 = 5;
const RING: u8 = 10;
const SOFT: u8 = 8;
const FILL: u8 = 15;

/// An added letter in one face: its record, its cell in the sheet (u, v,
/// width, height), the row of the cell its letters stand on, and the cell
/// of the letter it is made from.
#[derive(Clone, Debug, PartialEq)]
pub struct Added {
    pub code: u8,
    pub face: usize,
    pub record: [u8; 12],
    pub cell: [u16; 4],
    pub baseline: u16,
    pub like: [u16; 4],
}

fn record(table: &[u8], code: u8) -> Option<&[u8; 12]> {
    table.as_chunks::<12>().0.get(usize::from(code.checked_sub(0x20)?))
}

/// A record's cell in the sheet (u, v, width, height), None when empty.
fn cell(r: &[u8; 12], bias: u8) -> Option<[u16; 4]> {
    let (w, h) = (u16::from(r[2]) + 1, u16::from(r[3]) + 1);
    (w > 1 || h > 1).then(|| [u16::from(r[0]), u16::from(r[1].wrapping_add(bias)), w, h])
}

/// Where the added letters go, from the faces' tables as the executable
/// has them (medium, large, small): for each face and letter whose record
/// is empty, the first place (top to bottom, left to right) where a cell
/// of its source's size fits with a texel clear around it, among the cells
/// of every face.
pub fn layout(medium: &[u8], large: &[u8], small: &[u8]) -> Vec<Added> {
    let mut used = vec![false; 256 * 256];
    let mark = |used: &mut Vec<bool>, [u, v, w, h]: [u16; 4]| {
        for y in v.saturating_sub(1)..(v + h + 1).min(256) {
            for x in u.saturating_sub(1)..(u + w + 1).min(256) {
                used[usize::from(y) * 256 + usize::from(x)] = true;
            }
        }
    };
    for (table, bias) in [(medium, FACES[0].1), (large, FACES[1].1), (small, 0)] {
        for r in table.as_chunks::<12>().0 {
            if let Some(c) = cell(r, bias) {
                mark(&mut used, c);
            }
        }
    }
    let mut out = Vec::new();
    for (face, table) in [medium, large].into_iter().enumerate() {
        let bias = FACES[face].1;
        for &(code, source, _) in &ADDED {
            let (Some(own), Some(src)) = (record(table, code), record(table, source)) else { continue };
            let Some(like @ [_, _, w, h]) = cell(src, bias) else { continue };
            if cell(own, bias).is_some() {
                continue;
            }
            let free = |u: usize, v: usize| {
                (v..v + usize::from(h)).all(|y| (u..u + usize::from(w)).all(|x| !used[y * 256 + x]))
            };
            let Some((u, v)) = (0..ROWS - usize::from(h))
                .flat_map(|v| (0..256 - usize::from(w)).map(move |u| (u, v)))
                .find(|&(u, v)| free(u, v))
            else {
                continue;
            };
            let place = [u as u16, v as u16, w, h];
            mark(&mut used, place);
            let mut record = *src;
            record[0] = u as u8;
            record[1] = (v as u8).wrapping_sub(bias);
            let baseline = (h as i16 - i16::from(src[5] as i8)).clamp(0, h as i16) as u16;
            out.push(Added { code, face, record, cell: place, baseline, like });
        }
    }
    out
}

/// The picture of `added` (its cell's 4-bit indices, row by row), from
/// the sheet's (`texel(u, v)`): its source letter's, with the accent's
/// rows made the plain letter's (or the page) and a tilde drawn there,
/// stroked as the circumflex is (fill, a ring beside, soft corners).
pub fn draw(added: &Added, table: &[u8], texel: impl Fn(u16, u16) -> u8) -> Option<Vec<u8>> {
    let &(_, source, plain) = ADDED.iter().find(|a| a.0 == added.code)?;
    let bias = FACES[added.face].1;
    let pixels = |c: [u16; 4]| -> Vec<Vec<u8>> {
        (0..c[3]).map(|y| (0..c[2]).map(|x| texel(c[0] + x, c[1] + y)).collect()).collect()
    };
    let src = pixels(cell(record(table, source)?, bias)?);
    let base = pixels(cell(record(table, plain)?, bias)?);
    let (w, h) = (src[0].len(), src.len());
    let delta = h.checked_sub(base.len())?;
    // The accent's rows: those above the first that is the plain letter's.
    let accent = (0..h).find(|&y| y >= delta && base[y - delta] == src[y]).unwrap_or(h).min(h - 1);
    let mut out = src.clone();
    for (y, row) in out.iter_mut().enumerate().take(accent) {
        *row = if y >= delta && base[y - delta].len() == w { base[y - delta].clone() } else { vec![PAGE; w] };
    }
    // The circumflex's strokes: its first row and centre.
    let strokes: Vec<(usize, usize)> =
        (0..accent).flat_map(|y| (0..w).map(move |x| (x, y))).filter(|&(x, y)| src[y][x] == FILL).collect();
    let top = strokes.iter().map(|s| s.1).min()?;
    let centre = (strokes.iter().map(|s| s.0).sum::<usize>() as f32 / strokes.len() as f32).round() as i32;
    // A tilde: up from the left, across, down and up again.
    let tilde: [(i32, usize); 5] = [(-1, 0), (0, 0), (2, 0), (-2, 1), (1, 1)];
    let mut put = |x: i32, y: usize, v: u8| {
        if (0..w as i32).contains(&x) && y < accent.max(top + 2) && y < h {
            let p = &mut out[y][x as usize];
            *p = (*p).max(v);
        }
    };
    for &(dx, dy) in &tilde {
        let (x, y) = (centre + dx, top + dy);
        for (ox, oy, v) in [(-1, 0, RING), (1, 0, RING), (0, -1, RING), (0, 1, RING)].into_iter().chain([
            (-1, -1, SOFT),
            (1, -1, SOFT),
            (-1, 1, SOFT),
            (1, 1, SOFT),
        ]) {
            if let Some(yy) = y.checked_add_signed(oy) {
                put(x + ox, yy, v);
            }
        }
    }
    for &(dx, dy) in &tilde {
        put(centre + dx, top + dy, FILL);
    }
    Some(out.concat())
}

/// The VRAM the added pictures go into: (x, y, width, height) in 16-bit
/// pixels, and those pixels.
struct Native {
    rect: (i16, i16, i16, i16),
    pixels: Vec<u16>,
}

static NATIVE: OnceLock<Option<Native>> = OnceLock::new();

/// `len` bytes of PlayStation RAM at `address`.
fn ram(address: u32, len: usize) -> &'static [u8] {
    // SAFETY: the game's RAM, mapped at its own addresses; the glyph tables
    // are in the resident executable's data.
    unsafe { std::slice::from_raw_parts(address as usize as *const u8, len) }
}

/// The font's sheet as VRAM holds it: 64 x 256 pixels of four 4-bit
/// texels each.
fn sheet_from_vram() -> Vec<u16> {
    use crate::runtime::gpu::{Rect, pcrt_StoreImage};
    let mut page = vec![0u16; 64 * 256];
    pcrt_StoreImage(&Rect::new(960, 256, 64, 256), page.as_mut_ptr());
    page
}

fn texel(page: &[u16], u: u16, v: u16) -> u8 {
    ((page[usize::from(v) * 64 + usize::from(u / 4)] >> (4 * (u % 4))) & 0xF) as u8
}

/// Writes the added letters' records into the game's tables and, unless
/// the font is redrawn finer, makes their pictures (once).
fn build() -> Option<Native> {
    let medium = ram(FACES[0].0, RECORDS * 12);
    let large = ram(FACES[1].0, RECORDS * 12);
    let added = layout(medium, large, ram(SMALL, SMALL_RECORDS * 12));
    let tables = [medium.to_vec(), large.to_vec()];
    for a in &added {
        let at = FACES[a.face].0 + u32::from(a.code - 0x20) * 12;
        // SAFETY: the record of `a.code` in its face's table (in RAM).
        unsafe { std::ptr::copy_nonoverlapping(a.record.as_ptr(), at as usize as *mut u8, 12) };
    }
    if added.is_empty() || super::fontsheets::enabled() {
        return None;
    }
    let mut page = sheet_from_vram();
    let original = page.clone();
    for a in &added {
        let picture = draw(a, &tables[a.face], |u, v| texel(&original, u, v))?;
        let [u0, v0, w, _] = a.cell;
        for (i, &t) in picture.iter().enumerate() {
            let (u, v) = (u0 + (i % usize::from(w)) as u16, v0 + (i / usize::from(w)) as u16);
            let p = &mut page[usize::from(v) * 64 + usize::from(u / 4)];
            *p = (*p & !(0xF << (4 * (u % 4)))) | (u16::from(t) << (4 * (u % 4)));
        }
    }
    let x0 = added.iter().map(|a| a.cell[0] / 4).min()?;
    let x1 = added.iter().map(|a| (a.cell[0] + a.cell[2]).div_ceil(4)).max()?;
    let y0 = added.iter().map(|a| a.cell[1]).min()?;
    let y1 = added.iter().map(|a| a.cell[1] + a.cell[3]).max()?;
    let pixels = (y0..y1)
        .flat_map(|y| (x0..x1).map(move |x| (x, y)))
        .map(|(x, y)| page[usize::from(y) * 64 + usize::from(x)])
        .collect();
    Some(Native { rect: (960 + x0 as i16, 256 + y0 as i16, (x1 - x0) as i16, (y1 - y0) as i16), pixels })
}

/// Makes the added letters drawable: their records in the game's tables,
/// their pictures in VRAM (put back whenever the font's sheet is loaded
/// again over them).
pub fn ensure() {
    let _g = crate::runtime::irq::guard();
    let Some(native) = NATIVE.get_or_init(build) else { return };
    use crate::runtime::gpu::{Rect, pcrt_LoadImage, pcrt_StoreImage};
    let (x, y, w, h) = native.rect;
    let rect = Rect::new(x, y, w, h);
    let mut now = vec![0u16; native.pixels.len()];
    pcrt_StoreImage(&rect, now.as_mut_ptr());
    if now != native.pixels {
        pcrt_LoadImage(&rect, native.pixels.as_ptr());
    }
}

/// The added letters' cells for the font's glyph map (glyphmaps.rs), from
/// the executable's tables: (cell, letter, face index, baseline).
pub fn cells(exe: &[u8]) -> Vec<Added> {
    let table = |address: u32, n: usize| super::exe_at(exe, address, n * 12);
    match (table(FACES[0].0, RECORDS), table(FACES[1].0, RECORDS), table(SMALL, SMALL_RECORDS)) {
        (Some(m), Some(l), Some(s)) => layout(m, l, s),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(records: &[(u8, [u8; 6])]) -> Vec<u8> {
        let mut t = vec![0u8; RECORDS * 12];
        for &(code, [u, v, w, h, x, y]) in records {
            let at = usize::from(code - 0x20) * 12;
            t[at..at + 6].copy_from_slice(&[u, v, w - 1, h - 1, x, y]);
        }
        t
    }

    /// Added letters take free room, clear of every cell, the source's size.
    #[test]
    fn layout_avoids_cells() {
        let medium = table(&[(0xE2, [0, 0, 7, 11, 0xFE, 2]), (b'a', [8, 0, 7, 9, 0xFE, 2])]);
        let large = table(&[]);
        let added = layout(&medium, &large, &[]);
        assert_eq!(added.len(), 1);
        let a = &added[0];
        assert_eq!((a.code, a.face, a.cell), (0xE3, 0, [0, 0, 7, 11]));
        // V before the medium face's bias of 38.
        assert_eq!(a.record[..2], [0, 218]);
        assert_eq!((a.baseline, a.like), (9, [0, 38, 7, 11]));
    }

    /// The tilde replaces the circumflex; the letter below stays.
    #[test]
    fn tilde_replaces_circumflex() {
        let medium = table(&[(0xE2, [0, 0, 7, 4, 0, 0]), (b'a', [8, 1, 7, 2, 0, 0])]);
        let sheet: Vec<Vec<u8>> = [
            [5, 5, 5, 15, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
            [5, 5, 15, 5, 15, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9],
            [9, 9, 9, 9, 9, 9, 9, 5, 9, 9, 9, 9, 9, 9, 9],
            [9, 9, 9, 9, 9, 9, 9, 5, 5, 5, 5, 5, 5, 5, 5],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        let added =
            Added { code: 0xE3, face: 0, record: [0; 12], cell: [0, 0, 7, 4], baseline: 4, like: [0, 38, 7, 4] };
        let texel = |u: u16, v: u16| sheet.get(usize::from(v).wrapping_sub(38)).map_or(5, |r| r[usize::from(u)]);
        let p = draw(&added, &medium, texel).unwrap();
        let rows: Vec<&[u8]> = p.chunks(7).collect();
        assert_eq!(rows[0], [SOFT, RING, FILL, FILL, RING, FILL, RING]);
        assert_eq!(rows[1], [RING, FILL, RING, RING, FILL, RING, SOFT]);
        assert_eq!(rows[2..], [[9u8; 7]; 2]);
    }
}
