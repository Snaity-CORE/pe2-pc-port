//! The menus' texts where the other release's code and data point: for each
//! text the game's `dumps/strings` lists, the other release's text in its
//! place (a translation rewrites the texts and repoints their users), or,
//! for a text nothing points at, between its neighbours. The other
//! release's bytes are read through its own menus' font: each code is the
//! character whose picture the game's font has in the same shape.
//!
//! A release built again (the European ones) has its own font codes, so
//! many letters read as unknown pictures (`{?<face>-<hash>}`): release.rs
//! reads those from that release's own texts. Also copies the pictures the
//! other release changed (maps, signs, the captions' font) into `files/`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

use fancy_regex::Regex;
use indexmap::IndexMap;

use crate::Result;
use crate::dumps::{FACES, Picture, Version, latin1, listed, lzss, read_toml, rel, toml_files, u32_at};
use crate::py;

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("regex")
}

static WORD: LazyLock<Regex> = LazyLock::new(|| re(r"[^\W\d_]+"));
// A mark inside a word: a code read in the wrong face (SELE#"O).
static ODD: LazyLock<Regex> = LazyLock::new(|| re(r"(?<=[^\W\d_])[^\w\s'.,!?&-]+(?=[^\W\d_])"));
// A capital after a small letter: likely the same (IfNS), though names
// have some (FateKeeper).
static CAMEL: LazyLock<Regex> = LazyLock::new(|| re(r"(?<=[^\W\d_A-ZÀ-Þ])[A-ZÀ-Þ]"));
// A small letter after two capitals (JOg, ARk): the same, in the small
// face, whose small letters' codes are its own syllables.
static SHOUTED: LazyLock<Regex> = LazyLock::new(|| re(r"(?<=[A-ZÀ-Þ]{2})[a-zß-ÿ]"));
// Marks that are letters read in the wrong face: a line ending in ; or '
// between capitals.
static STRAY: LazyLock<Regex> = LazyLock::new(|| re(r";\s*\n?\z|(?<=[A-ZÀ-Þ])'(?=[A-ZÀ-Þ])"));
// The font's codes (\B9, \Sm, \Co, \Z): not words.
static CODES: LazyLock<Regex> = LazyLock::new(|| re(r"\\[A-Za-z][A-Za-z0-9]?"));

/// The font's codes whose next byte is their operand (main/textdraw.c): the
/// backslash, the code and the operand are not letters to read.
const OPERAND_CODES: &[u8] = b"BCDSUWbcdsuw";

/// Texts the game draws in a face the reading cannot tell (no word known):
/// (file, first address, last address) -> face. The memory card menu's
/// save places (Nursery: a syllable a translation drew reads "io" in the
/// large face, "ci" in the medium).
const FACE_AT: [(&str, u32, u32, usize); 1] = [("exe/80010000.toml", 0x8006_7340, 0x8006_7410, 1)];

/// Pictures a translation changes that stay the game's: PE2PC redraws them
/// finer from the game's own (the menus' font, with the letters
/// translations add), or mods replace the game's (the title page, the
/// HUD's sheet).
const PICTURES_KEPT: [&str; 5] = [
    "stage0/file1/1.pe2img",
    "stage0/file20162/0.pe2img",
    "stage0/file1/3.pe2img",
    "stage0/file20162/2.pe2img",
    "stage0/file2/2.pe2img",
];

/// Where a text of the game was found translated: (its original, the file
/// it is listed in, its address, the other release's bytes, their reading).
type Seen = (String, String, u32, Vec<u8>, String);

/// A code package read: its file, the game's load address and bytes, the
/// other release's load address and bytes, and the addresses of the texts
/// the extract lists in it.
type Package = (String, u32, Vec<u8>, u32, Vec<u8>, HashSet<u32>);

/// The sizes a record of a table of names may have (bytes).
const RECORDS: std::ops::Range<usize> = 16..132;

/// A text found for one of the game's: what the import wrote into the mod
/// (`strings/`) for release.rs.
pub struct Found {
    pub original: String,
    pub text: String,
}

/// Bytes a text may hold besides ASCII (strings.rs `in_font` in PE2PC): the
/// menus' font's accented letters and marks.
fn in_font(b: u8) -> bool {
    matches!(b, 0x20..=0x7E | 0x0A | 0xA1 | 0xBF | 0xC0..=0xCF | 0xD1..=0xD6 | 0xD9..=0xDC | 0xDF | 0xE0..=0xEF | 0xF1..=0xF6 | 0xF9..=0xFC)
}

