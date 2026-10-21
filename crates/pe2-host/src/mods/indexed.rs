//! Indexed texture replacements: a sheet's texels as palette indices, not
//! colours, so the game's own palette (the CLUT the primitive is drawn
//! with) still colours them when they are drawn.
//!
//! A 4- or 8-bit sheet is drawn with whatever CLUT the game picks for each
//! primitive: the UI font's fill and outlines, a room's palette blocks, an
//! enemy's variants. A picture in colours (`textures/<sheet>.png`) fixes one
//! of them. An indexed replacement keeps the indices instead, at any whole
//! multiple of the sheet's size, and the renderer looks each one up in the
//! CLUT the game set, as the console does: index colour 0 is transparent,
//! the colour's bit 15 still decides semi-transparency, and the
//! primitive's blending and shading apply unchanged.
//!
//! A replacement is two files in a mod's `textures/` folder:
//!
//! - `<sheet>-<4|8>bit.idx.png`: one value per texel, the index (0-15 for a
//!   4-bit sheet, 0-255 for an 8-bit one), as an 8-bit (or less) grey or
//!   indexed PNG. Its values are read as they are stored: no palette,
//!   gamma or colour conversion is applied.
//! - `<sheet>-<4|8>bit.idx.toml`: what the values are (`Header`).
//!
//! The depth is in the name because the game may draw the same sheet at
//! either depth (the title's 8-bit picture is also the menus' 4-bit
//! frames): each replaces the sheet where it is drawn at its depth.
//! `pe2 --extract` writes every 4- and 8-bit sheet at both depths in
//! `dumps/textures-indexed/`, at the sheet's own size, the headers saying
//! which depth the game is known to draw it at.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// What the header's `format` says.
pub const FORMAT: &str = "pe2pc-indexed-texture";
/// The format's version this build reads and writes.
pub const VERSION: u32 = 1;
/// The texels' order: left to right in a row, rows top to bottom, one
/// value each, the sheet's texel (u, v) at (u, v) times the scale.
pub const LAYOUT: &str = "sheet-rows";
/// The largest replacement side (the renderer's atlas layers).
pub const MAX_SIDE: u32 = 2048;

/// An indexed replacement's header (`<sheet>.idx.toml`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    /// `FORMAT`.
    pub format: String,
    /// `VERSION`.
    pub version: u32,
    /// The sheet it replaces: its hash, as its file name has it.
    pub sheet: String,
    /// Bits per texel: 4 or 8.
    pub depth: u32,
    /// The sheet's size in texels: 256 (4-bit) or 128 (8-bit) wide, as many
    /// rows as it has.
    pub native_width: u32,
    pub native_height: u32,
    /// The replacement's size: a whole multiple of the native size, the same
    /// for both.
    pub width: u32,
    pub height: u32,
    /// `LAYOUT`.
    pub layout: String,
    /// Where the depth came from when extracted (for information only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth_source: Option<String>,
}

/// An indexed replacement, read and checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indexed {
    pub sheet: u64,
    /// 0: 4 bits, 1: 8 bits (as the GPU's texture depth).
    pub depth: u32,
    pub native: (u32, u32),
    pub size: (u32, u32),
    /// `size.0 * size.1` indices, rows top to bottom.
    pub indices: Vec<u8>,
}

impl Indexed {
    /// How many replacement texels make one of the sheet's on a side.
    pub fn scale(&self) -> u32 {
        self.size.0 / self.native.0
    }
}

/// A sheet's texels (its rows of 64 halfwords) as indices at `depth` (0:
/// 4 bits, 1: 8), as the GPU reads them: each halfword's low bits first.
pub fn sheet_indices(rows: &[[u16; 64]], depth: u32) -> (u32, Vec<u8>) {
    let per = if depth == 0 { 4 } else { 2 };
    let width = 64 * per;
    let mut out = Vec::with_capacity(width * rows.len());
    for row in rows {
        for u in 0..width {
            let word = row[u / per];
            out.push(if depth == 0 { (word >> ((u % 4) * 4)) & 0xF } else { (word >> ((u % 2) * 8)) & 0xFF } as u8);
        }
    }
    (width as u32, out)
}

