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

/// Glyph shape hash (`shape_hash`) -> the text it shows: glyphs drawn as
/// the game's own but framed otherwise in their cells (the Brazilian
/// Portuguese translation's discs), and the letters they add. Made from the
/// game's glyphs and by eye; it holds no pictures.
const SHAPES: &[(u64, &str)] = &[
    // The game's glyphs (from their pictures in the game).
    (0x005337A725BED058, "[K.MADIGAN]"),
    (0x01259546E302B6F9, "!"),
    (0x05306F5FF6FB6947, "e"),
    (0x05BDD9B17FD80272, "C"),
    (0x08327A07B4EB5823, " "),
    (0x08328807B4EB6FED, ""),
    (0x0BBB2271C14641A9, "0"),
    (0x0DEF4422477696F1, "[STAFF]"),
    (0x14708AB2652BB40D, "[SERGEANT]"),
    (0x16F43C48FCEC6D0D, "[CREATURE]"),
    (0x1824C9A91892EB6D, "j"),
    (0x1835920BF0DDF535, "\""),
    (0x1BEFB4DDCD56FEA7, "m"),
    (0x1E1AAD364A0676E7, "U"),
    (0x21235D6E7C56D764, "R"),
    (0x2168B720F238DEC6, "K"),
    (0x23013C6A91DFA7EC, "[POLICE OFFICER]"),
    (0x27F40198C25F2194, "[SWAT OFFICER]"),
    (0x2967825901F81365, "Z"),
    (0x2A4F5AF206FA7878, "N"),
    (0x2EB3946B1A5DF32D, "[PIERCE]"),
    (0x30624E17A0D57C8A, "z"),
    (0x33655C34958B66D4, "[PRIVATE]"),
    (0x34E3C199399B737D, "[WARNING]"),
    (0x39D5285A6A4139F8, "2"),
    (0x3B511F6C40586A1E, "O"),
    (0x3E9BAA1639ABCFD3, ":"),
    (0x4771640C3C9CE413, "i"),
    (0x4925E9E31D396002, "/"),
    (0x4F79D4C4F726A2F5, "q"),
    (0x500681E81661A362, "]"),
    (0x502D83E8FED03105, "="),
    (0x5453DDCE1B1E93E3, "[RUPERT]"),
    (0x56E32E6A073B5C72, "P"),
    (0x6184BD6C57C3408E, "("),
    (0x622A690E3E6D402B, "[BALDWIN]"),
    (0x63D40860332FD421, "b"),
    (0x64310DAB543CE41C, "a"),
    (0x651DBEE984781851, "d"),
    (0x683D0DDC77F208E9, "E"),
    (0x69A5BD2385888089, "[DOUGLAS]"),
    (0x6DCCDE5DB0055AC0, "H"),
    (0x6E22B8C63195CD82, "s"),
    (0x6E55349383ED334F, "G"),
    (0x71B1C224FBF9F294, "[2ND LIEUTENANT]"),
    (0x779B2DAC6BEC39FF, ";"),
    (0x78400378F8F271AA, "D"),
    (0x7A143AD555DB4BD3, "c"),
    (0x7BDA00A949632B16, "[EVE]"),
    (0x8037A87DA9D0AB5F, "L"),
    (0x820124B00FE99214, "?"),
    (0x8522C9FB6D85C6BA, "&"),
    (0x8F457F15B2422E44, "-"),
    (0x91AC593DBCB628D1, "[PRESIDENT]"),
    (0x972D4FAF0A925284, "7"),
    (0x976CB677AE2C1723, "X"),
    (0x98BB04C523D9AADB, "*"),
    (0x9A5A0C40D14F68CF, "'"),
    (0x9F46BB56F8BEFA8F, "Y"),
    (0xA17439C73B9CC341, "W"),
    (0xA6F8FBA7F614C33A, "[KYLE]"),
    (0xA8E240D7CDE88A6C, "Q"),
    (0xA9D8DC4AA5A4DF75, "%"),
    (0xB09AC2A68F7A67B7, "o"),
    (0xB478677EE281F59A, "4"),
    (0xB760230A87EAAF1B, "u"),
    (0xB9C9CC374FCBEF46, "[TELEGRAPHIST]"),
    (0xB9EAB239F7043AA9, "A"),
    (0xBA012C087562963A, ","),
    (0xBA9C35A8A1B44D19, "."),
    (0xBB1B2F3E0F08610B, "f"),
    (0xBB36F6F51B885579, "M"),
    (0xBE491597D195013F, "l"),
    (0xBF15B04261AF09C2, "x"),
    (0xC12380D304FBE78C, "3"),
    (0xC17852982D218F31, "F"),
    (0xC17F8BF73E18BF11, "n"),
    (0xC2644D3CB1951537, "5"),
    (0xC4E7EE88E1C9B3DD, ">"),
    (0xCDAA17F206FFC0E3, "8"),
    (0xD0B579D5DD30D211, "v"),
    (0xD137A3B2C6778248, "[WARNING]"),
    (0xD16CE8720827D60F, "6"),
    (0xD1B690280896BE3C, "9"),
    (0xD219CBE158BF8800, "y"),
    (0xD520B46B09F57BF1, "[MALE VOICE]"),
    (0xD5F7480ABED28AF8, "g"),
    (0xD905BE5444D2A6E0, "r"),
    (0xDA147A233BB2905B, "#"),
    (0xDE2D495570599AD4, "[AYA]"),
    (0xDE9D89207FD4B651, "V"),
    (0xDEB0D9CFB417F55B, "J"),
    (0xDEEF11DC83EA2E9D, "S"),
    (0xE55AF89C92F4F9C6, "["),
    (0xE56BB28F2116DDFA, "[POLICE SGT.]"),
    (0xE5B745B6BE3D9838, "[???]"),
    (0xE643753D3435D511, "1"),
    (0xE9B36F447D51DF4D, "p"),
    (0xED9936A36A82493B, "No.9"),
    (0xF063541E96263D7F, "T"),
    (0xF1F2F321B0B43DDF, "[JODIE]"),
    (0xF292C1FF24A8C768, "B"),
    (0xF29E555E8F75E10E, "h"),
    (0xF390A247E3635327, "k"),
    (0xF78D478490E096A9, "w"),
    (0xF7EEBC37CC242746, "t"),
    (0xFB3A3FE99DE2CFE2, ")"),
    // The Brazilian Portuguese translation's letters (by eye).
    (0xF129FD3D8D573064, "á"),
    (0xCD1EFC8101830F24, "ã"),
    (0x6226768F418F0BBF, "é"),
    (0x9E19E6F3D52CC6CB, "ç"),
    (0x83AA67318EBEEA71, "S"),
    (0x252FC300159A1527, "ê"),
    (0xF72E31B11E00F18F, "ó"),
    (0xC3D62BF8456337C8, "-"),
    (0x3A4E05F7C1BD8740, "í"),
    (0xE498984445825B59, "["),
    (0x80E405FF8EBD7CCD, "]"),
    (0x1E512C8DA92B3977, "õ"),
    (0x329AD6DD24A08C85, "É"),
    (0x0780C1DCBE6D1763, "ú"),
    (0xBBC4F6C57257B254, "à"),
    (0xD09A1646AE191397, "ô"),
    (0x30B0FF3F93188BDF, "K"),
    (0x12A80FF08775176C, "â"),
    (0x8C6AB49A68409C81, "Ê"),
    (0x4D57BF7F4CF5AD10, "]"),
    (0x018E18965D6876EC, "º"),
    (0xB3E0E5ACB9C204F0, "'"),
    (0x617B185D7C497E6E, "Ó"),
    (0x87F193D12BC8520E, "Ó"),
    (0x93DD1F9A9FBF83A1, "Ã"),
    (0xAF6A984E5495FB51, "í"),
    (0x5FA8DEA31AAB3DB7, "i"),
    (0x9F3802867E91720B, "."),
    (0x56FBA095806687A3, "Á"),
    (0x4727A5973B996538, "Z"),
    (0xDA35179DB7674D6A, "X"),
    (0x5745FDB4B4901C9F, "<"),
    (0x84A09BC4C26E804C, "("),
];

