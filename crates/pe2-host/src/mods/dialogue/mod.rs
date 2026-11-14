//! Dialogue: the lines of the game's CAP2 dialogue files as text.
//!
//! A dialogue bundle (`.pe2cap2` chunk) holds CAP files (cap.rs): their
//! lines are streams of glyph indices into a table of cells of a 4-bit
//! texture page, loaded by one of the game's `.pe2img` pictures
//! (picture.rs). The glyphs are pictures, not characters; what character
//! each is comes from glyphs/dialogue.rs, by the picture. Which picture
//! holds a file's glyphs is not recorded anywhere the extract can read, so
//! the one where most of its glyphs are known is taken.

use std::collections::HashMap;

mod cap;
mod extract;
mod picture;

pub use cap::{Cap, caps, is_glyph, slots};
pub use extract::extract;
pub use picture::{Column, glyph, lzss, picture};

use super::glyphs::dialogue::{known, recognize};
use cap::{GAP, NEWLINE};

/// The glyph indices a CAP file's lines draw (up to `limit`).
fn uses(cap: &Cap, limit: usize) -> Vec<usize> {
    let lines = cap.lines();
    let codes = lines.iter().flat_map(|l| l.codes.iter());
    codes.filter(|&&c| is_glyph(c)).map(|&c| usize::from(c & 0x3FF)).take(limit).collect()
}

/// The share of `uses` whose glyph pictures in `column` are known, of
/// those not blank: blank glyphs (spaces) do not tell a page, a wrong one
/// being blank there too. A page knowing fewer than a third of all of them
/// is not it.
fn share(cap: &Cap, column: &Column, uses: &[usize]) -> f64 {
    let (mut hits, mut blank) = (0, 0);
    for &i in uses {
        match recognize(&glyph(column, cap.cell(i))) {
            Some(t) if t.trim().is_empty() => blank += 1,
            Some(_) => hits += 1,
            None => {}
        }
    }
    if hits * 3 < uses.len() {
        return 0.0;
    }
    hits as f64 / (uses.len() - blank).max(1) as f64
}

/// The speaker's name a name label's text (`[BALDWIN]`) gives.
pub fn label_name(text: &str) -> Option<&str> {
    let name = text.trim().strip_prefix('[')?.strip_suffix(']')?.trim();
    (!name.is_empty()).then_some(name)
}

/// A line's text: glyphs as their characters, line breaks, `{_}` for the
/// narrow space, `{color N}` where the colour changes, `{XXXX}` for the
/// other controls and `{gN}` for a glyph not known.
pub fn text(cap: &Cap, page: Option<&Column>, codes: &[u16]) -> String {
    text_by(codes, |index| page.and_then(|p| recognize(&glyph(p, cap.cell(index)))))
}

/// A line's text (as `text`), each glyph index's text from `glyph_text`.
pub fn text_by<S: AsRef<str>>(codes: &[u16], glyph_text: impl Fn(usize) -> Option<S>) -> String {
    let mut out = String::new();
    let mut color = 0;
    for &c in codes {
        match c {
            NEWLINE => out.push('\n'),
            GAP => out.push_str("{_}"),
            c if is_glyph(c) => {
                if (c >> 10) != color {
                    color = c >> 10;
                    out.push_str(&format!("{{color {color}}}"));
                }
                let index = usize::from(c & 0x3FF);
                match glyph_text(index) {
                    Some(t) => out.push_str(t.as_ref()),
                    None => out.push_str(&format!("{{g{index}}}")),
                }
            }
            c => out.push_str(&format!("{{{c:04X}}}")),
        }
    }
    fix_capital_i(&out)
}

/// Capital I and small l are one glyph: an `l` alone, before an
/// apostrophe, after a capital (not before a small letter: Ele), before
/// one, or starting a word before a consonant as I does (In, It, Inform,
/// Isso: words that start with l go on with a vowel, or lh: lhe) is an I.
fn fix_capital_i(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let letter = |i: usize| chars.get(i).is_some_and(|c| c.is_alphabetic());
    let upper = |i: usize| chars.get(i).is_some_and(|c| c.is_uppercase());
    let lower = |i: usize| chars.get(i).is_some_and(|c| c.is_lowercase());
    let vowel = |c: char| "aeiouyáàâãäéèêëíìîïóòôõöúùûü".contains(c);
    (0..chars.len())
        .map(|i| {
            if chars[i] != 'l' {
                return chars[i];
            }
            let starts = i == 0 || !letter(i - 1);
            let next = chars.get(i + 1).copied();
            let is_i = (starts && !letter(i + 1))
                || (starts && next == Some('\''))
                || (starts && next.is_some_and(|c| c.is_lowercase() && c != 'l' && c != 'h' && !vowel(c)))
                || (i > 0 && upper(i - 1) && !lower(i + 1))
                || (starts && upper(i + 1));
            if is_i { 'I' } else { 'l' }
        })
        .collect()
}

