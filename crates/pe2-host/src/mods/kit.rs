//! Tools for making mods (`pe2 --<tool>`, docs/MODDING.md): what PE2PC
//! knows of the discs put to work on a mod's files.
//!
//! - `pieces_from_backgrounds`: the pieces of rooms drawn over their
//!   backgrounds (pillars, railings, furniture edges) cut from the mod's
//!   enlarged backgrounds, so they match them exactly.
//! - `prepare` and `finish`: the textures to enlarge, picked from the
//!   extract and made ready for an upscaler (`prepare`); and the enlarged
//!   ones made a mod's textures, their transparency the game's (`finish`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::Picture;
use super::indexed;
use super::rooms::Texel;
use super::sheets::Catalog;

/// What `prepare` and `finish` did: the files written, and those left out
/// with why.
#[derive(Default)]
pub struct KitReport {
    pub written: Vec<PathBuf>,
    pub refused: Vec<String>,
}

/// A sheet of the extract's index (`dumps/textures/index.txt`): its
/// folder, its name (with a palette's for a file per palette), the depth
/// it is drawn at, and where its colours come from.
struct IndexEntry {
    folder: String,
    name: String,
    depth: u32,
    colours: String,
}

fn read_index(dumps: &Path) -> Result<Vec<IndexEntry>, String> {
    let path = dumps.join("textures").join("index.txt");
    let text =
        std::fs::read_to_string(&path).map_err(|e| format!("{}: {e} (extract the textures first)", path.display()))?;
    Ok(text
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let mut parts = l.splitn(5, ' ');
            let (file, _origin, _place, depth) = (parts.next()?, parts.next()?, parts.next()?, parts.next()?);
            let (folder, name) = file.rsplit_once('/').unwrap_or(("", file));
            let depth = depth.strip_suffix("-bit")?.parse().ok()?;
            Some(IndexEntry {
                folder: folder.to_string(),
                name: name.to_string(),
                depth,
                colours: parts.next().unwrap_or("").to_string(),
            })
        })
        .collect())
}

/// Whether a 4- or 8-bit sheet is best enlarged as its ramp (each index a
/// grey) rather than in colours: the interface and fonts, drawn with
/// several palettes whose colours look alike in each, and sheets whose
/// colours are not known.
fn by_ramp(s: &IndexEntry) -> bool {
    s.folder == "ui" || s.folder.starts_with("ui/") || s.colours.starts_with("none")
}

/// Puts in `work` (in the extract's folders) each texture to enlarge, from
/// the extract in `dumps`: the 4- and 8-bit sheets as their ramps (the
/// interface, fonts, sheets of unknown colours) or in colours, the 15-bit
/// ones in colours. Only those under `only` (a folder of the extract,
/// `rooms/mist`), if given. Pictures are opaque, the colours around their
/// clear parts spread into them: an upscaler then neither drops their
/// transparency nor rims them with black or white (`finish` gives the
/// game's transparency back).
pub fn prepare(dumps: &Path, work: &Path, only: Option<&str>) -> Result<KitReport, String> {
    let mut report = KitReport::default();
    let mut done = std::collections::HashSet::new();
    for s in read_index(dumps)? {
        if only.is_some_and(|o| !(s.folder == o || s.folder.starts_with(&format!("{o}/")))) {
            continue;
        }
        let ramp = s.depth < 15 && by_ramp(&s);
        let base = s.name.split('-').next().unwrap_or(&s.name);
        let (from, name) = if ramp {
            let name = indexed::stem_named(base, u32::from(s.depth == 8));
            (dumps.join("textures-ramp").join(&s.folder).join(format!("{name}.png")), name)
        } else {
            (dumps.join("textures").join(&s.folder).join(format!("{}.png", s.name)), s.name.clone())
        };
        if !done.insert((s.folder.clone(), name.clone())) {
            continue;
        }
        let picture = match super::read_png(&from, true) {
            Ok(p) => p,
            Err(e) => {
                report.refused.push(format!("{}: {e}", from.display()));
                continue;
            }
        };
        let dir = work.join(&s.folder);
        std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = dir.join(format!("{name}.png"));
        save_rgba(&path, picture.width, picture.height, &bleed(&picture))?;
        report.written.push(path);
    }
    Ok(report)
}

