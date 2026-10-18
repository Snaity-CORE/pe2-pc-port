//! Translations of the dialogue (docs/MODDING.md).
//!
//! A mod's `text/**/*.toml` files (as the extract writes them, see
//! dialogue.rs) give lines of CAP files, known by their hash, new text.
//! When the game loads a CAP file that has translated lines, it gets a copy
//! built here instead (pc_cap_translate, from the fork's captions.c): the
//! original with the translated lines' text added and pointed to, and a
//! longer glyph table whose new glyphs lie below the original ones in the
//! file's glyph page, where nothing else is. Those glyphs come from the
//! game's own dialogue font, collected from the disc on a thread of its own
//! when the game starts (dialogue::font); accented letters are made from
//! it, with marks drawn here. pc_cap_glyphs puts them into the page each
//! time the file's text is drawn, if they are not there already.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use serde::Deserialize;

use super::dialogue::{self, Cap};

/// A translation file (other fields, the original text, are for people).
#[derive(Deserialize)]
struct TextFile {
    cap: String,
    #[serde(default)]
    line: Vec<LineEntry>,
}

#[derive(Deserialize)]
struct LineEntry {
    record: usize,
    #[serde(default)]
    text: String,
}

/// CAP file hash -> record -> translated text.
type Translations = HashMap<u64, HashMap<usize, String>>;

fn translations() -> &'static Translations {
    static INDEX: OnceLock<Translations> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut out = Translations::new();
        if !super::enabled() {
            return out;
        }
        // Lowest priority first, so the highest overwrites it.
        for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
            let mut stack = vec![m.dir.join("text")];
            while let Some(dir) = stack.pop() {
                let Ok(entries) = std::fs::read_dir(&dir) else { continue };
                for e in entries.flatten() {
                    let path = e.path();
                    if path.is_dir() {
                        stack.push(path);
                        continue;
                    }
                    if !path.extension().is_some_and(|x| x.eq_ignore_ascii_case("toml")) {
                        continue;
                    }
                    let file = std::fs::read_to_string(&path)
                        .map_err(|e| e.to_string())
                        .and_then(|t| toml::from_str::<TextFile>(&t).map_err(|e| e.to_string()));
                    let file = match file {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("pe2: mods: {}: {e}", path.display());
                            continue;
                        }
                    };
                    let Ok(hash) = u64::from_str_radix(file.cap.trim(), 16) else {
                        eprintln!("pe2: mods: {}: cap is not a hash", path.display());
                        continue;
                    };
                    let lines = out.entry(hash).or_default();
                    for l in file.line.into_iter().filter(|l| !l.text.is_empty()) {
                        lines.insert(l.record, l.text);
                    }
                }
            }
        }
        out.retain(|_, lines| !lines.is_empty());
        if !out.is_empty() {
            let lines: usize = out.values().map(HashMap::len).sum();
            eprintln!("pe2: mods: {lines} translated lines in {} dialogue files", out.len());
        }
        out
    })
}

static DISCS: OnceLock<Vec<PathBuf>> = OnceLock::new();
type Font = HashMap<&'static str, Vec<u8>>;
static FONT: OnceLock<Font> = OnceLock::new();

/// With translations in use, starts collecting the dialogue font from the
/// first of `discs` (their images) on a thread of its own.
pub fn start(discs: &[PathBuf]) {
    let _ = DISCS.set(discs.to_vec());
    if translations().is_empty() {
        return;
    }
    let Some(image) = discs.first().cloned() else { return };
    std::thread::spawn(move || {
        let since = std::time::Instant::now();
        let mut disc = match crate::disc::open(&image) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("pe2: mods: translations: {}: {e}", image.display());
                return;
            }
        };
        let sources = super::files::dialogue_sources(disc.as_mut());
        let font = dialogue::font(&sources);
        eprintln!("pe2: mods: dialogue font: {} glyphs in {:.1} s", font.len(), since.elapsed().as_secs_f32());
        let _ = FONT.set(font);
    });
}

// ---- glyphs -------------------------------------------------------------

