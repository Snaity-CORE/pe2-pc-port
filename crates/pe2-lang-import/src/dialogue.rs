//! The dialogue (`text/`): each dialogue file's lines of the other
//! release's extract (dumps/text) put, as translations, into the lines in
//! the same place of the game's own (same file, same order, a line dropped
//! or added there allowed for), where both have the same controls (camera
//! changes, choices, icons).

use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::Result;
use crate::dumps::{literal, read_toml, rel, toml_files};
use crate::py;

static CONTROL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{([0-9A-F]{4})\}").expect("regex"));
static NOT_READ: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{g\d+\}").expect("regex"));
static END: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(\.\.\.|[.?!"\]]|)\s*$"#).expect("regex"));
static DIGITS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").expect("regex"));
const GAP: i32 = -2;

struct Line {
    record: i64,
    original: String,
    speaker: Option<String>,
}

fn lines_of(table: &toml::Table) -> Vec<Line> {
    let lines = table.get("line").and_then(|l| l.as_array()).cloned().unwrap_or_default();
    lines
        .iter()
        .filter_map(|l| {
            Some(Line {
                record: l.get("record")?.as_integer()?,
                original: l.get("original")?.as_str()?.to_string(),
                speaker: l.get("speaker").and_then(|s| s.as_str()).map(str::to_string),
            })
        })
        .collect()
}

fn controls(text: &str) -> Vec<&str> {
    CONTROL.captures_iter(text).map(|c| c.get(1).map_or("", |m| m.as_str())).collect()
}

/// What a line and its translation share: its controls, the distance to
/// the next record (the script's layout, kept by translators who only
/// retype the text), its number of lines, its closing punctuation, its
/// numbers, its bracketed labels, colour changes and speaker's name label.
struct Features<'a> {
    controls: Vec<&'a str>,
    step: Option<i64>,
    breaks: usize,
    end: &'a str,
    digits: Vec<&'a str>,
    brackets: usize,
    colours: usize,
    speaker: Option<&'a str>,
}

fn features(lines: &[Line]) -> Vec<Features<'_>> {
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let text = line.original.as_str();
            Features {
                controls: controls(text),
                step: lines.get(i + 1).map(|n| n.record - line.record),
                breaks: text.matches('\n').count(),
                end: END.captures(text).and_then(|c| c.get(1)).map_or("", |m| m.as_str()),
                digits: DIGITS.find_iter(text).map(|m| m.as_str()).collect(),
                brackets: text.matches('[').count(),
                colours: text.matches("{color").count(),
                speaker: line.speaker.as_deref(),
            }
        })
        .collect()
}

/// How alike the game's line `a` and the other release's `b` are. `names`:
/// the other release's speakers' labels in this file -> the game's names
/// they were found to be (`speaker_names`).
fn similarity(a: &Features, b: &Features, names: &HashMap<String, String>) -> i32 {
    let mut score = if a.controls == b.controls { 3 } else { -3 };
    score += if a.step.is_some() && a.step == b.step { 2 } else { 0 };
    score += i32::from(a.breaks == b.breaks);
    score += i32::from(a.end == b.end);
    score += if !a.digits.is_empty() && a.digits == b.digits { 2 } else { 0 };
    score += i32::from(a.brackets != 0 && a.brackets == b.brackets);
    score += i32::from(a.colours != 0 && a.colours == b.colours);
    // a line with a speaker's label is not one without
    match (a.speaker, b.speaker) {
        (None, Some(_)) | (Some(_), None) => score -= 2,
        (Some(mine), Some(theirs)) => {
            if let Some(name) = names.get(theirs) {
                score += if name == mine { 3 } else { -3 };
            }
        }
        (None, None) => {}
    }
    score
}

fn align(mine: &[Line], theirs: &[Line], names: &HashMap<String, String>) -> Vec<(Option<usize>, Option<usize>)> {
    let (a, b) = (features(mine), features(theirs));
    py::align(a.len(), b.len(), GAP, |i, j| similarity(&a[i], &b[j], names))
}

