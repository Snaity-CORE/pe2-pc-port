//! What the import does the way PE2PC's Python tools did (tools/import_*.py
//! in pe2-pc-port, which this program replaces): Python's tests of
//! letters and spaces, its `repr` of bytes (the names of the menus' font's
//! pictures are hashes of it), its first-of-equals `max` and `min`, and
//! difflib's matching (`SequenceMatcher`).

use std::collections::HashMap;
use std::hash::Hash;

/// Python's `str.isupper()`: a cased letter, and no small one.
pub fn is_upper(text: &str) -> bool {
    text.chars().any(char::is_uppercase) && !text.chars().any(char::is_lowercase)
}

/// Python's `str.isspace()` of one character.
pub fn is_space(c: char) -> bool {
    c.is_whitespace() || ('\x1c'..='\x1f').contains(&c)
}

/// Python's `str.strip()`.
pub fn strip(text: &str) -> &str {
    text.trim_matches(is_space)
}

/// Python's `repr` of a bytes object.
pub fn bytes_repr(data: &[u8]) -> String {
    let quote = if data.contains(&b'\'') && !data.contains(&b'"') { '"' } else { '\'' };
    let mut out = format!("b{quote}");
    for &b in data {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'\t' => out.push_str("\\t"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            _ if b as char == quote => {
                out.push('\\');
                out.push(quote);
            }
            0x20..=0x7E => out.push(b as char),
            _ => out.push_str(&format!("\\x{b:02x}")),
        }
    }
    out.push(quote);
    out
}

/// Python's `repr` of a tuple of bytes objects.
pub fn rows_repr(rows: &[Vec<u8>]) -> String {
    let items: Vec<String> = rows.iter().map(|r| bytes_repr(r)).collect();
    match items.len() {
        1 => format!("({},)", items[0]),
        _ => format!("({})", items.join(", ")),
    }
}

/// The first 12 hexadecimal digits of the SHA-1 of `text`.
pub fn short_hash(text: &str) -> String {
    sha1_smol::Sha1::from(text).digest().to_string()[..12].to_string()
}

/// The first item with the greatest key (Python's `max`; Rust's gives the
/// last).
pub fn first_max_by<T, K: PartialOrd>(items: impl IntoIterator<Item = T>, key: impl Fn(&T) -> K) -> Option<T> {
    let mut best: Option<(T, K)> = None;
    for item in items {
        let k = key(&item);
        if best.as_ref().is_none_or(|(_, b)| k > *b) {
            best = Some((item, k));
        }
    }
    best.map(|(item, _)| item)
}

/// The first item with the smallest key (Python's `min`).
pub fn first_min_by<T, K: PartialOrd>(items: impl IntoIterator<Item = T>, key: impl Fn(&T) -> K) -> Option<T> {
    let mut best: Option<(T, K)> = None;
    for item in items {
        let k = key(&item);
        if best.as_ref().is_none_or(|(_, b)| k < *b) {
            best = Some((item, k));
        }
    }
    best.map(|(item, _)| item)
}