/// Palette indices of the dialogue font: its outline, and its letters'
/// fill.
const OUTLINE: u8 = 8;
const FILL: u8 = 2;
/// Rows a capital is moved down to make room for a mark above it.
const CAPITAL_MARK_ROWS: usize = 4;

/// A glyph picture: its size and 4-bit texels.
#[derive(Clone)]
struct Glyph {
    w: usize,
    h: usize,
    texels: Vec<u8>,
}

impl Glyph {
    fn from_shape(shape: &[u8]) -> Glyph {
        Glyph { w: usize::from(shape[0]), h: usize::from(shape[1]), texels: shape[2..].to_vec() }
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        self.texels[y * self.w + x]
    }

    /// The same glyph with `rows` empty rows added on top.
    fn taller(&self, rows: usize) -> Glyph {
        let mut texels = vec![0; rows * self.w];
        texels.extend_from_slice(&self.texels);
        Glyph { w: self.w, h: self.h + rows, texels }
    }

    /// The middle of its drawn columns.
    fn middle(&self) -> usize {
        let used: Vec<usize> = (0..self.w).filter(|&x| (0..self.h).any(|y| self.at(x, y) != 0)).collect();
        match (used.first(), used.last()) {
            (Some(a), Some(b)) => (a + b).div_ceil(2),
            _ => self.w / 2,
        }
    }

    /// Draws `core` (rows of '#') in the fill colour with its bottom row at
    /// `bottom`, centred on the glyph, outlined as the font's letters are.
    fn mark(&mut self, core: &[&str], bottom: usize) {
        let cw = core.iter().map(|r| r.len()).max().unwrap_or(0);
        let x0 = self.middle().saturating_sub(cw / 2) as isize;
        let y0 = bottom as isize + 1 - core.len() as isize;
        let mut cells = Vec::new();
        for (dy, row) in core.iter().enumerate() {
            for (dx, c) in row.bytes().enumerate() {
                if c == b'#' {
                    cells.push((x0 + dx as isize, y0 + dy as isize));
                }
            }
        }
        let inside = |x: isize, y: isize| x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h;
        for &(x, y) in &cells {
            for (ox, oy) in (-1..=1).flat_map(|a| (-1..=1).map(move |b| (a, b))) {
                let (nx, ny) = (x + ox, y + oy);
                if inside(nx, ny) && self.texels[ny as usize * self.w + nx as usize] == 0 {
                    self.texels[ny as usize * self.w + nx as usize] = OUTLINE;
                }
            }
        }
        for &(x, y) in &cells {
            if inside(x, y) {
                self.texels[y as usize * self.w + x as usize] = FILL;
            }
        }
    }
}

/// Marks: their cores (drawn, then outlined).
const ACUTE: &[&str] = &[".##", "##."];
const GRAVE: &[&str] = &["##.", ".##"];
const CIRCUMFLEX: &[&str] = &[".##.", "#..#"];
const TILDE: &[&str] = &[".##.#", "#.##."];
const DIAERESIS: &[&str] = &["##.##"];
const CEDILLA: &[&str] = &[".#", "#."];

/// An accented letter: its base letter and mark.
fn accented(c: char) -> Option<(char, &'static [&'static str])> {
    let (base, mark): (char, &[&str]) = match c {
        'á' => ('a', ACUTE),
        'à' => ('a', GRAVE),
        'â' => ('a', CIRCUMFLEX),
        'ã' => ('a', TILDE),
        'ä' => ('a', DIAERESIS),
        'é' => ('e', ACUTE),
        'è' => ('e', GRAVE),
        'ê' => ('e', CIRCUMFLEX),
        'ë' => ('e', DIAERESIS),
        'í' => ('i', ACUTE),
        'ì' => ('i', GRAVE),
        'î' => ('i', CIRCUMFLEX),
        'ï' => ('i', DIAERESIS),
        'ó' => ('o', ACUTE),
        'ò' => ('o', GRAVE),
        'ô' => ('o', CIRCUMFLEX),
        'õ' => ('o', TILDE),
        'ö' => ('o', DIAERESIS),
        'ú' => ('u', ACUTE),
        'ù' => ('u', GRAVE),
        'û' => ('u', CIRCUMFLEX),
        'ü' => ('u', DIAERESIS),
        'ñ' => ('n', TILDE),
        'ç' => ('c', CEDILLA),
        'Ç' => ('C', CEDILLA),
        c if c.is_uppercase() => {
            let lower = c.to_lowercase().next()?;
            let (base, mark) = accented(lower)?;
            return Some((base.to_uppercase().next()?, mark));
        }
        _ => return None,
    };
    Some((base, mark))
}