/// The dialogue bundles and the pictures that may hold their glyphs: what
/// `extract` takes.
#[derive(Default)]
pub struct Sources {
    /// File key -> its `.pe2img` chunks' contents, and its `.pe2clut`
    /// ones' (palettes).
    pub pictures: HashMap<String, Vec<Vec<u8>>>,
    /// File key -> the chunk each of its pictures is.
    pub picture_chunks: HashMap<String, Vec<usize>>,
    pub palettes: HashMap<String, Vec<Vec<u8>>>,
    /// File key -> its code and data packages (`.pe2pkg`): where they load
    /// in RAM, and their contents (still compressed); and its room
    /// backgrounds' contents (`.bs`).
    pub packages: HashMap<String, Vec<(u32, Vec<u8>)>>,
    pub backgrounds: HashMap<String, Vec<Vec<u8>>>,
    /// (file key, chunk index, contents) of each `.pe2cap2` chunk.
    pub bundles: Vec<(String, usize, Vec<u8>)>,
    /// The same of each text chunk (`.txt`: items' details).
    pub texts: Vec<(String, usize, Vec<u8>)>,
}

/// Where a CAP file's glyph page may be, most likely first: its own file's
/// pictures, its folder's, the shared ones (stage 0), its stage's.
fn candidates<'s>(sources: &'s Sources, key: &str) -> Vec<&'s str> {
    let parts: Vec<&str> = key.split('/').collect();
    let folder = parts[..parts.len() - 1].join("/") + "/";
    let stage = format!("{}/", parts[0]);
    let mut out: Vec<&str> = Vec::new();
    for prefix in [key, folder.as_str(), "stage0/", stage.as_str()] {
        let mut keys: Vec<&str> =
            sources.pictures.keys().map(String::as_str).filter(|k| *k == key || k.starts_with(prefix)).collect();
        keys.sort_unstable();
        for k in keys {
            if !out.contains(&k) {
                out.push(k);
            }
        }
    }
    out
}

/// The glyph pages of CAP files, found among the pictures of `Sources`
/// (decoded as they are first looked at).
pub(super) struct Pages<'s> {
    sources: &'s Sources,
    decoded: HashMap<&'s str, Vec<Column>>,
}

impl<'s> Pages<'s> {
    pub(super) fn new(sources: &'s Sources) -> Pages<'s> {
        Pages { sources, decoded: HashMap::new() }
    }

    /// The page of CAP file `cap` of file `key`: the column where most of
    /// its glyphs are known, if most are.
    pub(super) fn find(&mut self, key: &str, cap: &Cap) -> Option<&Column> {
        let used = uses(cap, 300);
        if used.is_empty() {
            return None;
        }
        let mut best: Option<(&str, usize, f64)> = None;
        'search: for k in candidates(self.sources, key) {
            let sources = self.sources;
            let columns =
                self.decoded.entry(k).or_insert_with(|| sources.pictures[k].iter().flat_map(|p| picture(p)).collect());
            for (i, column) in columns.iter().enumerate() {
                let s = share(cap, column, &used);
                if best.is_none_or(|b| s > b.2) {
                    best = Some((k, i, s));
                }
                if s > 0.9 {
                    break 'search;
                }
            }
        }
        best.filter(|b| b.2 > 0.5).map(|b| &self.decoded[b.0][b.1])
    }
}

/// The glyph pages of every CAP file (by their sheets' hashes), with the
/// cells (u, v, width, height) of their speakers' name labels: wherever
/// they load, the game draws them with the captions' palettes.
pub fn glyph_pages(sources: &Sources) -> HashMap<u64, Vec<[u8; 4]>> {
    let mut pages = Pages::new(sources);
    let mut out: HashMap<u64, Vec<[u8; 4]>> = HashMap::new();
    for (key, _, payload) in &sources.bundles {
        for bytes in caps(payload) {
            let Some(cap) = Cap::new(bytes) else { continue };
            let Some(page) = pages.find(key, &cap) else { continue };
            let labels = out.entry(crate::mods::sheets::sheet_hash(&page.rows)).or_default();
            for line in cap.lines() {
                let cell = cap.cell(usize::from(line.title).wrapping_sub(1));
                if line.title != 0 && !labels.contains(&cell) {
                    labels.push(cell);
                }
            }
        }
    }
    out
}

/// The glyph pages of every CAP file (by their sheets' hashes), with each
/// cell (u, v, width, height) their lines draw and the text it shows (a
/// character, `[NAME]` for a speaker's name label, empty when not known),
/// sorted by row, then column: a map of the page for font makers. The game
/// draws capital I with the `l` glyph.
pub fn glyph_maps(sources: &Sources) -> HashMap<u64, Vec<([u8; 4], String)>> {
    let mut pages = Pages::new(sources);
    let mut out: HashMap<u64, Vec<([u8; 4], String)>> = HashMap::new();
    for (key, _, payload) in &sources.bundles {
        for bytes in caps(payload) {
            let Some(cap) = Cap::new(bytes) else { continue };
            let Some(page) = pages.find(key, &cap) else { continue };
            let cells = out.entry(crate::mods::sheets::sheet_hash(&page.rows)).or_default();
            let titles = cap.lines().into_iter().filter(|l| l.title != 0).map(|l| usize::from(l.title) - 1);
            for index in uses(&cap, usize::MAX).into_iter().chain(titles) {
                let cell = cap.cell(index);
                if cell[2] == 0 || cell[3] == 0 || cells.iter().any(|c| c.0 == cell) {
                    continue;
                }
                cells.push((cell, recognize(&glyph(page, cell)).map(|t| t.into_owned()).unwrap_or_default()));
            }
        }
    }
    for cells in out.values_mut() {
        cells.sort_by_key(|(c, _)| (c[1], c[0]));
    }
    out
}

