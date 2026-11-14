//! The game's `.pe2img` pictures as loaded into VRAM: columns of LZSS
//! strips; and the glyph cells cut from them.

use super::cap::{u16le, u32le};

/// A 64-halfword column of a picture is 32-row strips of this many bytes.
const STRIP_BYTES: usize = 0x1000;

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

#[cfg(test)]
mod tests {
    use super::*;

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
