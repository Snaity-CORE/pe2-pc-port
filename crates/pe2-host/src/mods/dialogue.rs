//! Dialogue: the lines of the game's CAP2 dialogue files as text.
//!
//! A dialogue bundle (`.pe2cap2` chunk) holds CAP files (doc/CAP_FORMAT.md
//! in the decompilation): their lines are streams of glyph indices into a
//! table of cells (u, v, width, height) of a 4-bit texture page, loaded by
//! one of the game's `.pe2img` pictures, with a few control codes. The
//! glyphs are pictures, not characters; what character each is comes from
//! `GLYPHS`, hashes of the game's glyph pictures (their cell size and texel
//! values) and the character each shows. Which picture holds a file's
//! glyphs is not recorded anywhere the extract can read, so the one where
//! most of its glyphs are known is taken.

use std::collections::HashMap;
use std::path::Path;

/// Glyph picture hash -> the text it shows (`[NAME]` for a speaker's name
/// label). Made by matching the game's glyph pictures by eye; it holds no
/// pictures.
const GLYPHS: &[(u64, &str)] = &[
    (0x6FDF84165ADA3F98, " "),
    (0xC29EA8516D3FDA7D, "e"),
    (0xE375E26F3851DE60, "t"),
    (0xD28C1E48110A7C1D, "o"),
    (0xE749FCD88CD93AAC, "a"),
    (0x7EBEE45E4C5A3943, "n"),
    (0x98A8A57845A8AB1B, "."),
    (0x2B5DED1F6B471494, "s"),
    (0x76AF88272BBC94C9, "i"),
    (0x5504185613D58134, "r"),
    (0xE24F0146D1A4354D, "l"),
    (0xEB0FC74E49BEE402, "h"),
    (0x6DE8A62D1B1CC35B, "d"),
    (0x32480DC0B7033BB1, "u"),
    (0xB8DE220347476410, "g"),
    (0x2D15598A175766A9, "c"),
    (0xD24E2C94C5F38319, "m"),
    (0x7A9E9406E46B11E8, "y"),
    (0xE08831EB07139E07, "f"),
    (0x34AE7F106492D64F, "w"),
    (0xDD3CF55515F9325D, "'"),
    (0x26BC581661906D57, "p"),
    (0x3C0B0B5696316F45, "k"),
    (0x4A1068F2A21A5813, "b"),
    (0xEB4279649BFC0235, "T"),
    (0x975768B694714708, ","),
    (0xD31784C2589BF35B, "v"),
    (0x671D4D68A88D07DF, "A"),
    (0x9B8373757C1CD89C, "?"),
    (0xC801A9866B3B0ACB, "!"),
    (0x890D85FC75F97459, "\""),
    (0xA17439C73B9CC341, "W"),
    (0x7587A82104C5B02A, "N"),
    (0xC976F0DF2E4F005F, "S"),
    (0x7F0E84CA4A538116, "H"),
    (0x26EC97B462723CFF, "M"),
    (0xF5779A93D1EA1A60, "C"),
    (0xDE2D495570599AD4, "[AYA]"),
    (0x694FCE67A33ADEAA, "-"),
    (0xEA619F2C283F5570, "B"),
    (0x11C626969C19FD08, "P"),
    (0x0DD627A937BD68A1, "Y"),
    (0x08328807B4EB6FED, ""),
    (0x668944725983039C, "O"),
    (0x392C967EF3920B8D, "L"),
    (0x622A690E3E6D402B, "[BALDWIN]"),
    (0xC27A051B50521483, "j"),
    (0x4C985B3CDA74DFE3, "E"),
    (0xA67C9FB0196E7248, "["),
    (0x0015B6BEE9288B04, "]"),
    (0x21C30E5E4D0BDBA6, "R"),
    (0x35B931C3D37A252F, "G"),
    (0xFCFF94B2F16D6E40, "D"),
    (0x730048DFDD6921F4, "x"),
    (0xB950C9CEBE297C43, "F"),
    (0x3D8673F7F4881C1F, "1"),
    (0xF4C45BDA39FCA2A7, "U"),
    (0xF1F2F321B0B43DDF, "[JODIE]"),
    (0x7B2BA102FACA479C, "z"),
    (0x2EB3946B1A5DF32D, "[PIERCE]"),
    (0x5453DDCE1B1E93E3, "[RUPERT]"),
    (0x2961EA66385CC290, "K"),
    (0xF9589DA28EE33F8B, "J"),
    (0x92A6FE11D88EA6F8, "2"),
    (0x69A5BD2385888089, "[DOUGLAS]"),
    (0xA6F8FBA7F614C33A, "[KYLE]"),
    (0xD48AA8A447FEDFD1, "8"),
    (0x850A83D6D4CDEDBF, "q"),
    (0x5E0438C2F948DD84, "9"),
    (0x945DAABD93B61223, "0"),
    (0x88A41484D509AEAC, "3"),
    (0x005337A725BED058, "[K.MADIGAN]"),
    (0x9891D6C4BE47FBC4, ")"),
    (0x0CFBB279757BEA1E, "Q"),
    (0x41A0774485B7EBBD, "5"),
    (0xA6D2A93F0665B299, ":"),
    (0xE5B745B6BE3D9838, "[???]"),
    (0x7B428D2953018AC7, "V"),
    (0x81B530DF05ECDCFE, "4"),
    (0x80A9D2A002C880B5, "6"),
    (0x91AC593DBCB628D1, "[PRESIDENT]"),
    (0xDE7A8D21D536209C, "7"),
    (0x0DEF4422477696F1, "[STAFF]"),
    (0xD137A3B2C6778248, "[WARNING]"),
    (0x71B1C224FBF9F294, "[2ND LIEUTENANT]"),
    (0xED9936A36A82493B, "No.9"),
    (0x8357EFFE0CBE7630, "("),
    (0x533AAEAD09385C1E, "&"),
    (0xD520B46B09F57BF1, "[MALE VOICE]"),
    (0x14708AB2652BB40D, "[SERGEANT]"),
    (0xFA100D1448E84963, "Z"),
    (0x7BDA00A949632B16, "[EVE]"),
    (0x27F40198C25F2194, "[SWAT OFFICER]"),
    (0x8C58729572CD76AC, "/"),
    (0x49564E94E7F146F7, "#"),
    (0xCBA05B295A900B7D, "X"),
    (0xE56BB28F2116DDFA, "[POLICE SGT.]"),
    (0x33655C34958B66D4, "[PRIVATE]"),
    (0xB9C9CC374FCBEF46, "[TELEGRAPHIST]"),
    (0x23013C6A91DFA7EC, "[POLICE OFFICER]"),
    (0x098C3463114A7B6B, ";"),
    (0x2246AA992F961DA9, ">"),
    (0x34E3C199399B737D, "[WARNING]"),
    (0x16F43C48FCEC6D0D, "[CREATURE]"),
    (0x748DF6D4FBCB5BDD, "*"),
    (0xD2AC0C49E0960A87, "="),
    (0xF94940C21EDF6D73, "%"),
];