fn read_text(data: &[u8], offset: usize) -> &[u8] {
    let rest = data.get(offset..).unwrap_or_default();
    &rest[..rest.iter().position(|&b| b == 0).unwrap_or(rest.len())]
}

/// The address a `lui` and the `addiu` (sign-extended) or `ori` after it
/// put together.
fn address_of(lui: u32, low: u32) -> u32 {
    let mut imm = (low & 0xFFFF) as i64;
    if low >> 26 == 0x09 && imm & 0x8000 != 0 {
        imm -= 0x10000;
    }
    ((((lui & 0xFFFF) as i64) << 16) + imm) as u32
}

/// An instruction without its immediate or jump target: what stays the
/// same where another build of the code does the same thing.
fn shape(word: u32) -> u32 {
    if matches!(word >> 26, 0x02 | 0x03) { word >> 26 } else { word >> 16 }
}

/// A piece of code and data: its words and its `lui`s that a later
/// `addiu`/`ori` completes (the first such, before another `lui` sets the
/// register again): offset -> (that one's offset, the address).
struct References<'a> {
    data: &'a [u8],
    /// Each word's value -> its offsets, in order.
    by_value: HashMap<u32, Vec<usize>>,
    pairs: BTreeMap<usize, (usize, u32)>,
    /// Each address the pairs put together -> their `lui`s' offsets.
    by_address: HashMap<u32, Vec<usize>>,
}

impl<'a> References<'a> {
    fn new(data: &'a [u8]) -> References<'a> {
        let word = |i: usize| u32_at(data, i);
        let mut by_value: HashMap<u32, Vec<usize>> = HashMap::new();
        let mut pairs = BTreeMap::new();
        let mut by_address: HashMap<u32, Vec<usize>> = HashMap::new();
        let end = data.len().saturating_sub(3);
        for i in (0..end).step_by(4) {
            let lui = word(i);
            by_value.entry(lui).or_default().push(i);
            if lui >> 26 != 0x0F {
                continue;
            }
            let reg = (lui >> 16) & 31;
            for j in (i + 4..(i + 68).min(end)).step_by(4) {
                let w = word(j);
                let op = w >> 26;
                if matches!(op, 0x09 | 0x0D) && (w >> 21) & 31 == reg {
                    let address = address_of(lui, w);
                    pairs.insert(i, (j, address));
                    by_address.entry(address).or_default().push(i);
                    break;
                }
                if op == 0x0F && (w >> 16) & 31 == reg {
                    break;
                }
            }
        }
        References { data, by_value, pairs, by_address }
    }

    /// The `lui`s strictly between offsets `a` and `b`, or whose `addiu` is,
    /// in order.
    fn within(&self, a: usize, b: usize) -> Vec<usize> {
        self.pairs
            .range(a.saturating_sub(68)..b)
            .filter(|&(&p, &(q, _))| (a < p && p < b) || (a < q && q < b))
            .map(|(&p, _)| p)
            .collect()
    }
}

/// How another build of the code lines up with the game's: the offsets of
/// the instructions of the same shapes in the same order (difflib's
/// matching blocks), the game's offset -> its place there, the matched
/// offsets in order, and the other's references.
struct CodeMap<'a> {
    to_theirs: HashMap<usize, usize>,
    matched: Vec<usize>,
    theirs: References<'a>,
}

impl<'a> CodeMap<'a> {
    fn new(data: &[u8], theirs: &'a [u8]) -> CodeMap<'a> {
        let shapes = |d: &[u8]| (0..d.len() / 4).map(|k| shape(u32_at(d, 4 * k))).collect::<Vec<u32>>();
        let blocks = py::matching_blocks(&shapes(data), &shapes(theirs), false);
        let to_theirs: HashMap<usize, usize> =
            blocks.iter().flat_map(|&(a, b, n)| (0..n).map(move |k| (4 * (a + k), 4 * (b + k)))).collect();
        let mut matched: Vec<usize> = to_theirs.keys().copied().collect();
        matched.sort_unstable();
        CodeMap { to_theirs, matched, theirs: References::new(theirs) }
    }
}

/// One code package (or the executable) of the game and the other
/// release's of the same place, which may load elsewhere (`their_base`).
struct Place<'a> {
    rel: String,
    mine: References<'a>,
    their_base: u32,
    theirs: &'a [u8],
    listed: HashSet<u32>,
    code_map: std::cell::OnceCell<CodeMap<'a>>,
}

