//! Translations of the texts held in the game's code: menus, items, Parasite
//! Energy, places, options (docs/MODDING.md, "Texts of the menus").
//!
//! They are strings in the executable's and the code packages' data, in
//! the menus' font's encoding (ASCII, Latin-1 accents, `\` codes). The
//! extract lists them in `dumps/strings/<file>/<load address>.toml`, each
//! once (and the disc's text files, items' details, in
//! `<file>/txt-<chunk>.toml`); a mod's `strings/**/*.toml` gives them new text. Everything the
//! game draws, measures or appends with the menus' font passes through
//! pc_text_translate first (the fork's textdraw.c and textutil.c), which
//! hands back the translation of a known original (found by its contents,
//! wherever it lies in memory), or the text itself.
//!
//! Some texts are fields, one per line (an item's name, its description's
//! lines, then the same for it unidentified), and the game draws them from
//! the middle of the string; so each line's rest of the text is known too,
//! as the same line onward of the translation, which therefore keeps the
//! original's number of lines.

use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

use serde::Deserialize;

/// Bytes a line of text may hold: the game copies lines into 64-byte
/// buffers (textutil.c TEXT_UI_LINE_BUFFER_BYTES), with their ending zero.
const LINE_BYTES: usize = 63;

/// The longest text looked up; longer ones are not the game's.
const MAX_TEXT: usize = 4096;

/// A translation file (other fields, the original's address, are for people).
#[derive(Deserialize)]
struct StringsFile {
    #[serde(default)]
    string: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    original: String,
    #[serde(default)]
    text: String,
    /// Where the original is (in RAM, as the extract lists it).
    at: Option<u32>,
    /// The translation is for the original there only (the same text
    /// elsewhere is translated otherwise, or not).
    #[serde(default)]
    here_only: bool,
}

/// An original text (as the game stores it), its translation with its
/// ending zero, where the game reads it, and whether that uses letters the
/// font lacks (menuglyphs.rs).
type Translation = (Box<[u8]>, Box<[u8]>, bool);

/// The translations: by the original's contents (its hash), and those of
/// one place by the original's address.
#[derive(Default)]
struct Table {
    by_text: HashMap<u64, Translation>,
    by_place: HashMap<u32, Translation>,
}

impl Table {
    fn is_empty(&self) -> bool {
        self.by_text.is_empty() && self.by_place.is_empty()
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

/// Text as the game stores it: Latin-1 (the font's accents), typographic
/// quotes and dashes made plain, anything else a `?`.
pub fn encode(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\u{2018}' | '\u{2019}' => out.push(b'\''),
            '\u{201C}' | '\u{201D}' => out.push(b'"'),
            '\u{2013}' | '\u{2014}' => out.push(b'-'),
            '\u{2026}' => out.extend_from_slice(b"..."),
            c if (c as u32) < 0x100 => out.push(c as u32 as u8),
            _ => out.push(b'?'),
        }
    }
    out
}

/// Text as people read it: each byte its Latin-1 character.
pub fn decode(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}

/// Where each line after the first starts: after a line break, or after
/// the `\n` and `\N` codes that also end one.
fn line_starts(text: &[u8]) -> Vec<usize> {
    (0..text.len())
        .filter(|&i| text[i] == b'\n' || (i > 0 && matches!(text[i], b'n' | b'N') && text[i - 1] == b'\\'))
        .map(|i| i + 1)
        .collect()
}

/// `text` with lines longer than the game's buffers hold cut short.
fn fit_lines(text: &[u8], what: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for (n, line) in text.split(|&b| b == b'\n').enumerate() {
        if n > 0 {
            out.push(b'\n');
        }
        if line.len() > LINE_BYTES {
            eprintln!("pe2: mods: strings: {what}: a line longer than {LINE_BYTES} characters, cut");
        }
        out.extend_from_slice(&line[..line.len().min(LINE_BYTES)]);
    }
    out
}

fn translation(original: &[u8], text: &[u8]) -> Translation {
    let mut stored = text.to_vec();
    stored.push(0);
    let added = text.iter().any(|&b| super::menuglyphs::is_added(b));
    (original.into(), stored.into(), added)
}

