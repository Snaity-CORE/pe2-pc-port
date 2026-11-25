//! One release's extract (`pe2 --extract`, its data folder's `dumps/`): its
//! executable, code packages, menus' font and the texts PE2PC lists.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::Result;

/// The menus' font's faces: their glyph tables (main/textdraw.c,
/// _gFontGlyphs*, records of 12 bytes from ' '), how many, and the V bias
/// each adds. A text is drawn in one of them, and a release may give a code
/// another picture in each.
pub const FACES: [(&str, u32, u32, u8); 3] =
    [("medium", 0x8005_EFB0, 224, 38), ("large", 0x8005_FA30, 224, 128), ("small", 0x8006_04B0, 91, 0)];

/// A picture: its rows of palette indices.
pub type Picture = Vec<Vec<u8>>;

pub struct Version {
    pub dumps: PathBuf,
    pub exe_name: String,
    pub exe_base: u32,
    /// The executable without its header, as it loads at `exe_base`.
    pub exe: Vec<u8>,
}

impl Version {
    pub fn open(dumps: &Path) -> Result<Version> {
        let files = dumps.join("files");
        for entry in std::fs::read_dir(&files).map_err(|e| format!("{}: {e}", files.display()))? {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }
            let data = std::fs::read(&path)?;
            if data.starts_with(b"PS-X EXE") && data.len() > 0x800 {
                return Ok(Version {
                    dumps: dumps.to_path_buf(),
                    exe_name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                    exe_base: u32_at(&data, 0x18),
                    exe: data[0x800..].to_vec(),
                });
            }
        }
        Err(format!("{}: no executable (extract again with a newer PE2PC)", files.display()).into())
    }

    /// Whether `other` runs this one's executable (a translation of the
    /// same discs), not another release's built again (SLES_025.59).
    pub fn same_code(&self, other: &Version) -> bool {
        other.exe_name == self.exe_name && other.exe.len() == self.exe.len()
    }

    /// The code packages of file `key` (stage1/101, say): (chunk file name,
    /// contents decompressed), by name.
    pub fn packages_of(&self, key: &str) -> Vec<(String, Vec<u8>)> {
        let dir = self.dumps.join("files").join(key);
        let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "pe2pkg"))
            .filter_map(|p| Some((p.file_name()?.to_string_lossy().into_owned(), lzss(&std::fs::read(&p).ok()?))))
            .collect();
        out.sort();
        out
    }

    /// For each face, code -> the cell's picture.
    pub fn fonts(&self) -> Result<Vec<HashMap<u8, Picture>>> {
        let maps = self.dumps.join("textures-glyphs");
        let mut sheet = None;
        for path in toml_files(&maps, false) {
            let map = read_toml(&path)?;
            if map.get("kind").and_then(|k| k.as_str()) == Some("ui-font") {
                sheet = map.get("sheets").and_then(|s| s.get(0)).and_then(|s| s.as_str()).map(str::to_string);
                break;
            }
        }
        let sheet = sheet.ok_or_else(|| format!("{}: no map of the menus' font", maps.display()))?;
        let rows = png_indices(&self.dumps.join("textures-indexed").join(format!("{sheet}-4bit.idx.png")))?;
        let mut out = Vec::new();
        for (_, table, count, bias) in FACES {
            let mut font = HashMap::new();
            let at = (table - self.exe_base) as usize;
            for code in 0..count as usize {
                let cell = &self.exe[at + code * 12..at + code * 12 + 4];
                let (u, v, w, h) =
                    (cell[0] as usize, cell[1].wrapping_add(bias) as usize, cell[2] as usize, cell[3] as usize);
                if w == 0 && h == 0 {
                    continue;
                }
                let picture = (0..=h)
                    .filter(|y| v + y < rows.len())
                    .map(|y| {
                        let row = &rows[v + y];
                        row[u.min(row.len())..(u + w + 1).min(row.len())].to_vec()
                    })
                    .collect();
                font.insert(0x20 + code as u8, picture);
            }
            out.push(font);
        }
        Ok(out)
    }
}