/// The replacement texel that position `at` (in the sheet's texels, a
/// fraction within one for the finer replacement texels) falls on,
/// nearest, inside the replacement: a model of what the renderer's shader
/// computes (runtime/gpu/hw.rs), for the tests.
#[cfg(test)]
pub fn texel_at(at: (f32, f32), native: (u32, u32), size: (u32, u32)) -> (u32, u32) {
    let f = |p: f32, n: u32, s: u32| ((p.max(0.0) / n as f32 * s as f32) as u32).min(s - 1);
    (f(at.0, native.0, size.0), f(at.1, native.1, size.1))
}

/// The 15-bit colour index `index` stands for in the CLUT at (`clut_x`,
/// `clut_y`) of `vram` (1024 x 512 halfwords): a model of what the
/// renderer's shader looks up, for the tests. Colour 0 is transparent; bit
/// 15 marks semi-transparency.
#[cfg(test)]
pub fn clut_colour(vram: &[u16], clut_x: u32, clut_y: u32, index: u8, depth: u32) -> u16 {
    let index = u32::from(index) & if depth == 0 { 0xF } else { 0xFF };
    vram[((clut_y & 511) * 1024 + ((clut_x + index) & 1023)) as usize]
}

/// The header's file next to an `.idx.png`.
pub fn header_path(png: &Path) -> PathBuf {
    let name = png.file_name().and_then(|n| n.to_str()).unwrap_or("");
    png.with_file_name(format!("{}.toml", name.trim_end_matches(".png")))
}

/// A replacement's file name, without `.idx.png`: `<sheet>-<4|8>bit`.
pub fn stem(sheet: u64, depth: u32) -> String {
    format!("{sheet:016x}-{}bit", [4, 8][depth.min(1) as usize])
}

/// The sheet and depth (0: 4 bits, 1: 8) a replacement's file name (less
/// `.idx.png`) names.
pub fn parse_stem(stem: &str) -> Option<(u64, u32)> {
    let (sheet, bits) = stem.split_once('-')?;
    let depth = match bits.to_ascii_lowercase().as_str() {
        "4bit" => 0,
        "8bit" => 1,
        _ => return None,
    };
    Some((u64::from_str_radix(sheet, 16).ok()?, depth))
}

/// Writes `indexed` as `<sheet>-<4|8>bit.idx.png` and `.idx.toml` in `dir`.
pub fn write(dir: &Path, indexed: &Indexed, depth_source: Option<&str>) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let name = format!("{:016x}", indexed.sheet);
    let png = dir.join(format!("{}.idx.png", stem(indexed.sheet, indexed.depth)));
    let file = std::fs::File::create(&png).map_err(|e| e.to_string())?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), indexed.size.0, indexed.size.1);
    enc.set_color(png::ColorType::Grayscale);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().and_then(|mut w| w.write_image_data(&indexed.indices)).map_err(|e| e.to_string())?;
    let header = Header {
        format: FORMAT.into(),
        version: VERSION,
        sheet: name,
        depth: [4, 8][indexed.depth.min(1) as usize],
        native_width: indexed.native.0,
        native_height: indexed.native.1,
        width: indexed.size.0,
        height: indexed.size.1,
        layout: LAYOUT.into(),
        depth_source: depth_source.map(str::to_string),
    };
    let text = toml::to_string(&header).map_err(|e| e.to_string())?;
    std::fs::write(header_path(&png), text).map_err(|e| e.to_string())?;
    Ok(png)
}

/// Reads and checks the replacement at `png` (with its header next to it).
pub fn read(png: &Path) -> Result<Indexed, String> {
    let text = std::fs::read_to_string(header_path(png)).map_err(|e| format!("{}: {e}", header_path(png).display()))?;
    let header: Header = toml::from_str(&text).map_err(|e| format!("{}: {e}", header_path(png).display()))?;
    let bytes = std::fs::read(png).map_err(|e| e.to_string())?;
    let (w, h, values) = decode_values(&bytes)?;
    let stem = png.file_name().and_then(|n| n.to_str()).unwrap_or("").trim_end_matches(".idx.png");
    check(&header, stem, (w, h), values)
}