/// A translation, and one for each line's rest of the text when the
/// translation has as many lines: (where in the original it starts, the
/// original from there with its translation).
fn with_lines(original: &[u8], text: &[u8], what: &str) -> Vec<(usize, Translation)> {
    let text = fit_lines(text, what);
    let mut out = vec![(0, translation(original, &text))];
    let (from, to) = (line_starts(original), line_starts(&text));
    if from.len() != to.len() {
        if !from.is_empty() {
            eprintln!(
                "pe2: mods: strings: {what}: {} lines, the original {}: drawn from its first line only",
                to.len() + 1,
                from.len() + 1
            );
        }
        return out;
    }
    for (&a, &b) in from.iter().zip(&to) {
        // An empty rest (a text ending with a line break) is any text's.
        if a < original.len() {
            out.push((a, translation(&original[a..], &text[b..])));
        }
    }
    out
}

/// The enabled mods' translation files, lowest priority first (so the
/// highest wins): (the file's path under `strings/`, with `/`, and its
/// contents).
fn mod_files() -> Vec<(String, StringsFile)> {
    let mut out = Vec::new();
    if !super::enabled() {
        return out;
    }
    for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
        let root = m.dir.join("strings");
        let mut stack = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for e in entries.flatten() {
                let path = e.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|x| x != "toml") {
                    continue;
                }
                match std::fs::read_to_string(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|t| toml::from_str::<StringsFile>(&t).map_err(|e| e.to_string()))
                {
                    Ok(f) => {
                        let rel = path.strip_prefix(&root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
                        out.push((rel, f));
                    }
                    Err(err) => eprintln!("pe2: mods: {}: {err}", path.display()),
                }
            }
        }
    }
    out
}

/// A translation file for one of the disc's text files: its file key and
/// chunk (`<key>/txt-<chunk>.toml`).
fn text_file(rel: &str) -> Option<(&str, usize)> {
    let (key, name) = rel.rsplit_once('/')?;
    Some((key, name.strip_prefix("txt-")?.strip_suffix(".toml")?.parse().ok()?))
}

/// Reads the enabled mods' translations of the game's code's texts.
fn read() -> Table {
    let mut table = Table::default();
    for (rel, file) in mod_files() {
        if text_file(&rel).is_some() {
            continue;
        }
        for entry in file.string.iter().filter(|e| !e.text.is_empty()) {
            let what = format!("strings/{rel}: {:?}", entry.original.lines().next().unwrap_or(""));
            let original: Vec<u8> = encode(&entry.original).into_iter().filter(|&b| b != b'\r').collect();
            for (offset, t) in with_lines(&original, &encode(&entry.text), &what) {
                match entry.at {
                    Some(at) if entry.here_only => _ = table.by_place.insert(at + offset as u32, t),
                    _ => _ = table.by_text.insert(super::fnv1a(t.0.iter().copied()), t),
                }
            }
        }
    }
    table
}

/// The disc's text files the mods translate (files.rs serves them in place
/// of the disc's): file key, chunk, and the chunk's new contents (lines
/// ended by CR LF as the disc's are, then a zero). The game reads a whole
/// file, so a translation may have any number of lines.
pub fn file_texts() -> Vec<(String, usize, Vec<u8>)> {
    let mut out: Vec<(String, usize, Vec<u8>)> = Vec::new();
    for (rel, file) in mod_files() {
        let Some((key, chunk)) = text_file(&rel) else { continue };
        let Some(entry) = file.string.iter().find(|e| !e.text.is_empty()) else { continue };
        let text = fit_lines(&encode(&entry.text), &format!("strings/{rel}"));
        let mut bytes = Vec::with_capacity(text.len() + text.len() / 16 + 1);
        for &b in &text {
            if b == b'\n' {
                bytes.push(b'\r');
            }
            bytes.push(b);
        }
        bytes.push(0);
        out.retain(|(k, c, _)| (k.as_str(), *c) != (key, chunk));
        out.push((key.to_string(), chunk, bytes));
    }
    out
}

/// Reads the mods' translations, before the game draws any text.
pub fn load() {
    let table = TABLE.get_or_init(read);
    if !table.is_empty() {
        eprintln!(
            "pe2: mods: {} texts of the menus translated, {} of them in one place only",
            table.by_text.len() + table.by_place.len(),
            table.by_place.len()
        );
    }
}

