//! Texture sheets: the game's textures as its discs store them.
//!
//! Textures come from `.pe2img` pictures, each a few columns of 64
//! halfwords (a texture page's width at 4 bits) loaded into VRAM in pieces
//! of 32 rows. A column is a sheet: one texture asset, the same wherever
//! and whenever the game loads it, and known by a hash of its contents.
//! The catalog lists the discs' sheets and the hash of each 32-row piece,
//! so that the pieces the game loads are recognized as the sheets they come
//! from (runtime/gpu/textures.rs): mods replace sheets, and dumps are of
//! sheets, whole (docs/MODDING.md).

use std::collections::HashMap;
use std::sync::OnceLock;

use super::dialogue::{Column, picture};

/// Rows in a piece of a sheet the game loads at once.
pub const PIECE_ROWS: usize = 32;

/// A sheet: its hash, its rows of 64 halfwords, and its pieces' hashes, as
/// the game loads them (the last one padded with zeros).
pub struct Sheet {
    pub hash: u64,
    pub rows: Vec<[u16; 64]>,
    pieces: Vec<u64>,
}

impl Sheet {
    fn new(hash: u64, rows: Vec<[u16; 64]>) -> Sheet {
        let pieces =
            rows.chunks(PIECE_ROWS).map(|c| piece_hash(c.iter().flatten().copied(), PIECE_ROWS - c.len())).collect();
        Sheet { hash, rows, pieces }
    }

    /// Its likely texel depth (0: 4 bits, 1: 8 bits), for showing it before
    /// the game draws it. 8-bit textures' palettes run in ramps of 16
    /// colours, so a texel's high nibble (its ramp) changes less from one
    /// texel to the next than its low one; 4-bit texels have no such order.
    pub fn likely_depth(&self) -> u32 {
        let bytes: Vec<u8> = self.rows.iter().flatten().flat_map(|w| w.to_le_bytes()).collect();
        let (mut high, mut low, mut pairs) = (0u32, 0u32, 0u32);
        for p in bytes.windows(2).filter(|p| p[0] != 0 || p[1] != 0) {
            high += u32::from(p[0] >> 4 == p[1] >> 4);
            low += u32::from(p[0] & 15 == p[1] & 15);
            pairs += 1;
        }
        if pairs < 64 || f64::from(high) <= f64::from(low) * 1.05 { 0 } else { 1 }
    }

    /// Its texels at `depth` as grey levels, value 0 transparent: a sheet
    /// seen without its palettes. A texel's grey is its value's place among
    /// the values the sheet uses (an 8-bit sheet uses few of the 256, often
    /// close together, which plain values would show as one grey).
    pub fn grey(&self, depth: u32) -> (usize, Vec<u8>) {
        let per = [4, 2, 1][depth.min(2) as usize];
        let width = 64 * per;
        let value = |row: &[u16; 64], u: usize| -> u16 {
            let word = row[u / per];
            match depth {
                0 => (word >> ((u % 4) * 4)) & 0xF,
                1 => (word >> ((u % 2) * 8)) & 0xFF,
                _ => word & 0x7FFF,
            }
        };
        let mut used: Vec<u16> = self.rows.iter().flat_map(|r| (0..width).map(|u| value(r, u))).collect();
        used.sort_unstable();
        used.dedup();
        let span = used.len().saturating_sub(1).max(1);
        let mut out = Vec::with_capacity(width * self.rows.len() * 4);
        for row in &self.rows {
            for u in 0..width {
                let v = value(row, u);
                let rank = used.binary_search(&v).unwrap_or(0);
                let g = (48 + rank * 207 / span) as u8;
                out.extend_from_slice(&[g, g, g, if v == 0 { 0 } else { 255 }]);
            }
        }
        (width, out)
    }
}

/// The hash of a piece: its halfwords, then `padding` rows of zeros.
pub fn piece_hash(halfwords: impl IntoIterator<Item = u16>, padding: usize) -> u64 {
    let zeros = std::iter::repeat_n(0u16, padding * 64);
    super::fnv1a(halfwords.into_iter().chain(zeros).flat_map(u16::to_le_bytes))
}

/// The discs' sheets, and which sheets (and which of their pieces) each
/// piece hash is found in.
#[derive(Default)]
pub struct Catalog {
    pub sheets: Vec<Sheet>,
    pieces: HashMap<u64, Vec<(u32, u16)>>,
    by_hash: HashMap<u64, u32>,
}

impl Catalog {
    /// The catalog of the sheets of `pictures` (`.pe2img` contents).
    pub fn new<'a>(pictures: impl Iterator<Item = &'a [u8]>) -> Catalog {
        let mut c = Catalog::default();
        for p in pictures {
            for Column { rows } in picture(p) {
                if rows.is_empty() {
                    continue;
                }
                let hash = super::fnv1a(
                    (rows.len() as u32)
                        .to_le_bytes()
                        .into_iter()
                        .chain(rows.iter().flatten().flat_map(|w| w.to_le_bytes())),
                );
                if c.by_hash.contains_key(&hash) {
                    continue;
                }
                let index = c.sheets.len() as u32;
                let sheet = Sheet::new(hash, rows);
                for (k, &piece) in sheet.pieces.iter().enumerate() {
                    c.pieces.entry(piece).or_default().push((index, k as u16));
                }
                c.by_hash.insert(hash, index);
                c.sheets.push(sheet);
            }
        }
        c
    }

    /// The sheets and piece numbers a piece of this hash is found in.
    pub fn pieces(&self, hash: u64) -> &[(u32, u16)] {
        self.pieces.get(&hash).map_or(&[], Vec::as_slice)
    }

    /// Whether piece `k` of sheet `sheet` has this hash.
    pub fn piece_is(&self, sheet: u32, k: usize, hash: u64) -> bool {
        self.sheets.get(sheet as usize).and_then(|s| s.pieces.get(k)) == Some(&hash)
    }
}

/// Saves each sheet of `catalog` as `<sheet>.png` in `dir`, in grey at its
/// likely depth, unless there is one already (one coloured by playing is
/// kept). Answers how many were saved.
pub fn extract(catalog: &Catalog, dir: &std::path::Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let mut saved = 0;
    for sheet in &catalog.sheets {
        let path = dir.join(format!("{}.png", super::texture_name((sheet.hash, 0))));
        if path.exists() {
            continue;
        }
        let (width, rgba) = sheet.grey(sheet.likely_depth());
        let file = std::io::BufWriter::new(std::fs::File::create(&path)?);
        let mut enc = png::Encoder::new(file, width as u32, sheet.rows.len() as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().and_then(|mut w| w.write_image_data(&rgba)).map_err(std::io::Error::other)?;
        saved += 1;
    }
    Ok(saved)
}

static CATALOG: OnceLock<Catalog> = OnceLock::new();

/// The catalog, once read from the disc (see mods::start).
pub fn catalog() -> Option<&'static Catalog> {
    CATALOG.get()
}

pub fn set_catalog(c: Catalog) {
    let _ = CATALOG.set(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sheet's pieces are hashed as the game loads them, padded.
    #[test]
    fn pieces_pad() {
        let sheet = Sheet::new(0, vec![[7; 64]; 40]);
        let pieces = &sheet.pieces;
        assert_eq!(pieces.len(), 2);
        assert_eq!(pieces[0], piece_hash(std::iter::repeat_n(7u16, 64 * 32), 0));
        assert_eq!(pieces[1], piece_hash(std::iter::repeat_n(7u16, 64 * 8), 24));
    }
}