/// `p`'s pixels made opaque, each clear one (alpha under half) given the
/// colour of the opaque ones around it, spread outwards a texel at a time.
fn bleed(p: &Picture) -> Vec<u8> {
    let (w, h) = (p.width as usize, p.height as usize);
    let mut rgba = p.rgba.clone();
    let mut known: Vec<bool> = (0..w * h).map(|i| p.rgba[i * 4 + 3] >= 128).collect();
    for _ in 0..w.max(h) {
        let mut next = known.clone();
        let mut grew = false;
        for y in 0..h {
            for x in 0..w {
                if known[y * w + x] {
                    continue;
                }
                let mut sum = [0u32; 3];
                let mut n = 0;
                for (dx, dy) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)] {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if (0..w as i32).contains(&nx)
                        && (0..h as i32).contains(&ny)
                        && known[ny as usize * w + nx as usize]
                    {
                        let at = (ny as usize * w + nx as usize) * 4;
                        for c in 0..3 {
                            sum[c] += u32::from(rgba[at + c]);
                        }
                        n += 1;
                    }
                }
                if let Some(mean) = sum.iter().map(|s| s.checked_div(n)).collect::<Option<Vec<u32>>>() {
                    for (c, m) in mean.into_iter().enumerate() {
                        rgba[(y * w + x) * 4 + c] = m as u8;
                    }
                    next[y * w + x] = true;
                    grew = true;
                }
            }
        }
        known = next;
        if !grew {
            break;
        }
    }
    for px in rgba.as_chunks_mut::<4>().0 {
        px[3] = 255;
    }
    rgba
}

/// `upscaled` with the transparency of `original` (its alpha, enlarged
/// smoothly to the upscaled size): what `prepare` made opaque, clear again
/// where the game's texels are.
fn with_alpha_of(upscaled: &Picture, original: &Picture) -> Vec<u8> {
    let (w, h) = (upscaled.width as usize, upscaled.height as usize);
    let (ow, oh) = (original.width as usize, original.height as usize);
    let alpha = |x: usize, y: usize| f32::from(original.rgba[(y.min(oh - 1) * ow + x.min(ow - 1)) * 4 + 3]);
    let mut out = upscaled.rgba.clone();
    for y in 0..h {
        for x in 0..w {
            // Bilinear, between the original texels' centres.
            let fx = ((x as f32 + 0.5) * ow as f32 / w as f32 - 0.5).max(0.0);
            let fy = ((y as f32 + 0.5) * oh as f32 / h as f32 - 0.5).max(0.0);
            let (x0, y0) = (fx as usize, fy as usize);
            let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
            let top = alpha(x0, y0) * (1.0 - tx) + alpha(x0 + 1, y0) * tx;
            let bottom = alpha(x0, y0 + 1) * (1.0 - tx) + alpha(x0 + 1, y0 + 1) * tx;
            out[(y * w + x) * 4 + 3] = (top * (1.0 - ty) + bottom * ty).round() as u8;
        }
    }
    out
}

/// Makes the enlarged textures in `work` (as `prepare` laid them out, each
/// enlarged by a whole factor) the textures of the mod at `mod_dir`, from
/// the extract in `dumps`: ramps and 4- and 8-bit sheets in colours become
/// indexed replacements (indexed.rs `from_upscale`), with the game's
/// transparency; 15-bit sheets stay in colours, their transparency the
/// game's again. What was made before is made again.
pub fn finish(dumps: &Path, work: &Path, mod_dir: &Path) -> Result<KitReport, String> {
    let mut report = KitReport::default();
    for path in super::files_under(work) {
        if !path.extension().is_some_and(|x| x.eq_ignore_ascii_case("png")) {
            continue;
        }
        let folder = path.parent().and_then(|p| p.strip_prefix(work).ok()).unwrap_or(Path::new("")).to_path_buf();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else { continue };
        match finish_one(dumps, &path, &folder, stem, &mod_dir.join("textures").join(&folder)) {
            Ok(written) => report.written.push(written),
            Err(e) => report.refused.push(format!("{}: {e}", path.display())),
        }
    }
    Ok(report)
}

