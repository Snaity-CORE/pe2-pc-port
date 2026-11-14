//! The dialogue as text files to translate (`pe2 --extract texts`).

use std::path::Path;

use super::{Cap, Pages, Sources, caps, glyph, is_glyph, label_name, text};
use crate::mods::glyphs::dialogue::recognize;

/// Saves each CAP file's lines as `<file>/<chunk>-<n>.toml` under `out`,
/// to be translated (see docs/MODDING.md). Answers how many files and
/// lines.
pub fn extract(sources: &Sources, out: &Path) -> std::io::Result<(usize, usize)> {
    let mut pages = Pages::new(sources);
    let (mut files, mut lines) = (0, 0);
    let mut speakers = std::collections::BTreeSet::new();
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
    Ok((files, lines))
}