impl<'a> Place<'a> {
    fn new(rel: String, data: &'a [u8], their_base: u32, theirs: &'a [u8], listed: HashSet<u32>) -> Place<'a> {
        Place { rel, mine: References::new(data), their_base, theirs, listed, code_map: std::cell::OnceCell::new() }
    }

    /// What the other build's code puts together where the game's `lui` at
    /// `i` and its `addiu` at `j` are: those same instructions there, lined
    /// up (`CodeMap`); else, in the stretch of code that did not line up
    /// around them, the address its own `lui`s put together in the same
    /// order, when both stretches put together as many (another build may
    /// load a text into another register, once for two uses).
    fn pointed_there(&self, i: usize, j: usize) -> Option<u32> {
        let map = self.code_map.get_or_init(|| CodeMap::new(self.mine.data, self.theirs));
        if let (Some(&ti), Some(&tj)) = (map.to_theirs.get(&i), map.to_theirs.get(&j))
            && tj.wrapping_sub(ti) == j - i
        {
            return Some(address_of(u32_at(self.theirs, ti), u32_at(self.theirs, tj)));
        }
        let k = map.matched.partition_point(|&x| x <= i);
        let m = map.matched.partition_point(|&x| x <= j);
        if k == 0 || m == map.matched.len() {
            return None;
        }
        let (lo, hi) = (map.matched[k - 1], map.matched[m]);
        let mine = self.mine.within(lo, hi);
        let there = map.theirs.within(map.to_theirs[&lo], map.to_theirs[&hi]);
        if mine.len() != there.len() {
            return None;
        }
        let n = mine.iter().position(|&p| p == i)?;
        Some(map.theirs.pairs[&there[n]].1)
    }

    /// Where the other release keeps the text the game keeps at `at`: what
    /// the same words and instructions that point at it point at there (the
    /// most common answer), where a text starts.
    fn where_there(&self, at: u32) -> Option<u32> {
        let theirs = self.theirs;
        let mut votes: IndexMap<u32, usize> = IndexMap::new();
        for &offset in self.mine.by_value.get(&at).map_or(&[][..], Vec::as_slice) {
            if offset + 4 <= theirs.len() {
                *votes.entry(u32_at(theirs, offset)).or_default() += 1;
            }
        }
        for &i in self.mine.by_address.get(&at).map_or(&[][..], Vec::as_slice) {
            let (j, _) = self.mine.pairs[&i];
            if let Some(there) = self.pointed_there(i, j) {
                *votes.entry(there).or_default() += 1;
            }
        }
        // a text's start, as the game's (not into another's words): after a
        // 0, or a word that is no letters (a text right after the code: the
        // French build's "Objet trouvé !")
        let (base, len) = (self.their_base as u64, theirs.len() as u64);
        let good = votes.into_iter().filter(|&(a, _)| {
            if !(base <= a as u64 && (a as u64) < base + len) {
                return false;
            }
            let o = (a - self.their_base) as usize;
            o == 0
                || theirs[o - 1] == 0
                || (o.is_multiple_of(4) && !theirs[o - 4..o].iter().all(|&b| b != 0 && in_font(b)))
        });
        py::first_max_by(good, |&(_, n)| n).map(|(a, _)| a)
    }
}

/// Whether the text at `offset` of `data` is in a table of names in records
/// of a fixed size: three slots of one size or more in a row around it,
/// each a text padded with zeros (or empty), three of them texts; not texts
/// one after another.
fn in_records(data: &[u8], offset: usize) -> bool {
    let slot = |at: isize, size: usize| -> Option<bool> {
        let record =
            if at >= 0 && at as usize + size <= data.len() { &data[at as usize..at as usize + size] } else { &[][..] };
        let end = record.iter().position(|&b| b == 0)?;
        if record[end..].iter().any(|&b| b != 0) { None } else { Some(end > 0) }
    };
    for size in RECORDS.step_by(4) {
        let slots: Vec<Option<bool>> = (-4..=4).map(|k| slot(offset as isize + k * size as isize, size)).collect();
        if slots[4] != Some(true) {
            continue;
        }
        let lo = (0..=4).rev().find(|&k| k == 0 || slots[k - 1].is_none()).unwrap_or(0);
        let hi = (4..=8).find(|&k| k == 8 || slots[k + 1].is_none()).unwrap_or(8);
        if slots[lo..=hi].iter().filter(|t| **t == Some(true)).count() >= 3 {
            return true;
        }
    }
    false
}

