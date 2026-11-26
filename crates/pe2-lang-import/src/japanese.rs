//! The Japanese release's dialogue. Its dialogue files (`.pe2cap2`, CAP
//! containers of the same layout as the game's) draw each room's lines with
//! a glyph picture of its own, of only the kana and kanji it needs, so
//! PE2PC's extract cannot read them. Each file's picture is known
//! (res/ja/pages.toml), and each glyph by the hash of its texels
//! (res/ja/glyphs.toml): the lines read from those go, as translations, into
//! the game's lines in the same place, drawn by PE2PC from the mod's font
//! (Noto Sans JP, fonts/captions.otf).

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::Result;
use crate::dumps::{literal, lzss_at};

const PAGES: &str = include_str!("../res/ja/pages.toml");
const GLYPHS: &str = include_str!("../res/ja/glyphs.toml");
const FONT: &[u8] = include_bytes!("../res/ja/NotoSansJP-Bold.otf");
const FONT_LICENSE: &str = include_str!("../res/ja/OFL.txt");

/// A glyph not known: its line stays English.
const UNKNOWN: char = '〓';

fn u16_at(data: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(data.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(data: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?))
}

/// The CAP containers of a dialogue chunk (`.pe2cap2`): its slots (a table
/// of 50 at the start, their data from 0x7F0) of kind 3 that begin "CAP".
fn caps(chunk: &[u8]) -> Vec<&[u8]> {
    const DATA: usize = 0x7F0;
    let slots: Vec<(u8, usize, usize)> = (0..50)
        .filter_map(|k| {
            let e = chunk.get(k * 16..k * 16 + 16)?;
            if e[0] == 0 {
                return None;
            }
            Some((e[0], u32_at(e, 4)? as usize, u32_at(e, 8)? as usize))
        })
        .collect();
    let Some(base) = slots.iter().map(|s| s.2).min() else { return Vec::new() };
    slots
        .iter()
        .filter(|s| s.0 == 3)
        .map(|&(_, size, dest)| {
            let at = (DATA + dest - base).min(chunk.len());
            &chunk[at..(at + size).min(chunk.len())]
        })
        .filter(|cap| cap.starts_with(b"CAP"))
        .collect()
}

/// A line of a CAP container: its record's offset and its codes.
struct Line {
    record: usize,
    codes: Vec<u16>,
}

impl Line {
    fn has_glyphs(&self) -> bool {
        self.codes.iter().any(|x| x & 0x8000 == 0)
    }

    fn choices(&self) -> Vec<u16> {
        self.codes.iter().copied().filter(|x| x & 0xFF00 == 0x8100).collect()
    }
}

/// A CAP container's lines (records of 12 bytes, 24 for a record without
/// text, from 0x10 after its script's count).
fn lines(cap: &[u8]) -> Vec<Line> {
    let Some(script) = u32_at(cap, 12).map(|s| s as usize) else { return Vec::new() };
    let Some(count) = u16_at(cap, script) else { return Vec::new() };
    let mut at = script + 0x10;
    let mut out = Vec::new();
    for _ in 0..(count as i16).max(0) {
        let Some(text) = u32_at(cap, at + 8) else { break };
        if text == 0xFFFF_FFFF {
            at += 24;
            continue;
        }
        let codes = (text as usize..).step_by(2).map_while(|q| u16_at(cap, q).filter(|&v| v != 0xFFFF)).collect();
        out.push(Line { record: at, codes });
        at += 12;
    }
    out
}

/// A CAP container's cell `i`: u, v, width, height on its glyph picture.
fn cell(cap: &[u8], i: usize) -> Option<[u8; 4]> {
    let at = u32_at(cap, 8)? as usize + 4 * i;
    cap.get(at..at + 4)?.try_into().ok()
}

/// What PE2PC knows a CAP container by (its `cap = ` in text/): the FNV-1a
/// hash of its bytes up to the end of its cells' table.
fn cap_hash(cap: &[u8]) -> u64 {
    let end = u32_at(cap, 16)
        .map_or(cap.len(), |c| (c as usize + 4 + u32_at(cap, c as usize).unwrap_or(0) as usize * 4).min(cap.len()));
    cap[..end].iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3))
}

/// A glyph picture's columns (`.pe2img`): each its rows of 128 bytes (256
/// texels of 4 bits), from LZSS strips of 32 rows.
fn picture(data: &[u8]) -> Vec<Vec<Vec<u8>>> {
    let (mut entries, mut at, mut end_y) = (Vec::new(), 0, 0);
    while let (Some(x), Some(y), Some(offset)) = (u16_at(data, at), u16_at(data, at + 2), u32_at(data, at + 4)) {
        at += 8;
        if x == 0xFFFF {
            end_y = y;
            break;
        }
        entries.push(offset as usize);
    }
    let Some(&first) = entries.first() else { return Vec::new() };
    let height = match (entries.len(), end_y) {
        (1, 0xFFFF) => 0x40,
        (1, y) if y & 0x8000 != 0 => usize::from(y & 0x7FFF),
        _ => 0x100,
    };
    let stream = data.get(first..).unwrap_or_default();
    let (mut strips, mut p) = (Vec::new(), 0);
    loop {
        while stream.get(p) == Some(&0) {
            p += 1;
        }
        if p >= stream.len() {
            break;
        }
        let (mut strip, end) = lzss_at(stream, p);
        if strip.is_empty() {
            break;
        }
        strip.resize(0x1000, 0);
        strips.push(strip);
        p = end;
    }
    let mut strips = strips.into_iter();
    entries
        .iter()
        .map(|_| {
            let mut rows = Vec::new();
            for _ in 0..height.div_ceil(32) {
                let strip = strips.next().unwrap_or_else(|| vec![0; 0x1000]);
                rows.extend(strip.chunks(128).map(<[u8]>::to_vec));
            }
            rows.truncate(height);
            rows
        })
        .collect()
}