/// Checks a header against its file name's sheet and its PNG's values.
fn check(header: &Header, stem: &str, (w, h): (u32, u32), values: Vec<u8>) -> Result<Indexed, String> {
    if header.format != FORMAT {
        return Err(format!("format is \"{}\", not \"{FORMAT}\"", header.format));
    }
    if header.version != VERSION {
        return Err(format!("version {} (this build reads {VERSION})", header.version));
    }
    if header.layout != LAYOUT {
        return Err(format!("layout \"{}\", not \"{LAYOUT}\"", header.layout));
    }
    let sheet = u64::from_str_radix(&header.sheet, 16).map_err(|_| format!("sheet \"{}\" is no hash", header.sheet))?;
    let depth = match header.depth {
        4 => 0,
        8 => 1,
        d => return Err(format!("depth {d} (4 or 8)")),
    };
    if parse_stem(stem) != Some((sheet, depth)) {
        return Err(format!(
            "file is named {stem}, but its header says sheet {} at {} bits ({})",
            header.sheet,
            header.depth,
            self::stem(sheet, depth)
        ));
    }
    let native_width = [256, 128][depth as usize];
    if header.native_width != native_width || header.native_height == 0 || header.native_height > 512 {
        return Err(format!(
            "native size {}x{} (a {}-bit sheet is {native_width} wide, 1 to 512 high)",
            header.native_width, header.native_height, header.depth
        ));
    }
    if (w, h) != (header.width, header.height) {
        return Err(format!("picture is {w}x{h}, its header says {}x{}", header.width, header.height));
    }
    let scale = w / header.native_width;
    if scale == 0 || w != header.native_width * scale || h != header.native_height * scale {
        return Err(format!(
            "{w}x{h} is not a whole multiple of {}x{} (the same on both sides)",
            header.native_width, header.native_height
        ));
    }
    if w > MAX_SIDE || h > MAX_SIDE {
        return Err(format!("{w}x{h} is larger than {MAX_SIDE} on a side"));
    }
    let limit = if depth == 0 { 16 } else { 256 };
    if let Some(at) = values.iter().position(|&v| u32::from(v) >= limit) {
        return Err(format!(
            "value {} at ({}, {}) is no {}-bit index (0 to {})",
            values[at],
            at as u32 % w,
            at as u32 / w,
            header.depth,
            limit - 1
        ));
    }
    Ok(Indexed { sheet, depth, native: (header.native_width, header.native_height), size: (w, h), indices: values })
}

/// A PNG's samples as stored: one per pixel, from a grey or indexed PNG of
/// 8 bits or fewer (fewer unpacked, values kept). Anything else (colour,
/// alpha, 16 bits) is refused: its values could not be indices.
fn decode_values(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let mut decoder = png::Decoder::new(bytes);
    decoder.set_transformations(png::Transformations::IDENTITY);
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    if !matches!(info.color_type, png::ColorType::Grayscale | png::ColorType::Indexed) {
        return Err(format!("{:?} PNG: indices need a grey or indexed one, without alpha", info.color_type));
    }
    let bits = match info.bit_depth {
        png::BitDepth::One => 1,
        png::BitDepth::Two => 2,
        png::BitDepth::Four => 4,
        png::BitDepth::Eight => 8,
        png::BitDepth::Sixteen => return Err("16-bit PNG: indices need 8 bits or fewer".into()),
    };
    let (w, h) = (info.width, info.height);
    let mut values = Vec::with_capacity((w * h) as usize);
    for row in buf[..info.buffer_size()].chunks_exact(info.line_size) {
        for x in 0..w as usize {
            let bit = x * bits;
            let byte = row[bit / 8];
            values.push((byte >> (8 - bits - bit % 8)) & ((1u16 << bits) - 1) as u8);
        }
    }
    Ok((w, h, values))
}

/// The grey level index `index` is shown as in a ramp picture: spread over
/// the whole range, 17 apart at 4 bits, 1 apart at 8 (index 0 black).
pub fn ramp_level(index: u8, depth: u32) -> u8 {
    if depth == 0 { (index & 0xF) * 17 } else { index }
}

/// `indexed` as a ramp picture (RGBA, opaque): each index its own grey,
/// for an upscaler to enlarge where a palette's colours would not tell the
/// indices apart (a font whose fill and outline look alike in each of its
/// palettes).
pub fn ramp(indexed: &Indexed) -> Vec<u8> {
    indexed
        .indices
        .iter()
        .flat_map(|&i| {
            let g = ramp_level(i, indexed.depth);
            [g, g, g, 255]
        })
        .collect()
}

/// Saves `indexed`'s ramp picture as `<sheet>-<4|8>bit.png` in `dir`.
pub fn write_ramp(dir: &Path, indexed: &Indexed) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}.png", stem(indexed.sheet, indexed.depth)));
    let file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), indexed.size.0, indexed.size.1);
    enc.set_color(png::ColorType::Grayscale);
    enc.set_depth(png::BitDepth::Eight);
    let levels: Vec<u8> = indexed.indices.iter().map(|&i| ramp_level(i, indexed.depth)).collect();
    enc.write_header().and_then(|mut w| w.write_image_data(&levels)).map_err(|e| e.to_string())?;
    Ok(path)
}