/// The reading of the other release's bytes through its own font.
struct Reader {
    /// For each face: the game's pictures -> their letters.
    by_picture: Vec<HashMap<Picture, char>>,
    theirs: Vec<HashMap<u8, Picture>>,
    vocabulary: HashSet<String>,
    stems: HashSet<String>,
    alphabet: HashSet<char>,
    /// Pictures not known: their names -> (code, picture).
    unknown: std::cell::RefCell<BTreeMap<String, u8>>,
}

struct Reading {
    text: String,
    missing: Vec<(String, u8)>,
    labelled: usize,
}

impl Reader {
    /// `raw` read in face `face`.
    fn decode_in(&self, raw: &[u8], face: usize) -> Reading {
        let (mut out, mut missing) = (String::new(), Vec::new());
        let mut operands = 0;
        for (i, &b) in raw.iter().enumerate() {
            // A code's letter and operand, as they are.
            if operands > 0 {
                operands -= 1;
                out.push(b as char);
                continue;
            }
            if b == 0x5C && i + 1 < raw.len() {
                operands = if OPERAND_CODES.contains(&raw[i + 1]) { 2 } else { 1 };
                out.push(b as char);
                continue;
            }
            let picture = self.theirs[face].get(&b);
            let Some(picture) = picture.filter(|_| b >= 0x21 && b != 0x5C && !b"{|}~".contains(&b)) else {
                out.push(b as char);
                continue;
            };
            match self.by_picture[face].get(picture) {
                Some(&c) => out.push(c),
                None => {
                    let name = format!("{}-{}", FACES[face].0, py::short_hash(&py::rows_repr(picture)));
                    out.push_str(&format!("{{?{name}}}"));
                    missing.push((name, b));
                }
            }
        }
        Reading { text: out, missing, labelled: 0 }
    }

    /// The reading in `face` when given and it knows all the codes; else
    /// the one in the face that knows all its codes, with no mark inside a
    /// word and no letter the translation never writes, and the best share
    /// of words the translation knows (a word 1, only its stem 1/4), less
    /// 1/4 for each labelled picture and 1/2 for each capital after a small
    /// letter, small letter after two capitals or one-letter word not a
    /// vowel or stray mark; the first face on a tie.
    fn decode_line(&self, raw: &[u8], face: Option<usize>) -> String {
        if let Some(face) = face {
            let reading = self.decode_in(raw, face);
            if reading.missing.is_empty() {
                return reading.text;
            }
        }
        let readings: Vec<Reading> = (0..FACES.len()).map(|f| self.decode_in(raw, f)).collect();
        for reading in &readings {
            for (name, code) in &reading.missing {
                self.unknown.borrow_mut().entry(name.clone()).or_insert(*code);
            }
        }
        let count = |r: &Regex, t: &str| r.find_iter(t).count();
        let score = |reading: &&Reading| {
            let text = CODES.replace_all(&reading.text, " ").into_owned();
            let lower = text.to_lowercase();
            let words: Vec<&str> = WORD.find_iter(&lower).flatten().map(|m| m.as_str()).collect();
            let foreign = words.iter().flat_map(|w| w.chars()).filter(|c| !self.alphabet.contains(c)).count();
            let known: f64 = words
                .iter()
                .map(|w| {
                    let stem: String = w.chars().take(4).collect();
                    if self.vocabulary.contains(*w) {
                        1.0
                    } else if w.chars().count() >= 4 && self.stems.contains(&stem) {
                        0.25
                    } else {
                        0.0
                    }
                })
                .sum();
            // A word of one letter that is not a vowel.
            let lone = words.iter().filter(|w| w.chars().count() == 1 && !"aeiouáàâãéêíóôõú".contains(**w)).count();
            let odd_case = count(&CAMEL, &text) + count(&SHOUTED, &text) + lone + count(&STRAY, &text);
            let doubt = reading.labelled as f64 * 0.25 + odd_case as f64 * 0.5 - known / words.len().max(1) as f64;
            (reading.missing.len(), count(&ODD, &text), foreign, doubt)
        };
        py::first_min_by(readings.iter(), score).map(|r| r.text.clone()).unwrap_or_default()
    }