/// The text the game should use for `text`: its translation, or itself.
#[unsafe(no_mangle)]
pub extern "C" fn pc_text_translate(text: *const u8) -> *const u8 {
    // Neither logs, locks nor allocates (but for the added letters, under
    // the IRQ guard): safe wherever the game is stopped.
    let Some(table) = TABLE.get().filter(|t| !t.is_empty()) else { return text };
    if text.is_null() {
        return text;
    }
    let mut len = 0;
    // SAFETY: the game's text, ended by a zero (read no further than that,
    // and than MAX_TEXT).
    while len < MAX_TEXT && unsafe { *text.add(len) } != 0 {
        len += 1;
    }
    // SAFETY: the `len` bytes just read.
    let bytes = unsafe { std::slice::from_raw_parts(text, len) };
    // Line ends of the disc's text files are CR LF; the game skips the CRs.
    let lines = || bytes.iter().copied().filter(|&b| b != b'\r');
    // This place's own translation first, then the text's.
    let found = table
        .by_place
        .get(&(text as usize as u32))
        .filter(|t| t.0.iter().copied().eq(lines()))
        .or_else(|| table.by_text.get(&super::fnv1a(lines())).filter(|t| t.0.iter().copied().eq(lines())));
    match found {
        Some((_, translated, added)) => {
            if *added {
                super::menuglyphs::ensure();
            }
            translated.as_ptr()
        }
        _ => text,
    }
}

/// The last code the menus' small face has a glyph for (textdraw.c
/// FONT_GLYPH_SMALL_COUNT): past it, the game would read other data as
/// glyphs.
const SMALL_LAST: u8 = b'z';

/// A Latin-1 letter without its accent (anything else past the small
/// face's codes a `?`).
fn plain(b: u8) -> u8 {
    match b {
        0xC0..=0xC5 => b'A',
        0xC7 => b'C',
        0xC8..=0xCB => b'E',
        0xCC..=0xCF => b'I',
        0xD1 => b'N',
        0xD2..=0xD6 => b'O',
        0xD9..=0xDC => b'U',
        0xE0..=0xE5 => b'a',
        0xE7 => b'c',
        0xE8..=0xEB => b'e',
        0xEC..=0xEF => b'i',
        0xF1 => b'n',
        0xF2..=0xF6 => b'o',
        0xF9..=0xFC => b'u',
        b if b <= SMALL_LAST => b,
        _ => b'?',
    }
}

/// The text to draw in a face of the menus' font: `text`, or for the small
/// face, when a translation put letters there it has no glyphs for, a copy
/// with them made plain (SELEÇÃO drawn as SELECAO, not as other data).
#[unsafe(no_mangle)]
pub extern "C" fn pc_text_for_face(text: *const u8, small: i32) -> *const u8 {
    if small == 0 || text.is_null() || TABLE.get().is_none_or(|t| t.is_empty()) {
        return text;
    }
    let mut len = 0;
    // SAFETY: as in pc_text_translate.
    while len < MAX_TEXT && unsafe { *text.add(len) } != 0 {
        len += 1;
    }
    // SAFETY: the `len` bytes just read.
    let bytes = unsafe { std::slice::from_raw_parts(text, len) };
    if bytes.iter().all(|&b| b <= SMALL_LAST) {
        return text;
    }
    // Made once for each text, kept for as long as the game runs.
    let _g = crate::runtime::irq::guard();
    type Made = std::sync::Mutex<HashMap<Vec<u8>, Box<[u8]>>>;
    static PLAIN: OnceLock<Made> = OnceLock::new();
    let mut made = PLAIN.get_or_init(Default::default).lock().unwrap();
    made.entry(bytes.to_vec()).or_insert_with(|| bytes.iter().map(|&b| plain(b)).chain([0]).collect()).as_ptr()
}

/// Bytes a string of the game's may hold besides ASCII: the accented
/// letters and the marks the menus' font has (textdraw.c's glyph tables).
fn in_font(b: u8) -> bool {
    matches!(b, 0x20..=0x7E | b'\n' | 0xA1 | 0xBF | 0xC0..=0xCF | 0xD1..=0xD6 | 0xD9..=0xDC | 0xDF | 0xE0..=0xEF | 0xF1..=0xF6 | 0xF9..=0xFC)
}

