//! Tools for making mods (`pe2 --<tool>`, docs/MODDING.md): what PE2PC
//! knows of the discs put to work on a mod's files.
//!
//! - `pieces_from_backgrounds`: the pieces of rooms drawn over their
//!   backgrounds (pillars, railings, furniture edges) cut from the mod's
//!   enlarged backgrounds, so they match them exactly.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::Picture;
use super::rooms::Texel;
use super::sheets::Catalog;

/// What `pieces_from_backgrounds` did.
#[derive(Default)]
pub struct PiecesReport {
    /// Sheets written, and those left as they were (a file already there).
    pub written: Vec<PathBuf>,
    pub kept: Vec<PathBuf>,
    /// Backgrounds of the mod whose cameras draw pieces, and why some were
    /// not used.
    pub backgrounds: usize,
    pub refused: Vec<String>,
}

/// A sheet being cut: its size at the backgrounds' scale and its pixels
/// (RGBA), where it is filed, and its name.
struct Cut {
    width: usize,
    height: usize,
    scale: usize,
    rgba: Vec<u8>,
    folder: PathBuf,
    name: String,
}

/// The mod's backgrounds, by file stem (its place or its hash).
fn mod_backgrounds(mod_dir: &Path) -> HashMap<String, PathBuf> {
    super::files_under(&mod_dir.join("backgrounds"))
        .into_iter()
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("png")))
        .filter_map(|p| Some((p.file_stem()?.to_str()?.to_ascii_lowercase(), p)))
        .collect()
}

/// Cuts the sheets of rooms' pieces from the mod at `mod_dir`'s enlarged
/// backgrounds, with the discs `images`: each sheet a camera's sprites draw
/// over a background the mod replaces is written to the mod's `textures/`
/// (in the extract's folders, named by place) at the background's scale,
/// each texel of it the block of the background where the game draws it.
/// Texels drawn tinted or semi-transparent, and those no such camera
/// draws, keep the game's colours, enlarged; clear ones stay clear (what
/// passes behind a piece shows there). A file already in the mod is kept.
pub fn pieces_from_backgrounds(images: &[PathBuf], mod_dir: &Path) -> Result<PiecesReport, String> {
    let backgrounds = mod_backgrounds(mod_dir);
    let mut report = PiecesReport::default();
    if backgrounds.is_empty() {
        return Err(format!("{}: no backgrounds/ with pictures", mod_dir.display()));
    }
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let mut cuts: HashMap<u64, Cut> = HashMap::new();
    for image in images {
        let sources = super::files::dialogue_sources_of(image, threads)?;
        let catalog = Catalog::new(&sources);
        let (painted, _, _) = super::rooms::paint(&sources, &catalog);
        let pieces = super::rooms::pieces(&sources, &catalog);
        let mut cameras: Vec<(&String, &Vec<Texel>)> = pieces.iter().collect();
        cameras.sort_by(|a, b| a.0.cmp(b.0));
        for (camera, texels) in cameras {
            let Some(picture) = camera_background(&sources, camera, &backgrounds, &mut report) else { continue };
            let Some(scale) = whole_scale(&picture) else {
                report.refused.push(format!(
                    "{camera}: its background is {}x{}, not a whole multiple of 320x240",
                    picture.width, picture.height
                ));
                continue;
            };
            report.backgrounds += 1;
            for t in texels {
                let Some(sheet) = catalog.sheets.get(t.sheet as usize) else { continue };
                let cut = cuts.entry(sheet.hash).or_insert_with(|| {
                    let base = painted.get(&t.sheet).map(|p| p.rgba.as_slice());
                    new_cut(sheet, t, scale, base, &sources)
                });
                if cut.scale != scale {
                    continue;
                }
                cut_texel(cut, t, &picture);
            }
        }
    }
    let mut cuts: Vec<Cut> = cuts.into_values().collect();
    cuts.sort_by(|a, b| a.name.cmp(&b.name));
    for cut in cuts {
        let dir = mod_dir.join("textures").join(&cut.folder);
        let path = dir.join(format!("{}.png", cut.name));
        if path.exists() {
            report.kept.push(path);
            continue;
        }
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        save_rgba(&path, (cut.width * cut.scale) as u32, (cut.height * cut.scale) as u32, &cut.rgba)?;
        report.written.push(path);
    }
    Ok(report)
}

/// The mod's background for `camera` (`<room folder>/file<n>`), by its
/// place or its hash, read.
fn camera_background(
    sources: &super::dialogue::Sources,
    camera: &str,
    backgrounds: &HashMap<String, PathBuf>,
    report: &mut PiecesReport,
) -> Option<Picture> {
    let chunks = sources.background_chunks.get(camera)?;
    let pictures = sources.backgrounds.get(camera)?;
    for (&chunk, bs) in chunks.iter().zip(pictures) {
        let place = super::place_name(camera, chunk).to_ascii_lowercase();
        let hash = super::files::background_pixels(bs).map(|p| format!("{:016x}", super::screen_hash(320, 240, &p)));
        let found = backgrounds.get(&place).or_else(|| hash.and_then(|h| backgrounds.get(&h)));
        if let Some(path) = found {
            return match super::read_png(path, false) {
                Ok(p) => Some(p),
                Err(e) => {
                    report.refused.push(format!("{}: {e}", path.display()));
                    None
                }
            };
        }
    }
    None
}