pub fn u32_at(data: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(data[at..at + 4].try_into().expect("4 bytes"))
}

/// One LZSS stream (PE2PC's dialogue/picture.rs `lzss`).
pub fn lzss(stream: &[u8]) -> Vec<u8> {
    lzss_at(stream, 0).0
}

/// The LZSS stream at `start` of `stream`, and where it ends.
pub fn lzss_at(stream: &[u8], start: usize) -> (Vec<u8>, usize) {
    let (mut ring, mut next, mut out) = ([0u8; 256], 0u8, Vec::new());
    let (mut pos, mut byte, mut mask) = (start, 0u8, 0u8);
    let mut bits = |n: u32| -> Option<u32> {
        let mut v = 0;
        for _ in 0..n {
            if mask == 0 {
                byte = *stream.get(pos)?;
                pos += 1;
                mask = 0x80;
            }
            v = (v << 1) | u32::from(byte & mask != 0);
            mask >>= 1;
        }
        Some(v)
    };
    while let Some(literal) = bits(1) {
        if literal == 1 {
            let Some(b) = bits(8) else { break };
            ring[next as usize] = b as u8;
            next = next.wrapping_add(1);
            out.push(b as u8);
            continue;
        }
        let Some(offset) = bits(8) else { break };
        if offset == 0 {
            break;
        }
        let Some(length) = bits(4) else { break };
        let mut at = (offset as u8).wrapping_sub(1);
        for _ in 0..length + 2 {
            let b = ring[at as usize];
            ring[next as usize] = b;
            next = next.wrapping_add(1);
            out.push(b);
            at = at.wrapping_add(1);
        }
    }
    (out, pos)
}

/// An 8-bit greyscale PNG's rows (an indexed sheet's palette indices).
pub fn png_indices(path: &Path) -> Result<Picture> {
    let file = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut reader = png::Decoder::new(std::io::BufReader::new(file)).read_info()?;
    let mut data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut data)?;
    let width = info.line_size;
    Ok(data[..info.buffer_size()].chunks(width).map(<[u8]>::to_vec).collect())
}

pub fn read_toml(path: &Path) -> Result<toml::Table> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(text.parse::<toml::Table>().map_err(|e| format!("{}: {e}", path.display()))?)
}

/// The `.toml` files in `dir` (and its folders, with `deep`), by path as
/// Python's sorted glob gives them on Windows (`\` between names).
pub fn toml_files(dir: &Path, deep: bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        for entry in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                if deep {
                    todo.push(path);
                }
            } else if path.extension().is_some_and(|x| x == "toml") {
                out.push(path);
            }
        }
    }
    out.sort_by_cached_key(|p| rel(dir, p).replace('/', "\\"));
    out
}

/// `path` under `dir`, its names separated by `/`.
pub fn rel(dir: &Path, path: &Path) -> String {
    let r = path.strip_prefix(dir).unwrap_or(path);
    r.components().map(|c| c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/")
}

/// A text the extract lists (`dumps/strings/**/*.toml`'s `[[string]]`).
#[derive(Clone, Debug)]
pub struct Listed {
    pub at: Option<u32>,
    pub original: String,
}

pub fn listed(path: &Path) -> Result<Vec<Listed>> {
    let table = read_toml(path)?;
    let strings = table.get("string").and_then(|s| s.as_array()).cloned().unwrap_or_default();
    Ok(strings
        .iter()
        .filter_map(|s| {
            Some(Listed {
                at: s.get("at").and_then(|a| a.as_integer()).map(|a| a as u32),
                original: s.get("original")?.as_str()?.to_string(),
            })
        })
        .collect())
}

/// A text's bytes in the game's encoding (Latin-1), if it has them.
pub fn latin1(text: &str) -> Option<Vec<u8>> {
    text.chars().map(|c| u8::try_from(u32::from(c)).ok()).collect()
}

/// A TOML multi-line literal string.
pub fn literal(text: &str) -> Result<String> {
    if text.contains("'''") {
        return Err(format!("a text holds ''': {text:?}").into());
    }
    Ok(format!("'''\n{text}'''"))
}