    /// Line by line: a text may change faces from one to the next.
    fn decode(&self, raw: &[u8], face: Option<usize>) -> String {
        raw.split(|&b| b == b'\n').map(|line| self.decode_line(line, face)).collect::<Vec<_>>().join("\n")
    }
}

/// Where each code package of an extract loads (files/load-addresses.txt:
/// its path -> its address), in the file's order.
fn load_addresses(dumps: &Path) -> Result<IndexMap<String, u32>> {
    let path = dumps.join("files").join("load-addresses.txt");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("{}: {e} (extract again with a newer PE2PC)", path.display()))?;
    Ok(text
        .lines()
        .filter_map(|line| {
            let (path, base) = line.rsplit_once(' ')?;
            Some((path.to_string(), u32::from_str_radix(base, 16).ok()?))
        })
        .collect())
}

fn has_letters(text: &str) -> usize {
    text.chars().filter(|c| c.is_alphabetic()).count()
}

/// The texts of the other release where its code points, for each of the
/// game's (`Found`), and the pictures it changed copied into the mod's
/// `files/`. Answers them and how many pictures.
pub fn import(game: &Version, other: &Version, out: &Path, report: &mut Vec<String>) -> Result<(Vec<Found>, usize)> {
    if game.same_code(other) {
        return Err(format!("{}: the game's own executable, not another release's", other.exe_name).into());
    }
    let (mine_fonts, their_fonts) = (game.fonts()?, other.fonts()?);
    let mut by_picture: Vec<HashMap<Picture, char>> = vec![HashMap::new(); FACES.len()];
    for (face, font) in mine_fonts.iter().enumerate() {
        let mut codes: Vec<&u8> = font.keys().collect();
        codes.sort();
        for &code in codes {
            let picture = &font[&code];
            if code >= 0x21 && !picture.is_empty() {
                by_picture[face].entry(picture.clone()).or_insert(code as char);
            }
        }
    }
    // The other release's words, from its dialogue: what tells which face a
    // text is read in when all know its codes.
    let mut vocabulary = HashSet::new();
    for path in toml_files(&other.dumps.join("text"), true) {
        let table = read_toml(&path)?;
        for line in table.get("line").and_then(|l| l.as_array()).into_iter().flatten() {
            if let Some(original) = line.get("original").and_then(|o| o.as_str()) {
                let lower = original.to_lowercase();
                vocabulary.extend(WORD.find_iter(&lower).flatten().map(|m| m.as_str().to_string()));
            }
        }
    }
    // Their first four letters too: a word the dialogue has in another form
    // (correr: corre, corremos).
    let stems = vocabulary.iter().filter(|w| w.chars().count() >= 4).map(|w| w.chars().take(4).collect()).collect();
    let alphabet = vocabulary.iter().flat_map(|w| w.chars()).collect();
    let reader = Reader { by_picture, theirs: their_fonts, vocabulary, stems, alphabet, unknown: Default::default() };

    let strings = game.dumps.join("strings");
    let lists = toml_files(&strings, true);
    let mut originals = HashSet::new();
    let mut listed_of = Vec::new();
    for path in &lists {
        let texts = listed(path)?;
        originals.extend(texts.iter().map(|t| t.original.clone()));
        listed_of.push(texts);
    }

    // The other release's text `raw` for `original` (at `at`), read, or
    // None (noted in the report when it is not a translation).
    let translation = |report: &mut Vec<String>, rel: &str, original: &str, at: u32, raw: &[u8]| -> Option<String> {
        // The libraries' messages to a debugger are not the game's.
        if original.contains('%') || original.contains("->") {
            return None;
        }
        // The same bytes: not translated (read in another face, it might
        // seem to be).
        if latin1(original).as_deref() == Some(raw) {
            return None;
        }
        let face = FACE_AT.iter().find(|(r, lo, hi, _)| *r == rel && (*lo..=*hi).contains(&at)).map(|f| f.3);
        let text = reader.decode(raw, face);
        if text == original || py::strip(&text).is_empty() {
            return None;
        }
        // Pointers there that lead elsewhere: no words, or another text.
        let wrong = has_letters(&text) < has_letters(original).min(2) || originals.contains(&text);
        if wrong || text.chars().any(|c| (c < '\x20' && c != '\n') || c == '\x7f') || text.contains('%') {
            report.push(format!(
                "{rel} 0x{at:08X}: not its text there, left out\n    here:  {original:?}\n    there: {text:?}"
            ));
            return None;
        }
        if text.matches('\n').count() != original.matches('\n').count() {
            report.push(format!(
                "{rel} 0x{at:08X}: another number of lines\n    here:  {original:?}\n    there: {text:?}"
            ));
        }
        Some(text)
    };

    // Where each package loads: another build's may load elsewhere.
    let (game_loads, their_loads) = (load_addresses(&game.dumps)?, load_addresses(&other.dumps)?);
    let mut found_texts = Vec::new();
    let mut seen: Vec<Seen> = Vec::new();
    // The code packages read, kept for the texts found again elsewhere.
    let mut packages: Vec<Package> = Vec::new();
    let newest = lists.iter().filter_map(|p| p.metadata().ok()?.modified().ok()).max();
    for (path, listed) in lists.iter().zip(&listed_of) {
        let modified = path.metadata().ok().and_then(|m| m.modified().ok());
        if let (Some(m), Some(n)) = (modified, newest)
            && m + std::time::Duration::from_secs(600) < n
        {
            continue; // left from an older extract
        }
        let rel = rel(&strings, path);
        let (key, name) = rel.rsplit_once('/').unwrap_or(("", &rel));
        let name = name.trim_end_matches(".toml");
        // PE2PC's own texts (pc/): on no disc.
        if listed.is_empty() || key == "pc" {
            continue;
        }
        if let Some(file) = name.strip_prefix("txt-") {
            // A text file of the disc (an item's details): the other
            // release's file of the same place.
            let there = other.dumps.join("files").join(key).join(format!("{file}.txt"));
            let Ok(data) = std::fs::read(&there) else {
                report.push(format!("{rel}: not found in the other release"));
                continue;
            };
            let raw: Vec<u8> =
                data.split(|&b| b == 0).next().unwrap_or_default().iter().copied().filter(|&b| b != b'\r').collect();
            let original = &listed[0].original;
            if latin1(original).as_deref() == Some(&raw[..]) {
                continue;
            }
            let text = reader.decode(&raw, None);
            if &text != original {
                found_texts.push(Found { original: original.clone(), text });
            }
            continue;
        }
        let Ok(base) = u32::from_str_radix(name, 16) else { continue };
        let first = latin1(&listed[0].original).unwrap_or_default();
        let first_at = listed[0].at.unwrap_or(base).wrapping_sub(base) as usize;
        let (data, theirs) = if key == "exe" {
            (game.exe.clone(), Some((other.exe_base, other.exe.clone())))
        } else {
            let theirs: HashMap<String, Vec<u8>> = other.packages_of(key).into_iter().collect();
            match game
                .packages_of(key)
                .into_iter()
                .find(|(_, d)| d.get(first_at..first_at + first.len()) == Some(&first[..]))
            {
                Some((n, d)) => {
                    let t = theirs.get(&n).cloned().zip(their_loads.get(&format!("{key}/{n}"))).map(|(t, &b)| (b, t));
                    (d, t)
                }
                None => (Vec::new(), None),
            }
        };
        let Some((their_base, theirs)) = theirs else {
            report.push(format!("{rel}: not found in the other release"));
            continue;
        };
        let ats: Vec<u32> = listed.iter().filter_map(|s| s.at).collect();
        let place = Place::new(rel.clone(), &data, their_base, &theirs, ats.iter().copied().collect());
        let mut found: HashMap<u32, u32> = HashMap::new();
        for &at in &ats {
            if let Some(there) = place.where_there(at) {
                found.insert(at, there);
            }
        }
        // Texts nothing points at: in order between found neighbours, when
        // the other release has one text there (in another build, its texts
        // there may be in another order).
        let mut i = 0;
        while i < ats.len() {
            if found.contains_key(&ats[i]) {
                i += 1;
                continue;
            }
            let mut j = i;
            while j < ats.len() && !found.contains_key(&ats[j]) {
                j += 1;
            }
            if 0 < i && j < ats.len() {
                let mut start = (found[&ats[i - 1]] - their_base) as usize;
                start += read_text(&theirs, start).len() + 1;
                let stop = (found[&ats[j]] - their_base) as usize;
                let (mut between, mut at) = (Vec::new(), start);
                while at < stop {
                    let t = read_text(&theirs, at);
                    if !t.is_empty() {
                        between.push(at);
                    }
                    at += t.len() + 1;
                }
                if between.len() == j - i && j - i == 1 {
                    found.insert(ats[i], their_base + between[0] as u32);
                }
            }
            i = j;
        }
        // Then where it was, if a text starts there, in tables of names in
        // records of a fixed size (translated in place: the map's rooms).
        for &at in &ats {
            let offset = at.wrapping_sub(base) as usize;
            if !in_records(&data, offset) || found.contains_key(&at) {
                continue;
            }
            if offset < theirs.len()
                && (offset == 0 || theirs[offset - 1] == 0)
                && !read_text(&theirs, offset).trim_ascii().is_empty()
            {
                found.insert(at, their_base + offset as u32);
            }
        }
        for s in listed {
            let Some(at) = s.at else { continue };
            let Some(&there) = found.get(&at) else { continue };
            let raw = read_text(&theirs, (there - their_base) as usize).to_vec();
            if let Some(text) = translation(report, &rel, &s.original, at, &raw) {
                seen.push((s.original.clone(), rel.clone(), at, raw, text));
            }
        }
        drop(place);
        packages.push((rel, base, data, their_base, theirs, ats.into_iter().collect()));
    }

    // The packages the extract listed no text of (each was listed where
    // first found): a room's copy of the shop's, say, translated otherwise
    // than the menus' same text. Where each loads is in load-addresses.txt.
    let mut known: HashSet<String> = packages.iter().map(|p| p.0.clone()).collect();
    for (path, &base) in &game_loads {
        let Some(&their_base) = their_loads.get(path) else { continue };
        let dir = path.rsplit_once('/').map_or("", |d| d.0);
        let rel = if dir.is_empty() { format!("{base:08X}.toml") } else { format!("{dir}/{base:08X}.toml") };
        if !known.insert(rel.clone()) {
            continue;
        }
        let data = lzss(&std::fs::read(game.dumps.join("files").join(path))?);
        let theirs = lzss(&std::fs::read(other.dumps.join("files").join(path))?);
        packages.push((rel, base, data, their_base, theirs, HashSet::new()));
    }

    // The same texts elsewhere (the extract lists each once), found as the
    // other release's code and data point at them.
    let by_bytes: HashMap<Vec<u8>, &String> = originals
        .iter()
        .filter(|o| !o.contains('%') && o.chars().count() >= 2)
        .filter_map(|o| Some((latin1(o)?, o)))
        .collect();
    for (rel, base, data, their_base, theirs, listed) in &packages {
        let place = Place::new(rel.clone(), data, *their_base, theirs, listed.clone());
        for offset in (0..data.len()).step_by(4) {
            if offset >= 4 && data[offset - 4..offset].iter().all(|&b| b != 0 && in_font(b)) {
                continue;
            }
            let Some(&original) = by_bytes.get(read_text(data, offset)) else { continue };
            let at = base + offset as u32;
            if place.listed.contains(&at) {
                continue;
            }
            let Some(there) = place.where_there(at) else { continue };
            let raw = read_text(theirs, (there - their_base) as usize).to_vec();
            if let Some(text) = translation(report, &place.rel, original, at, &raw) {
                seen.push((original.clone(), place.rel.clone(), at, raw, text));
            }
        }
    }

    // Each text's translation is the one most of its places have (the first
    // listed on a tie); a place translated otherwise gets its own, for there
    // only, unless its bytes read as the chosen one in some face.
    let mut places_of: IndexMap<&str, Vec<&Seen>> = IndexMap::new();
    for record in &seen {
        places_of.entry(record.0.as_str()).or_default().push(record);
    }
    for (original, records) in places_of {
        // Told apart by more than capitals: in the small face, which has
        // capitals only, small letters show as capitals anyway.
        let mut counts: IndexMap<String, usize> = IndexMap::new();
        for record in &records {
            *counts.entry(record.4.to_lowercase()).or_default() += 1;
        }
        let first = records[0].4.to_lowercase();
        let wanted =
            py::first_max_by(counts.iter(), |(t, n)| (**n, **t == first)).map(|(t, _)| t.clone()).unwrap_or_default();
        let variants: Vec<&String> = records.iter().map(|r| &r.4).filter(|t| t.to_lowercase() == wanted).collect();
        let main = variants.iter().find(|t| !py::is_upper(t)).unwrap_or(&variants[0]).to_string();
        let mut shown = false;
        for (_, _, _, raw, text) in &records {
            let same = *text == main || (0..FACES.len()).any(|f| reader.decode_in(raw, f).text == main);
            if same && shown {
                continue;
            }
            found_texts
                .push(Found { original: original.to_string(), text: if same { main.clone() } else { text.clone() } });
            shown = shown || same;
        }
    }

    for (name, code) in reader.unknown.borrow().iter() {
        report.push(format!("code 0x{code:02X}: picture {name} not known"));
    }
    let pictures = import_pictures(game, other, out)?;
    Ok((found_texts, pictures))
}