fn shapes() -> &'static HashMap<u64, &'static str> {
    static MAP: std::sync::OnceLock<HashMap<u64, &'static str>> = std::sync::OnceLock::new();
    MAP.get_or_init(|| SHAPES.iter().copied().collect())
}

/// A glyph picture's (`glyph`) shape: its texels without the empty columns
/// at its sides (its height kept, so marks stay where they are).
pub fn shape_hash(picture: &[u8]) -> u64 {
    let (w, h) = (usize::from(picture[0]), usize::from(picture[1]));
    let texels = &picture[2..];
    let used = |x: usize| (0..h).any(|y| texels[y * w + x] != 0);
    let Some(x0) = (0..w).find(|&x| used(x)) else { return super::fnv1a([0, picture[1]]) };
    let x1 = (0..w).rev().find(|&x| used(x)).unwrap_or(x0);
    let rows = (0..h).flat_map(|y| texels[y * w + x0..=y * w + x1].iter().copied());
    super::fnv1a([(x1 - x0 + 1) as u8, picture[1]].into_iter().chain(rows))
}

/// What a glyph picture shows: by its hash, else by its shape, else read
/// as a run of letters (`read`). Pictures recur (the same glyphs in many
/// files, and pages tried for each), so what each shows is kept.
fn recognize(picture: &[u8]) -> Option<std::borrow::Cow<'static, str>> {
    let hash = super::fnv1a(picture.iter().copied());
    if let Some(&t) = known().get(&hash) {
        return Some(t.into());
    }
    static SEEN: std::sync::OnceLock<std::sync::Mutex<HashMap<u64, Option<String>>>> = std::sync::OnceLock::new();
    let seen = SEEN.get_or_init(Default::default);
    if let Some(t) = seen.lock().unwrap().get(&hash) {
        return t.clone().map(Into::into);
    }
    let text = shapes().get(&shape_hash(picture)).map(|t| t.to_string()).or_else(|| read(picture)).map(|t| {
        // Wide blanks at its sides are spaces (the translation's cells
        // carry the space between words).
        let (before, after) = blank_sides(picture);
        let space = |n: usize| if n >= SPACE_COLUMNS && !t.trim().is_empty() { " " } else { "" };
        format!("{}{t}{}", space(before), space(after))
    });
    seen.lock().unwrap().insert(hash, text.clone());
    text.map(Into::into)
}

