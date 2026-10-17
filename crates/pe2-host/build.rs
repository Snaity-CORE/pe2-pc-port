//! Links the game library built by `python tools/build_game.py`, and the
//! application icon: the project's res/icon.png, or a picture of the
//! player's own at `local/icon.png` (any size, square, transparency kept;
//! ignored by git).

use std::path::{Path, PathBuf};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let game_dir = root.join("build/game");
    let lib = game_dir.join("pe2game.lib");
    if !lib.exists() {
        panic!(
            "{} not found: run `python tools/build_game.py` first",
            lib.display()
        );
    }
    println!("cargo:rerun-if-changed={}", lib.display());
    println!("cargo:rustc-link-search=native={}", game_dir.display());
    // Whole-archive: the thunk tables reference every native body, and the
    // game's functions are reached only through them.
    println!("cargo:rustc-link-lib=static:+whole-archive=pe2game");

    // PSX memory is identity-mapped at 0x80000000 and 0x1F800000, so the image
    // must be 32-bit, large-address-aware and loaded at a fixed base that
    // keeps out of those ranges. The absolute-symbol object has no SafeSEH
    // table.
    icon(&root);

    for arg in [
        "/LARGEADDRESSAWARE",
        "/FIXED",
        "/DYNAMICBASE:NO",
        "/BASE:0x00400000",
        "/SAFESEH:NO",
        "/MAP",
    ] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}

/// Sizes in pe2.exe's icon (Explorer, the taskbar), and of the picture the
/// windows get at run time (icon.rgba).
const ICON_SIZES: [u32; 6] = [256, 64, 48, 32, 24, 16];
const WINDOW_ICON_SIZE: u32 = 128;

/// pe2.res (linked into pe2.exe) and icon.rgba (for the windows, cfg
/// `pe2_icon`), in OUT_DIR, from local/icon.png or else res/icon.png.
fn icon(root: &Path) {
    println!("cargo::rustc-check-cfg=cfg(pe2_icon)");
    let local = root.join("local");
    let own = local.join("icon.png");
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/icon.png");
    // Watch the folder (it changes when a picture is added) or, without it,
    // the repository's root; and the project's picture.
    println!("cargo:rerun-if-changed={}", if local.exists() { local.display() } else { root.display() });
    println!("cargo:rerun-if-changed={}", project.display());
    let path = if own.exists() { own } else { project };
    if !path.exists() {
        return;
    }
    println!("cargo:rerun-if-changed={}", path.display());
    let (rgba, size) = match load_square_rgba(&path) {
        Ok(found) => found,
        Err(err) => {
            println!("cargo:warning={}: {err}; no icon", path.display());
            return;
        }
    };
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let images: Vec<(u32, Vec<u8>)> =
        ICON_SIZES.iter().map(|&s| (s, encode_png(&downscale(&rgba, size, s), s))).collect();
    std::fs::write(out.join("pe2.res"), icon_resources(&images)).unwrap();
    std::fs::write(out.join("icon.rgba"), downscale(&rgba, size, WINDOW_ICON_SIZE)).unwrap();
    println!("cargo:rustc-link-arg-bins={}", out.join("pe2.res").display());
    println!("cargo:rustc-cfg=pe2_icon");
}

/// The picture as 8-bit RGBA, padded to a square with transparency.
fn load_square_rgba(path: &Path) -> Result<(Vec<u8>, u32), String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = png::Decoder::new(std::io::BufReader::new(file));
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| e.to_string())?;
    let (w, h) = (info.width, info.height);
    let channels = info.color_type.samples();
    let pixel = |x: u32, y: u32| -> [u8; 4] {
        let p = &buf[(y * w + x) as usize * channels..][..channels];
        match channels {
            1 => [p[0], p[0], p[0], 255],
            2 => [p[0], p[0], p[0], p[1]],
            3 => [p[0], p[1], p[2], 255],
            _ => [p[0], p[1], p[2], p[3]],
        }
    };
    let size = w.max(h);
    let (ox, oy) = ((size - w) / 2, (size - h) / 2);
    let mut out = vec![0u8; (size * size * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let at = (((y + oy) * size + x + ox) * 4) as usize;
            out[at..at + 4].copy_from_slice(&pixel(x, y));
        }
    }
    Ok((out, size))
}