/// The other release's speakers' labels in a file -> the game's names. A
/// translation draws its own labels, which the extract may not read, so
/// each is the name most of the lines it is paired with in a first
/// alignment have (at least two of them, and most).
fn speaker_names(mine: &[Line], theirs: &[Line]) -> HashMap<String, String> {
    let mut votes: indexmap::IndexMap<&str, indexmap::IndexMap<&str, usize>> = indexmap::IndexMap::new();
    for (m, t) in align(mine, theirs, &HashMap::new()) {
        if let (Some(m), Some(t)) = (m, t)
            && let (Some(ms), Some(ts)) = (&mine[m].speaker, &theirs[t].speaker)
        {
            *votes.entry(ts).or_default().entry(ms).or_default() += 1;
        }
    }
    let mut names = HashMap::new();
    for (label, counter) in votes {
        let total: usize = counter.values().sum();
        // Counter.most_common(1): the most, the first counted on a tie
        if let Some((name, n)) = py::first_max_by(counter, |(_, n)| *n)
            && n >= 2
            && n * 2 > total
        {
            names.insert(label.to_string(), name.to_string());
        }
    }
    names
}

/// Writes the mod's `text/` from the game's and the other release's
/// `dumps/text`; answers (files, lines) and adds its notes to `report`.
pub fn import(game: &Path, other: &Path, out: &Path, name: &str, report: &mut Vec<String>) -> Result<(usize, usize)> {
    let (mut files, mut count) = (0, 0);
    for game_path in toml_files(game, true) {
        let rel = rel(game, &game_path);
        let other_path = other.join(&rel);
        if !other_path.exists() {
            report.push(format!("{rel}: not in the other release"));
            continue;
        }
        let game_table = read_toml(&game_path)?;
        let Some(cap) = game_table.get("cap").and_then(|c| c.as_str()) else {
            continue; // not a dialogue file (speakers.toml)
        };
        let (mine, theirs) = (lines_of(&game_table), lines_of(&read_toml(&other_path)?));
        let mut text_out = vec![format!("# {name}: {rel}"), format!("cap = \"{cap}\""), String::new()];
        for (m, t) in align(&mine, &theirs, &speaker_names(&mine, &theirs)) {
            let (m, t) = match (m, t) {
                (None, Some(t)) => {
                    report.push(format!("{rel}: a line only there, left out\n    there: {:?}", theirs[t].original));
                    continue;
                }
                (Some(m), None) => {
                    report.push(format!(
                        "{rel} record 0x{:04X}: no line there, left out\n    here:  {:?}",
                        mine[m].record, mine[m].original
                    ));
                    continue;
                }
                (Some(m), Some(t)) => (&mine[m], &theirs[t]),
                (None, None) => continue,
            };
            let mut text = t.original.clone();
            if controls(&text) != controls(&m.original) {
                report.push(format!(
                    "{rel} record 0x{:04X}: controls differ, left out\n    here:  {:?}\n    there: {text:?}",
                    m.record, m.original
                ));
                continue;
            }
            if NOT_READ.is_match(&text) {
                let letters =
                    CONTROL.replace_all(&NOT_READ.replace_all(&text, ""), "").chars().any(char::is_alphabetic);
                if !letters {
                    // not a letter read (its glyphs' picture not found): the
                    // game's line stays
                    report.push(format!("{rel} record 0x{:04X}: no glyph read, left out", m.record));
                    continue;
                }
                report
                    .push(format!("{rel} record 0x{:04X}: glyphs not read, shown as ?\n    there: {text:?}", m.record));
                text = NOT_READ.replace_all(&text, "?").into_owned();
            }
            text_out.push("[[line]]".into());
            text_out.push(format!("record = 0x{:04X}", m.record));
            if let Some(speaker) = &m.speaker {
                text_out.push(format!("speaker = \"{speaker}\""));
            }
            text_out.push(format!("original = {}", literal(&m.original)?));
            text_out.push(format!("text = {}", literal(&text)?));
            text_out.push(String::new());
            count += 1;
        }
        let dest = out.join("text").join(&rel);
        std::fs::create_dir_all(dest.parent().expect("a folder"))?;
        std::fs::write(&dest, text_out.join("\n"))?;
        files += 1;
    }
    Ok((files, count))
}