/// An RGBA picture: its size and pixels.
pub struct Rgba<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [u8],
}

/// The indices of an enlarged picture of the sheet `native` (say, its
/// colour preview made larger by an AI upscaler), at the enlarged
/// picture's size, a whole multiple of the sheet's.
///
/// `preview` is the picture of the sheet the enlarged one was made from,
/// at the sheet's size: it tells which colour each index stands for in it
/// (an index's mean colour where the preview shows it). Each enlarged pixel
/// then takes, of the indices of its own texel and the eight around it in
/// the sheet, the one whose colour is nearest its own (transparent pixels
/// nearest transparent ones; a tie keeps its own texel's index). So the
/// upscaler decides where edges go, and the indices stay the sheet's: an
/// index whose colour a palette repeats keeps its place, and no index
/// appears that is not near it in the original.
pub fn from_upscale(native: &Indexed, preview: &Rgba, upscaled: &Rgba) -> Result<Indexed, String> {
    let (w, h) = native.size;
    if native.size != native.native {
        return Err("the indexed sheet must be at its own size (as extracted)".into());
    }
    if (preview.width, preview.height) != (w, h) || preview.pixels.len() != (w * h * 4) as usize {
        return Err(format!(
            "the preview is {}x{}, but the {}-bit sheet is {w}x{h} (a preview of the other depth?)",
            preview.width,
            preview.height,
            [4, 8][native.depth as usize]
        ));
    }
    let scale = upscaled.width / w;
    if scale == 0 || upscaled.width != w * scale || upscaled.height != h * scale {
        return Err(format!(
            "the enlarged picture is {}x{}, not a whole multiple of {w}x{h} (the same on both sides)",
            upscaled.width, upscaled.height
        ));
    }
    if upscaled.pixels.len() != (upscaled.width * upscaled.height * 4) as usize
        || upscaled.width > MAX_SIDE
        || upscaled.height > MAX_SIDE
    {
        return Err(format!("the enlarged picture is larger than {MAX_SIDE} on a side, or incomplete"));
    }
    // Each index's colour in the preview (RGB and alpha), where it shows.
    let mut sums = vec![[0u64; 5]; 256];
    for (i, &index) in native.indices.iter().enumerate() {
        let p = &preview.pixels[i * 4..i * 4 + 4];
        let s = &mut sums[usize::from(index)];
        for c in 0..4 {
            s[c] += u64::from(p[c]);
        }
        s[4] += 1;
    }
    let colour: Vec<Option<[i64; 4]>> =
        sums.iter().map(|s| (s[4] > 0).then(|| [0, 1, 2, 3].map(|c| (s[c] / s[4]) as i64))).collect();
    let distance = |a: [i64; 4], b: &[u8]| -> i64 {
        // Transparent and opaque never mix: alpha weighs as much as all of
        // the colour.
        let rgb: i64 = (0..3).map(|c| (a[c] - i64::from(b[c])).pow(2)).sum();
        rgb + 3 * (a[3] - i64::from(b[3])).pow(2)
    };
    let (uw, uh) = (upscaled.width, upscaled.height);
    let mut indices = Vec::with_capacity((uw * uh) as usize);
    for y in 0..uh {
        for x in 0..uw {
            let (u, v) = (x / scale, y / scale);
            let own = native.indices[(v * w + u) as usize];
            let pixel = &upscaled.pixels[((y * uw + x) * 4) as usize..((y * uw + x) * 4 + 4) as usize];
            let mut best = (i64::MAX, own);
            let mut consider = |index: u8| {
                if let Some(c) = colour[usize::from(index)] {
                    let d = distance(c, pixel);
                    if d < best.0 {
                        best = (d, index);
                    }
                }
            };
            // Its own texel's index first, so a tie keeps it.
            consider(own);
            for dv in -1i32..=1 {
                for du in -1i32..=1 {
                    let (nu, nv) = (u as i32 + du, v as i32 + dv);
                    if (0..w as i32).contains(&nu) && (0..h as i32).contains(&nv) {
                        consider(native.indices[(nv as u32 * w + nu as u32) as usize]);
                    }
                }
            }
            indices.push(best.1);
        }
    }
    Ok(Indexed { size: (uw, uh), indices, ..native.clone() })
}

