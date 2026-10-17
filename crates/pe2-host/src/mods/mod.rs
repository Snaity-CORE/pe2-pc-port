//! Mods: folders of the player's files that change what the game shows, and
//! dumping what the game shows so mods can be made (docs/MODDING.md).
//!
//! Mods live in `mods/<name>/` in the data folder (paths.rs), each with an
//! optional `mod.toml` (name, author, version, priority, enabled). Assets
//! are found by the hash of what they replace, so a mod never needs the
//! game's own files. What they replace is never touched in the console's
//! VRAM: the game sees its own data, and only the picture changes.
//!
//! Music: see music.rs. Backgrounds and full-screen pictures, as
//! `backgrounds/<hash>.png`, where the hash is `screen_hash` of the
//! 320x240 (or so) image the game sends to the GPU. With `[mods] dump`,
//! each such image is saved once as `dumps/backgrounds/<hash>.png`.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use serde::Deserialize;

pub mod music;

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
        .filter(|e| e.path().is_dir())
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

/// Whether mods are used: PE2_MODS=0/1, else pe2.toml's [mods] enabled,
/// else on.
fn enabled() -> bool {
    std::env::var("PE2_MODS").ok().map(|v| v.trim() != "0").or(crate::config::get().mods.enabled).unwrap_or(true)
}

/// Whether what the game shows is dumped: PE2_DUMP=0/1, else pe2.toml's
/// [mods] dump, else off.
fn dumping() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("PE2_DUMP").ok().map(|v| v.trim() != "0").or(crate::config::get().mods.dump).unwrap_or(false)
    })
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

enum Slot {
    Loading,
    Ready(Arc<Picture>),
    Failed,
}

struct Backgrounds {
    /// Hash -> the file of the mod that wins for it.
    files: HashMap<u64, PathBuf>,
    loaded: Mutex<HashMap<u64, Slot>>,
}

fn backgrounds() -> &'static Backgrounds {
    static INDEX: OnceLock<Backgrounds> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut files = HashMap::new();
        if enabled() {
            let mods: Vec<Mod> = installed().into_iter().filter(|m| m.enabled).collect();
            // Lowest priority first, so the highest overwrites it.
            for m in mods.iter().rev() {
                let Ok(entries) = std::fs::read_dir(m.dir.join("backgrounds")) else { continue };
                for e in entries.flatten() {
                    let path = e.path();
                    let hash = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .filter(|_| path.extension().is_some_and(|x| x.eq_ignore_ascii_case("png")))
                        .and_then(|s| u64::from_str_radix(s, 16).ok());
                    if let Some(hash) = hash {
                        files.insert(hash, path);
                    }
                }
            }
            for m in &mods {
                let version = m.version.as_deref().map(|v| format!(" {v}")).unwrap_or_default();
                let author = m.author.as_deref().map(|a| format!(" by {a}")).unwrap_or_default();
                let about = m.description.as_deref().map(|d| format!(": {d}")).unwrap_or_default();
                eprintln!("pe2: mod: {}{version}{author}{about}", m.name);
            }
            if !mods.is_empty() {
                eprintln!("pe2: mods: {} on, {} backgrounds", mods.len(), files.len());
            }
        }
        Backgrounds { files, loaded: Mutex::new(HashMap::new()) }
    })
}

/// The background a mod supplies for the screen image `hash`, once loaded:
/// the first call starts loading it on a thread of its own and answers
/// None (the game's own image shows until it is ready).
pub fn background(hash: u64) -> Option<Arc<Picture>> {
    let index = backgrounds();
    let path = index.files.get(&hash)?;
    let mut loaded = index.loaded.lock().unwrap();
    match loaded.get(&hash) {
        Some(Slot::Ready(p)) => return Some(p.clone()),
        Some(Slot::Loading | Slot::Failed) => return None,
        None => {}
    }
    loaded.insert(hash, Slot::Loading);
    let path = path.clone();
    std::thread::spawn(move || {
        let slot = match read_png(&path) {
            Ok(p) => Slot::Ready(Arc::new(p)),
            Err(err) => {
                eprintln!("pe2: mods: {}: {err}", path.display());
                Slot::Failed
            }
        };
        backgrounds().loaded.lock().unwrap().insert(hash, slot);
    });
    None
}

