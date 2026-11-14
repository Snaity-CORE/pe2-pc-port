//! What the game's pictures of text show, known by hashes of the pictures:
//! the loading captions' font, the menus' font and the speakers' name
//! labels translations draw. The tables are written by
//! tools/gen_glyph_tables.py from the discs' dumps (the game's, and each
//! translation's with its labels in tools/glyph_labels/); they hold no
//! pictures. A picture not in them is not known: what draws it leaves it
//! as the game has it.

use std::collections::HashMap;
use std::sync::OnceLock;

mod captions;
mod menus;
mod speakers;

/// A table by its hashes.
fn index(
    table: &'static [(u64, &'static str)],
    cell: &'static OnceLock<HashMap<u64, &'static str>>,
) -> &'static HashMap<u64, &'static str> {
    cell.get_or_init(|| table.iter().copied().collect())
}

/// The loading captions' glyph of picture `hash` (`caption_picture`).
pub fn caption(hash: u64) -> Option<&'static str> {
    static INDEX: OnceLock<HashMap<u64, &'static str>> = OnceLock::new();
    index(captions::TABLE, &INDEX).get(&hash).copied()
}

/// The menus' font's glyph of picture `hash` (`menu_picture`).
pub fn menu(hash: u64) -> Option<&'static str> {
    static INDEX: OnceLock<HashMap<u64, &'static str>> = OnceLock::new();
    index(menus::TABLE, &INDEX).get(&hash).copied()
}

/// The translations' speakers' name labels (`[NAME]`), by the hash
/// dialogue.rs knows a glyph picture by.
pub fn speakers() -> &'static [(u64, &'static str)] {
    speakers::TABLE
}

/// The texel at (x, y) of a 4-bit sheet's rows.
fn texel(rows: &[[u16; 64]], x: u16, y: u16) -> Option<u8> {
    let word = rows.get(usize::from(y))?.get(usize::from(x / 4))?;
    Some((word >> ((x % 4) * 4) & 15) as u8)
}

/// The hash of the picture in `cell` (u, v, width, height) of a 4-bit
/// sheet, after `head`: the bytes given, then each texel's index, rows
/// first. None when the cell is not all in the sheet.
fn picture(rows: &[[u16; 64]], head: &[u8], [u, v, w, h]: [u16; 4]) -> Option<u64> {
    let mut bytes = head.to_vec();
    for y in v..v + h {
        for x in u..u + w {
            bytes.push(texel(rows, x, y)?);
        }
    }
    Some(super::fnv1a(bytes))
}

/// A loading captions' glyph picture's hash: its width, height and texels.
pub fn caption_picture(rows: &[[u16; 64]], cell: [u16; 4]) -> Option<u64> {
    picture(rows, &[cell[2] as u8, cell[3] as u8], cell)
}

/// A menus' font's glyph picture's hash: its face (the index of the
/// glyphmaps.rs UI_FACES), width, height and texels.
pub fn menu_picture(rows: &[[u16; 64]], face: u8, cell: [u16; 4]) -> Option<u64> {
    picture(rows, &[face, cell[2] as u8, cell[3] as u8], cell)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each picture is known as one text.
    #[test]
    fn tables_name_each_picture_once() {
        for table in [captions::TABLE, menus::TABLE, speakers::TABLE] {
            let keys: std::collections::HashSet<u64> = table.iter().map(|g| g.0).collect();
            assert_eq!(keys.len(), table.len());
        }
    }

    /// A picture's texels are read low nibble first, rows first, after
    /// the face (menus) or straight after the size (captions).
    #[test]
    fn pictures_hash_their_texels() {
        let mut rows = vec![[0u16; 64]; 2];
        rows[0][0] = 0x4321;
        rows[1][0] = 0x8765;
        assert_eq!(menu_picture(&rows, 1, [1, 0, 2, 2]), Some(super::super::fnv1a([1, 2, 2, 2, 3, 6, 7])));
        assert_eq!(caption_picture(&rows, [0, 1, 4, 1]), Some(super::super::fnv1a([4, 1, 5, 6, 7, 8])));
        assert_eq!(caption_picture(&rows, [0, 1, 4, 2]), None);
    }
}