/// The glyph for `c` in `font`: the game's, or an accented letter made
/// from it.
fn glyph_for(font: &Font, c: char) -> Option<Glyph> {
    let own = |c: char| {
        // Capital I is the font's small l.
        let c = if c == 'I' { 'l' } else { c };
        font.get(c.encode_utf8(&mut [0; 4]) as &str).map(|s| Glyph::from_shape(s))
    };
    if let Some(g) = own(c) {
        return Some(g);
    }
    let (base, mark) = accented(c)?;
    let mut g = own(base)?;
    if std::ptr::eq(mark, CEDILLA) {
        // Below the letter, over its bottom outline.
        let bottom = g.h - 1;
        g.mark(mark, bottom);
    } else if base == 'i' {
        // Dotless: the stem from the small letters' top, its outline above.
        let top = 5;
        for y in 0..top {
            for x in 0..g.w {
                g.texels[y * g.w + x] = 0;
            }
        }
        for x in 0..g.w {
            g.texels[top * g.w + x] = if g.at(x, top) != 0 { OUTLINE } else { 0 };
        }
        g.mark(mark, top - 2);
    } else if base.is_uppercase() {
        let mut t = g.taller(CAPITAL_MARK_ROWS);
        // The capital's top outline is two rows below the cell's top.
        t.mark(mark, CAPITAL_MARK_ROWS);
        g = t;
    } else {
        // Above the small letters' top outline (row 5).
        g.mark(mark, 3);
    }
    Some(g)
}

// ---- translated CAP files -----------------------------------------------

/// A CAP file with translated lines, as built (unrelocated), and the glyphs
/// it adds to its page: `rows` rows of 64 halfwords from row `top`.
struct Translated {
    words: Vec<u32>,
    top: u16,
    rows: u16,
    block: Vec<u16>,
}

/// Width of a 4-bit texture page, in texels; its height.
const PAGE: usize = 256;

/// A translated line's codes: `{_}` a narrow space, `{color N}`, `{XXXX}`
/// a control, `{gN}` glyph N; characters through `index_of`.
fn encode(text: &str, mut index_of: impl FnMut(char) -> u16) -> Vec<u16> {
    let mut out = Vec::new();
    let mut color = 0u16;
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        if c == '{'
            && let Some(end) = rest.find('}')
        {
            let token = &rest[1..end];
            rest = &rest[end + 1..];
            if token == "_" {
                out.push(0xFFFD);
            } else if let Some(n) = token.strip_prefix("color ") {
                color = n.trim().parse().unwrap_or(0);
            } else if let Some(n) = token.strip_prefix('g') {
                out.push(n.parse::<u16>().unwrap_or(0) & 0x3FF | color << 10);
            } else if let Ok(code) = u16::from_str_radix(token, 16) {
                out.push(code);
            }
            continue;
        }
        rest = &rest[c.len_utf8()..];
        match c {
            '\n' => out.push(0xFFFE),
            '\r' => {}
            c => out.push(index_of(c) | color << 10),
        }
    }
    out
}