/// Each CAP file's glyphs' texts (by its hash; by glyph index, None where
/// not known or not drawn by its lines): what a mod's font draws instead
/// (mods/capfont.rs), and `[NAME]` for its speakers' name labels
/// (labels.rs).
pub fn cap_texts(sources: &Sources) -> HashMap<u64, Vec<Option<String>>> {
    // The bundles shared out among the cores, each part with pages of its
    // own; the first file of each hash, in disc order, kept.
    let cores = std::thread::available_parallelism().map_or(4, |n| n.get());
    let per = sources.bundles.len().div_ceil(cores).max(1);
    let parts: Vec<Vec<(u64, Vec<Option<String>>)>> = std::thread::scope(|scope| {
        let jobs: Vec<_> = sources
            .bundles
            .chunks(per)
            .map(|bundles| {
                scope.spawn(move || {
                    let mut pages = Pages::new(sources);
                    let mut found = Vec::new();
                    for (key, _, payload) in bundles {
                        for bytes in caps(payload) {
                            let Some(cap) = Cap::new(bytes) else { continue };
                            let Some(page) = pages.find(key, &cap) else { continue };
                            let used = uses(&cap, usize::MAX);
                            let mut texts = vec![None; used.iter().max().map_or(0, |m| m + 1)];
                            for index in used {
                                texts[index] = recognize(&glyph(page, cap.cell(index))).map(|t| t.into_owned());
                            }
                            // And the speakers' name labels (labels.rs), as
                            // `[NAME]` (one is labelled without: No.9). A
                            // title pointing at a letter the lines draw is
                            // no label (a line showing none: its h).
                            let letters: std::collections::HashSet<usize> =
                                uses(&cap, usize::MAX).into_iter().collect();
                            for line in cap.lines().into_iter().filter(|l| l.title != 0) {
                                let index = usize::from(line.title - 1);
                                if letters.contains(&index) {
                                    continue;
                                }
                                if index >= texts.len() {
                                    texts.resize(index + 1, None);
                                }
                                let name = recognize(&glyph(page, cap.cell(index)));
                                texts[index] =
                                    name.map(|n| if n.starts_with('[') { n.into_owned() } else { format!("[{n}]") });
                            }
                            found.push((cap.hash(), texts));
                        }
                    }
                    found
                })
            })
            .collect();
        jobs.into_iter().map(|j| j.join().unwrap_or_default()).collect()
    });
    let mut out = HashMap::new();
    for (hash, texts) in parts.into_iter().flatten() {
        out.entry(hash).or_insert(texts);
    }
    out
}

/// The game's dialogue font: for each text the glyph table knows (a
/// character, or a speaker's name label), its glyph picture (size, then
/// texels) as the game's pages have it.
pub fn font(sources: &Sources) -> HashMap<&'static str, Vec<u8>> {
    let wanted = known().values().collect::<std::collections::HashSet<_>>().len();
    let mut pages = Pages::new(sources);
    let mut out = HashMap::new();
    for (key, _, payload) in &sources.bundles {
        for bytes in caps(payload) {
            let Some(cap) = Cap::new(bytes) else { continue };
            let Some(page) = pages.find(key, &cap) else { continue };
            for index in uses(&cap, usize::MAX) {
                let shape = glyph(page, cap.cell(index));
                if let Some(&t) = known().get(&super::fnv1a(shape.iter().copied())) {
                    out.entry(t).or_insert(shape);
                }
            }
            if out.len() == wanted {
                return out;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Capital I comes back where the shared glyph stands for it.
    #[test]
    fn capital_i() {
        assert_eq!(
            fix_capital_i("l've seen it. ls it? lt's all, l think. OK lT"),
            "I've seen it. Is it? It's all, I think. OK IT"
        );
        assert_eq!(fix_capital_i("lots of lines fill the hall"), "lots of lines fill the hall");
        // Longer words: I before a consonant, l before a vowel.
        assert_eq!(
            fix_capital_i("lnform me. lsso é. lmagine. Look, lá, lugar, lhe, lyrics"),
            "Inform me. Isso é. Imagine. Look, lá, lugar, lhe, lyrics"
        );
        // After a capital, before a small letter: an l (All, Ele, Please).
        assert_eq!(fix_capital_i("All. Ele. Please. MlST"), "All. Ele. Please. MIST");
    }
}
