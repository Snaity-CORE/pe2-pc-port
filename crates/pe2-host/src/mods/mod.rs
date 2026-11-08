//! Mods: folders of the player's files that change what the game shows
//! (docs/MODDING.md); what they replace is extracted from the discs
//! (files.rs, `pe2 --extract`).
//!
//! Mods live in `mods/<name>/` in the data folder (paths.rs), each with an
//! optional `mod.toml` (name, author, version, priority, enabled). Assets
//! are found by the hash of what they replace, so a mod never needs the
//! game's own files. What they replace is never touched in the console's
//! VRAM: the game sees its own data, and only the picture changes.
//!
//! Backgrounds and full-screen pictures: backgrounds.rs; music: music.rs;
//! movies: movies.rs; game files: files.rs; dialogue: dialogue.rs and
//! translate.rs; the menus' texts: strings.rs; textures: here (the
//! libraries) and runtime/gpu/textures.rs; cheats and memory patches:
//! patches.rs. Here: what is installed, the libraries of pictures they
//! supply (loaded as wanted, within a budget), and the start.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use serde::Deserialize;

pub mod backgrounds;
pub mod capfont;
pub mod credits;
pub mod dialogue;
pub mod files;
pub mod fontsheets;
pub mod glyphmaps;
pub mod indexed;
pub mod install;
pub mod labels;
pub mod menuglyphs;
pub mod movies;
pub mod music;
pub mod names;
pub mod patches;
pub mod rooms;
pub mod sheets;
pub mod strings;
pub mod translate;

pub use backgrounds::{background, background_wait, file_read, has_background, place_name, wants_screens};
pub use patches::apply_patches;

/// A mod's `mod.toml` (every field optional).
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Manifest {
    name: Option<String>,
    author: Option<String>,
    version: Option<String>,
    description: Option<String>,
    /// Higher wins where two mods replace the same thing (default 0).
    priority: i64,
    /// False keeps the mod installed but unused (default true).
    enabled: Option<bool>,
}

/// An installed mod.
#[derive(Debug)]
pub struct Mod {
    pub dir: PathBuf,
    pub name: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub priority: i64,
    pub enabled: bool,
}

/// The folder mods are installed in.
pub fn mods_dir() -> PathBuf {
    crate::paths::user_dir().join("mods")
}

/// The installed mods, highest priority first (then by folder name).
pub fn installed() -> Vec<Mod> {
    let Ok(entries) = std::fs::read_dir(mods_dir()) else { return Vec::new() };
    let mut mods: Vec<Mod> = entries
        .flatten()
        // Not the folders the mods folder keeps for itself (a .zip being
        // installed: install.rs).
        .filter(|e| e.path().is_dir() && !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| {
            let dir = e.path();
            let folder = e.file_name().to_string_lossy().into_owned();
            let manifest = match std::fs::read_to_string(dir.join("mod.toml")) {
                Ok(text) => toml::from_str::<Manifest>(&text).unwrap_or_else(|err| {
                    eprintln!("pe2: mods: {}: {err}", dir.join("mod.toml").display());
                    Manifest::default()
                }),
                Err(_) => Manifest::default(),
            };
            Mod {
                name: manifest.name.unwrap_or(folder),
                author: manifest.author,
                version: manifest.version,
                description: manifest.description,
                priority: manifest.priority,
                enabled: manifest.enabled.unwrap_or(true),
                dir,
            }
        })
        .collect();
    mods.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.dir.cmp(&b.dir)));
    mods
}

