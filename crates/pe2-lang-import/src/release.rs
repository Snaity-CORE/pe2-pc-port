//! The menus' texts of another release whose code was built again (the
//! European ones), from what strings.rs found where its code points:
//!
//! - for the texts of one line (places, menus, messages): what strings.rs
//!   found, its letters it could not read (`{?<face>-<hash>}`) read from
//!   that release's own texts (its dumps/strings, read by the extract): the
//!   one text there that matches the letters read, each unread one standing
//!   for one to four characters;
//! - for the texts of several lines (items, Parasite Energy: a name, then
//!   its description): each code file's texts paired in order, as both
//!   releases were built from the same sources (Needleman-Wunsch on their
//!   `\` codes, numbers, pad buttons, number of lines and texts left as
//!   they were), keeping the pairs whose names share a model or number
//!   (M4A1, PA3, 40mm) or read the same;
//! - for the items' details (their text files on the discs): the other
//!   release's file of the same name, whole;
//! - the loading captions (`captions.toml`, PE2PC's mods/load_captions.rs).
//!
//! Writes the mod's `strings/` and `captions.toml`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use indexmap::IndexMap;
use regex::Regex;

use crate::Result;
use crate::dumps::{Listed, Version, listed, literal, rel, toml_files, u32_at};
use crate::py;
use crate::strings::Found;

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("regex")
}

static CODE: LazyLock<Regex> = LazyLock::new(|| re(r"\\[A-Za-z0-9]{1,3}|%[-0-9.]*[a-zA-Z]"));
static DIGITS: LazyLock<Regex> = LazyLock::new(|| re(r"\d+"));
static BUTTONS: LazyLock<Regex> = LazyLock::new(|| re(r"[{}~|]"));
static UNREAD: LazyLock<Regex> = LazyLock::new(|| re(r"\{\?[^}]*\}"));
static MODEL: LazyLock<Regex> = LazyLock::new(|| re(r"[A-Za-z]*\d[\w.]*|\b[A-Z]{2,}\b"));
static END_MARK: LazyLock<Regex> = LazyLock::new(|| re(r"[.!?:]?\z"));
static ACRONYM: LazyLock<Regex> = LazyLock::new(|| re(r"\b[A-Z]{2,}\b"));
static BREAK: LazyLock<Regex> = LazyLock::new(|| re(r"\n|\\[nN]"));
const GAP: i32 = -2;

/// A pair where the other release's code points, against one the
/// alignment gives (which only tells pointed pairs apart: a shifted run of
/// short texts misleads it).
const POINTED: u32 = 10;

fn all<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    r.find_iter(text).map(|m| m.as_str()).collect()
}

/// A record of fields, one per line (an item: its name, its description's
/// lines), against a text of one line.
fn several_lines(text: &str) -> bool {
    text.matches('\n').count() > 1
}

/// An item's name: a text's first line.
fn name(text: &str) -> &str {
    text.trim_matches('\n').split('\n').next().unwrap_or_default()
}

struct Features<'a> {
    text: &'a str,
    codes: Vec<&'a str>,
    digits: Vec<&'a str>,
    buttons: Vec<&'a str>,
    lines: usize,
    length: usize,
}

fn features<'a>(texts: &[&'a Listed]) -> Vec<Features<'a>> {
    texts
        .iter()
        .map(|t| {
            let o = t.original.as_str();
            Features {
                text: o,
                codes: all(&CODE, o),
                digits: all(&DIGITS, o),
                buttons: all(&BUTTONS, o),
                lines: o.matches('\n').count(),
                length: o.chars().count(),
            }
        })
        .collect()
}