/// A PNG as RGBA (any colour type or depth png can expand to 8 bits).
fn read_png(path: &Path) -> Result<Picture, String> {
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
            1 | 2 => [p[0], p[0], p[0], 255],
            _ => [p[0], p[1], p[2], 255],
        })
        .collect();
    Ok(Picture { width: info.width, height: info.height, rgba })
}

/// With dumping on, saves screen image `hash` (w x h 15-bit pixels) as
/// `dumps/backgrounds/<hash>.png` in the data folder, once, on a thread of
/// its own.
pub fn dump_screen(hash: u64, w: i32, h: i32, pixels: &[u16]) {
    static SEEN: Mutex<Option<HashSet<u64>>> = Mutex::new(None);
    if !dumping() || !SEEN.lock().unwrap().get_or_insert_with(HashSet::new).insert(hash) {
        return;
    }
    let dir = crate::paths::user_file("dumps").join("backgrounds");
    let path = dir.join(format!("{hash:016x}.png"));
    if path.exists() {
        return;
    }
    let rgb: Vec<u8> = pixels
        .iter()
        .flat_map(|&p| [((p & 31) << 3) as u8, (((p >> 5) & 31) << 3) as u8, (((p >> 10) & 31) << 3) as u8])
        .collect();
    std::thread::spawn(move || {
        let _ = std::fs::create_dir_all(&dir);
        let written = std::fs::File::create(&path).map_err(|e| e.to_string()).and_then(|file| {
            let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
            enc.set_color(png::ColorType::Rgb);
            enc.set_depth(png::BitDepth::Eight);
            let mut writer = enc.write_header().map_err(|e| e.to_string())?;
            writer.write_image_data(&rgb).map_err(|e| e.to_string())
        });
        match written {
            Ok(()) => eprintln!("pe2: dumped {}", path.display()),
            Err(err) => eprintln!("pe2: dump {}: {err}", path.display()),
        }
    });
}

/// Whether anything here wants screen images: mods with backgrounds, or
/// dumping.
pub fn wants_screens() -> bool {
    dumping() || !backgrounds().files.is_empty()
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

// ---- memory patches ---------------------------------------------------------

/// A mod's `patches.toml`: GameShark codes, and plain writes.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct PatchFile {
    cheat: Vec<CheatEntry>,
    patch: Vec<PatchEntry>,
}

/// `[[cheat]]`: GameShark (Action Replay) codes, "AAAAAAAA VVVV" each.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct CheatEntry {
    name: Option<String>,
    enabled: Option<bool>,
    codes: Vec<String>,
}

/// `[[patch]]`: a value written to an address each frame.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct PatchEntry {
    name: Option<String>,
    enabled: Option<bool>,
    address: u32,
    /// 1, 2 or 4 bytes (default 4).
    size: Option<u32>,
    value: u32,
}

/// A comparison of a GameShark condition code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Test {
    Equal,
    Different,
    Less,
    Greater,
}

/// One code, as it runs each frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Code {
    /// Write `size` bytes.
    Write { address: u32, size: u32, value: u32 },
    /// Add (or, negative, subtract) to `size` bytes.
    Add { address: u32, size: u32, delta: i32 },
    /// Run the next code only if `size` bytes at `address` pass `test`.
    If { address: u32, size: u32, test: Test, value: u32 },
    /// The next code (a write) `count` times, its address advanced by
    /// `step` bytes and its value by `increment` each time.
    Repeat { count: u32, step: u32, increment: u32 },
}