/// When the game starts, on threads of their own: the menus' and captions'
/// sheets redrawn from fonts first (fontsheets.rs, from two files of disc
/// 1); then, from every disc at once, what mods and the dialogue font
/// need: the texture sheets' catalog, the dialogue font for translations
/// (disc 1), and what character each dialogue glyph is.
pub fn start(discs: &[PathBuf]) {
    strings::load();
    backgrounds::resolve_background_places(discs);
    if let Some(first) = discs.first() {
        fontsheets::start(first.clone());
    }
    // What each dialogue glyph is: for a mod's font, and for the speakers'
    // name labels, which translated copies redraw (labels.rs).
    let texts = capfont::active() || translate::wanted();
    let (font, textures) = (translate::wanted() && !capfont::active(), wants_textures());
    if discs.is_empty() || !(font || textures || texts) {
        return;
    }
    let images = discs.to_vec();
    std::thread::spawn(move || {
        let since = std::time::Instant::now();
        // PE2_TRACE=startup: how long each step takes.
        let step = |what: &str, disc: usize| {
            if crate::runtime::trace("startup") {
                eprintln!("[startup] disc {}: {what} at {:.2} s", disc + 1, since.elapsed().as_secs_f32());
            }
        };
        // Disc 1 alone when only its dialogue font is wanted.
        let wanted = if textures || texts { images.len() } else { 1 };
        let per_disc = std::thread::scope(|scope| {
            let jobs: Vec<_> = images[..wanted]
                .iter()
                .enumerate()
                .map(|(number, image)| {
                    scope.spawn(move || {
                        // The cores shared among the discs.
                        let threads = std::thread::available_parallelism().map_or(4, |n| n.get()) / wanted;
                        let sources = match files::dialogue_sources_of(image, threads) {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("pe2: mods: {}: {e}", image.display());
                                return None;
                            }
                        };
                        step("files read", number);
                        let catalog = textures.then(|| sheets::Catalog::new(&files::catalog_sources(&sources)));
                        let glyphs = (font && number == 0).then(|| dialogue::font(&sources));
                        let known = texts.then(|| dialogue::cap_texts(&sources));
                        step("done", number);
                        Some((catalog, glyphs, known))
                    })
                })
                .collect();
            jobs.into_iter().map(|j| j.join().ok().flatten()).collect::<Vec<_>>()
        });
        // In disc order: disc 1's first.
        let mut catalog = sheets::Catalog::default();
        let mut known = std::collections::HashMap::new();
        for (part, glyphs, texts) in per_disc.into_iter().flatten() {
            if let Some(part) = part {
                catalog.extend(part);
            }
            if let Some(f) = glyphs {
                eprintln!("pe2: mods: dialogue font: {} glyphs in {:.1} s", f.len(), since.elapsed().as_secs_f32());
                translate::set_font(f);
            }
            for (hash, t) in texts.into_iter().flatten() {
                known.entry(hash).or_insert(t);
            }
        }
        if texts {
            eprintln!(
                "pe2: mods: characters of {} dialogue files in {:.1} s",
                known.len(),
                since.elapsed().as_secs_f32()
            );
            capfont::set_texts(known);
        }
        if textures {
            eprintln!(
                "pe2: mods: {} texture sheets from {} discs in {:.1} s",
                catalog.sheets.len(),
                images.len(),
                since.elapsed().as_secs_f32()
            );
            catalog.complete = true;
            sheets::set_catalog(catalog);
        }
    });
}

/// Mods are on with [general] experimental and [mods] enabled (PE2_MODS=1
/// or 0 says instead). Read once (asked each vertical blank and file read).
fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        let config = crate::config::get();
        std::env::var("PE2_MODS")
            .ok()
            .map(|v| v.trim() != "0")
            .unwrap_or(config.general.experimental.unwrap_or(false) && config.mods.enabled.unwrap_or(true))
    })
}

/// `len` bytes at RAM address `address` of a PS-X EXE (its header, then
/// its text from the load address in its header).
fn exe_at(exe: &[u8], address: u32, len: usize) -> Option<&[u8]> {
    if exe.get(..8) != Some(b"PS-X EXE") {
        return None;
    }
    let base = u32::from_le_bytes(exe.get(0x18..0x1C)?.try_into().ok()?);
    let at = address.checked_sub(base)? as usize + 0x800;
    exe.get(at..at.checked_add(len)?)
}

/// A 64-bit FNV-1a hash: stable across builds and versions, as file names
/// need.
fn fnv1a(bytes: impl IntoIterator<Item = u8>) -> u64 {
    bytes.into_iter().fold(0xCBF2_9CE4_8422_2325, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01B3))
}

