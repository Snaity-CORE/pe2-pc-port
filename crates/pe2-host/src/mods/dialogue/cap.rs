//! The game's CAP2 dialogue files (doc/CAP_FORMAT.md in the decompilation):
//! their lines, streams of glyph indices into a table of cells (u, v,
//! width, height) of a 4-bit texture page, with a few control codes; and
//! the dialogue bundles (`.pe2cap2` chunks) that hold them.

pub const END: u16 = 0xFFFF;

pub const NEWLINE: u16 = 0xFFFE;

/// A narrow space (3 pixels).
pub const GAP: u16 = 0xFFFD;

pub(super) fn u16le(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

pub(super) fn u32le(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
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
        crate::mods::fnv1a(self.bytes[..self.len()].iter().copied())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_glyph_index_cannot_wrap_into_another_cell() {
        let cap = Cap { bytes: &[1, 2, 3, 4], glyphs: 0, sequences: 0, commands: 0 };
        assert_eq!(cap.cell(0), [1, 2, 3, 4]);
        assert_eq!(cap.cell(usize::MAX), [0; 4]);
    }
}