/// Parses one GameShark line ("80012345 0063"), or says why it cannot.
fn parse_code(line: &str) -> Result<Code, String> {
    let shape = || format!("{line:?}: not \"AAAAAAAA VVVV\"");
    let mut parts = line.split_whitespace();
    let (Some(a), Some(v), None) = (parts.next(), parts.next(), parts.next()) else { return Err(shape()) };
    if a.len() != 8 || v.len() != 4 {
        return Err(shape());
    }
    let word = u32::from_str_radix(a, 16).map_err(|_| shape())?;
    let value = u32::from_str_radix(v, 16).map_err(|_| shape())?;
    let address = 0x8000_0000 | (word & 0x00FF_FFFF);
    let condition = |size, test| Code::If { address, size, test, value: if size == 1 { value & 0xFF } else { value } };
    Ok(match word >> 24 {
        0x80 => Code::Write { address, size: 2, value },
        0x30 => Code::Write { address, size: 1, value: value & 0xFF },
        0x10 => Code::Add { address, size: 2, delta: value as i32 },
        0x11 => Code::Add { address, size: 2, delta: -(value as i32) },
        0x20 => Code::Add { address, size: 1, delta: (value & 0xFF) as i32 },
        0x21 => Code::Add { address, size: 1, delta: -((value & 0xFF) as i32) },
        0xD0 => condition(2, Test::Equal),
        0xD1 => condition(2, Test::Different),
        0xD2 => condition(2, Test::Less),
        0xD3 => condition(2, Test::Greater),
        0xE0 => condition(1, Test::Equal),
        0xE1 => condition(1, Test::Different),
        0xE2 => condition(1, Test::Less),
        0xE3 => condition(1, Test::Greater),
        0x50 => Code::Repeat { count: (word >> 8) & 0xFF, step: word & 0xFF, increment: value },
        kind => return Err(format!("{line:?}: code type {kind:02X} is not supported")),
    })
}

/// A cheat or patch ready to run: its codes in order.
struct Patch {
    codes: Vec<Code>,
}

/// The patches of the mods in use, lowest priority first (so the highest
/// writes last and wins).
fn patches() -> &'static [Patch] {
    static PATCHES: OnceLock<Vec<Patch>> = OnceLock::new();
    PATCHES.get_or_init(|| {
        let mut out = Vec::new();
        if !enabled() {
            return out;
        }
        for m in installed().into_iter().filter(|m| m.enabled).rev() {
            let path = m.dir.join("patches.toml");
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let file: PatchFile = match toml::from_str(&text) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("pe2: mods: {}: {err}", path.display());
                    continue;
                }
            };
            for c in file.cheat.iter().filter(|c| c.enabled.unwrap_or(true)) {
                let name = c.name.as_deref().unwrap_or("cheat");
                let codes: Result<Vec<Code>, String> = c.codes.iter().map(|l| parse_code(l)).collect();
                match codes {
                    Ok(codes) => out.push(Patch { codes }),
                    Err(err) => eprintln!("pe2: mods: {}: {name}: {err}; skipped", path.display()),
                }
            }
            for p in file.patch.iter().filter(|p| p.enabled.unwrap_or(true)) {
                let size = p.size.unwrap_or(4);
                if ![1, 2, 4].contains(&size) {
                    let name = p.name.as_deref().unwrap_or("patch");
                    eprintln!("pe2: mods: {}: {name}: size must be 1, 2 or 4; skipped", path.display());
                    continue;
                }
                out.push(Patch { codes: vec![Code::Write { address: p.address, size, value: p.value }] });
            }
        }
        if !out.is_empty() {
            eprintln!("pe2: mods: {} cheats and patches", out.len());
        }
        out
    })
}

/// Where PlayStation address `address` (KUSEG, KSEG0 or KSEG1) is in the
/// RAM mapping, if it is in RAM.
fn ram(address: u32) -> Option<*mut u8> {
    let physical = address & 0x1FFF_FFFF;
    (physical < 0x0020_0000).then_some((0x8000_0000u32 | physical) as usize as *mut u8)
}

fn read(address: u32, size: u32) -> Option<u32> {
    let p = ram(address)?;
    // SAFETY: inside the PlayStation RAM mapping (see `ram`).
    Some(unsafe {
        match size {
            1 => u32::from(p.read_volatile()),
            2 => u32::from(p.cast::<u16>().read_unaligned()),
            _ => p.cast::<u32>().read_unaligned(),
        }
    })
}