/// The hash a screen image is known by: its size and its pixels as the game
/// sent them (15-bit, mask bit included).
pub fn screen_hash(w: i32, h: i32, pixels: &[u16]) -> u64 {
    let size = [w as u16, h as u16].into_iter().flat_map(u16::to_le_bytes);
    fnv1a(size.chain(pixels.iter().flat_map(|p| p.to_le_bytes())))
}

/// An image a mod supplies, as RGBA.
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// What a loaded file weighs in memory, for the libraries' budgets.
trait Weight {
    fn bytes(&self) -> usize;
}

impl Weight for Picture {
    fn bytes(&self) -> usize {
        self.rgba.len()
    }
}

impl Weight for indexed::Indexed {
    fn bytes(&self) -> usize {
        self.indices.len()
    }
}

enum Slot<T> {
    Loading,
    /// Loaded, and when last wanted (Library::clock).
    Ready(Arc<T>, u64),
    Failed,
}

/// The files mods supply of one kind (`<mod>/<dir>/<name><suffix>`), by
/// the key their name gives, loaded (by `load`) when first wanted. What is
/// loaded stays up to `budget` bytes: past it the files wanted longest ago
/// are let go (to be loaded again if wanted again). The 32-bit game has
/// 4 GB in all, and a mod's backgrounds at 8x weigh 20 MB each.
struct Library<K, T = Picture> {
    /// Key -> the file of the mod that wins for it.
    files: HashMap<K, PathBuf>,
    /// The same, once complete, when some of the files name their keys only
    /// by what must be found first (backgrounds named by place).
    resolved: OnceLock<HashMap<K, PathBuf>>,
    loaded: Mutex<HashMap<K, Slot<T>>>,
    load: fn(&Path) -> Result<T, String>,
    budget: usize,
    clock: std::sync::atomic::AtomicU64,
}

/// The files mods supply in `<mod>/<dir>/` and the folders inside it (the
/// extract files them by what they are: characters/aya, rooms/akropolis/
/// square...) whose names end in `suffix`, with the rest of their names:
/// lowest priority first, so the highest overwrites it.
fn mod_files(dir: &str, suffix: &str) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    if enabled() {
        for m in installed().iter().filter(|m| m.enabled).rev() {
            for path in files_under(&m.dir.join(dir)) {
                let stem = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .filter(|n| n.len() > suffix.len() && n.is_char_boundary(n.len() - suffix.len()))
                    .and_then(|n| {
                        let (stem, end) = n.split_at(n.len() - suffix.len());
                        end.eq_ignore_ascii_case(suffix).then(|| stem.to_string())
                    });
                if let Some(stem) = stem {
                    out.push((path, stem));
                }
            }
        }
    }
    out
}

impl<K: std::hash::Hash + Eq + Copy + Send + Sync + 'static, T: Weight + Send + Sync + 'static> Library<K, T> {
    fn scan(
        dir: &str,
        suffix: &str,
        key: impl Fn(&str) -> Option<K>,
        load: fn(&Path) -> Result<T, String>,
        budget: usize,
    ) -> Library<K, T> {
        let files = mod_files(dir, suffix).into_iter().filter_map(|(path, stem)| Some((key(&stem)?, path))).collect();
        Library {
            files,
            resolved: OnceLock::new(),
            loaded: Mutex::new(HashMap::new()),
            load,
            budget,
            clock: Default::default(),
        }
    }

    /// What is loaded for `key`, once loaded: the first call starts loading
    /// it on a thread of its own and answers None (the game's own shows
    /// until it is ready).
    fn get(&'static self, key: K) -> Option<Arc<T>> {
        let path = self.resolved.get().unwrap_or(&self.files).get(&key)?;
        let now = self.clock.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        let mut loaded = self.loaded.lock().unwrap();
        match loaded.get_mut(&key) {
            Some(Slot::Ready(p, when)) => {
                *when = now;
                return Some(p.clone());
            }
            Some(Slot::Loading | Slot::Failed) => return None,
            None => {}
        }
        loaded.insert(key, Slot::Loading);
        let path = path.clone();
        std::thread::spawn(move || {
            let slot = match (self.load)(&path) {
                Ok(p) => Slot::Ready(Arc::new(p), self.clock.load(std::sync::atomic::Ordering::Relaxed)),
                Err(err) => {
                    eprintln!("pe2: mods: {}: {err}", path.display());
                    Slot::Failed
                }
            };
            let mut loaded = self.loaded.lock().unwrap();
            loaded.insert(key, slot);
            self.trim(&mut loaded, key);
        });
        None
    }

