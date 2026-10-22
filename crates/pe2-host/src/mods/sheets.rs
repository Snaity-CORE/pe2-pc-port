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
//! depth from it) to be shown with before the game draws it. Some sheets'
//! palettes are fixed by the game's code instead (`FIXED`).

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
    /// Where that palette comes from (for the extract's index).
    pub palette_source: &'static str,
    /// Where it was first found: `<file>/<chunk>#<column>`, and where in
    /// VRAM that loads it (x, y).
    pub origin: String,
    pub place: (u16, u16),
}

impl Sheet {
    fn new(hash: u64, rows: Vec<[u16; 64]>) -> Sheet {
        let pieces =
            rows.chunks(PIECE_ROWS).map(|c| piece_hash(c.iter().flatten().copied(), PIECE_ROWS - c.len())).collect();
        Sheet { hash, rows, pieces, palette: None, palette_source: "", origin: String::new(), place: (0, 0) }
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

/// Palettes the game's code fixes for the sheets loaded at a place: the
/// page column (VRAM x, y), the sheets' depth, the file whose `.pe2clut`
/// holds the palette, and where the palette is in VRAM.
type Fixed = ((u16, u16), u32, &'static str, (usize, usize));
const FIXED: &[Fixed] = &[
    // Rooms' dialogue fonts: captions.c draws them from page (384, 0) with
    // CLUT 0x3D50 (and its colour variants next to it), from the system
    // text palettes stage 0's file 1 loads at (256, 243).
    ((384, 0), 0, "stage0/file1", (256, 245)),
];

/// The captions' palettes (see `FIXED`), for glyph pages wherever they
/// load (some actors move the captions' page, captions.c func_800E6D4C,
/// and rooms keep speakers' names in pages of their own): the text's
/// (CLUT 0x3D50), and the speakers' name labels' (0x3D93).
const CAPTIONS: &str = "stage0/file1";
const CAPTION_TEXT: (usize, usize) = (256, 245);
const CAPTION_LABELS: (usize, usize) = (304, 246);

/// The `n` colours at (`x`, `y`) in VRAM as one of `chunks` (`.pe2clut`)
/// loads them.
fn palette_at(chunks: &[Vec<u8>], x: usize, y: usize, n: usize) -> Option<Vec<u16>> {
    chunks.iter().find_map(|c| {
        let field = |at: usize| Some(usize::from(u16::from_le_bytes([*c.get(at)?, *c.get(at + 1)?])));
        let (cx, cy, h, w) = (field(0)?, field(2)?, field(4)?, field(6)?);
        if !(cx..cx + w).contains(&x) || !(cy..cy + h).contains(&y) || x + n > cx + w {
            return None;
        }
        let bytes = lzss(c, &mut 16);
        let at = ((y - cy) * w + (x - cx)) * 2;
        let row = bytes.get(at..at + n * 2)?;
        Some(row.as_chunks::<2>().0.iter().map(|b| u16::from_le_bytes(*b)).collect())
    })
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

/// The hash a sheet is known by: its height, then its halfwords.
pub fn sheet_hash(rows: &[[u16; 64]]) -> u64 {
    super::fnv1a(
        (rows.len() as u32).to_le_bytes().into_iter().chain(rows.iter().flatten().flat_map(|w| w.to_le_bytes())),
    )
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
    ui_palettes: Vec<Vec<u16>>,
}

impl Catalog {
    /// The UI font uses resident 16-colour CLUTs, not its file's 256-colour
    /// title-image palette: textUploadPalettes (main/textutil.c) loads the
    /// fill palettes (Text_FillClutPixels, 0x80060910 in the USA main
    /// executable, 4 CLUTs) at (256, 243) and the outline ones
    /// (Text_OutlineClutPixels, 0x800609B0, 3 CLUTs) at (0x3D0, 0x1FF). The
    /// sheet's preview takes the first fill palette.
    pub fn apply_ui_font_palette(&mut self, executable: &[u8]) {
        const FILL: (u32, usize) = (0x8006_0910, 4);
        const OUTLINE: (u32, usize) = (0x8006_09B0, 3);
        let table = |(address, count): (u32, usize)| -> Option<Vec<Vec<u16>>> {
            let bytes = super::exe_at(executable, address, count * 32)?;
            Some(
                bytes
                    .as_chunks::<32>()
                    .0
                    .iter()
                    .map(|row| row.as_chunks::<2>().0.iter().map(|b| u16::from_le_bytes(*b)).collect())
                    .collect(),
            )
        };
        let (Some(fill), Some(outline)) = (table(FILL), table(OUTLINE)) else { return };
        self.ui_palettes = fill.into_iter().chain(outline).collect();
        let palette = &self.ui_palettes[0];
        for sheet in &mut self.sheets {
            if sheet.origin == "stage0/file1/0#0" && sheet.place == (960, 256) {
                sheet.palette = Some(palette.clone());
                sheet.palette_source = "resident UI font fill palette (4-bit)";
            }
        }
    }

    /// Merge another disc's catalog without changing existing sheet indices.
    pub fn extend(&mut self, other: Catalog) {
        for sheet in other.sheets {
            if self.by_hash.contains_key(&sheet.hash) {
                continue;
            }
            let index = self.sheets.len() as u32;
            for (k, &piece) in sheet.pieces.iter().enumerate() {
                self.pieces.entry(piece).or_default().push((index, k as u16));
            }
            self.by_hash.insert(sheet.hash, index);
            self.sheets.push(sheet);
        }
    }

    /// The catalog of the sheets of the pictures (`.pe2img`) of `sources`,
    /// with the palettes their files give them.
    pub fn new(sources: &Sources) -> Catalog {
        let mut c = Catalog::default();
        let mut keys: Vec<&String> = sources.pictures.keys().collect();
        keys.sort_unstable();
        for key in keys {
            let (wide, narrow) = palettes(sources.palettes.get(key).map_or(&[][..], Vec::as_slice));
            let columns = sources.pictures[key]
                .iter()
                .enumerate()
                .flat_map(|(n, p)| picture(p).into_iter().enumerate().map(move |(c, col)| (n, c, col)));
            for (i, (n, column, Column { x, y, rows })) in columns.enumerate() {
                if rows.is_empty() {
                    continue;
                }
                // Its file's palettes, in column order: 8-bit ones if any.
                let mut given = [wide.as_slice(), narrow.as_slice()]
                    .into_iter()
                    .find(|p| !p.is_empty())
                    .map(|p| p[i.min(p.len() - 1)].clone());
                let mut source = "its file's palette";
                if given.is_none()
                    && let Some(&(_, depth, file, (px, py))) = FIXED.iter().find(|f| f.0 == (x, y))
                {
                    let chunks = sources.palettes.get(file).map_or(&[][..], Vec::as_slice);
                    given = palette_at(chunks, px, py, [16, 256][depth as usize]);
                    source = "the palette the game's code sets";
                }
                let hash = sheet_hash(&rows);
                if let Some(&known) = c.by_hash.get(&hash) {
                    let sheet = &mut c.sheets[known as usize];
                    if sheet.palette.is_none() {
                        sheet.palette = given;
                        sheet.palette_source = source;
                    }
                    continue;
                }
                let index = c.sheets.len() as u32;
                let mut sheet = Sheet::new(hash, rows);
                sheet.palette = given;
                sheet.palette_source = source;
                sheet.origin = format!("{key}/{n}#{column}");
                sheet.place = (x, y);
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

    /// The glyph pages of `sources`' CAP files (dialogue.rs glyph_pages)
    /// coloured as captions.c draws them: 4-bit, the text through its
    /// palette and the speakers' name labels through theirs. By catalog
    /// index.
    pub fn glyph_pages(&self, sources: &Sources) -> HashMap<u32, super::rooms::Painted> {
        let chunks = sources.palettes.get(CAPTIONS).map_or(&[][..], Vec::as_slice);
        let (Some(text), Some(labels)) = (
            palette_at(chunks, CAPTION_TEXT.0, CAPTION_TEXT.1, 16),
            palette_at(chunks, CAPTION_LABELS.0, CAPTION_LABELS.1, 16),
        ) else {
            return HashMap::new();
        };
        let mut out = HashMap::new();
        for (hash, cells) in super::dialogue::glyph_pages(sources) {
            let Some(&i) = self.by_hash.get(&hash) else { continue };
            let sheet = &self.sheets[i as usize];
            let (width, mut rgba) = sheet.through(0, &text);
            let (_, named) = sheet.through(0, &labels);
            for [u, v, w, h] in cells {
                for y in usize::from(v)..(usize::from(v) + usize::from(h)).min(sheet.rows.len()) {
                    for x in usize::from(u)..(usize::from(u) + usize::from(w)).min(width) {
                        let at = (y * width + x) * 4;
                        rgba[at..at + 4].copy_from_slice(&named[at..at + 4]);
                    }
                }
            }
            out.insert(i, super::rooms::Painted { depth: 0, width, rgba, source: "the captions' palettes" });
        }
        out
    }

    /// The sheet of this hash (catalog index).
    pub fn find(&self, hash: u64) -> Option<u32> {
        self.by_hash.get(&hash).copied()
    }

    /// Whether piece `k` of sheet `sheet` has this hash.
    pub fn piece_is(&self, sheet: u32, k: usize, hash: u64) -> bool {
        self.sheets.get(sheet as usize).and_then(|s| s.pieces.get(k)) == Some(&hash)
    }
}

/// Saves each sheet of `catalog` as `<sheet>.png` in `dir`, coloured as
/// rooms' sprites draw it (`painted`), else through the palette its file
/// gives it, else in grey (at its likely depth); and `index.txt`, a line
/// per sheet: its name, where it comes from, its depth and its colours'
/// source. Answers how many sheets have colours.
pub fn extract(
    catalog: &Catalog,
    painted: &HashMap<u32, super::rooms::Painted>,
    dir: &std::path::Path,
) -> std::io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let mut coloured = 0;
    let mut index = String::from(
        "# sheet, where it is on the disc (file/chunk#column), where it loads in VRAM, depth, colours
",
    );
    for (i, sheet) in catalog.sheets.iter().enumerate() {
        // The fill and two outline passes use different CLUTs. Export
        // palette-specific replacements so one pass cannot erase another.
        if sheet.origin == "stage0/file1/0#0" && !catalog.ui_palettes.is_empty() {
            for palette in &catalog.ui_palettes {
                let hash = super::fnv1a(palette.iter().flat_map(|w| w.to_le_bytes()));
                let name = super::texture_name((sheet.hash, hash));
                let (width, rgba) = sheet.through(0, palette);
                let file = std::io::BufWriter::new(std::fs::File::create(dir.join(format!("{name}.png")))?);
                let mut enc = png::Encoder::new(file, width as u32, sheet.rows.len() as u32);
                enc.set_color(png::ColorType::Rgba);
                enc.set_depth(png::BitDepth::Eight);
                enc.write_header().and_then(|mut w| w.write_image_data(&rgba)).map_err(std::io::Error::other)?;
                index.push_str(&format!(
                    "{name} {} {},{} 4-bit resident UI font palette\n",
                    sheet.origin, sheet.place.0, sheet.place.1
                ));
            }
            coloured += 1;
            continue;
        }
        let name = super::texture_name((sheet.hash, 0));
        let path = dir.join(format!("{name}.png"));
        let (width, rgba, depth, source) = match painted.get(&(i as u32)) {
            Some(p) => (p.width, p.rgba.clone(), p.depth, p.source),
            None => {
                let depth = sheet.depth();
                let (width, rgba) = sheet.picture(depth);
                let given = sheet.palette.as_ref().is_some_and(|p| p.len() == [16, 256][depth.min(1) as usize]);
                (width, rgba, depth, if given { sheet.palette_source } else { "none (grey)" })
            }
        };
        if !source.starts_with("none") {
            coloured += 1;
        }
        index.push_str(&format!(
            "{name} {} {},{} {}-bit {source}
",
            sheet.origin,
            sheet.place.0,
            sheet.place.1,
            [4, 8, 15][depth.min(2) as usize]
        ));
        let file = std::io::BufWriter::new(std::fs::File::create(&path)?);
        let mut enc = png::Encoder::new(file, width as u32, sheet.rows.len() as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().and_then(|mut w| w.write_image_data(&rgba)).map_err(std::io::Error::other)?;
    }
    std::fs::write(dir.join("index.txt"), index)?;
    Ok(coloured)
}

/// Saves each 4- and 8-bit sheet of `catalog` as indexed replacements at
/// its own size in `dir` (indexed.rs): its texels as indices, no palette
/// applied, at both depths (the game may draw a sheet either way). Each
/// header says what is known of its depth: the one the game draws it at
/// where that is known (rooms' sprites and code, the captions, its file's
/// palette, the UI font), else the likely one, and the other as such.
/// Each sheet's ramp picture, at its drawn depth, goes in `ramps`
/// (indexed.rs `ramp`). Answers how many sheets, and how many of a guessed
/// depth.
pub fn extract_indexed(
    catalog: &Catalog,
    painted: &HashMap<u32, super::rooms::Painted>,
    dir: &std::path::Path,
    ramps: &std::path::Path,
) -> std::io::Result<(usize, usize)> {
    std::fs::create_dir_all(dir)?;
    let (mut written, mut guessed) = (0, 0);
    for (i, sheet) in catalog.sheets.iter().enumerate() {
        let (depth, source) = if sheet.origin == "stage0/file1/0#0" && !catalog.ui_palettes.is_empty() {
            (0, "the UI font")
        } else if let Some(p) = painted.get(&(i as u32)) {
            (p.depth, p.source)
        } else if let Some(p) = &sheet.palette {
            (if p.len() == 16 { 0 } else { 1 }, sheet.palette_source)
        } else {
            guessed += 1;
            (sheet.likely_depth(), "guessed")
        };
        if depth > 1 {
            continue;
        }
        for d in [0, 1] {
            let (width, indices) = super::indexed::sheet_indices(&sheet.rows, d);
            let is_drawn = d == depth;
            let rows = sheet.rows.len() as u32;
            let replacement = super::indexed::Indexed {
                sheet: sheet.hash,
                depth: d,
                native: (width, rows),
                size: (width, rows),
                indices,
            };
            let note = if is_drawn { format!("drawn so: {source}") } else { format!("the other depth ({source})") };
            super::indexed::write(dir, &replacement, Some(&note)).map_err(std::io::Error::other)?;
            // The ramp picture to enlarge, at the depth it is drawn.
            if is_drawn {
                super::indexed::write_ramp(ramps, &replacement).map_err(std::io::Error::other)?;
            }
        }
        written += 1;
    }
    Ok((written, guessed))
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

    #[test]
    fn ui_font_uses_resident_four_bit_palette() {
        // The fill palettes (4) and the outline ones (3) where the
        // executable holds them (Text_FillClut's record between them); each
        // colour its CLUT's number, 1 to 7.
        let mut exe = vec![0; 0x800 + 0xA0 + 3 * 32];
        exe[..8].copy_from_slice(b"PS-X EXE");
        exe[0x18..0x1C].copy_from_slice(&0x8006_0910u32.to_le_bytes());
        for clut in 0..7 {
            for i in 0..16 {
                let at = 0x800 + if clut < 4 { clut * 32 } else { 0xA0 + (clut - 4) * 32 } + i * 2;
                exe[at..at + 2].copy_from_slice(&(clut as u16 + 1).to_le_bytes());
            }
        }
        let mut sheet = Sheet::new(1, vec![[0x1234; 64]; 256]);
        sheet.origin = "stage0/file1/0#0".into();
        sheet.place = (960, 256);
        sheet.palette = Some(vec![0; 256]);
        let mut catalog = Catalog::default();
        catalog.sheets.push(sheet);
        catalog.apply_ui_font_palette(&exe);
        assert_eq!(catalog.sheets[0].depth(), 0);
        assert_eq!(catalog.sheets[0].picture(0).0, 256);
        // The preview takes the first fill palette, not an outline one.
        assert_eq!(catalog.sheets[0].palette.as_deref(), Some(&[1u16; 16][..]));
        assert_eq!(catalog.ui_palettes.len(), 7);
        assert_eq!(catalog.ui_palettes[4][0], 5);
    }

    #[test]
    fn merge_disc_catalogs_keeps_indices_and_piece_lookup() {
        let mut first = Catalog::default();
        let mut second = Catalog::default();
        first.sheets.push(Sheet::new(10, vec![[1; 64]; 32]));
        second.sheets.push(Sheet::new(10, vec![[1; 64]; 32]));
        second.sheets.push(Sheet::new(20, vec![[2; 64]; 32]));
        let mut merged = Catalog::default();
        merged.extend(first);
        merged.extend(second);
        assert_eq!(merged.sheets.len(), 2);
        assert_eq!(merged.find(10), Some(0));
        assert_eq!(merged.find(20), Some(1));
        let hash = piece_hash(std::iter::repeat_n(2, 64 * 32), 0);
        assert_eq!(merged.pieces(hash), &[(1, 0)]);
    }

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