/// Whether `s`, a run of the font's bytes, reads as text rather than code
/// or numbers that happen to be printable: a word of three letters (or a
/// capitalized two-letter one alone: On, OK, HP), few accented bytes,
/// mostly letters and spaces; short ones a plain word.
fn looks_like_text(s: &[u8]) -> bool {
    let two = s.len() == 2 && s[0].is_ascii_uppercase() && s[1].is_ascii_alphabetic();
    let ascii_word = two || s.windows(3).any(|w| w.iter().all(u8::is_ascii_alphabetic));
    let high = s.iter().filter(|&&b| b > 0x7E).count();
    let letters = s.iter().filter(|&&b| char::from(b).is_alphabetic() || b == b' ').count();
    if !ascii_word || high * 5 > s.len() || letters * 10 < s.len() * 6 {
        return false;
    }
    if s.len() >= 8 {
        return true;
    }
    // A short one: letters and a few marks, with no capital after a small
    // letter in a word (code bytes look like "EkTkP", "lZlZ").
    s.iter().all(|&b| b.is_ascii_alphabetic() || b" .'-&!?:#".contains(&b))
        && !s.windows(2).any(|w| w[0].is_ascii_lowercase() && w[1].is_ascii_uppercase())
}

/// The addresses `data` (loaded at `base`) refers to: its words, and what
/// its code's `lui` + `addiu` (or `ori`) pairs put together.
pub fn references(data: &[u8]) -> std::collections::HashSet<u32> {
    let words: Vec<u32> = data.as_chunks::<4>().0.iter().map(|w| u32::from_le_bytes(*w)).collect();
    let mut out: std::collections::HashSet<u32> = words.iter().copied().collect();
    for (i, &lui) in words.iter().enumerate() {
        if lui >> 26 != 0x0F {
            continue;
        }
        let (reg, high) = ((lui >> 16) & 31, lui << 16);
        for &next in words.iter().skip(i + 1).take(16) {
            let op = next >> 26;
            if (op == 0x09 || op == 0x0D) && (next >> 21) & 31 == reg {
                let low = next & 0xFFFF;
                out.insert(if op == 0x09 { high.wrapping_add(low as u16 as i16 as u32) } else { high | low });
            }
        }
    }
    out
}

/// The texts in `data` (loaded at `base`): runs of the font's bytes ended
/// by a zero, starting at a word boundary after a byte no text has (a
/// zero, or a pointer's or code's: strings are word-aligned in the game's
/// data, and some follow a table), that look like text; two letters
/// only where the code or data refer to them, among other texts (two
/// random bytes often read as letters).
pub fn scan(data: &[u8], base: u32) -> Vec<(u32, Vec<u8>)> {
    let found = scan_all(data, base);
    let near_text = |at: u32| {
        found.iter().any(|(a, s)| s.len() > 2 && (a.abs_diff(at) <= 16 || (a + s.len() as u32).abs_diff(at) <= 16))
    };
    found.iter().filter(|(at, s)| s.len() > 2 || near_text(*at)).cloned().collect()
}

fn scan_all(data: &[u8], base: u32) -> Vec<(u32, Vec<u8>)> {
    let mut out = Vec::new();
    let mut referred: Option<std::collections::HashSet<u32>> = None;
    let mut i = 0;
    while i < data.len() {
        if i % 4 != 0 || (i > 0 && data[i - 1] != 0 && in_font(data[i - 1])) {
            i += 1;
            continue;
        }
        let end = data[i..].iter().position(|&b| !in_font(b)).map_or(data.len(), |n| i + n);
        let at = base + i as u32;
        if end < data.len()
            && data[end] == 0
            && end - i >= 2
            && looks_like_text(&data[i..end])
            && (end - i > 2 || referred.get_or_insert_with(|| references(data)).contains(&at))
        {
            out.push((at, data[i..end].to_vec()));
            i = (end + 4) & !3;
        } else {
            i += 4;
        }
    }
    out
}

fn literal(text: &str) -> String {
    if text.contains("'''") { format!("{text:?}") } else { format!("'''\n{text}'''") }
}