fn build(cap: &Cap, lines: &HashMap<usize, String>, font: &Font) -> Option<Translated> {
    let originals = cap.lines();
    // The original glyph table: as long as its lines and speakers need.
    let used = originals.iter().flat_map(|l| {
        let glyphs = l.codes.iter().filter(|&&c| dialogue::is_glyph(c)).map(|&c| usize::from(c & 0x3FF));
        glyphs.chain((l.title != 0).then(|| usize::from(l.title - 1)))
    });
    let cells = used.max().map_or(0, |m| m + 1);
    let top = (0..cells)
        .map(|i| cap.cell(i))
        .filter(|c| c[2] != 0 && c[3] != 0)
        .map(|c| usize::from(c[1]) + usize::from(c[3]))
        .max()
        .unwrap_or(0)
        + 1;

    // The glyphs the translations draw, side by side in bands below it.
    let mut glyphs: Vec<Glyph> = Vec::new();
    let mut index: HashMap<char, u16> = HashMap::new();
    let mut texts: Vec<(usize, Vec<u16>)> = Vec::new();
    for line in &originals {
        let Some(text) = lines.get(&line.record) else { continue };
        let mut codes = encode(text, |c| {
            *index.entry(c).or_insert_with(|| {
                let g = glyph_for(font, c).or_else(|| glyph_for(font, '?')).unwrap_or(Glyph {
                    w: 1,
                    h: 1,
                    texels: vec![0],
                });
                glyphs.push(g);
                (cells + glyphs.len() - 1) as u16
            })
        });
        if line.codes.last() == Some(&0xFFFE) && codes.last() != Some(&0xFFFE) {
            codes.push(0xFFFE);
        }
        codes.push(0xFFFF);
        texts.push((line.record, codes));
    }
    if cells + glyphs.len() > 0x400 {
        return None;
    }
    let band = glyphs.iter().map(|g| g.h).max().unwrap_or(0);
    let mut placed = Vec::new();
    let (mut u, mut v) = (0, top);
    for g in &glyphs {
        if u + g.w > PAGE {
            (u, v) = (0, v + band);
        }
        placed.push((u, v + band - g.h));
        u += g.w;
    }
    let rows = if glyphs.is_empty() { 0 } else { v + band - top };
    if top + rows > PAGE {
        eprintln!("pe2: mods: a translation needs more glyphs than its page holds");
        return None;
    }
    let mut block = vec![0u16; 64 * rows];
    for (g, &(gu, gv)) in glyphs.iter().zip(&placed) {
        for y in 0..g.h {
            for x in 0..g.w {
                let (tu, tv) = (gu + x, gv - top + y);
                block[tv * 64 + tu / 4] |= u16::from(g.at(x, y)) << ((tu % 4) * 4);
            }
        }
    }

    // The file: the original, then the longer glyph table, then the texts.
    let mut bytes = cap.bytes[..cap.len()].to_vec();
    bytes.resize(bytes.len().next_multiple_of(4), 0);
    let table = bytes.len();
    for i in 0..cells {
        bytes.extend_from_slice(&cap.cell(i));
    }
    for (g, &(gu, gv)) in glyphs.iter().zip(&placed) {
        bytes.extend_from_slice(&[gu as u8, gv as u8, g.w as u8, g.h as u8]);
    }
    bytes[8..12].copy_from_slice(&(table as u32).to_le_bytes());
    for (record, codes) in texts {
        let at = bytes.len();
        bytes.extend(codes.iter().flat_map(|c| c.to_le_bytes()));
        bytes[record + 8..record + 12].copy_from_slice(&(at as u32).to_le_bytes());
    }
    bytes.resize(bytes.len().next_multiple_of(4), 0);
    let words = bytes.as_chunks::<4>().0.iter().map(|w| u32::from_le_bytes(*w)).collect();
    Some(Translated { words, top: top as u16, rows: rows as u16, block })
}

/// The translations given to the game: by the address of the CAP file it
/// loaded, the built file (shared by loads of the same file) and the copy
/// the game was given (relocated by it).
struct Live {
    translated: Arc<Translated>,
    copy: Box<[u32]>,
}

fn live() -> &'static Mutex<HashMap<usize, Live>> {
    static LIVE: OnceLock<Mutex<HashMap<usize, Live>>> = OnceLock::new();
    LIVE.get_or_init(Default::default)
}

fn built() -> &'static Mutex<HashMap<u64, Option<Arc<Translated>>>> {
    static BUILT: OnceLock<Mutex<HashMap<u64, Option<Arc<Translated>>>>> = OnceLock::new();
    BUILT.get_or_init(Default::default)
}