fn finish_one(dumps: &Path, path: &Path, folder: &Path, stem: &str, out: &Path) -> Result<PathBuf, String> {
    let upscaled = super::read_png(path, true)?;
    let read_native = |base: &str, depth: u32| {
        indexed::read(&dumps.join("textures-indexed").join(format!("{}.idx.png", indexed::stem_named(base, depth))))
    };
    // A ramp: its sheet's indices' greys.
    if let Some((_, depth)) = indexed::parse_stem(stem) {
        let base = stem.rsplit_once('-').map_or(stem, |n| n.0);
        let native = read_native(base, depth)?;
        let ramp = indexed::ramp(&native);
        let made = indexed::from_upscale(
            &native,
            &indexed::Rgba { width: native.size.0, height: native.size.1, pixels: &ramp },
            &indexed::Rgba { width: upscaled.width, height: upscaled.height, pixels: &upscaled.rgba },
        )?;
        return indexed::write_named(out, base, &made, Some(&format!("from the enlarged {stem}.png")));
    }
    // In colours: the extract's preview tells its depth (its width) and its
    // transparency.
    let original_path = dumps.join("textures").join(folder).join(format!("{stem}.png"));
    let original =
        super::read_png(&original_path, true).map_err(|e| format!("its preview {}: {e}", original_path.display()))?;
    let scale = upscaled.width / original.width.max(1);
    if scale == 0 || upscaled.width != original.width * scale || upscaled.height != original.height * scale {
        return Err(format!(
            "{}x{} is not a whole multiple of the sheet's {}x{}",
            upscaled.width, upscaled.height, original.width, original.height
        ));
    }
    let rgba = with_alpha_of(&upscaled, &original);
    let base = stem.split('-').next().unwrap_or(stem);
    match original.width {
        256 | 128 => {
            let depth = u32::from(original.width == 128);
            let native = read_native(base, depth)?;
            let made = indexed::from_upscale(
                &native,
                &indexed::Rgba { width: original.width, height: original.height, pixels: &original.rgba },
                &indexed::Rgba { width: upscaled.width, height: upscaled.height, pixels: &rgba },
            )?;
            indexed::write_named(out, base, &made, Some(&format!("from the enlarged {stem}.png")))
        }
        _ => {
            std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
            let to = out.join(format!("{stem}.png"));
            save_rgba(&to, upscaled.width, upscaled.height, &rgba)?;
            Ok(to)
        }
    }
}

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

/// The widest a side-by-side picture of `check` is.
const CHECK_WIDTH: usize = 2048;

/// Looks over the textures and backgrounds of the mod at `mod_dir` against
/// the extract in `dumps`: for each, a picture of the game's next to the
/// mod's at the same size (`out`, in the mod's folders; indexed ones in
/// the colours of the game's preview), and `out/report.txt` with what PE2PC
/// would refuse or draw wrong (a size not a whole multiple, another shape,
/// an indexed file it cannot read, a name the extract has no picture for).
/// Answers the pictures written, and the problems in `refused`.
pub fn check(dumps: &Path, mod_dir: &Path, out: &Path) -> Result<KitReport, String> {
    let by_stem = |dir: &Path| -> HashMap<String, PathBuf> {
        super::files_under(dir)
            .into_iter()
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("png")))
            .filter_map(|p| Some((p.file_stem()?.to_str()?.to_string(), p)))
            .collect()
    };
    let previews = by_stem(&dumps.join("textures"));
    let backgrounds = by_stem(&dumps.join("backgrounds"));
    if previews.is_empty() && backgrounds.is_empty() {
        return Err(format!("{}: no extract (pe2 --extract textures,backgrounds first)", dumps.display()));
    }
    let mut report = KitReport::default();
    let mut lines = Vec::new();
    for kind in ["textures", "backgrounds"] {
        let root = mod_dir.join(kind);
        for path in super::files_under(&root) {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if !name.to_ascii_lowercase().ends_with(".png") {
                continue;
            }
            let rel = path.strip_prefix(mod_dir).unwrap_or(&path).to_path_buf();
            let shown = rel.to_string_lossy().replace('\\', "/");
            let result = if kind == "backgrounds" {
                check_background(&path, &backgrounds)
            } else {
                check_texture(dumps, &path, &previews)
            };
            match result {
                Ok((original, replacement)) => {
                    let side = side_by_side(&original, &replacement);
                    let to = out.join(&rel);
                    if let Some(dir) = to.parent() {
                        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
                    }
                    save_rgba(&to, side.width, side.height, &side.rgba)?;
                    lines.push(format!(
                        "ok  {shown}: {}x{}, the game's {}x{}",
                        replacement.width, replacement.height, original.width, original.height
                    ));
                    report.written.push(to);
                }
                Err(why) => {
                    lines.push(format!("!!  {shown}: {why}"));
                    report.refused.push(format!("{shown}: {why}"));
                }
            }
        }
    }
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let text = format!(
        "# {}: the mod's textures and backgrounds against the extract (!!: a problem).\n{}\n",
        mod_dir.display(),
        lines.join("\n")
    );
    std::fs::write(out.join("report.txt"), text).map_err(|e| e.to_string())?;
    Ok(report)
}