fn similarity(a: &Features, b: &Features) -> i32 {
    if a.text == b.text {
        return 6;
    }
    let mut score = if a.codes == b.codes { 3 } else { -3 };
    score += if !a.digits.is_empty() && a.digits == b.digits {
        2
    } else if a.digits != b.digits {
        -1
    } else {
        0
    };
    score += if !a.buttons.is_empty() && a.buttons == b.buttons {
        2
    } else if a.buttons != b.buttons {
        -2
    } else {
        0
    };
    score += if a.lines == b.lines { 1 } else { -1 };
    let ratio = b.length as f64 / a.length.max(1) as f64;
    score += if (0.6..=2.2).contains(&ratio) { 1 } else { -1 };
    score
}

/// Whether an item's name and its translation share a model or number
/// (M4A1, PA3, 40mm), or read the same.
fn names_agree(mine: &str, theirs: &str) -> bool {
    let (a, b) = (name(mine), name(theirs));
    if a == b {
        return true;
    }
    let models: HashSet<&str> = all(&MODEL, a).into_iter().collect();
    all(&MODEL, b).iter().any(|m| models.contains(m))
}

/// Runs of records of the same name: an item's, or a Parasite Energy's
/// levels, each its own record.
fn groups<'a>(records: &[&'a Listed]) -> Vec<Vec<&'a Listed>> {
    let mut out: Vec<Vec<&Listed>> = Vec::new();
    for &r in records {
        match out.last_mut() {
            Some(run) if name(&run[run.len() - 1].original) == name(&r.original) => run.push(r),
            _ => out.push(vec![r]),
        }
    }
    out
}

fn at_of(record: &Listed) -> u32 {
    record.at.unwrap_or_default()
}

