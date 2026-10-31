//! Translations of the texts held in the game's code: menus, items, Parasite
//! Energy, places, options (docs/MODDING.md, "Texts of the menus").
//!
//! They are strings in the executable's and the code packages' data, in
//! the menus' font's encoding (ASCII, Latin-1 accents, `\` codes). The
//! extract lists them in `dumps/strings/<file>/<load address>.toml`, each
//! once; a mod's `strings/**/*.toml` gives them new text. Everything the
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
}

/// Original text (as the game stores it) -> translation, with its ending
/// zero, where the game reads it, and whether it uses letters the font
/// lacks (menuglyphs.rs).
type Table = HashMap<u64, (Box<[u8]>, Box<[u8]>, bool)>;

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

fn add(table: &mut Table, original: &[u8], text: &[u8]) {
    let mut stored = text.to_vec();
    stored.push(0);
    let added = text.iter().any(|&b| super::menuglyphs::is_added(b));
    table.insert(super::fnv1a(original.iter().copied()), (original.into(), stored.into(), added));
}

/// Adds a translation, and one for each line's rest of the text when the
/// translation has as many lines.
fn add_with_lines(table: &mut Table, original: &[u8], text: &[u8], what: &str) {
    let text = fit_lines(text, what);
    add(table, original, &text);
    let (from, to) = (line_starts(original), line_starts(&text));
    if from.len() != to.len() {
        if !from.is_empty() {
            eprintln!(
                "pe2: mods: strings: {what}: {} lines, the original {}: drawn from its first line only",
                to.len() + 1,
                from.len() + 1
            );
        }
        return;
    }
    for (&a, &b) in from.iter().zip(&to) {
        // An empty rest (a text ending with a line break) is any text's.
        if a < original.len() {
            add(table, &original[a..], &text[b..]);
        }
    }
}

/// Reads the enabled mods' translations (highest priority wins).
fn read() -> Table {
    let mut table = Table::new();
    if !super::enabled() {
        return table;
    }
    for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
        let mut stack = vec![m.dir.join("strings")];
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
                let file = match std::fs::read_to_string(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|t| toml::from_str::<StringsFile>(&t).map_err(|e| e.to_string()))
                {
                    Ok(f) => f,
                    Err(err) => {
                        eprintln!("pe2: mods: {}: {err}", path.display());
                        continue;
                    }
                };
                for entry in file.string.iter().filter(|e| !e.text.is_empty()) {
                    let what = format!("{}: {:?}", path.display(), entry.original.lines().next().unwrap_or(""));
                    add_with_lines(&mut table, &encode(&entry.original), &encode(&entry.text), &what);
                }
            }
        }
    }
    table
}

/// Reads the mods' translations, before the game draws any text.
pub fn load() {
    let table = TABLE.get_or_init(read);
    if !table.is_empty() {
        eprintln!("pe2: mods: {} texts of the menus translated", table.len());
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
    match table.get(&super::fnv1a(bytes.iter().copied())) {
        Some((original, translated, added)) if **original == *bytes => {
            if *added {
                super::menuglyphs::ensure();
            }
            translated.as_ptr()
        }
        _ => text,
    }
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

/// The texts in `data` (loaded at `base`): runs of the font's bytes ended
/// by a zero, starting at a word boundary after a zero (strings are
/// word-aligned in the game's data), that look like text.
pub fn scan(data: &[u8], base: u32) -> Vec<(u32, Vec<u8>)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < data.len() {
        if i % 4 != 0 || (i > 0 && data[i - 1] != 0) {
            i += 1;
            continue;
        }
        let end = data[i..].iter().position(|&b| !in_font(b)).map_or(data.len(), |n| i + n);
        if end < data.len() && data[end] == 0 && end - i >= 2 && looks_like_text(&data[i..end]) {
            out.push((base + i as u32, data[i..end].to_vec()));
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
/// once (where it is first found), into `out/<file>/<load address>.toml`.
/// Returns how many files and texts.
pub fn extract(
    executable: &[u8],
    packages: &HashMap<String, Vec<(u32, Vec<u8>)>>,
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
    let (mut seen, mut files, mut texts) = (std::collections::HashSet::new(), 0, 0);
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
        texts += found.len();
    }
    Ok((files, texts))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Texts are found word-aligned after a zero; code bytes are not.
    #[test]
    fn scan_finds_texts() {
        let mut data = b"Option\0\0Exit\0\0\0\0EkTkP\0\0\0Customize game settings.\0".to_vec();
        data.extend_from_slice(&[0, 0, 0, 0x10, 0x20, 0, 0]);
        data.extend_from_slice(b"On\0\0x2\0\0");
        let found: Vec<String> = scan(&data, 0x8000_0000).into_iter().map(|(_, s)| decode(&s)).collect();
        assert_eq!(found, ["Option", "Exit", "Customize game settings.", "On"]);
        assert_eq!(scan(&data, 0x8000_0000)[2].0, 0x8000_0018);
    }

    /// The translation of a text found by its contents, and of the rest of
    /// it from each line on.
    #[test]
    fn translates_fields() {
        let mut table = Table::new();
        add_with_lines(
            &mut table,
            b"PA3\nPump-action\nshotgun.\n\n",
            &encode("PA3\nEspingarda\nde repetição.\n\n"),
            "test",
        );
        let get = |s: &[u8]| table.get(&super::super::fnv1a(s.iter().copied())).map(|(_, t, _)| t.to_vec());
        assert_eq!(get(b"PA3\nPump-action\nshotgun.\n\n").unwrap(), b"PA3\nEspingarda\nde repeti\xE7\xE3o.\n\n\0");
        assert_eq!(get(b"shotgun.\n\n").unwrap(), b"de repeti\xE7\xE3o.\n\n\0");
        assert!(get(b"").is_none());
    }

    /// Typographic marks become the font's; a line too long is cut.
    #[test]
    fn encodes_for_the_font() {
        assert_eq!(encode("“Ação”… ok"), b"\"A\xE7\xE3o\"... ok");
        assert_eq!(fit_lines(&[b'a'; 70], "test").len(), LINE_BYTES);
    }
}