/// A background against the extract's of the same name (its place): any
/// size of the game's shape (4:3).
fn check_background(path: &Path, backgrounds: &HashMap<String, PathBuf>) -> Result<(Picture, Picture), String> {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let original = backgrounds.get(stem).ok_or("the extract has no background of that name (named by hash?)")?;
    let (original, replacement) = (super::read_png(original, false)?, super::read_png(path, false)?);
    if u64::from(replacement.width) * 3 != u64::from(replacement.height) * 4 {
        return Err(format!("{}x{} is not 4:3, as the game's 320x240", replacement.width, replacement.height));
    }
    Ok((original, replacement))
}

/// A texture (in colours, or indexed) against the extract's preview of
/// its sheet: a whole multiple of it; an indexed one shown in the
/// preview's colours, each index the mean of the texels it colours.
fn check_texture(dumps: &Path, path: &Path, previews: &HashMap<String, PathBuf>) -> Result<(Picture, Picture), String> {
    let file = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let indexed_stem = file.strip_suffix(".idx.png").or_else(|| file.strip_suffix(".IDX.PNG"));
    if let Some(stem) = indexed_stem {
        let made = indexed::read(path)?;
        let base = stem.rsplit_once('-').map_or(stem, |n| n.0);
        let native_path = dumps.join("textures-indexed").join(format!("{stem}.idx.png"));
        let native =
            indexed::read(&native_path).map_err(|_| "the extract has no sheet of that name (named by hash?)")?;
        // The preview: the sheet's, or the first of its palettes'.
        let preview = previews
            .get(base)
            .or_else(|| previews.iter().filter(|(k, _)| k.starts_with(&format!("{base}-"))).map(|(_, p)| p).min());
        let colours = match preview.map(|p| super::read_png(p, true)) {
            Some(Ok(p)) if p.width == native.size.0 && p.height == native.size.1 => index_colours(&native, &p.rgba),
            _ => (0..256)
                .map(|i| {
                    let g = indexed::ramp_level(i as u8, native.depth);
                    [g, g, g, 255]
                })
                .collect(),
        };
        let paint = |t: &indexed::Indexed| Picture {
            width: t.size.0,
            height: t.size.1,
            rgba: t.indices.iter().flat_map(|&i| colours[usize::from(i)]).collect(),
        };
        return Ok((paint(&native), paint(&made)));
    }
    let stem = file.strip_suffix(".png").or_else(|| file.strip_suffix(".PNG")).unwrap_or(file);
    let original = previews.get(stem).ok_or("the extract has no texture of that name (named by hash?)")?;
    let (original, replacement) = (super::read_png(original, true)?, super::read_png(path, true)?);
    let scale = replacement.width / original.width.max(1);
    if scale == 0 || replacement.width != original.width * scale || replacement.height != original.height * scale {
        return Err(format!(
            "{}x{} is not a whole multiple of the sheet's {}x{} (a preview of the other depth?)",
            replacement.width, replacement.height, original.width, original.height
        ));
    }
    Ok((original, replacement))
}

/// Each index's colour in `preview` (RGBA of `native`'s size): the mean of
/// the texels it colours; grey where it colours none.
fn index_colours(native: &indexed::Indexed, preview: &[u8]) -> Vec<[u8; 4]> {
    let mut sums = vec![[0u64; 5]; 256];
    for (i, &index) in native.indices.iter().enumerate() {
        let s = &mut sums[usize::from(index)];
        for c in 0..4 {
            s[c] += u64::from(preview[i * 4 + c]);
        }
        s[4] += 1;
    }
    sums.iter()
        .map(|s| if s[4] == 0 { [128, 128, 128, 255] } else { [0, 1, 2, 3].map(|c| (s[c] / s[4]) as u8) })
        .collect()
}