/// The records of several lines (items, Parasite Energy) of a code file,
/// paired: the extract lists each text once, where first found, so a
/// release whose levels of a Parasite Energy read the same in places lists
/// fewer of them. Aligns the runs of records of one name, then pairs a
/// run's records in order; a level the other release lists no text for
/// keeps the game's.
fn pair_records<'a>(
    mine: &[&'a Listed],
    theirs: &[&'a Listed],
    report: &mut Vec<String>,
    key: &str,
) -> Vec<(&'a Listed, &'a Listed)> {
    // Two runs' names: the same 4, sharing a model 3, looking alike 2.
    let name_score = |x: &[&Listed], y: &[&Listed]| {
        let (a, b) = (&x[0].original, &y[0].original);
        if name(a) == name(b) {
            4
        } else if names_agree(a, b) {
            3
        } else if py::ratio(&name(a).to_lowercase(), &name(b).to_lowercase()) >= 0.5 {
            2
        } else {
            0
        }
    };
    let (ga, gb) = (groups(mine), groups(theirs));
    // in order, a run is likelier its counterpart than a gap (the 1)
    let steps = py::align(ga.len(), gb.len(), GAP, |i, j| {
        (if ga[i].len() == gb[j].len() { 2 } else { 0 }) + name_score(&ga[i], &gb[j]) + 1
    });
    for step in &steps {
        match *step {
            (Some(x), None) => {
                report.push(format!("{key}: no counterpart there for {:?}, left out", name(&ga[x][0].original)))
            }
            (None, Some(y)) => report.push(format!("{key}: {:?} only there", name(&gb[y][0].original))),
            _ => {}
        }
    }
    // Next to a gap, which runs pair is a guess: there, only names that
    // look alike are taken.
    let gaps: Vec<usize> =
        steps.iter().enumerate().filter(|(_, s)| s.0.is_none() || s.1.is_none()).map(|(k, _)| k).collect();
    let mut out = Vec::new();
    for (k, step) in steps.iter().enumerate() {
        let (Some(x), Some(y)) = *step else { continue };
        let (x, y) = (&ga[x], &gb[y]);
        if gaps.iter().any(|&g| k.abs_diff(g) <= 2) && name_score(x, y) == 0 {
            report.push(format!(
                "{key}: {:?} and {:?} next to a gap, left out",
                name(&x[0].original),
                name(&y[0].original)
            ));
            continue;
        }
        out.extend(x.iter().copied().zip(y.iter().copied()));
        for m in x.iter().skip(y.len()) {
            report.push(format!(
                "{key} 0x{:08X}: no text there for this level, left out\n    here:  {:?}",
                at_of(m),
                m.original
            ));
        }
    }
    out
}

/// Whether `text` has the form of one-line `original`: the same `\` codes,
/// numbers and pad buttons, the same closing mark (or none, or a colon
/// where it has none: "Obtained" before an item's name is "Vous avez
/// trouvé :"), the same spaces at its ends, a capital first where it has
/// one.
fn same_form(original: &str, text: &str) -> bool {
    let end = |t: &str| END_MARK.find(t.trim_end_matches(py::is_space)).map(|m| m.as_str().to_string());
    let first_upper = |t: &str| t.chars().next().is_some_and(|c| py::is_upper(&c.to_string()));
    let ends = |t: &str| (t.chars().next().is_some_and(py::is_space), t.chars().last().is_some_and(py::is_space));
    all(&CODE, original) == all(&CODE, text)
        && all(&DIGITS, original) == all(&DIGITS, text)
        && all(&BUTTONS, original) == all(&BUTTONS, text)
        && (end(original) == end(text) || (end(original).as_deref() == Some("") && end(text).as_deref() == Some(":")))
        && first_upper(original) == first_upper(text)
        && ends(original) == ends(text)
}

/// Whether `text` may be the translation of one-line `original`: its form
/// (`same_form`), the acronyms kept (EXP, HP), and, for a title in
/// capitals, capitals.
fn plausible(original: &str, text: &str) -> bool {
    let kept: HashSet<&str> = all(&ACRONYM, text).into_iter().chain([original]).collect();
    same_form(original, text)
        && all(&ACRONYM, original).iter().all(|a| kept.contains(a))
        && (py::is_upper(text) || !py::is_upper(original))
}

/// The texts of one line of a code file, paired: (game text, its
/// translation). The candidates are where the other release's code points
/// (`pointer`) and the alignment's; a candidate that reads as one of the
/// game's own texts, or is not `plausible`, is none. A code file's texts
/// are in the same order in both releases, so of the pointed pairs found in
/// the file's list, those of the heaviest chain in that order are kept (a
/// pair out of order, or a text given to two, drops out), with an aligned
/// pair of the same form alone between two pointed pairs next to it. A
/// pointed text the extract listed in another file is kept when all the
/// game text's places agree.
fn chain_single<'a>(
    mine: &'a [Listed],
    theirs: &[Listed],
    pointer: &HashMap<String, Vec<String>>,
    game_texts: &HashSet<String>,
) -> Vec<(&'a Listed, String)> {
    let a: Vec<&Listed> = mine.iter().filter(|t| !several_lines(&t.original)).collect();
    let b: Vec<&Listed> = theirs.iter().filter(|t| !several_lines(&t.original)).collect();
    let mut places: HashMap<&str, Vec<usize>> = HashMap::new();
    for (j, t) in b.iter().enumerate() {
        places.entry(&t.original).or_default().push(j);
    }
    let mut weight: HashMap<(usize, usize), u32> = HashMap::new();
    let (fa, fb) = (features(&a), features(&b));
    for pair in py::align(fa.len(), fb.len(), GAP, |i, j| similarity(&fa[i], &fb[j])) {
        if let (Some(i), Some(j)) = pair {
            *weight.entry((i, j)).or_default() += 1;
        }
    }
    let mut out = Vec::new();
    for (i, m) in a.iter().enumerate() {
        let texts: HashSet<&String> = pointer
            .get(&m.original)
            .into_iter()
            .flatten()
            .filter(|t| !game_texts.contains(*t) && plausible(&m.original, t))
            .collect();
        for text in &texts {
            for &j in places.get(text.as_str()).into_iter().flatten() {
                *weight.entry((i, j)).or_default() += POINTED;
            }
        }
        if texts.len() == 1 {
            let text = texts.iter().next().expect("one");
            if !places.contains_key(text.as_str()) {
                out.push((*m, text.to_string()));
            }
        }
    }
    let mut pairs: Vec<(usize, usize)> =
        weight.keys().copied().filter(|p| !game_texts.contains(&b[p.1].original)).collect();
    pairs.sort_unstable();
    // the heaviest chain increasing in both orders
    let (mut best, mut before) = (Vec::<u32>::new(), Vec::<Option<usize>>::new());
    for (k, &(i, j)) in pairs.iter().enumerate() {
        // max of (best[q], q): the greatest, the last on a tie
        let mut top = (0, None);
        for q in 0..k {
            if pairs[q].0 < i && pairs[q].1 < j && best[q] >= top.0 {
                top = (best[q], Some(q));
            }
        }
        best.push(top.0 + weight[&(i, j)]);
        before.push(top.1);
    }
    let mut k = py::first_max_by(0..pairs.len(), |&q| best[q]);
    let mut chain = Vec::new();
    while let Some(q) = k {
        chain.push(pairs[q]);
        k = before[q];
    }
    chain.reverse();
    // pointed pairs, and an aligned one alone between two pointed pairs
    // that are next to it in both lists (of a longer gap the lists may
    // differ: the extract lists a text once, where first found)
    let pointed: Vec<usize> = (0..chain.len()).filter(|&n| weight[&chain[n]] >= POINTED).collect();
    for &n in &pointed {
        out.push((a[chain[n].0], b[chain[n].1].original.clone()));
    }
    for w in pointed.windows(2) {
        let (p, q) = (w[0], w[1]);
        let ((i0, j0), (i1, j1)) = (chain[p], chain[q]);
        if q - p == 2 && i1 - i0 == 2 && j1 - j0 == 2 {
            let (i, j) = chain[p + 1];
            if same_form(&a[i].original, &b[j].original) {
                out.push((a[i], b[j].original.clone()));
            }
        }
    }
    out
}

