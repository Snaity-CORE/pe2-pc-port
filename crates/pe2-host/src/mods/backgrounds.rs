//! Backgrounds and full-screen pictures (docs/MODDING.md): a mod's
//! `backgrounds/<hash>.png`, where the hash is `screen_hash` of the
//! 320x240 (or so) image the game sends to the GPU, or
//! `backgrounds/<place>.png` (`place_name`: the disc file and chunk it comes
//! from, the same on any version's discs), found on the discs when the game
//! starts and remembered in `background-places.tsv`. The extract saves each
//! one the discs hold as `dumps/backgrounds/<folder>/<place>.png`. Loaded
//! as their disc file is read (`file_read`), the GPU shows them in place of
//! the game's screen images (runtime/gpu/mod.rs).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use super::{Library, Mod, Picture, enabled, files, indexed_textures, installed, mod_files, read_png, textures};

/// Bytes of loaded backgrounds kept (a few rooms' worth at 8x).
const BACKGROUNDS_BUDGET: usize = 320 << 20;

fn backgrounds() -> &'static Library<u64> {
    static INDEX: OnceLock<Library<u64>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let library = Library::scan(
            "backgrounds",
            ".png",
            |s| match background_name(s)? {
                BackgroundName::Hash(hash) => Some(hash),
                BackgroundName::Place(..) => None,
            },
            |p| read_png(p, false),
            BACKGROUNDS_BUDGET,
        );
        if enabled() {
            let mods: Vec<Mod> = installed().into_iter().filter(|m| m.enabled).collect();
            for m in &mods {
                let version = m.version.as_deref().map(|v| format!(" {v}")).unwrap_or_default();
                let author = m.author.as_deref().map(|a| format!(" by {a}")).unwrap_or_default();
                let about = m.description.as_deref().map(|d| format!(": {d}")).unwrap_or_default();
                eprintln!("pe2: mod: {}{version}{author}{about}", m.name);
            }
            if !mods.is_empty() {
                eprintln!(
                    "pe2: mods: {} on, {} backgrounds ({} by place), {} textures, {} indexed textures",
                    mods.len(),
                    library.files.len() + background_places().len(),
                    background_places().len(),
                    textures().files.len(),
                    indexed_textures().files.len()
                );
            }
        }
        library
    })
}

/// What a background's file name says: the hash of the picture the game
/// shows, or where the picture is on the discs (`place_name`).
#[derive(Debug, PartialEq)]
enum BackgroundName {
    Hash(u64),
    Place(String, usize),
}

/// The name of the background at chunk `chunk` of the disc file `key`:
/// `stage1/1001/file2` chunk 3 is `stage1_1001_file2_3`.
pub fn place_name(key: &str, chunk: usize) -> String {
    format!("{}_{chunk}", key.replace('/', "_"))
}

fn background_name(stem: &str) -> Option<BackgroundName> {
    if stem.len() == 16
        && let Ok(hash) = u64::from_str_radix(stem, 16)
    {
        return Some(BackgroundName::Hash(hash));
    }
    let (key, chunk) = stem.rsplit_once('_')?;
    let key = key.replace('_', "/");
    if !(key.starts_with("stage") && key.contains("/file")) {
        return None;
    }
    Some(BackgroundName::Place(key, chunk.parse().ok()?))
}

/// The mods' backgrounds named by place, lowest priority first.
pub(crate) fn background_places() -> &'static [(PathBuf, String)] {
    static PLACES: OnceLock<Vec<(PathBuf, String)>> = OnceLock::new();
    PLACES.get_or_init(|| {
        mod_files("backgrounds", ".png")
            .into_iter()
            .filter(|(_, stem)| matches!(background_name(stem), Some(BackgroundName::Place(..))))
            .collect()
    })
}

/// Finds on the discs the pictures of the backgrounds mods name by place,
/// then lets them be used. What was found before is kept in
/// `background-places.tsv` in the data folder, so with the same discs only
/// places not looked up yet are read (on a thread: a few seconds for a
/// whole translation's, while the title shows, when only those named by
/// hash are used).
pub(super) fn resolve_background_places(discs: &[PathBuf]) {
    if background_places().is_empty() || discs.is_empty() {
        return;
    }
    let since = std::time::Instant::now();
    let wanted: std::collections::HashSet<files::Place> = background_places()
        .iter()
        .filter_map(|(_, stem)| match background_name(stem)? {
            BackgroundName::Place(key, chunk) => Some((key, chunk)),
            BackgroundName::Hash(_) => None,
        })
        .collect();
    let cache = crate::paths::user_file("background-places.tsv");
    let signature = discs_signature(discs);
    let mut known = read_places(&cache, &signature);
    known.retain(|place, _| wanted.contains(place));
    let missing: std::collections::HashSet<files::Place> =
        wanted.iter().filter(|p| !known.contains_key(*p)).cloned().collect();
    if missing.is_empty() {
        use_background_places(&wanted, &known, since);
        return;
    }
    let discs = discs.to_vec();
    std::thread::spawn(move || {
        // Each disc on a thread of its own.
        let found: Vec<_> = std::thread::scope(|scope| {
            let jobs: Vec<_> =
                discs.iter().map(|image| scope.spawn(|| files::background_hashes(image, &missing))).collect();
            jobs.into_iter()
                .zip(&discs)
                .filter_map(|(job, image)| match job.join() {
                    Ok(Ok(found)) => Some(found),
                    Ok(Err(e)) => {
                        eprintln!("pe2: mods: {}: {e}", image.display());
                        None
                    }
                    Err(_) => None,
                })
                .flatten()
                .collect()
        });
        // Looked up: those not found too (no background there).
        for place in &missing {
            known.entry(place.clone()).or_default();
        }
        // A place on both discs is there twice, most often the same picture.
        for (place, hash) in found {
            let hashes = known.entry(place).or_default();
            if !hashes.contains(&hash) {
                hashes.push(hash);
            }
        }
        use_background_places(&wanted, &known, since);
        if let Err(e) = write_places(&cache, &signature, &known) {
            eprintln!("pe2: mods: {}: {e}", cache.display());
        }
    });
}