/// The blank columns at a glyph picture's left and right.
fn blank_sides(picture: &[u8]) -> (usize, usize) {
    let (w, h) = (usize::from(picture[0]), usize::from(picture[1]));
    let used = |x: usize| (0..h).any(|y| picture[2 + y * w + x] != 0);
    match (0..w).find(|&x| used(x)) {
        Some(first) => (first, w - 1 - (0..w).rev().find(|&x| used(x)).unwrap_or(first)),
        None => (0, 0),
    }
}

/// Columns of blank between two letters of a picture that make a space.
const SPACE_COLUMNS: usize = 4;

/// The widest letter a picture of several is read with, in texels.
const LETTER_COLUMNS: usize = 16;

/// Columns `x0..x1` of a glyph picture, as a picture.
fn columns(picture: &[u8], x0: usize, x1: usize) -> Vec<u8> {
    let (w, h) = (usize::from(picture[0]), usize::from(picture[1]));
    let mut out = vec![(x1 - x0) as u8, picture[1]];
    out.extend((0..h).flat_map(|y| picture[2 + y * w + x0..2 + y * w + x1].iter().copied()));
    out
}

/// A glyph picture of several letters (the translation's discs put whole
/// syllables in one cell, their letters at times touching), read from the
/// left: at each place the widest run of columns that is a known letter;
/// a wide gap is a space. The letters, or the column reading stopped at.
fn read_letters(picture: &[u8]) -> Result<String, usize> {
    let (w, h) = (usize::from(picture[0]), usize::from(picture[1]));
    let used: Vec<bool> = (0..w).map(|x| (0..h).any(|y| picture[2 + y * w + x] != 0)).collect();
    let (mut out, mut x, mut gap, mut letters) = (String::new(), 0, 0, 0);
    while x < w {
        if !used[x] {
            gap += 1;
            x += 1;
            continue;
        }
        let found = (1..=LETTER_COLUMNS.min(w - x))
            .rev()
            .filter(|&k| used[x + k - 1])
            .find_map(|k| shapes().get(&shape_hash(&columns(picture, x, x + k))).map(|t| (k, *t)));
        let Some((k, t)) = found else { return Err(x) };
        if letters > 0 && gap >= SPACE_COLUMNS {
            out.push(' ');
        }
        out.push_str(t);
        (x, gap, letters) = (x + k, 0, letters + 1);
    }
    // Two apostrophes side by side are the translation's quotes.
    if letters < 2 { Err(w) } else { Ok(out.replace("''", "\"")) }
}

/// A glyph picture of several letters as text (`read_letters`).
fn read(picture: &[u8]) -> Option<String> {
    read_letters(picture).ok()
}

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
            let cells = out.entry(super::sheets::sheet_hash(&page.rows)).or_default();
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
/// (mods/capfont.rs).
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
        // Longer words: I before a consonant, l before a vowel.
        assert_eq!(
            fix_capital_i("lnform me. lsso é. lmagine. Look, lá, lugar, lhe, lyrics"),
            "Inform me. Isso é. Imagine. Look, lá, lugar, lhe, lyrics"
        );
        // After a capital, before a small letter: an l (All, Ele, Please).
        assert_eq!(fix_capital_i("All. Ele. Please. MlST"), "All. Ele. Please. MIST");
    }

    /// A glyph's shape is its picture without the blank columns at its
    /// sides: the same letter framed otherwise in its cell is the same.
    #[test]
    fn shapes_ignore_blank_sides() {
        // 3 x 2 with the letter in the middle column; 4 x 2 with it first.
        let centred = [3, 2, 0, 5, 0, 0, 6, 0];
        let left = [4, 2, 5, 0, 0, 0, 6, 0, 0, 0];
        assert_eq!(shape_hash(&centred), shape_hash(&left));
        assert_eq!(blank_sides(&centred), (1, 1));
        assert_eq!(blank_sides(&left), (0, 3));
        // Another height is another shape (a comma is not an apostrophe).
        assert_ne!(shape_hash(&centred), shape_hash(&[3, 1, 0, 5, 0]));
        // Columns cut out as a picture of their own.
        assert_eq!(columns(&left, 0, 1), vec![1, 2, 5, 6]);
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
