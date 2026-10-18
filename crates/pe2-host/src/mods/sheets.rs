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
//!
//! A sheet's texels are palette indices; which palette colours them is the
//! drawing's choice. Many files carry their pictures' palettes (`.pe2clut`)
//! in the order of their columns: one 256-colour row per 8-bit column, one
//! 16-colour row per 4-bit one. Such a sheet gets that palette (and its
//! depth from it) to be shown with before the game draws it.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::dialogue::{Column, Sources, lzss, picture};

/// Rows in a piece of a sheet the game loads at once.
pub const PIECE_ROWS: usize = 32;

/// A sheet: its hash, its rows of 64 halfwords, its pieces' hashes, as
/// the game loads them (the last one padded with zeros), and the palette
/// its file gives it, if one does (16 or 256 colours).
pub struct Sheet {
    pub hash: u64,
    pub rows: Vec<[u16; 64]>,
    pieces: Vec<u64>,
    pub palette: Option<Vec<u16>>,
}

impl Sheet {
    fn new(hash: u64, rows: Vec<[u16; 64]>) -> Sheet {
        let pieces =
            rows.chunks(PIECE_ROWS).map(|c| piece_hash(c.iter().flatten().copied(), PIECE_ROWS - c.len())).collect();
        Sheet { hash, rows, pieces, palette: None }
    }

    /// Its texel depth (0: 4 bits, 1: 8 bits): its palette's, else likely.
    pub fn depth(&self) -> u32 {
        match self.palette.as_ref().map(Vec::len) {
            Some(16) => 0,
            Some(_) => 1,
            None => self.likely_depth(),
        }
    }

    /// Its texels at `depth` as RGBA: through its palette if it has one of
    /// that depth (colour 0 transparent, as the console draws it), else in
    /// grey (see `grey`).
    pub fn picture(&self, depth: u32) -> (usize, Vec<u8>) {
        match &self.palette {
            Some(p) if p.len() == [16, 256][depth.min(1) as usize] && depth < 2 => self.through(depth, p),
            _ => self.grey(depth),
        }
    }

    /// Its texels at `depth` through `palette`.
    pub fn through(&self, depth: u32, palette: &[u16]) -> (usize, Vec<u8>) {
        let per = [4, 2, 1][depth.min(2) as usize];
        let width = 64 * per;
        let mut out = Vec::with_capacity(width * self.rows.len() * 4);
        for row in &self.rows {
            for u in 0..width {
                let word = row[u / per];
                let t = match depth {
                    0 => palette.get(usize::from((word >> ((u % 4) * 4)) & 0xF)).copied().unwrap_or(0),
                    1 => palette.get(usize::from((word >> ((u % 2) * 8)) & 0xFF)).copied().unwrap_or(0),
                    _ => word,
                };
                out.extend_from_slice(&rgba(t));
            }
        }
        (width, out)
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

/// A 15-bit colour as RGBA, 0 transparent.
fn rgba(t: u16) -> [u8; 4] {
    let c = |s: u16| ((t >> s & 31) << 3) as u8;
    [c(0), c(5), c(10), if t == 0 { 0 } else { 255 }]
}

/// The palettes of a file's `.pe2clut` chunks (an image header: x, y,
/// height, width; then the colours, LZSS): its rows of 256 colours or more
/// (8-bit palettes, the first 256 of each), and its 16-colour ones.
fn palettes(chunks: &[Vec<u8>]) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let (mut wide, mut narrow) = (Vec::new(), Vec::new());
    for c in chunks {
        let (Some(h), Some(w)) = (c.get(4..6), c.get(6..8)) else { continue };
        let (h, w) = (usize::from(u16::from_le_bytes([h[0], h[1]])), usize::from(u16::from_le_bytes([w[0], w[1]])));
        let mut at = 16;
        let bytes = lzss(c, &mut at);
        let colours: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|b| u16::from_le_bytes(*b)).collect();
        for row in colours.chunks(w.max(1)).take(h) {
            if row.len() >= 256 {
                wide.push(row[..256].to_vec());
            } else if row.len() == 16 {
                narrow.push(row.to_vec());
            }
        }
    }
    (wide, narrow)
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
    /// The catalog of the sheets of the pictures (`.pe2img`) of `sources`,
    /// with the palettes their files give them.
    pub fn new(sources: &Sources) -> Catalog {
        let mut c = Catalog::default();
        let mut keys: Vec<&String> = sources.pictures.keys().collect();
        keys.sort_unstable();
        for key in keys {
            let (wide, narrow) = palettes(sources.palettes.get(key).map_or(&[][..], Vec::as_slice));
            let columns = sources.pictures[key].iter().flat_map(|p| picture(p));
            for (i, Column { rows }) in columns.enumerate() {
                if rows.is_empty() {
                    continue;
                }
                // Its file's palettes, in column order: 8-bit ones if any.
                let given = [wide.as_slice(), narrow.as_slice()]
                    .into_iter()
                    .find(|p| !p.is_empty())
                    .map(|p| p[i.min(p.len() - 1)].clone());
                let hash = super::fnv1a(
                    (rows.len() as u32)
                        .to_le_bytes()
                        .into_iter()
                        .chain(rows.iter().flatten().flat_map(|w| w.to_le_bytes())),
                );
                if let Some(&known) = c.by_hash.get(&hash) {
                    let sheet = &mut c.sheets[known as usize];
                    if sheet.palette.is_none() {
                        sheet.palette = given;
                    }
                    continue;
                }
                let index = c.sheets.len() as u32;
                let mut sheet = Sheet::new(hash, rows);
                sheet.palette = given;
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

/// Saves each sheet of `catalog` as `<sheet>.png` in `dir`, through the
/// palette its file gives it (else in grey, at its likely depth), unless
/// there is one already (one coloured by playing is kept). Answers how
/// many were saved.
pub fn extract(catalog: &Catalog, dir: &std::path::Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let mut saved = 0;
    for sheet in &catalog.sheets {
        let path = dir.join(format!("{}.png", super::texture_name((sheet.hash, 0))));
        if path.exists() {
            continue;
        }
        let (width, rgba) = sheet.picture(sheet.depth());
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