/// A cell's texels, one byte each, and the hash glyphs.toml knows it by.
fn glyph_hash(column: &[Vec<u8>], [u, v, w, h]: [u8; 4]) -> String {
    let mut bytes = vec![w, h];
    for y in usize::from(v)..usize::from(v) + usize::from(h) {
        let row = column.get(y);
        for x in usize::from(u)..usize::from(u) + usize::from(w) {
            let b = row.and_then(|r| r.get(x / 2)).copied().unwrap_or(0);
            bytes.push(if x % 2 == 0 { b & 15 } else { b >> 4 });
        }
    }
    sha1_smol::Sha1::from(&bytes).digest().to_string()[..12].to_string()
}

/// A line's text: its glyphs' characters, its controls as PE2PC's text/
/// writes them, full-width Latin letters and digits made half-width (as
/// the font draws them best).
fn text(codes: &[u16], chars: &HashMap<usize, char>) -> String {
    let (mut s, mut color) = (String::new(), 0);
    for &x in codes {
        match x {
            0xFFFE => s.push('\n'),
            0xFFFD => s.push_str("{_}"),
            x if x & 0x8000 != 0 => s.push_str(&format!("{{{x:04X}}}")),
            x => {
                let c = (x >> 10) & 0x1F;
                if c != color {
                    s.push_str(&format!("{{color {c}}}"));
                    color = c;
                }
                s.push(chars.get(&usize::from(x & 0x3FF)).copied().unwrap_or(UNKNOWN));
            }
        }
    }
    let half = |c: char| match c {
        'Ａ'..='ｚ' | '０'..='９' => char::from_u32(c as u32 - 0xFEE0).unwrap_or(c),
        c => c,
    };
    s.chars().map(half).collect::<String>().trim_end_matches('\n').to_string()
}

/// Writes the mod's `text/` and `fonts/` from the game's extract `game` and
/// the Japanese release's `other` (their dumps/). Answers how many lines.
pub fn import(game: &Path, other: &Path, out: &Path, report: &mut Vec<String>) -> Result<usize> {
    let pages: toml::Table = PAGES.parse()?;
    let glyphs: toml::Table = GLYPHS.parse()?;
    let (game, other) = (game.join("files"), other.join("files"));
    let mut pictures: HashMap<String, Vec<Vec<Vec<u8>>>> = HashMap::new();
    let mut total = 0;
    for (file, page) in &pages {
        let (Some((key, rest)), Some((picture_name, column))) =
            (file.rsplit_once('/'), page.as_str().and_then(|p| p.split_once('#')))
        else {
            return Err(format!("res/ja/pages.toml: {file}").into());
        };
        let (Some((chunk, n)), Ok(column)) = (rest.split_once('-'), column.parse::<usize>()) else {
            return Err(format!("res/ja/pages.toml: {file}").into());
        };
        let n: usize = n.parse()?;
        let read = |dir: &Path| std::fs::read(dir.join(key).join(format!("{chunk}.pe2cap2")));
        let (Ok(mine), Ok(theirs)) = (read(&game), read(&other)) else {
            report.push(format!("{file}: not in both releases, stays English"));
            continue;
        };
        let (Some(mine), Some(theirs)) = (caps(&mine).get(n).copied(), caps(&theirs).get(n).copied()) else {
            report.push(format!("{file}: not in both releases, stays English"));
            continue;
        };
        let (my_lines, their_lines) = (lines(mine), lines(theirs));
        if my_lines.len() != their_lines.len() {
            report.push(format!(
                "{file}: {} lines here, {} in the game's, stays English",
                their_lines.len(),
                my_lines.len()
            ));
            continue;
        }
        if !pictures.contains_key(picture_name) {
            let path = other.join(format!("{picture_name}.pe2img"));
            let data = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            pictures.insert(picture_name.to_string(), picture(&data));
        }
        let column = pictures[picture_name].get(column).map_or(&[][..], Vec::as_slice);
        let cells: HashSet<usize> = their_lines
            .iter()
            .flat_map(|l| &l.codes)
            .filter(|x| *x & 0x8000 == 0)
            .map(|x| usize::from(x & 0x3FF))
            .collect();
        let chars: HashMap<usize, char> = cells
            .into_iter()
            .filter_map(|i| Some((i, glyphs.get(&glyph_hash(column, cell(theirs, i)?))?.as_str()?.chars().next()?)))
            .collect();
        let mut toml = format!("cap = \"{:016x}\"\n", cap_hash(mine));
        let mut count = 0;
        for (mine, theirs) in my_lines.iter().zip(&their_lines) {
            if !mine.has_glyphs() || !theirs.has_glyphs() {
                continue;
            }
            let text = text(&theirs.codes, &chars);
            if mine.choices() != theirs.choices() || text.contains(UNKNOWN) {
                report.push(format!(
                    "{file} record 0x{:04X}: other choices or a glyph not known, stays English",
                    mine.record
                ));
                continue;
            }
            toml += &format!("\n[[line]]\nrecord = 0x{:04X}\ntext = {}\n", mine.record, literal(&text)?);
            count += 1;
        }
        if count > 0 {
            let path = out.join("text").join(key).join(format!("{chunk}-{n}.toml"));
            std::fs::create_dir_all(path.parent().unwrap_or(out))?;
            std::fs::write(&path, toml)?;
            total += count;
        }
    }
    let fonts = out.join("fonts");
    std::fs::create_dir_all(&fonts)?;
    std::fs::write(fonts.join("captions.otf"), FONT)?;
    std::fs::write(fonts.join("OFL.txt"), FONT_LICENSE)?;
    Ok(total)
}