const END: u16 = 0xFFFF;
const NEWLINE: u16 = 0xFFFE;
/// A narrow space (3 pixels).
const GAP: u16 = 0xFFFD;
/// A 64-halfword column of a picture is 32-row strips of this many bytes.
const STRIP_BYTES: usize = 0x1000;

fn u16le(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32le(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// One LZSS stream from `stream[*at..]` (a 256-byte ring dictionary; flag
/// 1: a literal byte, 0: an 8-bit offset, 0 ending the stream, and a 4-bit
/// length + 2), `*at` left after it.
pub fn lzss(stream: &[u8], at: &mut usize) -> Vec<u8> {
    let (mut dict, mut next) = ([0u8; 256], 0u8);
    let (mut byte, mut mask) = (0u8, 0u8);
    let mut out = Vec::new();
    let mut bits = |n: u32, at: &mut usize| -> Option<u32> {
        let mut v = 0;
        for _ in 0..n {
            if mask == 0 {
                byte = *stream.get(*at)?;
                *at += 1;
                mask = 0x80;
            }
            v = (v << 1) | u32::from(byte & mask != 0);
            mask >>= 1;
        }
        Some(v)
    };
    while let Some(flag) = bits(1, at) {
        if flag == 1 {
            let Some(b) = bits(8, at) else { break };
            dict[usize::from(next)] = b as u8;
            next = next.wrapping_add(1);
            out.push(b as u8);
        } else {
            let Some(offset) = bits(8, at) else { break };
            if offset == 0 {
                break;
            }
            let Some(len) = bits(4, at) else { break };
            let mut from = (offset as u8).wrapping_sub(1);
            for _ in 0..len + 2 {
                let b = dict[usize::from(from)];
                dict[usize::from(next)] = b;
                next = next.wrapping_add(1);
                out.push(b);
                from = from.wrapping_add(1);
            }
        }
    }
    out
}

/// The LZSS strips of a picture: independent streams, each of up to
/// STRIP_BYTES.
fn strips(stream: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut at = 0;
    loop {
        while stream.get(at) == Some(&0) {
            at += 1;
        }
        if at >= stream.len() {
            break;
        }
        let mut strip = lzss(stream, &mut at);
        if strip.is_empty() {
            break;
        }
        strip.resize(STRIP_BYTES, 0);
        out.push(strip);
    }
    out
}

/// A column of a picture as loaded into VRAM: where (in halfwords and
/// rows), and its rows of 64 halfwords.
pub struct Column {
    pub x: u16,
    pub y: u16,
    pub rows: Vec<[u16; 64]>,
}

/// A `.pe2img` picture's columns (its table of columns, then their strips).
pub fn picture(data: &[u8]) -> Vec<Column> {
    let mut entries = Vec::new();
    let mut end_y = 0;
    let mut at = 0;
    while let (Some(x), Some(y), Some(offset)) = (u16le(data, at), u16le(data, at + 2), u32le(data, at + 4)) {
        at += 8;
        if x == 0xFFFF {
            end_y = y;
            break;
        }
        entries.push((x, y, offset as usize));
    }
    let Some(&(_, _, first)) = entries.first() else { return Vec::new() };
    let height = match (entries.len(), end_y) {
        (1, 0xFFFF) => 0x40,
        (1, y) if y & 0x8000 != 0 => usize::from(y & 0x7FFF),
        _ => 0x100,
    };
    let mut strips = strips(data.get(first..).unwrap_or_default()).into_iter();
    entries
        .iter()
        .map(|&(x, y, _)| {
            let mut rows = Vec::with_capacity(height);
            for _ in 0..height.div_ceil(32) {
                let strip = strips.next().unwrap_or_else(|| vec![0; STRIP_BYTES]);
                for r in strip.as_chunks::<128>().0 {
                    rows.push(std::array::from_fn(|i| u16::from_le_bytes([r[i * 2], r[i * 2 + 1]])));
                }
            }
            rows.truncate(height);
            Column { x, y, rows }
        })
        .collect()
}

/// A glyph cell's picture: its size, then its 4-bit texels, from a page
/// starting at `column`.
pub fn glyph(column: &Column, cell: [u8; 4]) -> Vec<u8> {
    let [u, v, w, h] = cell;
    let mut out = vec![w, h];
    for y in 0..usize::from(h) {
        let row = column.rows.get(usize::from(v) + y);
        for x in 0..usize::from(w) {
            let t = usize::from(u) + x;
            let word = row.and_then(|r| r.get(t / 4)).copied().unwrap_or(0);
            out.push(((word >> ((t % 4) * 4)) & 0xF) as u8);
        }
    }
    out
}

/// A line of a CAP file: where its record is (the file offset), its
/// speaker's name label (a glyph index + 1; 0: none) and its codes.
pub struct Line {
    pub record: usize,
    pub title: u8,
    pub codes: Vec<u16>,
}

/// A CAP file's parts: its glyph cells, records and command table offsets.
pub struct Cap<'a> {
    pub bytes: &'a [u8],
    glyphs: usize,
    sequences: usize,
    commands: usize,
}

impl<'a> Cap<'a> {
    pub fn new(bytes: &'a [u8]) -> Option<Cap<'a>> {
        if bytes.get(..3)? != b"CAP" {
            return None;
        }
        let (g, s, c) = (u32le(bytes, 8)? as usize, u32le(bytes, 12)? as usize, u32le(bytes, 16)? as usize);
        (g < bytes.len() && s < bytes.len() && c < bytes.len()).then_some(Cap {
            bytes,
            glyphs: g,
            sequences: s,
            commands: c,
        })
    }

    /// The file's length: up to the end of its command table.
    pub fn len(&self) -> usize {
        let count = u32le(self.bytes, self.commands).unwrap_or(0) as usize;
        (self.commands + 4 + count * 4).min(self.bytes.len())
    }

    /// The hash a CAP file is known by (of its bytes as on disc).
    pub fn hash(&self) -> u64 {
        super::fnv1a(self.bytes[..self.len()].iter().copied())
    }

    pub fn cell(&self, index: usize) -> [u8; 4] {
        let Some(at) = index.checked_mul(4).and_then(|n| self.glyphs.checked_add(n)) else { return [0; 4] };
        let Some(end) = at.checked_add(4) else { return [0; 4] };
        self.bytes.get(at..end).and_then(|c| c.try_into().ok()).unwrap_or([0; 4])
    }

    /// Its lines, as Gp_RelocCapFile walks the records: `count` of them,
    /// an end marker skipping the next slot (the next sequence's command).
    pub fn lines(&self) -> Vec<Line> {
        let count = u16le(self.bytes, self.sequences).unwrap_or(0) as i16;
        let mut out = Vec::new();
        let mut at = self.sequences + 0x10;
        for _ in 0..count.max(0) {
            let Some(text) = u32le(self.bytes, at + 8) else { break };
            if text == u32::MAX {
                at += 24;
                continue;
            }
            let mut codes = Vec::new();
            let mut q = text as usize;
            while let Some(v) = u16le(self.bytes, q) {
                if v == END {
                    break;
                }
                codes.push(v);
                q += 2;
            }
            out.push(Line { record: at, title: self.bytes[at], codes });
            at += 12;
        }
        out
    }
}

/// Whether a code draws a glyph (else it is a control).
pub fn is_glyph(code: u16) -> bool {
    code & 0x8000 == 0
}

/// The resources of a dialogue bundle of `kind` (2: images, 3: data), in
/// order.
pub fn resources(payload: &[u8], kind: u8) -> Vec<&[u8]> {
    slots(payload).into_iter().filter(|s| s.1 == kind).map(|s| s.2).collect()
}

/// A bundle's resources with their slots: the place of each in its
/// directory, which is where the game keeps it (fs.c D_8006C338), its kind
/// and its contents.
pub fn slots(payload: &[u8]) -> Vec<(usize, u8, &[u8])> {
    const DIRECTORY: usize = 0x7F0;
    let entries: Vec<(usize, u8, usize, usize)> = (0..50)
        .filter_map(|k| {
            let e = payload.get(k * 16..k * 16 + 16)?;
            if e[0] == 0 {
                return None;
            }
            Some((k, e[0], u32le(e, 4)? as usize, u32le(e, 8)? as usize))
        })
        .collect();
    let Some(base) = entries.iter().map(|e| e.3).min() else { return Vec::new() };
    entries
        .iter()
        .filter_map(|&(k, kind, size, dest)| {
            Some((k, kind, payload.get(DIRECTORY + dest - base..DIRECTORY + dest - base + size)?))
        })
        .collect()
}

/// The CAP files of a dialogue bundle (its resources of untyped data that
/// are CAP files), in order.
pub fn caps(payload: &[u8]) -> Vec<&[u8]> {
    resources(payload, 3).into_iter().filter(|b| b.starts_with(b"CAP")).collect()
}

/// Glyph picture hashes -> text.
fn known() -> &'static HashMap<u64, &'static str> {
    static MAP: std::sync::OnceLock<HashMap<u64, &'static str>> = std::sync::OnceLock::new();
    MAP.get_or_init(|| GLYPHS.iter().copied().collect())
}

/// The glyph indices a CAP file's lines draw (up to `limit`).
fn uses(cap: &Cap, limit: usize) -> Vec<usize> {
    let lines = cap.lines();
    let codes = lines.iter().flat_map(|l| l.codes.iter());
    codes.filter(|&&c| is_glyph(c)).map(|&c| usize::from(c & 0x3FF)).take(limit).collect()
}

/// The share of `uses` whose glyph pictures in `column` are known.
fn share(cap: &Cap, column: &Column, uses: &[usize]) -> f64 {
    let hits = uses.iter().filter(|&&i| known().contains_key(&super::fnv1a(glyph(column, cap.cell(i))))).count();
    hits as f64 / uses.len().max(1) as f64
}

/// A line's text: glyphs as their characters, line breaks, `{_}` for the
/// narrow space, `{color N}` where the colour changes, `{XXXX}` for the
/// other controls and `{gN}` for a glyph not known.
pub fn text(cap: &Cap, page: Option<&Column>, codes: &[u16]) -> String {
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
                match page.and_then(|p| known().get(&super::fnv1a(glyph(p, cap.cell(index))))) {
                    Some(t) => out.push_str(t),
                    None => out.push_str(&format!("{{g{index}}}")),
                }
            }
            c => out.push_str(&format!("{{{c:04X}}}")),
        }
    }
    fix_capital_i(&out)
}