/// What strings.rs found: game text -> its translations, their unread
/// letters read from the other release's own texts. Those not found there,
/// once, are left out (answered: how many).
fn pointer_texts(found: &[Found], other_texts: &HashSet<String>) -> (HashMap<String, Vec<String>>, usize) {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    let mut left = 0;
    for f in found {
        let mut text = f.text.clone();
        if text.is_empty() {
            continue;
        }
        if text.matches('\n').count() != f.original.matches('\n').count() {
            left += 1;
            continue;
        }
        if UNREAD.is_match(&text) {
            // its pieces: the letters read, and each unread one
            let mut parts: Vec<(bool, &str)> = Vec::new();
            let mut at = 0;
            for m in UNREAD.find_iter(&text) {
                if m.start() > at {
                    parts.push((false, &text[at..m.start()]));
                }
                parts.push((true, m.as_str()));
                at = m.end();
            }
            if at < text.len() {
                parts.push((false, &text[at..]));
            }
            let unread = parts.iter().filter(|p| p.0).count();
            let read: usize = parts.iter().filter(|p| !p.0).map(|p| py::strip(p.1).chars().count()).sum();
            if read < 2 || read < unread {
                left += 1; // too little read to tell
                continue;
            }
            let body: String =
                parts.iter().map(|&(u, p)| if u { ".{1,4}".to_string() } else { regex::escape(p) }).collect();
            let pattern = re(&format!(r"(?s)\A{body}\n?\z"));
            let lines = text.matches('\n').count();
            let matches: Vec<&String> =
                other_texts.iter().filter(|t| pattern.is_match(t) && t.matches('\n').count() == lines).collect();
            if matches.len() != 1 {
                left += 1;
                continue;
            }
            text = matches[0].clone();
        } else if !other_texts.contains(&text) {
            left += 1;
            continue;
        }
        out.entry(f.original.clone()).or_default().push(text);
    }
    (out, left)
}