    /// What is loaded for `key`, waiting up to `limit` for it to load (no
    /// longer than that: what is still loading then keeps loading).
    fn wait(&'static self, key: K, limit: std::time::Duration) -> Option<Arc<T>> {
        let since = std::time::Instant::now();
        loop {
            if let Some(p) = self.get(key) {
                return Some(p);
            }
            let failed = matches!(self.loaded.lock().unwrap().get(&key), Some(Slot::Failed) | None);
            if failed || since.elapsed() >= limit {
                return None;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    /// Lets go of the files wanted longest ago while past the budget, but
    /// never `keep` (just loaded).
    fn trim(&self, loaded: &mut HashMap<K, Slot<T>>, keep: K) {
        let weight = |s: &Slot<T>| if let Slot::Ready(p, _) = s { p.bytes() } else { 0 };
        let mut total: usize = loaded.values().map(weight).sum();
        while total > self.budget {
            let oldest = loaded
                .iter()
                .filter(|(k, _)| **k != keep)
                .filter_map(|(k, s)| if let Slot::Ready(_, when) = s { Some((*k, *when)) } else { None })
                .min_by_key(|(_, when)| *when);
            let Some((k, _)) = oldest else { break };
            total -= loaded.remove(&k).as_ref().map_or(0, weight);
        }
    }
}

/// The files in `dir` and the folders inside it, sorted (a name found twice
/// is the later's).
pub fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else { continue };
        for e in entries.flatten() {
            let path = e.path();
            if path.is_dir() { stack.push(path) } else { out.push(path) }
        }
    }
    out.sort();
    out
}

/// Bytes of loaded textures kept, of each kind.
const TEXTURES_BUDGET: usize = 256 << 20;

/// A texture's key: the hash of its sheet (sheets.rs), and of the palette
/// it is drawn with (0: any, or none for 15-bit textures).
pub type TextureKey = (u64, u64);

/// The file name a texture is known by: `<sheet>-<palette>`, or `<sheet>`
/// for any palette.
pub fn texture_name((texture, palette): TextureKey) -> String {
    if palette == 0 { format!("{texture:016x}") } else { format!("{texture:016x}-{palette:016x}") }
}

/// A replacement must keep the native sheet's aspect ratio at any scale.
pub fn texture_dimensions_match(w: u32, h: u32, native_w: i32, native_h: i32) -> bool {
    w > 0 && h > 0 && native_w > 0 && native_h > 0 && u64::from(w) * native_h as u64 == u64::from(h) * native_w as u64
}

#[cfg(test)]
mod texture_size_tests {
    use super::texture_dimensions_match;

    #[test]
    fn rejects_wrong_depth_export_and_accepts_scaled_sheet() {
        assert!(texture_dimensions_match(1024, 1024, 256, 256));
        assert!(texture_dimensions_match(512, 1024, 128, 256));
        assert!(!texture_dimensions_match(512, 1024, 256, 256));
        assert!(!texture_dimensions_match(0, 1024, 256, 256));
    }
}

fn textures() -> &'static Library<TextureKey> {
    static INDEX: OnceLock<Library<TextureKey>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let key = |s: &str| {
            let (t, p) = s.split_once('-').unwrap_or((s, "0"));
            Some((u64::from_str_radix(t, 16).ok()?, u64::from_str_radix(p, 16).ok()?))
        };
        Library::scan("textures", ".png", key, |p| read_png(p, true), TEXTURES_BUDGET)
    })
}