/// Capital I and small l are one glyph: an `l` alone, before an
/// apostrophe, next to capitals, or starting a word as I does (In, It, If,
/// Is) is an I.
fn fix_capital_i(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let letter = |i: usize| chars.get(i).is_some_and(|c| c.is_alphabetic());
    let upper = |i: usize| chars.get(i).is_some_and(|c| c.is_uppercase());
    (0..chars.len())
        .map(|i| {
            if chars[i] != 'l' {
                return chars[i];
            }
            let starts = i == 0 || !letter(i - 1);
            let next = chars.get(i + 1).copied();
            let is_i = (starts && !letter(i + 1))
                || (starts && next == Some('\''))
                || (starts && matches!(next, Some('n' | 't' | 'f' | 's')) && !letter(i + 2))
                || (i > 0 && upper(i - 1))
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
    pub palettes: HashMap<String, Vec<Vec<u8>>>,
    /// File key -> its code and data packages (`.pe2pkg`): where they load
    /// in RAM, and their contents (still compressed); and its room
    /// backgrounds' contents (`.bs`).
    pub packages: HashMap<String, Vec<(u32, Vec<u8>)>>,
    pub backgrounds: HashMap<String, Vec<Vec<u8>>>,
    /// (file key, chunk index, contents) of each `.pe2cap2` chunk.
    pub bundles: Vec<(String, usize, Vec<u8>)>,
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
struct Pages<'s> {
    sources: &'s Sources,
    decoded: HashMap<&'s str, Vec<Column>>,
}

impl<'s> Pages<'s> {
    fn new(sources: &'s Sources) -> Pages<'s> {
        Pages { sources, decoded: HashMap::new() }
    }

    /// The page of CAP file `cap` of file `key`: the column where most of
    /// its glyphs are known, if most are.
    fn find(&mut self, key: &str, cap: &Cap) -> Option<&Column> {
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
            let labels = out.entry(super::sheets::sheet_hash(&page.rows)).or_default();
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

/// Saves each CAP file's lines as `<file>/<chunk>-<n>.toml` under `out`,
/// to be translated (see docs/MODDING.md). Answers how many files and
/// lines.
pub fn extract(sources: &Sources, out: &Path) -> std::io::Result<(usize, usize)> {
    let mut pages = Pages::new(sources);
    let (mut files, mut lines) = (0, 0);
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
            for line in cap.lines() {
                if !line.codes.iter().any(|&c| is_glyph(c)) {
                    continue;
                }
                let original = text(&cap, page, &line.codes);
                let original = original.strip_suffix('\n').unwrap_or(&original);
                toml.push_str(&format!("\n[[line]]\nrecord = 0x{:04X}\n", line.record));
                if line.title != 0 {
                    let speaker = text(&cap, page, &[u16::from(line.title - 1)]);
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
    Ok((files, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_glyph_index_cannot_wrap_into_another_cell() {
        let cap = Cap { bytes: &[1, 2, 3, 4], glyphs: 0, sequences: 0, commands: 0 };
        assert_eq!(cap.cell(0), [1, 2, 3, 4]);
        assert_eq!(cap.cell(usize::MAX), [0; 4]);
    }

    /// Capital I comes back where the shared glyph stands for it.
    #[test]
    fn capital_i() {
        assert_eq!(
            fix_capital_i("l've seen it. ls it? lt's all, l think. OK lT"),
            "I've seen it. Is it? It's all, I think. OK IT"
        );
        assert_eq!(fix_capital_i("lots of lines fill the hall"), "lots of lines fill the hall");
    }

    /// A strip decodes literals and back-references into its dictionary.
    #[test]
    fn lzss_strip() {
        let mut w = BitWriter::default();
        for b in *b"ab" {
            w.put(1, 1);
            w.put(u32::from(b), 8);
        }
        // Back to dictionary position 0 (stored as 1), 2 + 2 bytes; then
        // the end (offset 0).
        w.put(0, 1);
        w.put(1, 8);
        w.put(2, 4);
        w.put(0, 1);
        w.put(0, 8);
        let s = strips(&w.bytes);
        assert_eq!(&s[0][..6], b"ababab");
    }

    #[derive(Default)]
    struct BitWriter {
        bytes: Vec<u8>,
        used: u32,
    }

    impl BitWriter {
        fn put(&mut self, v: u32, n: u32) {
            for i in (0..n).rev() {
                if self.used.is_multiple_of(8) {
                    self.bytes.push(0);
                }
                if v >> i & 1 != 0 {
                    *self.bytes.last_mut().unwrap() |= 0x80 >> (self.used % 8);
                }
                self.used += 1;
            }
        }
    }
}