/// The whole factor a background is enlarged by, if one.
fn whole_scale(p: &Picture) -> Option<usize> {
    let s = p.width as usize / 320;
    (s >= 1 && p.width as usize == 320 * s && p.height as usize == 240 * s).then_some(s)
}

/// A sheet to cut, at `scale`: the game's colours of it enlarged (texel by
/// texel) where known, clear elsewhere; filed as the extract files it.
fn new_cut(
    sheet: &super::sheets::Sheet,
    t: &Texel,
    scale: usize,
    base: Option<&[u8]>,
    sources: &super::dialogue::Sources,
) -> Cut {
    let (width, height) = (t.width, t.height);
    let mut rgba = vec![0u8; width * scale * height * scale * 4];
    if let Some(base) = base.filter(|b| b.len() == width * height * 4) {
        for y in 0..height * scale {
            for x in 0..width * scale {
                let from = ((y / scale) * width + x / scale) * 4;
                let to = (y * width * scale + x) * 4;
                rgba[to..to + 4].copy_from_slice(&base[from..from + 4]);
            }
        }
    }
    // The sheet's folder: its picture's chunk's (origin
    // `<file>/<picture>#<column>`), as the extract has it.
    let (key, rest) = sheet.origin.rsplit_once('/').unwrap_or((&sheet.origin, ""));
    let picture = rest.split('#').next().and_then(|n| n.parse::<usize>().ok());
    let folder = match picture.and_then(|n| sources.picture_chunks.get(key)?.get(n)) {
        Some(&chunk) => super::names::folder(key, chunk),
        None => PathBuf::from("other"),
    };
    Cut { width, height, scale, rgba, folder, name: sheet.file_name() }
}

/// Sets texel `t` of `cut`: the block of `background` where it is drawn,
/// for a texel drawn plainly; clear for colour 0.
fn cut_texel(cut: &mut Cut, t: &Texel, background: &Picture) {
    let s = cut.scale;
    let (tx, ty) = (t.texel % cut.width, t.texel / cut.width);
    let (sx, sy) = t.screen;
    let inside = (0..320).contains(&sx) && (0..240).contains(&sy);
    for j in 0..s {
        for i in 0..s {
            let to = ((ty * s + j) * cut.width * s + tx * s + i) * 4;
            if t.colour == 0 {
                cut.rgba[to..to + 4].copy_from_slice(&[0; 4]);
            } else if t.plain && inside {
                let from = ((sy as usize * s + j) * background.width as usize + sx as usize * s + i) * 4;
                cut.rgba[to..to + 3].copy_from_slice(&background.rgba[from..from + 3]);
                cut.rgba[to + 3] = 255;
            }
        }
    }
}

/// Saves RGBA pixels as a PNG.
fn save_rgba(path: &Path, w: u32, h: u32, rgba: &[u8]) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().and_then(|mut w| w.write_image_data(rgba)).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plain texel takes its background block; colour 0 stays clear; a
    /// tinted one keeps what it had.
    #[test]
    fn texels_are_cut_from_the_background() {
        let mut cut = Cut {
            width: 2,
            height: 1,
            scale: 2,
            rgba: vec![9; 2 * 2 * 2 * 4],
            folder: PathBuf::new(),
            name: String::new(),
        };
        // A 640x480 background: red at (2..4, 2..4), the 2x block of (1, 1).
        let mut bg = vec![0u8; 640 * 480 * 4];
        for y in 2..4 {
            for x in 2..4 {
                bg[(y * 640 + x) * 4..(y * 640 + x) * 4 + 4].copy_from_slice(&[200, 10, 10, 255]);
            }
        }
        let background = Picture { width: 640, height: 480, rgba: bg };
        let t = |texel, colour, plain| Texel {
            sheet: 0,
            depth: 0,
            width: 2,
            height: 1,
            texel,
            colour,
            screen: (1, 1),
            plain,
        };
        cut_texel(&mut cut, &t(0, 0x1234, true), &background);
        fn px(cut: &Cut, x: usize, y: usize) -> [u8; 4] {
            cut.rgba[(y * 4 + x) * 4..(y * 4 + x) * 4 + 4].try_into().unwrap()
        }
        assert!((0..2).all(|y| (0..2).all(|x| px(&cut, x, y) == [200, 10, 10, 255])));
        cut_texel(&mut cut, &t(1, 0x1234, false), &background);
        assert_eq!(px(&cut, 2, 0), [9, 9, 9, 9]);
        cut_texel(&mut cut, &t(1, 0, true), &background);
        assert_eq!(px(&cut, 3, 1), [0, 0, 0, 0]);
        assert_eq!(whole_scale(&Picture { width: 1280, height: 960, rgba: vec![] }), Some(4));
        assert_eq!(whole_scale(&Picture { width: 1000, height: 750, rgba: vec![] }), None);
    }
}