fn write(address: u32, size: u32, value: u32) {
    let Some(p) = ram(address) else { return };
    // SAFETY: inside the PlayStation RAM mapping (see `ram`).
    unsafe {
        match size {
            1 => p.write_volatile(value as u8),
            2 => p.cast::<u16>().write_unaligned(value as u16),
            _ => p.cast::<u32>().write_unaligned(value),
        }
    }
}

/// Runs `codes` once, against `read` and `write`.
fn run_codes(codes: &[Code], read: impl Fn(u32, u32) -> Option<u32>, mut write: impl FnMut(u32, u32, u32)) {
    let mut i = 0;
    while i < codes.len() {
        match codes[i] {
            Code::Write { address, size, value } => write(address, size, value),
            Code::Add { address, size, delta } => {
                if let Some(v) = read(address, size) {
                    write(address, size, v.wrapping_add(delta as u32));
                }
            }
            Code::If { address, size, test, value } => {
                let pass = read(address, size).is_some_and(|v| match test {
                    Test::Equal => v == value,
                    Test::Different => v != value,
                    Test::Less => v < value,
                    Test::Greater => v > value,
                });
                if !pass {
                    i += 1;
                }
            }
            Code::Repeat { count, step, increment } => {
                if let Some(Code::Write { address, size, value }) = codes.get(i + 1).copied() {
                    for k in 0..count {
                        write(address + k * step, size, value.wrapping_add(k * increment));
                    }
                    i += 1;
                }
            }
        }
        i += 1;
    }
}

/// Applies the mods' cheats and patches: each vertical blank, before the
/// game's own handler, with the game stopped, as a cheat cartridge does.
pub fn apply_patches() {
    for p in patches() {
        run_codes(&p.codes, read, write);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The hash names files, so it must never change.
    #[test]
    fn screen_hash_is_stable() {
        assert_eq!(fnv1a([]), 0xCBF2_9CE4_8422_2325);
        assert_eq!(fnv1a(*b"a"), 0xAF63_DC4C_8601_EC8C);
        assert_ne!(screen_hash(2, 1, &[1, 2]), screen_hash(1, 2, &[1, 2]));
    }

    /// GameShark lines parse to the codes they stand for.
    #[test]
    fn gameshark_codes() {
        assert_eq!(parse_code("80073BA0 0032"), Ok(Code::Write { address: 0x80073BA0, size: 2, value: 0x32 }));
        assert_eq!(parse_code("30012345 00FF"), Ok(Code::Write { address: 0x80012345, size: 1, value: 0xFF }));
        assert_eq!(parse_code("11012344 0002"), Ok(Code::Add { address: 0x80012344, size: 2, delta: -2 }));
        assert_eq!(
            parse_code("D0012344 1234"),
            Ok(Code::If { address: 0x80012344, size: 2, test: Test::Equal, value: 0x1234 })
        );
        assert_eq!(parse_code("50000402 0001"), Ok(Code::Repeat { count: 4, step: 2, increment: 1 }));
        assert!(parse_code("C0012344 0001").is_err());
        assert!(parse_code("80012344").is_err());
    }

    /// Conditions guard the next code; repeaters repeat it.
    #[test]
    fn codes_run() {
        let read = |a: u32, _| Some(if a == 0x10 { 5 } else { 0 });
        let codes = [
            Code::If { address: 0x10, size: 2, test: Test::Equal, value: 5 },
            Code::Write { address: 0x20, size: 2, value: 1 },
            Code::If { address: 0x10, size: 2, test: Test::Equal, value: 6 },
            Code::Write { address: 0x30, size: 2, value: 1 },
            Code::Repeat { count: 3, step: 2, increment: 10 },
            Code::Write { address: 0x40, size: 2, value: 1 },
        ];
        let mut writes = Vec::new();
        run_codes(&codes, read, |a, _, v| writes.push((a, v)));
        assert_eq!(writes, [(0x20, 1), (0x40, 1), (0x42, 11), (0x44, 21)]);
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