/// Code file (its folder under dumps/strings) -> its texts' files (named
/// by its load address, which differs between releases), and the items'
/// details' files (relative path -> path).
fn code_files(strings: &Path) -> (BTreeMap<String, Vec<PathBuf>>, BTreeMap<String, PathBuf>) {
    let (mut code, mut details) = (BTreeMap::<String, Vec<PathBuf>>::new(), BTreeMap::new());
    for path in toml_files(strings, true) {
        let rel = rel(strings, &path);
        if rel.starts_with("pc/") {
            continue; // PE2PC's own screens
        }
        let (dir, file) = rel.rsplit_once('/').unwrap_or(("", &rel));
        if file.starts_with("txt-") {
            details.insert(rel.clone(), path);
        } else {
            code.entry(dir.to_string()).or_default().push(path);
        }
    }
    (code, details)
}

struct Entry {
    at: u32,
    here_only: bool,
    original: String,
    text: String,
}

/// Writes the mod's `strings/` and `captions.toml`. `places`: the loading
/// captions' places the other release leaves untranslated (their English
/// text, lines split by \n) -> their translation. Answers (texts, items'
/// details, captions).
pub fn import(
    game: &Version,
    other: &Version,
    found: &[Found],
    places: &toml::Table,
    out: &Path,
    report: &mut Vec<String>,
) -> Result<(usize, usize, usize)> {
    let (mine_code, mine_details) = code_files(&game.dumps.join("strings"));
    let (their_code, their_details) = code_files(&other.dumps.join("strings"));
    let mut loaded: HashMap<PathBuf, Vec<Listed>> = HashMap::new();
    for path in mine_code.values().chain(their_code.values()).flatten() {
        loaded.insert(path.clone(), listed(path)?);
    }
    let texts_of = |code: &BTreeMap<String, Vec<PathBuf>>| -> HashSet<String> {
        code.values().flatten().flat_map(|p| loaded[p].iter().map(|t| t.original.clone())).collect()
    };
    let other_texts = texts_of(&their_code);
    let game_texts = texts_of(&mine_code);

    // texts of one line: where the other release's code points
    let (single, unresolved) = pointer_texts(found, &other_texts);
    report.push(format!("{unresolved} texts found where the code points not among the other release's, left out"));

    // per code file: the texts of several lines (records) and of one line
    let mut entries: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
    // one-line game text -> (code file, address, translation), at its first place
    let mut chosen: IndexMap<String, (String, u32, String)> = IndexMap::new();
    for (key, mine_paths) in &mine_code {
        let Some(their_paths) = their_code.get(key) else { continue };
        if mine_paths.len() != 1 || their_paths.len() != 1 {
            continue;
        }
        let (mine, theirs) = (&loaded[&mine_paths[0]], &loaded[&their_paths[0]]);
        for (m, text) in chain_single(mine, theirs, &single, &game_texts) {
            chosen.entry(m.original.clone()).or_insert((key.clone(), at_of(m), text));
        }
        let records: Vec<&Listed> = mine.iter().filter(|t| several_lines(&t.original)).collect();
        let other_records: Vec<&Listed> = theirs.iter().filter(|t| several_lines(&t.original)).collect();
        for (m, t) in pair_records(&records, &other_records, report, key) {
            if t.original == m.original {
                continue;
            }
            if BREAK.find_iter(&t.original).count() != BREAK.find_iter(&m.original).count() {
                report.push(format!(
                    "{key} 0x{:08X}: another number of lines, left out\n    here:  {:?}\n    there: {:?}",
                    at_of(m),
                    m.original,
                    t.original
                ));
                continue;
            }
            entries.entry(key.clone()).or_default().push(Entry {
                at: at_of(m),
                here_only: true,
                original: m.original.clone(),
                text: t.original.clone(),
            });
        }
    }
    // a one-line text's translation applies wherever the game uses it
    for (original, (key, at, text)) in chosen {
        entries.entry(key).or_default().push(Entry { at, here_only: false, original, text });
    }
    let count = entries.values().map(Vec::len).sum();

    let strings_dir = out.join("strings");
    for (key, mut es) in entries {
        es.sort_by_key(|e| e.at);
        let mut lines =
            vec![format!("# Texts of {key}, from the other release's discs (pe2-lang-import)."), String::new()];
        for e in es {
            lines.push("[[string]]".into());
            lines.push(format!("at = 0x{:08X}", e.at));
            if e.here_only {
                lines.push("here_only = true".into());
            }
            lines.push(format!("original = {}", literal(&e.original)?));
            lines.push(format!("text = {}", literal(&e.text)?));
            lines.push(String::new());
        }
        let dest = strings_dir.join(&key).join("release.toml");
        std::fs::create_dir_all(dest.parent().expect("a folder"))?;
        std::fs::write(dest, lines.join("\n"))?;
    }

    // the items' details: the other release's file of the same name
    let mut details = 0;
    for (rel, path) in &mine_details {
        let Some(there) = their_details.get(rel) else {
            report.push(format!("{rel}: no such file in the other release"));
            continue;
        };
        let (m, t) = (listed(path)?, listed(there)?);
        let (Some(m), Some(t)) = (m.first(), t.first()) else { continue };
        if m.original == t.original {
            continue;
        }
        let dest = strings_dir.join(rel);
        std::fs::create_dir_all(dest.parent().expect("a folder"))?;
        let lines = [
            "# An item's details, from the other release's discs.".to_string(),
            String::new(),
            "[[string]]".into(),
            format!("original = {}", literal(&m.original)?),
            format!("text = {}", literal(&t.original)?),
            String::new(),
        ];
        std::fs::write(dest, lines.join("\n"))?;
        details += 1;
    }

    let captions = import_load_captions(game, other, places, out, report)?;
    Ok((count, details, captions))
}