/// Box-filtered downscale (alpha-weighted, so transparent pixels do not
/// darken the edges).
fn downscale(rgba: &[u8], from: u32, to: u32) -> Vec<u8> {
    let mut out = vec![0u8; (to * to * 4) as usize];
    let span = |i: u32| ((i as u64 * from as u64 / to as u64) as u32, (((i + 1) as u64 * from as u64 / to as u64) as u32).max(i * from / to + 1));
    for ty in 0..to {
        let (y0, y1) = span(ty);
        for tx in 0..to {
            let (x0, x1) = span(tx);
            let mut sum = [0u64; 4];
            for y in y0..y1 {
                for x in x0..x1 {
                    let p = &rgba[((y * from + x) * 4) as usize..][..4];
                    let a = u64::from(p[3]);
                    for c in 0..3 {
                        sum[c] += u64::from(p[c]) * a;
                    }
                    sum[3] += a;
                }
            }
            let n = u64::from((y1 - y0) * (x1 - x0));
            let at = ((ty * to + tx) * 4) as usize;
            for c in 0..3 {
                out[at + c] = sum[c].checked_div(sum[3]).unwrap_or(0) as u8;
            }
            out[at + 3] = (sum[3] / n) as u8;
        }
    }
    out
}

fn encode_png(rgba: &[u8], size: u32) -> Vec<u8> {
    let mut data = Vec::new();
    let mut enc = png::Encoder::new(&mut data, size, size);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().unwrap();
    writer.write_image_data(rgba).unwrap();
    drop(writer);
    data
}

/// A compiled resource file (.res, which the linker takes as is) holding
/// the images as RT_ICON resources 1.. (PNG-compressed, as Windows accepts
/// since Vista) and their RT_GROUP_ICON 1, the application's icon.
fn icon_resources(images: &[(u32, Vec<u8>)]) -> Vec<u8> {
    const RT_ICON: u16 = 3;
    const RT_GROUP_ICON: u16 = 14;
    fn entry(out: &mut Vec<u8>, kind: u16, id: u16, flags: u16, data: &[u8]) {
        let words = |v: &[u32]| v.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>();
        out.extend(words(&[data.len() as u32, 32]));
        out.extend([0xFF, 0xFF]);
        out.extend(kind.to_le_bytes());
        out.extend([0xFF, 0xFF]);
        out.extend(id.to_le_bytes());
        out.extend(words(&[0]));
        out.extend(flags.to_le_bytes());
        out.extend(0x0409u16.to_le_bytes());
        out.extend(words(&[0, 0]));
        out.extend_from_slice(data);
        while !out.len().is_multiple_of(4) {
            out.push(0);
        }
    }
    let mut out = Vec::new();
    // The empty entry every .res file starts with.
    entry(&mut out, 0, 0, 0, &[]);
    let mut group = Vec::new();
    group.extend([0u8, 0, 1, 0]);
    group.extend((images.len() as u16).to_le_bytes());
    for (i, (size, png)) in images.iter().enumerate() {
        let id = i as u16 + 1;
        entry(&mut out, RT_ICON, id, 0x1010, png);
        let dim = if *size >= 256 { 0 } else { *size as u8 };
        group.extend([dim, dim, 0, 0]);
        group.extend(1u16.to_le_bytes());
        group.extend(32u16.to_le_bytes());
        group.extend((png.len() as u32).to_le_bytes());
        group.extend(id.to_le_bytes());
    }
    entry(&mut out, RT_GROUP_ICON, 1, 0x1030, &group);
    out
}