/// `indexed` made `factor` times larger, each texel repeated (nearest): the
/// values stay the same indices.
pub fn upscale(indexed: &Indexed, factor: u32) -> Result<Indexed, String> {
    let size = (indexed.size.0 * factor, indexed.size.1 * factor);
    if factor == 0 || size.0 > MAX_SIDE || size.1 > MAX_SIDE {
        return Err(format!(
            "{}x{} times {factor} is out of 1 to {MAX_SIDE} on a side",
            indexed.size.0, indexed.size.1
        ));
    }
    let mut indices = Vec::with_capacity((size.0 * size.1) as usize);
    for y in 0..size.1 {
        let row = (y / factor * indexed.size.0) as usize;
        indices.extend((0..size.0).map(|x| indexed.indices[row + (x / factor) as usize]));
    }
    Ok(Indexed { size, indices, ..indexed.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("pe2-indexed-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    fn sample(depth: u32) -> Indexed {
        let mut rows = vec![[0u16; 64]; 4];
        for (r, row) in rows.iter_mut().enumerate() {
            for (i, w) in row.iter_mut().enumerate() {
                *w = ((r * 64 + i) as u16).wrapping_mul(0x1357);
            }
        }
        let (w, indices) = sheet_indices(&rows, depth);
        Indexed { sheet: 0xabc, depth, native: (w, 4), size: (w, 4), indices }
    }

    #[test]
    fn indices_follow_the_gpus_texel_order() {
        let mut row = [0u16; 64];
        row[0] = 0x4321;
        row[1] = 0xBA98;
        let (w, i) = sheet_indices(&[row], 0);
        assert_eq!(w, 256);
        assert_eq!(&i[..8], &[1, 2, 3, 4, 8, 9, 10, 11]);
        let (w, i) = sheet_indices(&[row], 1);
        assert_eq!(w, 128);
        assert_eq!(&i[..4], &[0x21, 0x43, 0x98, 0xBA]);
    }

    #[test]
    fn export_and_import_keep_every_index() {
        for depth in [0, 1] {
            let d = dir(&format!("roundtrip{depth}"));
            let original = sample(depth);
            let png = write(&d, &original, Some("test")).unwrap();
            assert_eq!(read(&png).unwrap(), original);
            let big = upscale(&original, 4).unwrap();
            let png = write(&d, &big, None).unwrap();
            let back = read(&png).unwrap();
            assert_eq!(back, big);
            assert_eq!(back.scale(), 4);
            // Every enlarged texel is its sheet texel's index.
            for y in 0..big.size.1 {
                for x in 0..big.size.0 {
                    let native = original.indices[((y / 4) * original.size.0 + x / 4) as usize];
                    assert_eq!(big.indices[(y * big.size.0 + x) as usize], native);
                }
            }
            let _ = std::fs::remove_dir_all(&d);
        }
    }

    #[test]
    fn packed_png_values_are_unpacked_unchanged() {
        // A 4-bit grey PNG, as an editor saving 16 levels might write it.
        let mut bytes = Vec::new();
        let mut enc = png::Encoder::new(&mut bytes, 4, 1);
        enc.set_color(png::ColorType::Grayscale);
        enc.set_depth(png::BitDepth::Four);
        enc.write_header().unwrap().write_image_data(&[0x1F, 0x70]).unwrap();
        assert_eq!(decode_values(&bytes).unwrap(), (4, 1, vec![1, 15, 7, 0]));
    }

    #[test]
    fn invalid_replacements_are_refused() {
        let good = Header {
            format: FORMAT.into(),
            version: VERSION,
            sheet: "0000000000000abc".into(),
            depth: 4,
            native_width: 256,
            native_height: 2,
            width: 512,
            height: 4,
            layout: LAYOUT.into(),
            depth_source: None,
        };
        let values = || vec![0u8; 512 * 4];
        let stem = "0000000000000abc-4bit";
        assert!(check(&good, stem, (512, 4), values()).is_ok());
        // The name's depth must be the header's.
        assert!(check(&good, "0000000000000abc-8bit", (512, 4), values()).is_err());
        assert!(check(&good, "0000000000000abc", (512, 4), values()).is_err());
        let bad = |f: &dyn Fn(&mut Header)| {
            let mut h = good.clone();
            f(&mut h);
            check(&h, stem, (h.width, h.height), vec![0; (h.width * h.height) as usize]).is_err()
        };
        assert!(bad(&|h| h.version = 2));
        assert!(bad(&|h| h.format = "other".into()));
        assert!(bad(&|h| h.layout = "columns".into()));
        assert!(bad(&|h| h.depth = 15));
        assert!(bad(&|h| h.native_width = 128));
        assert!(bad(&|h| h.width = 384));
        assert!(bad(&|h| h.height = 6));
        assert!(bad(&|h| h.sheet = "0000000000000def".into()));
        // Picture and header disagree.
        assert!(check(&good, stem, (256, 2), vec![0; 512]).is_err());
        // A 4-bit index past 15.
        let mut v = values();
        v[5] = 16;
        assert!(check(&good, stem, (512, 4), v).is_err());
    }

    #[test]
    fn colour_pngs_are_refused() {
        let mut bytes = Vec::new();
        let mut enc = png::Encoder::new(&mut bytes, 1, 1);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().unwrap().write_image_data(&[1, 2, 3, 255]).unwrap();
        assert!(decode_values(&bytes).is_err());
    }

    /// A sheet of `w` x `h` from rows of indices, and its preview through
    /// `palette` (RGBA per index).
    fn sheet_and_preview(rows: &[&[u8]], palette: &[[u8; 4]]) -> (Indexed, Vec<u8>) {
        let (w, h) = (rows[0].len() as u32, rows.len() as u32);
        let indices: Vec<u8> = rows.iter().flat_map(|r| r.iter().copied()).collect();
        let preview = indices.iter().flat_map(|&i| palette[usize::from(i)]).collect();
        (Indexed { sheet: 1, depth: 0, native: (w, h), size: (w, h), indices }, preview)
    }

    #[test]
    fn an_upscale_by_repeating_gives_the_indices_back() {
        let palette = [[0, 0, 0, 0], [200, 30, 30, 255], [30, 200, 30, 255], [30, 30, 200, 255]];
        let (sheet, preview) = sheet_and_preview(&[&[0, 1, 2], &[3, 2, 1]], &palette);
        let big = upscale(&sheet, 3).unwrap();
        let pixels: Vec<u8> = big.indices.iter().flat_map(|&i| palette[usize::from(i)]).collect();
        let prev = Rgba { width: 3, height: 2, pixels: &preview };
        let up = Rgba { width: 9, height: 6, pixels: &pixels };
        assert_eq!(from_upscale(&sheet, &prev, &up).unwrap().indices, big.indices);
    }

    #[test]
    fn a_repeated_colour_keeps_each_texels_own_index() {
        // Indices 1 and 2 are the same colour (as palettes often repeat a
        // black): each enlarged texel keeps its own.
        let palette = [[0, 0, 0, 0], [10, 10, 10, 255], [10, 10, 10, 255]];
        let (sheet, preview) = sheet_and_preview(&[&[1, 2]], &palette);
        let pixels: Vec<u8> = (0..4 * 2).flat_map(|_| [10, 10, 10, 255]).collect();
        let prev = Rgba { width: 2, height: 1, pixels: &preview };
        let up = Rgba { width: 4, height: 2, pixels: &pixels };
        assert_eq!(from_upscale(&sheet, &prev, &up).unwrap().indices, vec![1, 1, 2, 2, 1, 1, 2, 2]);
    }

    #[test]
    fn smoothed_edges_take_the_nearest_index_around() {
        // Between a red and a blue texel the upscaler put a reddish, a
        // purple and a bluish pixel: nearest wins; a colour of no index near
        // (green, index 3, is far away) never appears.
        let palette = [[0, 0, 0, 0], [255, 0, 0, 255], [0, 0, 255, 255], [0, 255, 0, 255]];
        let (sheet, preview) = sheet_and_preview(&[&[1, 2], &[1, 2], &[3, 3]], &palette);
        let row = |a: [u8; 4], b: [u8; 4], c: [u8; 4], d: [u8; 4]| [a, b, c, d];
        let (red, blue) = ([255, 0, 0, 255], [0, 0, 255, 255]);
        let mut pixels = Vec::new();
        for _ in 0..4 {
            pixels.extend(row(red, [200, 0, 60, 255], [60, 0, 200, 255], blue).iter().flatten());
        }
        for _ in 0..2 {
            pixels.extend(row([0, 255, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255], [0, 255, 0, 255]).iter().flatten());
        }
        let prev = Rgba { width: 2, height: 3, pixels: &preview };
        let up = Rgba { width: 4, height: 6, pixels: &pixels };
        let out = from_upscale(&sheet, &prev, &up).unwrap();
        assert_eq!(&out.indices[..4], &[1, 1, 2, 2]);
        assert!(out.indices[..16].iter().all(|&i| i != 3));
    }

    #[test]
    fn a_smoothed_ramp_gives_clean_indices_back() {
        // Fill (index 15) next to outline (index 5) next to nothing: as a
        // ramp, then blurred across each edge, as an upscaler would.
        let sheet = Indexed { sheet: 1, depth: 0, native: (3, 1), size: (3, 1), indices: vec![0, 5, 15] };
        let preview = ramp(&sheet);
        let levels = [0u8, 0, 30, 85, 85, 140, 255, 255];
        let pixels: Vec<u8> = levels.iter().flat_map(|&g| [g, g, g, 255]).collect();
        // 8 pixels wide would not be a whole multiple of 3: use 6.
        let pixels = &pixels[..6 * 4];
        let out = from_upscale(
            &sheet,
            &Rgba { width: 3, height: 1, pixels: &preview },
            &Rgba { width: 6, height: 2, pixels: &[pixels, pixels].concat() },
        )
        .unwrap();
        // Each blurred grey takes the nearest index's: 30 is nearer black
        // (0) than the outline's 85, 140 nearer 85 than the fill's 255.
        assert_eq!(&out.indices[..6], &[0, 0, 0, 5, 5, 5]);
        assert_eq!(ramp_level(15, 0), 255);
        assert_eq!(ramp_level(200, 1), 200);
    }

    #[test]
    fn mismatched_pictures_are_refused() {
        let palette = [[0, 0, 0, 0], [255, 255, 255, 255]];
        let (sheet, preview) = sheet_and_preview(&[&[1, 0]], &palette);
        let prev = Rgba { width: 2, height: 1, pixels: &preview };
        // Not a whole multiple, or not the same on both sides.
        let bad = vec![0u8; 3 * 4];
        assert!(from_upscale(&sheet, &prev, &Rgba { width: 3, height: 1, pixels: &bad }).is_err());
        let bad = vec![0u8; 4 * 4];
        assert!(from_upscale(&sheet, &prev, &Rgba { width: 4, height: 1, pixels: &bad }).is_err());
        // A preview of another size (the other depth).
        let wrong = vec![0u8; 4 * 4];
        let up = vec![0u8; 4 * 2 * 4];
        assert!(
            from_upscale(
                &sheet,
                &Rgba { width: 4, height: 1, pixels: &wrong },
                &Rgba { width: 4, height: 2, pixels: &up }
            )
            .is_err()
        );
    }

    #[test]
    fn upscaled_texels_map_back_to_their_sheet_texel() {
        let (native, size) = ((256, 32), (1024, 128));
        // Sheet texel (10, 3) covers replacement texels 40-43, 12-15.
        assert_eq!(texel_at((10.0, 3.0), native, size), (40, 12));
        assert_eq!(texel_at((10.99, 3.99), native, size), (43, 15));
        assert_eq!(texel_at((10.5, 3.5), native, size), (42, 14));
        // At the sheet's size, the same texel the game reads.
        assert_eq!(texel_at((10.7, 3.2), native, native), (10, 3));
        // Never outside the replacement.
        assert_eq!(texel_at((300.0, -2.0), native, size), (1023, 0));
    }

    #[test]
    fn indices_take_the_clut_the_game_set() {
        let mut vram = vec![0u16; 1024 * 512];
        // Two CLUTs for the same index: the fill's and an outline's.
        vram[245 * 1024 + 256 + 3] = 0x7FFF;
        vram[511 * 1024 + 0x3D0 + 3] = 0x801F;
        assert_eq!(clut_colour(&vram, 256, 245, 3, 0), 0x7FFF);
        assert_eq!(clut_colour(&vram, 0x3D0, 511, 3, 0), 0x801F);
        // A 4-bit index never reaches past its 16 colours.
        assert_eq!(clut_colour(&vram, 256, 245, 0x13, 0), 0x7FFF);
        // Index 0's colour 0 stays transparent.
        assert_eq!(clut_colour(&vram, 256, 245, 0, 0), 0);
    }
}