/// The loading captions (main/text.h TextStream): a record is 32 bytes, its
/// script's address at +16 and its cell table's at +20; a script ends with
/// END, a LINE between its lines, and a glyph | FILTERED is one the loading
/// screens draw (the times the European releases add).
const END: u8 = 0xFF;
const LINE: u8 = 0xFE;
const FILTERED: u8 = 0x80;
/// The captions' cell table (main/caption_draw.c gTextCaptionGlyphCells).
const CAPTIONS: u32 = 0x8006_27E0;
/// The captions' cells' letters (None: not a letter a translation uses):
/// the game's, then the ones the European releases add.
const CAPTION_LETTERS: [Option<char>; 0x4E] = {
    let game = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890:;',.?";
    let mut out = [None; 0x4E];
    let mut k = 0;
    while k < game.len() {
        out[k] = Some(game[k] as char);
        k += 1;
    }
    out[0x45] = Some(' ');
    out[0x46] = Some('é');
    out[0x47] = Some('è');
    out[0x48] = Some('ü');
    out[0x49] = Some('ß');
    out[0x4A] = Some('-');
    out[0x4C] = Some('º');
    out[0x4D] = Some('Á');
    out
};

/// A caption record's fields: x, y, then 6 more halfwords, the script's and
/// the cells' addresses, line height, a halfword, the plate's width and
/// height.
type Fields = [i64; 14];

fn fields_at(exe: &[u8], at: usize) -> Fields {
    let half = |k: usize| i16::from_le_bytes([exe[at + 2 * k], exe[at + 2 * k + 1]]) as i64;
    let mut f = [0; 14];
    for (k, v) in f.iter_mut().enumerate().take(8) {
        *v = half(k);
    }
    f[8] = u32_at(exe, at + 16) as i64;
    f[9] = u32_at(exe, at + 20) as i64;
    for k in 0..4 {
        f[10 + k] = half(12 + k);
    }
    f
}