/// `a` and `b` side by side at `b`'s size (at most CHECK_WIDTH wide in
/// all), over a chequer that shows their clear parts; `a` enlarged texel
/// by texel.
fn side_by_side(a: &Picture, b: &Picture) -> Picture {
    let gap = 8;
    let fit = (CHECK_WIDTH - gap) as f32 / 2.0 / b.width.max(1) as f32;
    let shrink = fit.min(1.0);
    let (w, h) = (((b.width as f32 * shrink) as usize).max(1), ((b.height as f32 * shrink) as usize).max(1));
    let (width, height) = (w * 2 + gap, h);
    let mut rgba = vec![0u8; width * height * 4];
    for (n, p) in [a, b].into_iter().enumerate() {
        let x0 = n * (w + gap);
        for y in 0..h {
            for x in 0..w {
                let sx = (x * p.width as usize / w).min(p.width as usize - 1);
                let sy = (y * p.height as usize / h).min(p.height as usize - 1);
                let px = &p.rgba[(sy * p.width as usize + sx) * 4..(sy * p.width as usize + sx) * 4 + 4];
                let chequer = if (x / 16 + y / 16) % 2 == 0 { 96u32 } else { 160 };
                let alpha = u32::from(px[3]);
                let to = (y * width + x0 + x) * 4;
                for c in 0..3 {
                    rgba[to + c] = ((u32::from(px[c]) * alpha + chequer * (255 - alpha)) / 255) as u8;
                }
                rgba[to + 3] = 255;
            }
        }
    }
    Picture { width: width as u32, height: height as u32, rgba }
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

    /// Side by side at the mod's size, the game's enlarged texel by texel,
    /// clear parts over the chequer; an index's colour is the mean of the
    /// preview's texels it colours.
    #[test]
    fn checks_show_both_at_one_size() {
        let a = Picture { width: 1, height: 1, rgba: vec![255, 0, 0, 255] };
        let b = Picture { width: 4, height: 2, rgba: [[0u8, 0, 255, 0]; 8].concat() };
        let side = side_by_side(&a, &b);
        assert_eq!((side.width, side.height), (4 * 2 + 8, 2));
        assert_eq!(&side.rgba[..4], &[255, 0, 0, 255]);
        // b is clear: the chequer shows.
        let at = (12) * 4;
        assert_eq!(&side.rgba[at..at + 4], &[96, 96, 96, 255]);
        let native = indexed::Indexed { sheet: 1, depth: 0, native: (2, 1), size: (2, 1), indices: vec![3, 3] };
        let colours = index_colours(&native, &[10, 20, 30, 255, 30, 40, 50, 255]);
        assert_eq!(colours[3], [20, 30, 40, 255]);
        assert_eq!(colours[0], [128, 128, 128, 255]);
    }

    /// Clear pixels take the colour of the opaque ones around them, and the
    /// picture comes out opaque; the opaque ones keep theirs.
    #[test]
    fn clear_pixels_take_their_neighbours_colour() {
        // 3x1: red, clear, clear.
        let p = Picture { width: 3, height: 1, rgba: vec![200, 0, 0, 255, 9, 9, 9, 0, 9, 9, 9, 0] };
        let out = bleed(&p);
        assert_eq!(out, vec![200, 0, 0, 255, 200, 0, 0, 255, 200, 0, 0, 255]);
    }

    /// The original's transparency, enlarged smoothly: clear where its
    /// texels are clear, opaque where they are not, between at the edge.
    #[test]
    fn transparency_comes_back_from_the_original() {
        let original = Picture { width: 2, height: 1, rgba: vec![1, 1, 1, 255, 1, 1, 1, 0] };
        let upscaled = Picture { width: 8, height: 4, rgba: vec![50; 8 * 4 * 4] };
        let out = with_alpha_of(&upscaled, &original);
        let alpha = |x: usize, y: usize| out[(y * 8 + x) * 4 + 3];
        assert_eq!(alpha(0, 0), 255);
        assert_eq!(alpha(7, 3), 0);
        assert!(alpha(3, 1) > 128 && alpha(4, 1) < 128);
        // The colours are the upscaled picture's.
        assert_eq!(out[0], 50);
    }

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
