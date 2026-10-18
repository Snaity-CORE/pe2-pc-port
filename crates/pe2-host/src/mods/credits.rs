//! The credits' sheets: coloured as the bonus screens after the ending
//! draw them (the decompilation's aya/replay_bonus).
//!
//! Their file (STAGE0 file 21000) loads its pictures and palettes, and its
//! bundle's first resource is an `STF1` file: the credits' rows, each a
//! list of commands (glyphs of a glyph table, palette changes, sprites of a
//! sprite table). Glyphs are drawn from the pages at (896, 256) or (960, 0)
//! with a palette chosen by the row's commands, from the last rows of the
//! file's own picture (VRAM rows 510 and 511); sprites name their page,
//! depth and palette. The extract lays the file out in VRAM as the game
//! loads it and walks every row, colouring each texel drawn.

use std::collections::HashMap;

use super::dialogue::{Sources, picture, slots};
use super::rooms::{Painted, colour_texel};
use super::sheets::{Catalog, sheet_hash};

/// The credits' file, and the system text palettes loaded when they show.
const FILE: &str = "stage0/file21000";
const SYSTEM: &str = "stage0/file1";

const VRAM_WIDTH: usize = 1024;
const VRAM_HEIGHT: usize = 512;

/// VRAM as the credits' files fill it, and which sheet (catalog index) each
/// 64-halfword column of each row holds.
struct Vram {
    words: Vec<u16>,
    sheets: HashMap<(usize, usize), (u32, usize)>,
}

impl Vram {
    fn get(&self, x: usize, y: usize) -> u16 {
        self.words.get((y % VRAM_HEIGHT) * VRAM_WIDTH + x % VRAM_WIDTH).copied().unwrap_or(0)
    }

    /// Colours texel (`u`, `v`) of the page at (`px`, `py`) of `depth`
    /// with the palette at (`cx`, `cy`).
    fn texel(
        &self,
        painted: &mut HashMap<u32, Painted>,
        catalog: &Catalog,
        (px, py, depth): (usize, usize, u32),
        (cx, cy): (usize, usize),
        (u, v): (usize, usize),
    ) {
        let per = [4, 2, 1][depth.min(2) as usize];
        let (x, y) = (px + u / per, py + v);
        let Some(&(sheet, top)) = self.sheets.get(&(x / 64 * 64, y)) else { return };
        let word = self.get(x, y);
        let colour = match depth {
            0 => self.get(cx + usize::from((word >> ((u % 4) * 4)) & 0xF), cy),
            1 => self.get(cx + usize::from((word >> ((u % 2) * 8)) & 0xFF), cy),
            _ => word,
        };
        let rows = catalog.sheets[sheet as usize].rows.len();
        let width = 64 * per;
        let texel = (y - top) * width + (x % 64) * per + u % per;
        colour_texel(painted, "the credits", (sheet, depth, width, rows, texel, colour));
    }
}

/// The credits' sheets, coloured as the credits draw them (by catalog
/// index).
pub fn paint(sources: &Sources, catalog: &Catalog) -> HashMap<u32, Painted> {
    let mut painted = HashMap::new();
    let mut vram = Vram { words: vec![0; VRAM_WIDTH * VRAM_HEIGHT], sheets: HashMap::new() };
    for chunk in [FILE, SYSTEM].iter().flat_map(|k| sources.palettes.get(*k).into_iter().flatten()) {
        if let Some((x, y, w, colours)) = super::rooms::image(chunk) {
            for (i, &c) in colours.iter().enumerate() {
                let (cx, cy) = (x as usize + i % w, y as usize + i / w);
                if cx < VRAM_WIDTH && cy < VRAM_HEIGHT {
                    vram.words[cy * VRAM_WIDTH + cx] = c;
                }
            }
        }
    }
    for column in sources.pictures.get(FILE).into_iter().flatten().flat_map(|p| picture(p)) {
        let Some(sheet) = catalog.find(sheet_hash(&column.rows)) else { continue };
        let (cx, cy) = (usize::from(column.x), usize::from(column.y));
        for (r, row) in column.rows.iter().enumerate() {
            let y = cy + r;
            if y >= VRAM_HEIGHT || cx + 64 > VRAM_WIDTH {
                break;
            }
            vram.words[y * VRAM_WIDTH + cx..y * VRAM_WIDTH + cx + 64].copy_from_slice(row);
            vram.sheets.insert((cx, y), (sheet, cy));
        }
    }
    let Some(stf) = sources
        .bundles
        .iter()
        .filter(|b| b.0 == FILE)
        .flat_map(|b| slots(&b.2).into_iter().map(|s| s.2.to_vec()).collect::<Vec<_>>())
        .find(|r| r.starts_with(b"STF"))
    else {
        return painted;
    };
    let word = |at: usize| stf.get(at..at + 4).map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let (Some(glyphs), Some(table), Some(sprites)) = (word(0x0C), word(0x10), word(0x14)) else { return painted };
    let lines = word(table).unwrap_or(0);
    for line in 0..lines {
        let Some(mut at) = word(table + 4 + line * 8) else { break };
        let mut palette = 7usize;
        while let Some(&[op, arg]) = stf.get(at..at + 2).and_then(|c| c.as_array::<2>()) {
            if op == 0xFF {
                break;
            }
            let arg = usize::from(arg);
            match op {
                // A glyph (u, v, width, height | 0x80 for the page at (960, 0)).
                0 => {
                    let Some(&[u, v, w, h]) =
                        stf.get(glyphs + arg * 4..glyphs + arg * 4 + 4).and_then(|g| g.as_array::<4>())
                    else {
                        break;
                    };
                    let page = if h & 0x80 != 0 { (960, 0, 0) } else { (896, 256, 0) };
                    let clut = ((0x38 | (palette & 3)) * 16, 510 + palette / 4);
                    for dv in 0..usize::from(h & 0x7F) {
                        for du in 0..usize::from(w) {
                            vram.texel(&mut painted, catalog, page, clut, (usize::from(u) + du, usize::from(v) + dv));
                        }
                    }
                }
                3 => palette = arg,
                // A sprite: page x, page bits, palette x, y, u, v, depth,
                // width, height; drawn a page at a time from u to 256.
                8 => {
                    let Some(s) = stf.get(sprites + arg * 16..sprites + arg * 16 + 16) else { break };
                    let half = |i: usize| usize::from(u16::from_le_bytes([s[i], s[i + 1]]));
                    let depth = u32::from(s[10] & 3);
                    let (mut page_x, page_y) = (half(0) & 0x3C0, if half(2) & 0x100 != 0 { 256 } else { 0 });
                    let clut = (half(4) & 0x3F0, half(6));
                    let (mut u, v, mut width, height) = (usize::from(s[8]), usize::from(s[9]), half(12), half(14));
                    while width > 0 {
                        let piece = width.min(256 - u);
                        for dv in 0..height {
                            for du in 0..piece {
                                vram.texel(&mut painted, catalog, (page_x, page_y, depth), clut, (u + du, v + dv));
                            }
                        }
                        width -= piece;
                        u = 0;
                        page_x += 0x100 >> [2, 1, 0][depth.min(2) as usize];
                    }
                }
                _ => {}
            }
            at += 2;
        }
    }
    painted
}