/// The CAP file the game should use for the one it loaded at `file`: a
/// translated copy, or `file` itself.
#[unsafe(no_mangle)]
pub extern "C" fn pc_cap_translate(file: *mut u8) -> *mut u8 {
    if translations().is_empty() || file.is_null() {
        return file;
    }
    let read = |at: usize| {
        // SAFETY: a CAP file's header, then its command table count, in the
        // bundle the game loaded.
        unsafe { file.add(at).cast::<u32>().read_unaligned() }
    };
    let mut live = live().lock().unwrap();
    // Loaded again where it already is, relocated: as given before.
    if read(8) as i32 <= 0 {
        return live.get(&(file as usize)).map_or(file, |l| l.copy.as_ptr() as *mut u8);
    }
    live.remove(&(file as usize));
    let commands = read(16) as usize;
    let len = commands + 4 + read(commands) as usize * 4;
    if len > 1 << 20 {
        return file;
    }
    // SAFETY: as above, `len` bytes of it.
    let bytes = unsafe { std::slice::from_raw_parts(file, len) };
    let Some(cap) = Cap::new(bytes) else { return file };
    let hash = cap.hash();
    let Some(lines) = translations().get(&hash) else { return file };
    let Some(font) = FONT.get() else {
        if crate::runtime::trace("text") {
            eprintln!("[text] {hash:016x}: the dialogue font is not collected yet");
        }
        return file;
    };
    let translated =
        built().lock().unwrap().entry(hash).or_insert_with(|| build(&cap, lines, font).map(Arc::new)).clone();
    let Some(translated) = translated else { return file };
    if crate::runtime::trace("text") {
        eprintln!("[text] {hash:016x}: {} lines translated, {} glyph rows", lines.len(), translated.rows);
    }
    let copy: Box<[u32]> = translated.words.clone().into_boxed_slice();
    let ptr = copy.as_ptr() as *mut u8;
    live.insert(file as usize, Live { translated, copy });
    ptr
}

/// Before the text of CAP file `file` is drawn from the glyph page at
/// (`x`, `y`): the glyphs its translation adds are put there, unless they
/// already are.
#[unsafe(no_mangle)]
pub extern "C" fn pc_cap_glyphs(file: *const u8, x: i32, y: i32) {
    if translations().is_empty() || file.is_null() {
        return;
    }
    let live = live().lock().unwrap();
    let Some(t) = live.values().find(|l| l.copy.as_ptr().cast::<u8>() == file).map(|l| l.translated.clone()) else {
        return;
    };
    drop(live);
    if t.rows == 0 {
        return;
    }
    use crate::runtime::gpu::{Rect, pcrt_LoadImage, pcrt_StoreImage};
    let rect = Rect::new(x as i16, (y + i32::from(t.top)) as i16, 64, t.rows as i16);
    let mut now = vec![0u16; t.block.len()];
    pcrt_StoreImage(&rect, now.as_mut_ptr());
    if now != t.block {
        pcrt_LoadImage(&rect, t.block.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tokens, colours and characters become codes.
    #[test]
    fn encodes_tokens() {
        let mut next = 0x100;
        let mut seen = HashMap::new();
        let codes = encode("Oi{_}{color 2}a\n{8101}Sim{g5}", |c| {
            *seen.entry(c).or_insert_with(|| {
                next += 1;
                next
            })
        });
        assert_eq!(
            codes,
            [
                0x101,
                0x102,
                0xFFFD,
                0x103 | 2 << 10,
                0xFFFE,
                0x8101,
                0x104 | 2 << 10,
                0x102 | 2 << 10,
                0x105 | 2 << 10,
                5 | 2 << 10
            ]
        );
    }

    /// A mark is drawn in the fill colour, outlined, centred.
    #[test]
    fn marks() {
        let mut g = Glyph { w: 7, h: 5, texels: vec![0; 35] };
        g.texels[4 * 7 + 1] = 2;
        g.texels[4 * 7 + 5] = 2;
        g.mark(&["#"], 1);
        assert_eq!(g.at(3, 1), FILL);
        assert_eq!(g.at(2, 0), OUTLINE);
        assert_eq!(g.at(4, 2), OUTLINE);
        assert_eq!(g.at(0, 0), 0);
    }
}
