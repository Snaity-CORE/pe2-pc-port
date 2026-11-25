//! The dialogue as text files to translate (`pe2 --extract texts`).

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use super::{Cap, Column, Pages, Sources, caps, glyph, is_glyph, label_name, text, text_by};
use crate::mods::glyphs::dialogue::recognize;

/// The glyph pictures no table knows (glyphs/dialogue.rs), by their hash:
/// how often they are drawn, and some words they are in.
type Unknown = HashMap<u64, (usize, BTreeSet<String>)>;

/// Notes the glyphs of a line not known: each word holding one, the glyph
/// as `#` and the other unknown ones as `?` (so a translation's letters can
/// be named from the words they are in: fran#ais).
fn note_unknown(cap: &Cap, page: &Column, codes: &[u16], unknown: &mut Unknown) {
    const OPEN: char = '\u{1}';
    let marked = text_by(codes, |index| {
        let picture = glyph(page, cap.cell(index));
        Some(
            recognize(&picture)
                .map(|t| t.into_owned())
                .unwrap_or_else(|| format!("{OPEN}{:016X}", crate::mods::fnv1a(picture.iter().copied()))),
        )
    });
    for word in marked.split_whitespace().filter(|w| w.contains(OPEN)) {
        let parts: Vec<&str> = word.split(OPEN).collect();
        let hashes: BTreeSet<&str> = parts[1..].iter().map(|p| &p[..16]).collect();
        for hash in hashes {
            let shown: String = parts[1..].iter().fold(parts[0].to_string(), |mut s, p| {
                s.push(if &p[..16] == hash { '#' } else { '?' });
                s.push_str(&p[16..]);
                s
            });
            let entry = unknown.entry(u64::from_str_radix(hash, 16).unwrap_or(0)).or_default();
            entry.0 += 1;
            if entry.1.len() < 8 {
                entry.1.insert(shown);
            }
        }
    }
}

/// Saves each CAP file's lines as `<file>/<chunk>-<n>.toml` under `out`,
/// to be translated (see docs/MODDING.md). Answers how many files and
/// lines.
pub fn extract(sources: &Sources, out: &Path) -> std::io::Result<(usize, usize)> {
    let mut pages = Pages::new(sources);
    let (mut files, mut lines) = (0, 0);
    let mut speakers = BTreeSet::new();
    let mut unknown = Unknown::new();
    // Each name label's picture where it is drawn, for matching another
    // version's labels to these (tools/gen_glyph_tables.py).
    let mut labels = String::from(
        "# Each line's speaker's name label: file, chunk, dialogue file, line, the\n\
         # hash of its picture (glyphs/dialogue.rs GLYPHS) and the name read from it.\n",
    );
    for (key, chunk, payload) in &sources.bundles {
        for (n, bytes) in caps(payload).into_iter().enumerate() {
            let Some(cap) = Cap::new(bytes) else { continue };
            let page = pages.find(key, &cap);
            let mut toml = format!(
                "# Dialogue file {n} of {key}, chunk {chunk}.\n\
                 # Write each line's translation in `text`; an empty one keeps the original.\n\
                 # {{_}}: a narrow space; {{color N}}: a colour; {{XXXX}}: a control (keep them).\n"
            );
            if page.is_none() {
                toml.push_str("# The glyphs' picture was not found: glyphs show as {gN}.\n");
            }
            toml.push_str(&format!("cap = \"{:016x}\"\n", cap.hash()));
            let mut count = 0;
            for (ln, line) in cap.lines().into_iter().enumerate() {
                if let Some(page) = page.filter(|_| line.title != 0) {
                    let picture = glyph(page, cap.cell(usize::from(line.title - 1)));
                    let name = recognize(&picture).unwrap_or_default();
                    let hash = crate::mods::fnv1a(picture.iter().copied());
                    labels.push_str(&format!("{key}\t{chunk}\t{n}\t{ln}\t{hash:016X}\t{}\n", name.trim()));
                }
                if !line.codes.iter().any(|&c| is_glyph(c)) {
                    continue;
                }
                let original = text(&cap, page, &line.codes);
                if let Some(page) = page.filter(|_| original.contains("{g")) {
                    note_unknown(&cap, page, &line.codes, &mut unknown);
                }
                let original = original.strip_suffix('\n').unwrap_or(&original);
                toml.push_str(&format!("\n[[line]]\nrecord = 0x{:04X}\n", line.record));
                if line.title != 0 {
                    let speaker = text(&cap, page, &[u16::from(line.title - 1)]);
                    let name = label_name(&speaker).unwrap_or(speaker.trim());
                    if !name.is_empty() && !name.starts_with("{g") {
                        speakers.insert(name.to_string());
                    }
                    let speaker = speaker.trim_start_matches('[').trim_end_matches(']');
                    toml.push_str(&format!("speaker = \"{}\"\n", speaker.replace('"', "\\\"")));
                }
                toml.push_str(&format!("original = '''\n{original}'''\ntext = ''\n"));
                count += 1;
            }
            if count == 0 {
                continue;
            }
            let path = out.join(key).join(format!("{chunk}-{n}.toml"));
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            std::fs::write(path, toml)?;
            files += 1;
            lines += count;
        }
    }
    // The speakers' names, for a translation's (labels.rs).
    let mut toml = String::from(
        "# The speakers' names on the dialogue's labels. A mod translates them in\n\
         # its text/speakers.toml: \"NAME\" = \"translation\" (only those it changes).\n",
    );
    for name in speakers {
        let quoted = name.replace('\\', "\\\\").replace('"', "\\\"");
        toml.push_str(&format!("# \"{quoted}\" = \"{quoted}\"\n"));
    }
    std::fs::create_dir_all(out)?;
    std::fs::write(out.join(crate::mods::labels::FILE), toml)?;
    std::fs::write(out.join("labels.tsv"), labels)?;
    // The glyphs not known, most drawn first, to be named in glyphs/dialogue.rs.
    let mut rows: Vec<_> = unknown.into_iter().collect();
    rows.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(&b.0)));
    let mut tsv = String::from(
        "# Glyph pictures no table knows (glyphs/dialogue.rs GLYPHS): the hash of\n\
         # the picture, how often it is drawn, words it is in (it as #, others not known as ?).\n",
    );
    for (hash, (count, words)) in rows {
        let words: Vec<String> = words.into_iter().collect();
        tsv.push_str(&format!("{hash:016X}\t{count}\t{}\n", words.join(" ")));
    }
    std::fs::write(out.join("glyphs-unknown.tsv"), tsv)?;
    Ok((files, lines))
}