/// The texture a mod supplies for `key`, once loaded (see Library::get).
pub fn texture(key: TextureKey) -> Option<Arc<Picture>> {
    textures().get(key)
}

/// Indexed replacements (indexed.rs: `textures/<sheet>-<4|8>bit.idx.png`
/// and its header), by sheet and depth (0: 4 bits, 1: 8).
fn indexed_textures() -> &'static Library<(u64, u32), indexed::Indexed> {
    static INDEX: OnceLock<Library<(u64, u32), indexed::Indexed>> = OnceLock::new();
    INDEX.get_or_init(|| Library::scan("textures", ".idx.png", indexed::parse_stem, indexed::read, TEXTURES_BUDGET))
}

/// The indexed replacement for `sheet` drawn at `depth` (0: 4 bits, 1: 8):
/// a mod's, once loaded (see Library::get), else one redrawn from the
/// built-in fonts; and whether it is a mod's (a mod's loads later and must
/// then take the built-in one's place).
pub fn indexed_texture(sheet: u64, depth: u32) -> Option<(Arc<indexed::Indexed>, bool)> {
    match indexed_textures().get((sheet, depth)) {
        Some(m) => Some((m, true)),
        None => fontsheets::get(sheet, depth).map(|f| (f, false)),
    }
}

/// Whether anything here wants to know the textures drawn: mods with
/// textures, or the menus' and captions' sheets redrawn from fonts.
pub fn wants_textures() -> bool {
    (above_console() && (!textures().files.is_empty() || !indexed_textures().files.is_empty())) || fontsheets::enabled()
}

/// Whether the game is drawn above the console's resolution. Finer
/// textures and fonts are only drawn there: at it, they would be shrunk to
/// its texels, worse than the game's own (backgrounds are averaged down).
pub fn above_console() -> bool {
    crate::runtime::gpu::scale() > 1
}

/// A PNG as RGBA (any colour type or depth png can expand to 8 bits),
/// opaque unless `alpha`.
pub fn read_png(path: &Path, alpha: bool) -> Result<Picture, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = png::Decoder::new(std::io::BufReader::new(file));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let channels = info.color_type.samples();
    let rgba = buf[..info.buffer_size()]
        .chunks_exact(channels)
        .flat_map(|p| match channels {
            1 => [p[0], p[0], p[0], 255],
            2 => [p[0], p[0], p[0], if alpha { p[1] } else { 255 }],
            3 => [p[0], p[1], p[2], 255],
            _ => [p[0], p[1], p[2], if alpha { p[3] } else { 255 }],
        })
        .collect();
    Ok(Picture { width: info.width, height: info.height, rgba })
}

/// Saves 15-bit pixels as an RGB PNG (each channel's 5 bits shifted up, so
/// the colours are the console's exactly).
pub fn save_png(path: &Path, w: u32, h: u32, pixels: &[u16]) -> Result<(), String> {
    let rgb: Vec<u8> = pixels
        .iter()
        .flat_map(|&p| [((p & 31) << 3) as u8, (((p >> 5) & 31) << 3) as u8, (((p >> 10) & 31) << 3) as u8])
        .collect();
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w, h);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().map_err(|e| e.to_string())?;
    writer.write_image_data(&rgb).map_err(|e| e.to_string())
}