/// The caption records pointing at cell table `table`, in address order:
/// (fields, script).
fn caption_records(base: u32, exe: &[u8], table: u32) -> Vec<(Fields, Vec<u8>)> {
    let mut out = Vec::new();
    let mut at = 20;
    while at + 12 < exe.len() {
        if u32_at(exe, at) == table {
            let f = fields_at(exe, at - 20);
            let chars = f[8];
            if (base as i64) <= chars && chars < base as i64 + exe.len() as i64 {
                let start = (chars - base as i64) as usize;
                let window = &exe[start..(start + 256).min(exe.len())];
                if let Some(end) = window.iter().position(|&c| c == END) {
                    out.push((f, window[..end].to_vec()));
                }
            }
        }
        at += 4;
    }
    out
}

fn caption_text(line: &[u8]) -> String {
    line.iter().filter_map(|&c| CAPTION_LETTERS.get((c & !FILTERED) as usize).copied().flatten()).collect()
}

/// A caption's date (its first line up to its time: the FILTERED glyphs) and
/// its place (its other lines).
fn caption_lines(script: &[u8]) -> (String, String) {
    let mut lines = script.split(|&c| c == LINE);
    let first = lines.next().unwrap_or_default();
    let shown = first.iter().position(|&c| c & FILTERED != 0).unwrap_or(first.len());
    let date = caption_text(&first[..shown]).trim_end_matches([' ', ',', '.']).to_string();
    (date, lines.map(caption_text).collect::<Vec<_>>().join("\n"))
}

/// The width of a script's widest line, in the cells' pixels.
fn caption_width(script: &[u8], cells: &[u8]) -> i64 {
    script
        .split(|&c| c == LINE)
        .map(|line| line.iter().map(|&c| *cells.get(4 * (c & !FILTERED) as usize + 2).unwrap_or(&0) as i64).sum())
        .max()
        .unwrap_or(0)
}

fn hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02x}")).collect()
}