/// The pictures (`<file>/<chunk>.pe2img`) that are the dialogue's glyph
/// pages, from the extract's maps of them (dumps/textures-glyphs, kind
/// "dialogue"; a sheet is named by its file and its place among the file's
/// pictures).
fn dialogue_pages(dumps: &Path) -> Result<HashSet<String>> {
    let mut sheets = Vec::new();
    for path in toml_files(&dumps.join("textures-glyphs"), false) {
        let map = read_toml(&path)?;
        if map.get("kind").and_then(|k| k.as_str()) == Some("dialogue") {
            sheets.extend(
                map.get("sheets")
                    .and_then(|s| s.as_array())
                    .into_iter()
                    .flatten()
                    .filter_map(|s| s.as_str())
                    .map(str::to_string),
            );
        }
    }
    let files = dumps.join("files");
    let mut pages = HashSet::new();
    let mut todo = vec![files.clone()];
    while let Some(folder) = todo.pop() {
        let mut pictures = Vec::new();
        for entry in std::fs::read_dir(&folder)?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
            } else if let Some(n) = path.file_name().and_then(|n| n.to_str()).filter(|n| n.ends_with(".pe2img")) {
                pictures.push(n.to_string());
            }
        }
        pictures.sort_by_key(|n| n.split('.').next().and_then(|i| i.parse::<u64>().ok()).unwrap_or(0));
        let key = if folder == files { ".".to_string() } else { rel(&files, &folder) };
        for (index, picture) in pictures.iter().enumerate() {
            let prefix = format!("{}_{index}_", key.replace('/', "_"));
            if sheets.iter().any(|s| s.starts_with(&prefix)) {
                pages.insert(format!("{key}/{picture}"));
            }
        }
    }
    Ok(pages)
}