/// `picture` resized to `w` x `h` (area average when shrinking, bilinear
/// when growing), as RGBA.
pub fn resize(picture: &Picture, w: u32, h: u32) -> Vec<u8> {
    let (sw, sh) = (picture.width as usize, picture.height as usize);
    let (w, h) = (w as usize, h as usize);
    if (sw, sh) == (w, h) {
        return picture.rgba.clone();
    }
    let src = &picture.rgba;
    let mut out = vec![255u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let mut sum = [0f32; 3];
            if sw >= w && sh >= h {
                let (x0, x1) = (x * sw / w, ((x + 1) * sw / w).max(x * sw / w + 1));
                let (y0, y1) = (y * sh / h, ((y + 1) * sh / h).max(y * sh / h + 1));
                for sy in y0..y1 {
                    for sx in x0..x1 {
                        let p = &src[(sy * sw + sx) * 4..][..3];
                        for c in 0..3 {
                            sum[c] += f32::from(p[c]);
                        }
                    }
                }
                let n = ((x1 - x0) * (y1 - y0)) as f32;
                sum.iter_mut().for_each(|s| *s /= n);
            } else {
                let fx = ((x as f32 + 0.5) * sw as f32 / w as f32 - 0.5).clamp(0.0, (sw - 1) as f32);
                let fy = ((y as f32 + 0.5) * sh as f32 / h as f32 - 0.5).clamp(0.0, (sh - 1) as f32);
                let (x0, y0) = (fx as usize, fy as usize);
                let (x1, y1) = ((x0 + 1).min(sw - 1), (y0 + 1).min(sh - 1));
                let (tx, ty) = (fx - x0 as f32, fy - y0 as f32);
                let at = |sx: usize, sy: usize, c: usize| f32::from(src[(sy * sw + sx) * 4 + c]);
                for (c, s) in sum.iter_mut().enumerate() {
                    let top = at(x0, y0, c) * (1.0 - tx) + at(x1, y0, c) * tx;
                    let bottom = at(x0, y1, c) * (1.0 - tx) + at(x1, y1, c) * tx;
                    *s = top * (1.0 - ty) + bottom * ty;
                }
            }
            let o = &mut out[(y * w + x) * 4..][..3];
            for c in 0..3 {
                o[c] = sum[c].round().clamp(0.0, 255.0) as u8;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Past its budget a library lets go of what was wanted longest ago,
    /// never what was just loaded.
    #[test]
    fn libraries_keep_to_their_budget() {
        let picture = |n: usize| Picture { width: 1, height: 1, rgba: vec![0; n] };
        let library: Library<u64> = Library {
            files: HashMap::new(),
            resolved: OnceLock::new(),
            loaded: Mutex::new(HashMap::new()),
            load: |_| Err(String::new()),
            budget: 25,
            clock: Default::default(),
        };
        let mut loaded = library.loaded.lock().unwrap();
        loaded.insert(1, Slot::Ready(Arc::new(picture(10)), 3));
        loaded.insert(2, Slot::Ready(Arc::new(picture(10)), 1));
        loaded.insert(3, Slot::Loading);
        loaded.insert(4, Slot::Ready(Arc::new(picture(10)), 0));
        library.trim(&mut loaded, 4);
        let mut kept: Vec<u64> = loaded.keys().copied().collect();
        kept.sort();
        assert_eq!(kept, [1, 3, 4]);
    }

    /// Mods' files are found in the folders inside too (the extract's
    /// rooms/<area>/<room>...).
    #[test]
    fn files_are_found_in_folders() {
        let dir = std::env::temp_dir().join(format!("pe2-files-under-{}", std::process::id()));
        let nested = dir.join("rooms").join("dryfield").join("gas_station");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(dir.join("a.png"), b"").unwrap();
        std::fs::write(nested.join("b.png"), b"").unwrap();
        let found = files_under(&dir);
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(found, [dir.join("a.png"), nested.join("b.png")]);
    }

    /// The hash names files, so it must never change.
    #[test]
    fn screen_hash_is_stable() {
        assert_eq!(fnv1a([]), 0xCBF2_9CE4_8422_2325);
        assert_eq!(fnv1a(*b"a"), 0xAF63_DC4C_8601_EC8C);
        assert_ne!(screen_hash(2, 1, &[1, 2]), screen_hash(1, 2, &[1, 2]));
    }

    /// Resizing keeps flat colours and fills the target.
    #[test]
    fn resize_keeps_colours() {
        let picture = Picture { width: 2, height: 2, rgba: [10, 20, 30, 255].repeat(4) };
        for (w, h) in [(8, 8), (1, 1), (3, 5)] {
            let out = resize(&picture, w, h);
            assert_eq!(out.len(), (w * h * 4) as usize);
            assert!(out.as_chunks::<4>().0.iter().all(|p| *p == [10, 20, 30, 255]));
        }
    }
}