/// difflib's `SequenceMatcher(None, a, b, autojunk)`: `matching_blocks`,
/// (start in a, start in b, length), in order, without the final empty one.
pub fn matching_blocks<T: Eq + Hash>(a: &[T], b: &[T], autojunk: bool) -> Vec<(usize, usize, usize)> {
    let mut b2j: HashMap<&T, Vec<usize>> = HashMap::new();
    for (j, x) in b.iter().enumerate() {
        b2j.entry(x).or_default().push(j);
    }
    // Popular elements (more than 1% of a long b) are junk with autojunk.
    if autojunk && b.len() >= 200 {
        let most = b.len() / 100 + 1;
        b2j.retain(|_, js| js.len() <= most);
    }
    // The length of the match ending at the last element of a looked at
    // and each j of b (at j + 1), for that element and the next: difflib's
    // j2len, kept all zeros between calls.
    let mut lens = [vec![0usize; b.len() + 1], vec![0usize; b.len() + 1]];
    let mut longest = |alo: usize, ahi: usize, blo: usize, bhi: usize| {
        let (mut besti, mut bestj, mut bestsize) = (alo, blo, 0);
        let [j2len, next] = &mut lens;
        let (mut set, mut next_set) = (Vec::new(), Vec::new());
        for (i, x) in a.iter().enumerate().take(ahi).skip(alo) {
            for &j in b2j.get(x).map_or(&[][..], Vec::as_slice) {
                if j < blo {
                    continue;
                }
                if j >= bhi {
                    break;
                }
                let k = j2len[j] + 1;
                next[j + 1] = k;
                next_set.push(j + 1);
                if k > bestsize {
                    (besti, bestj, bestsize) = (i + 1 - k, j + 1 - k, k);
                }
            }
            for &j in &set {
                j2len[j] = 0;
            }
            set.clear();
            std::mem::swap(j2len, next);
            std::mem::swap(&mut set, &mut next_set);
        }
        for &j in &set {
            j2len[j] = 0;
        }
        // Grown over equal elements the popular ones left out (difflib's
        // isjunk is None: no element is junk).
        while besti > alo && bestj > blo && a[besti - 1] == b[bestj - 1] {
            (besti, bestj, bestsize) = (besti - 1, bestj - 1, bestsize + 1);
        }
        while besti + bestsize < ahi && bestj + bestsize < bhi && a[besti + bestsize] == b[bestj + bestsize] {
            bestsize += 1;
        }
        (besti, bestj, bestsize)
    };
    let mut queue = vec![(0, a.len(), 0, b.len())];
    let mut blocks = Vec::new();
    while let Some((alo, ahi, blo, bhi)) = queue.pop() {
        let (i, j, k) = longest(alo, ahi, blo, bhi);
        if k > 0 {
            blocks.push((i, j, k));
            if alo < i && blo < j {
                queue.push((alo, i, blo, j));
            }
            if i + k < ahi && j + k < bhi {
                queue.push((i + k, ahi, j + k, bhi));
            }
        }
    }
    blocks.sort_unstable();
    // Adjacent blocks as one.
    let mut out: Vec<(usize, usize, usize)> = Vec::new();
    for (i, j, k) in blocks {
        match out.last_mut() {
            Some((i1, j1, k1)) if *i1 + *k1 == i && *j1 + *k1 == j => *k1 += k,
            _ => out.push((i, j, k)),
        }
    }
    out
}

/// difflib's `SequenceMatcher(None, a, b).ratio()` of two texts.
pub fn ratio(a: &str, b: &str) -> f64 {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let total = a.len() + b.len();
    if total == 0 {
        return 1.0;
    }
    let matched: usize = matching_blocks(&a, &b, true).iter().map(|m| m.2).sum();
    2.0 * matched as f64 / total as f64
}

/// The best alignment of `n` and `m` elements (Needleman-Wunsch on
/// `score(i, j)`, `gap` for an element left alone): (i, j) pairs in order,
/// either None where the other list has none.
pub fn align(n: usize, m: usize, gap: i32, score: impl Fn(usize, usize) -> i32) -> Vec<(Option<usize>, Option<usize>)> {
    let mut best = vec![vec![0i32; m + 1]; n + 1];
    for (i, row) in best.iter_mut().enumerate() {
        row[0] = i as i32 * gap;
    }
    for (j, cell) in best[0].iter_mut().enumerate() {
        *cell = j as i32 * gap;
    }
    for i in 1..=n {
        for j in 1..=m {
            best[i][j] = (best[i - 1][j - 1] + score(i - 1, j - 1)).max(best[i - 1][j] + gap).max(best[i][j - 1] + gap);
        }
    }
    let (mut pairs, mut i, mut j) = (Vec::new(), n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && best[i][j] == best[i - 1][j - 1] + score(i - 1, j - 1) {
            pairs.push((Some(i - 1), Some(j - 1)));
            (i, j) = (i - 1, j - 1);
        } else if i > 0 && (j == 0 || best[i][j] == best[i - 1][j] + gap) {
            pairs.push((Some(i - 1), None));
            i -= 1;
        } else {
            pairs.push((None, Some(j - 1)));
            j -= 1;
        }
    }
    pairs.reverse();
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reprs_are_pythons() {
        assert_eq!(bytes_repr(b"a'\x00\x7f\\"), "b\"a'\\x00\\x7f\\\\\"");
        assert_eq!(bytes_repr(b"a'\""), "b'a\\'\"'");
        assert_eq!(rows_repr(&[vec![1]]), "(b'\\x01',)");
        assert_eq!(rows_repr(&[vec![], vec![b'x']]), "(b'', b'x')");
    }

    #[test]
    fn matching_is_difflibs() {
        // SequenceMatcher(None, "abxcd", "abcd").get_matching_blocks()
        assert_eq!(matching_blocks(b"abxcd", b"abcd", false), [(0, 0, 2), (3, 2, 2)]);
        // SequenceMatcher(None, "paralizer", "paralysantes").ratio()
        assert!((ratio("paralizer", "paralysantes") - 0.571_428_571_428_571_4).abs() < 1e-12);
    }
}