/// Copies into the mod's `files/` every picture (`.pe2img` chunk) the other
/// release changed (its words: maps, signs, the captions' font), but
/// PICTURES_KEPT and the dialogue's glyph pages (the game's lines left
/// untranslated draw from them by the game's codes). Answers how many.
fn import_pictures(game: &Version, other: &Version, out: &Path) -> Result<usize> {
    let kept = dialogue_pages(&game.dumps)?;
    let (ours, theirs) = (game.dumps.join("files"), other.dumps.join("files"));
    let mut copied = 0;
    let mut todo = vec![ours.clone()];
    while let Some(folder) = todo.pop() {
        for entry in std::fs::read_dir(&folder)?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
                continue;
            }
            if path.extension().is_none_or(|x| x != "pe2img") {
                continue;
            }
            let rel = rel(&ours, &path);
            let there = theirs.join(&rel);
            if PICTURES_KEPT.contains(&rel.as_str()) || kept.contains(&rel) || !there.exists() {
                continue;
            }
            let data = std::fs::read(&there)?;
            if data == std::fs::read(&path)? {
                continue;
            }
            let dest = out.join("files").join(&rel);
            std::fs::create_dir_all(dest.parent().expect("a folder"))?;
            std::fs::write(dest, data)?;
            copied += 1;
        }
    }
    Ok(copied)
}