/// Where each place's pictures are now known: the backgrounds named by place
/// are used from here on, with those named by hash.
fn use_background_places(
    wanted: &std::collections::HashSet<files::Place>,
    hashes: &HashMap<files::Place, Vec<u64>>,
    since: std::time::Instant,
) {
    let mut by_file: HashMap<String, Vec<u64>> = HashMap::new();
    for (place, found) in hashes {
        by_file.entry(place.0.clone()).or_default().extend(found);
    }
    let _ = BACKGROUNDS_BY_FILE.set(by_file);
    // All of them again in priority order, those named by hash too.
    let mut files = HashMap::new();
    for (path, stem) in mod_files("backgrounds", ".png") {
        match background_name(&stem) {
            Some(BackgroundName::Hash(hash)) => {
                files.insert(hash, path);
            }
            Some(BackgroundName::Place(key, chunk)) => {
                for &hash in hashes.get(&(key, chunk)).into_iter().flatten() {
                    files.insert(hash, path.clone());
                }
            }
            None => {}
        }
    }
    let missing = wanted.iter().filter(|p| hashes.get(*p).is_none_or(Vec::is_empty)).count();
    let not = if missing > 0 { format!(", {missing} not (no background there)") } else { String::new() };
    eprintln!(
        "pe2: mods: {} backgrounds by place found on the discs in {:.1} s{not}",
        wanted.len() - missing,
        since.elapsed().as_secs_f32()
    );
    let _ = backgrounds().resolved.set(files);
}

/// The disc images as the places' cache was made from: their paths, sizes
/// and times of change.
fn discs_signature(discs: &[PathBuf]) -> String {
    discs
        .iter()
        .map(|d| {
            let meta = std::fs::metadata(d).ok();
            let size = meta.as_ref().map_or(0, std::fs::Metadata::len);
            let time = meta
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |t| t.as_secs());
            format!("{} {size} {time}", d.display())
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

/// The places' cache: `# discs <signature>`, then a line for each place
/// looked up, `<file key> <chunk> <hashes>` (none when there was no
/// background there). Empty when made from other discs.
fn read_places(cache: &Path, signature: &str) -> HashMap<files::Place, Vec<u64>> {
    let Ok(text) = std::fs::read_to_string(cache) else { return HashMap::new() };
    let mut lines = text.lines();
    if lines.next() != Some(&format!("# discs {signature}")) {
        return HashMap::new();
    }
    lines
        .filter_map(|line| {
            let mut parts = line.split(' ');
            let key = parts.next()?.to_string();
            let chunk = parts.next()?.parse().ok()?;
            let hashes = parts.map(|h| u64::from_str_radix(h, 16)).collect::<Result<_, _>>().ok()?;
            Some(((key, chunk), hashes))
        })
        .collect()
}

fn write_places(cache: &Path, signature: &str, places: &HashMap<files::Place, Vec<u64>>) -> std::io::Result<()> {
    let mut lines: Vec<String> = places
        .iter()
        .map(|((key, chunk), hashes)| {
            std::iter::once(format!("{key} {chunk}"))
                .chain(hashes.iter().map(|h| format!("{h:016x}")))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();
    lines.sort();
    std::fs::write(cache, format!("# discs {signature}\n{}\n", lines.join("\n")))
}

/// Disc file -> the screen images of its backgrounds that mods name by
/// place (found when the game starts).
static BACKGROUNDS_BY_FILE: OnceLock<HashMap<String, Vec<u64>>> = OnceLock::new();

/// The drive starts reading the disc file `key`: the mods' backgrounds of
/// its pictures start loading, to be ready when the game shows them.
pub fn file_read(key: &str) {
    let Some(hashes) = BACKGROUNDS_BY_FILE.get().and_then(|m| m.get(key)) else { return };
    for &hash in hashes {
        backgrounds().get(hash);
    }
}

/// Whether a mod has a background for the screen image `hash` (loaded or
/// not).
pub fn has_background(hash: u64) -> bool {
    let library = backgrounds();
    library.resolved.get().unwrap_or(&library.files).contains_key(&hash)
}

/// The background a mod supplies for the screen image `hash`, once loaded
/// (see Library::get).
pub fn background(hash: u64) -> Option<Arc<Picture>> {
    backgrounds().get(hash)
}

/// The same, waiting up to `limit` for it to load.
pub fn background_wait(hash: u64, limit: std::time::Duration) -> Option<Arc<Picture>> {
    backgrounds().wait(hash, limit)
}

/// Whether anything here wants screen images: mods with backgrounds (named
/// by hash or by place).
pub fn wants_screens() -> bool {
    !backgrounds().files.is_empty() || !background_places().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Backgrounds are named by hash or by place.
    #[test]
    fn background_names() {
        assert_eq!(place_name("stage1/1001/file2", 3), "stage1_1001_file2_3");
        assert_eq!(background_name("stage1_1001_file2_3"), Some(BackgroundName::Place("stage1/1001/file2".into(), 3)));
        assert_eq!(background_name("stage0_file120_0"), Some(BackgroundName::Place("stage0/file120".into(), 0)));
        assert_eq!(background_name("00000000deadbeef"), Some(BackgroundName::Hash(0xDEAD_BEEF)));
        assert_eq!(background_name("my_picture_1"), None);
        assert_eq!(background_name("stage1_1001_file2"), None);
    }
}