/// Writes the other release's loading captions into the mod's
/// `captions.toml`. A release built again keeps the game's captions, in
/// their order, and adds its own after the dated ones in a secondary
/// record, drawn elsewhere (at the bottom right): the game's record is shown
/// with that one's script, place and plate, and the other release's cell
/// table (the letters it adds). It keeps the captions of a place alone
/// (loading a saved game) untranslated: those are made of the date of a
/// translated one and the translation of their place, a translated
/// caption's or one of `places`. Answers how many.
fn import_load_captions(
    game: &Version,
    other: &Version,
    places: &toml::Table,
    out: &Path,
    report: &mut Vec<String>,
) -> Result<usize> {
    let (base, exe) = (game.exe_base, &game.exe);
    let (other_base, their_exe) = (other.exe_base, &other.exe);
    let mine = caption_records(base, exe, CAPTIONS);
    if mine.is_empty() {
        report.push("loading captions: none found in the game's executable".into());
        return Ok(0);
    }
    // the other release's cell table: the game's first cells, then its own
    let head = &exe[(CAPTIONS - base) as usize..(CAPTIONS - base) as usize + 32];
    let Some(at) = their_exe.windows(32).position(|w| w == head) else {
        report.push("loading captions: the other release's cell table not found".into());
        return Ok(0);
    };
    let theirs = caption_records(other_base, their_exe, at as u32 + other_base);
    let Some(first_script) = theirs.iter().map(|(f, _)| f[8]).min() else {
        report.push("loading captions: none there".into());
        return Ok(0);
    };
    let cells = &their_exe[at..(first_script - other_base as i64) as usize];
    // the game's records where it draws them, each then maybe a secondary one
    let spots: HashSet<(i64, i64)> = mine.iter().map(|(f, _)| (f[0], f[1])).collect();
    let shown: Vec<usize> = (0..theirs.len()).filter(|&k| spots.contains(&(theirs[k].0[0], theirs[k].0[1]))).collect();
    if shown.len() != mine.len() {
        report.push(format!("loading captions: {} records there for {} here", shown.len(), mine.len()));
        return Ok(0);
    }
    // the game's script -> the secondary record
    let mut paired: IndexMap<&[u8], &(Fields, Vec<u8>)> = IndexMap::new();
    for ((_, script), &k) in mine.iter().zip(&shown) {
        if k + 1 < theirs.len() && !shown.contains(&(k + 1)) {
            paired.insert(script, &theirs[k + 1]);
        }
    }
    if paired.is_empty() {
        report.push("loading captions: none translated there".into());
        return Ok(0);
    }
    // a caption's first line is its date (and time), the others its place
    let mut dates: IndexMap<String, (String, Fields)> = IndexMap::new();
    let mut place_texts: HashMap<String, String> = HashMap::new();
    let mut plates: HashMap<usize, i64> = HashMap::new(); // a plate's height by lines
    for (script, (f, text)) in &paired {
        let ((date, place), (there, their_place)) = (caption_lines(script), caption_lines(text));
        dates.entry(date).or_insert((there, *f));
        place_texts.entry(place).or_insert(their_place);
        let lines = text.iter().filter(|&&c| c == LINE).count() + 1;
        let plate = plates.entry(lines).or_insert(0);
        *plate = (*plate).max(f[13]);
    }
    for (place, text) in places {
        if let Some(text) = text.as_str() {
            place_texts.insert(place.clone(), text.to_string());
        }
    }
    let letters: HashMap<char, u8> = CAPTION_LETTERS[..(cells.len() / 4).min(CAPTION_LETTERS.len())]
        .iter()
        .enumerate()
        .filter_map(|(n, c)| Some(((*c)?, n as u8)))
        .collect();
    let edge = paired.values().map(|(f, _)| f[0]).max().unwrap_or(0);
    let right = paired.values().filter(|(f, _)| f[0] < edge).map(|(f, text)| f[0] + caption_width(text, cells)).max();
    let mut made: Vec<(Vec<u8>, Vec<u8>, i64, Fields)> =
        paired.iter().map(|(script, (f, text))| (script.to_vec(), text.clone(), f[0], *f)).collect();
    for (_, script) in &mine {
        if paired.contains_key(&script[..]) {
            continue;
        }
        let (date, place) = caption_lines(script);
        let (Some((there, f)), Some(their_place)) = (dates.get(&date), place_texts.get(&place)) else {
            report.push(format!("loading captions: {date:?}, {place:?} not translated"));
            continue;
        };
        let words: Vec<String> = format!("{there}\n{their_place}").split('\n').map(str::to_string).collect();
        if words.iter().any(|line| line.chars().any(|c| !letters.contains_key(&c)))
            || !plates.contains_key(&words.len())
        {
            report.push(format!(
                "loading captions: {words:?}: a letter the captions' cells lack, or no plate there for so many lines"
            ));
            continue;
        }
        let text = words
            .iter()
            .map(|line| line.chars().map(|c| letters[&c]).collect::<Vec<u8>>())
            .collect::<Vec<_>>()
            .join(&LINE);
        let x = right.map_or(edge, |right| edge.min(right - caption_width(&text, cells)));
        let mut f = *f;
        f[13] = plates[&words.len()];
        made.push((script.clone(), text, x, f));
    }
    let mut lines = vec![
        "# The loading captions of the other release's discs (pe2-lang-import, PE2PC's mods/load_captions.rs)."
            .to_string(),
        String::new(),
        format!("cells = \"{}\"", hex(cells)),
        String::new(),
    ];
    for (script, text, x, f) in &made {
        lines.extend([
            "[[caption]]".to_string(),
            format!("original = \"{}\"", hex(script)),
            format!("text = \"{}\"", hex(text)),
            format!("x = {x}"),
            format!("y = {}", f[1]),
            format!("line_height = {}", f[10]),
            format!("box_width = {}", f[12]),
            format!("box_height = {}", f[13]),
            String::new(),
        ]);
    }
    std::fs::write(out.join("captions.toml"), lines.join("\n"))?;
    Ok(made.len())
}