/// Writes the texts of the executable and of every code package, each text
/// once (where it is first found), into `out/<file>/<load address>.toml`,
/// and the disc's text files (`texts`: file key, chunk, contents; items'
/// details) into `out/<file>/txt-<chunk>.toml`. Returns how many files and
/// texts.
pub fn extract(
    executable: &[u8],
    packages: &HashMap<String, Vec<(u32, Vec<u8>)>>,
    texts: &[(String, usize, Vec<u8>)],
    out: &Path,
) -> std::io::Result<(usize, usize)> {
    let mut sources: Vec<(String, u32, Vec<u8>)> = Vec::new();
    if executable.get(..8) == Some(b"PS-X EXE") && executable.len() > 0x800 {
        let base = u32::from_le_bytes(executable[0x18..0x1C].try_into().unwrap());
        sources.push(("exe".into(), base, executable[0x800..].to_vec()));
    }
    let mut keys: Vec<&String> = packages.keys().collect();
    keys.sort();
    for key in keys {
        for (base, payload) in &packages[key] {
            sources.push((key.clone(), *base, super::dialogue::lzss(payload, &mut 0)));
        }
    }
    let (mut seen, mut files, mut count) = (std::collections::HashSet::new(), 0, 0);
    for (key, base, data) in &sources {
        let found: Vec<_> = scan(data, *base).into_iter().filter(|(_, s)| seen.insert(s.clone())).collect();
        if found.is_empty() {
            continue;
        }
        let what = if key == "exe" { "the executable".to_string() } else { key.clone() };
        let mut toml = format!(
            "# Texts of the game's code: {what}, loaded at 0x{base:08X}.\n\
             # Write each one's translation in `text`; an empty one keeps the original.\n\
             # Keep the `\\` codes and the number of lines; a line holds up to {LINE_BYTES} characters.\n"
        );
        for (at, s) in &found {
            toml.push_str(&format!("\n[[string]]\nat = 0x{at:08X}\noriginal = {}\ntext = ''\n", literal(&decode(s))));
        }
        let path = out.join(key).join(format!("{base:08X}.toml"));
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, toml)?;
        files += 1;
        count += found.len();
    }
    for (key, chunk, data) in texts {
        let text: Vec<u8> = data.iter().copied().take_while(|&b| b != 0).filter(|&b| b != b'\r').collect();
        if !text.iter().any(u8::is_ascii_alphabetic) || !seen.insert(text.clone()) {
            continue;
        }
        let toml = format!(
            "# Text of the game's file {key}, chunk {chunk} (an item's details: lines after the first five).\n\
             # Write its translation in `text`; an empty one keeps the original.\n\
             # Keep the `\\` codes and the number of lines; a line holds up to {LINE_BYTES} characters.\n\
             \n[[string]]\noriginal = {}\ntext = ''\n",
            literal(&decode(&text))
        );
        let path = out.join(key).join(format!("txt-{chunk}.toml"));
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, toml)?;
        files += 1;
        count += 1;
    }
    Ok((files, count))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Texts are found word-aligned after a zero; code bytes are not.
    #[test]
    fn scan_finds_texts() {
        let mut data = b"Option\0\0Exit\0\0\0\0EkTkP\0\0\0Customize game settings.\0".to_vec();
        data.extend_from_slice(&[0, 0, 0, 0x10, 0x20, 0, 0]);
        data.extend_from_slice(b"On\0\0Bq\0\0x2\0\0");
        // A word referring to "On" (at 0x38); none to "Bq".
        data.extend_from_slice(&0x8000_0038u32.to_le_bytes());
        let found: Vec<String> = scan(&data, 0x8000_0000).into_iter().map(|(_, s)| decode(&s)).collect();
        assert_eq!(found, ["Option", "Exit", "Customize game settings.", "On"]);
        assert_eq!(scan(&data, 0x8000_0000)[2].0, 0x8000_0018);
    }

    /// The translation of a text found by its contents, and of the rest of
    /// it from each line on.
    #[test]
    fn translates_fields() {
        let pieces =
            with_lines(b"PA3\nPump-action\nshotgun.\n\n", &encode("PA3\nEspingarda\nde repetição.\n\n"), "test");
        assert_eq!(pieces.iter().map(|p| p.0).collect::<Vec<_>>(), [0, 4, 16, 25]);
        let get = |s: &[u8]| pieces.iter().find(|p| *p.1.0 == *s).map(|p| p.1.1.to_vec());
        assert_eq!(get(b"PA3\nPump-action\nshotgun.\n\n").unwrap(), b"PA3\nEspingarda\nde repeti\xE7\xE3o.\n\n\0");
        assert_eq!(get(b"shotgun.\n\n").unwrap(), b"de repeti\xE7\xE3o.\n\n\0");
        assert!(get(b"").is_none());
    }

    /// The small face gets plain letters.
    #[test]
    fn small_face_plain() {
        let latin: Vec<u8> = encode("SELEÇÃO {x}").iter().map(|&b| plain(b)).collect();
        assert_eq!(latin, b"SELECAO ?x?");
    }

    /// Typographic marks become the font's; a line too long is cut.
    #[test]
    fn encodes_for_the_font() {
        assert_eq!(encode("“Ação”… ok"), b"\"A\xE7\xE3o\"... ok");
        assert_eq!(fit_lines(&[b'a'; 70], "test").len(), LINE_BYTES);
    }
}
