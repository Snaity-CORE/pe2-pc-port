//! Rooms' sprites: the colours of the texture sheets of the pieces of rooms
//! drawn over their backgrounds (what passes in front of the characters).
//!
//! Each camera of a room with such pieces has a file of their picture
//! (`.pe2img`, at the texture pages of VRAM columns 896 and 960), and the
//! room's package (its folder's file 0, `.pe2pkg`) has a table of sprites
//! for it (`SpriteSource` in the decompilation's gameplay/sprites.h, 20
//! bytes each): a rectangle of a texture page, the palette it is drawn
//! with, and where on the screen. The palettes are blocks of 256-colour
//! rows the room's dialogue bundle loads (from VRAM row 255), one per
//! camera. So every sprite tells which palette colours which texels of
//! which sheet, and the extract colours sheets so, exactly.
//!
//! Tables, cameras and blocks come in the same order, but not always one
//! for one; the pieces are cut from the backgrounds, so each table is
//! matched to the camera and block whose colours, placed where its sprites
//! are drawn, match that camera's background.

use std::collections::{BTreeMap, HashMap};

use super::dialogue::{Column, Sources, lzss, picture, resources};
use super::sheets::{Catalog, sheet_hash};

/// A sheet coloured from the sprites that draw it: its depth, width in
/// texels, and texels (RGBA; those no sprite draws transparent).
pub struct Painted {
    pub depth: u32,
    pub width: usize,
    pub rgba: Vec<u8>,
}

/// A room sprite, as the package stores it.
#[derive(Clone, Copy, Debug)]
struct Sprite {
    page_x: i32,
    page_y: i32,
    depth: u32,
    clut_x: i32,
    clut_y: i32,
    w: i32,
    h: i32,
    x0: i32,
    y0: i32,
    u: i32,
    v: i32,
    semi: bool,
}

fn i16le(b: &[u8], at: usize) -> i32 {
    i32::from(i16::from_le_bytes([b[at], b[at + 1]]))
}

/// A 20-byte record, if it looks like a room sprite.
fn sprite(r: &[u8]) -> Option<Sprite> {
    let tpage = u16::from_le_bytes([r[0], r[1]]);
    let clut = u16::from_le_bytes([r[2], r[3]]);
    let (w, h, x0, y0) = (i16le(r, 4), i16le(r, 6), i16le(r, 8), i16le(r, 10));
    let (u, v, rgb, flags) = (r[14], r[15], [r[16], r[17], r[18]], r[19]);
    let depth = u32::from((tpage >> 7) & 3);
    let clut_y = i32::from(clut >> 6);
    let plausible = tpage >> 9 == 0
        && depth < 2
        && (240..512).contains(&clut_y)
        && flags <= 3
        && rgb == [128; 3]
        && (1..=256).contains(&w)
        && (1..=256).contains(&h)
        && (-400..400).contains(&x0)
        && (-300..300).contains(&y0);
    plausible.then(|| Sprite {
        page_x: i32::from(tpage & 0xF) * 64,
        page_y: i32::from((tpage >> 4) & 1) * 256,
        depth,
        clut_x: i32::from(clut & 0x3F) * 16,
        clut_y,
        w,
        h,
        x0,
        y0,
        u: i32::from(u),
        v: i32::from(v),
        semi: flags & 2 != 0,
    })
}

/// The sprite tables of a package's contents: runs of two or more
/// consecutive sprite records, in order.
fn tables(blob: &[u8]) -> Vec<Vec<Sprite>> {
    let mut found: Vec<(usize, Vec<Sprite>)> = Vec::new();
    let mut taken = vec![false; blob.len()];
    for start in 0..20 {
        let mut run: Vec<Sprite> = Vec::new();
        let mut first = start;
        let mut at = start;
        loop {
            let s = (at + 20 <= blob.len()).then(|| sprite(&blob[at..at + 20])).flatten();
            match s {
                Some(s) => {
                    if run.is_empty() {
                        first = at;
                    }
                    run.push(s);
                }
                None => {
                    if run.len() >= 2 && !taken[first] {
                        taken[first..at].fill(true);
                        found.push((first, std::mem::take(&mut run)));
                    }
                    run.clear();
                    if at + 20 > blob.len() {
                        break;
                    }
                }
            }
            at += 20;
        }
    }
    found.sort_by_key(|t| t.0);
    found.into_iter().map(|t| t.1).collect()
}

/// A block of palettes: where in VRAM (x, y), its width, its colours.
type Block = (i32, i32, usize, Vec<u16>);

/// An image chunk (x, y, height, width, then LZSS colours) as a block.
fn image(chunk: &[u8]) -> Option<Block> {
    if chunk.len() < 16 {
        return None;
    }
    let field = |at: usize| u16::from_le_bytes([chunk[at], chunk[at + 1]]);
    let (x, y, h, w) = (field(0), field(2), usize::from(field(4)), usize::from(field(6)));
    let mut at = 16;
    let bytes = lzss(chunk, &mut at);
    let colours: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|b| u16::from_le_bytes(*b)).take(w * h).collect();
    (colours.len() == w * h && w > 0).then_some((i32::from(x), i32::from(y), w, colours))
}

/// A camera with pieces: its pictures' columns, each with its sheet
/// (catalog index), and its background.
struct Camera<'a> {
    key: &'a str,
    columns: Vec<(Column, u32)>,
    background: Option<Vec<u16>>,
}

/// Walks every `step`th texel of `table`'s sprites, coloured from `block`,
/// in `camera`'s sheets: `paint` gets each (sheet, depth, width, height,
/// texel, colour). Answers how far, in sum, the sprites' colours are from
/// the background where they are drawn, and over how many texels (the
/// semi-transparent ones and colour 0 left out).
fn walk(
    table: &[Sprite],
    camera: &Camera,
    block: &Block,
    step: usize,
    mut paint: impl FnMut(u32, u32, usize, usize, usize, u16),
) -> (u64, u64) {
    let (bx, by, bw, colours) = block;
    let (mut error, mut samples) = (0u64, 0u64);
    for s in table {
        let per = [4, 2][s.depth as usize];
        let row = s.clut_y - by;
        if row < 0 {
            continue;
        }
        for dy in (0..s.h).step_by(step) {
            for dx in (0..s.w).step_by(step) {
                let (tu, tv) = (s.u + dx, s.v + dy);
                let (hx, hy) = (s.page_x + tu / per, s.page_y + tv);
                let Some((column, sheet)) = camera.columns.iter().find(|(c, _)| {
                    (i32::from(c.x)..i32::from(c.x) + 64).contains(&hx)
                        && (i32::from(c.y)..i32::from(c.y) + c.rows.len() as i32).contains(&hy)
                }) else {
                    continue;
                };
                let (lx, ly) = ((hx - i32::from(column.x)) as usize, (hy - i32::from(column.y)) as usize);
                let word = column.rows[ly][lx];
                let index = if s.depth == 0 { (word >> ((tu % 4) * 4)) & 0xF } else { (word >> ((tu % 2) * 8)) & 0xFF };
                let at = (row * *bw as i32 + s.clut_x - bx + i32::from(index)) as usize;
                let Some(&colour) = colours.get(at) else { continue };
                let width = 64 * per as usize;
                let texel = ly * width + lx * per as usize + (tu % per) as usize;
                paint(*sheet, s.depth, width, column.rows.len(), texel, colour);
                // Checked against the background the piece is cut of.
                let (sx, sy) = (s.x0 + 160 + dx, s.y0 + 120 + dy);
                if let Some(bg) = &camera.background
                    && colour != 0
                    && !s.semi
                    && (0..320).contains(&sx)
                    && (0..240).contains(&sy)
                {
                    let b = bg[(sy * 320 + sx) as usize];
                    for sh in [0, 5, 10] {
                        let diff = i32::from(b >> sh & 31) - i32::from(colour >> sh & 31);
                        error += u64::from(diff.unsigned_abs());
                    }
                    samples += 1;
                }
            }
        }
    }
    (error, samples)
}

/// Colours the sheets rooms' sprites draw, from `sources`; answers them
/// (by catalog index), and how many tables were checked against their
/// backgrounds and how many of those did not match.
pub fn paint(sources: &Sources, catalog: &Catalog) -> (HashMap<u32, Painted>, usize, usize) {
    let mut painted: HashMap<u32, Painted> = HashMap::new();
    let (mut checked, mut mismatched) = (0, 0);
    // Folders and their cameras' files with pictures, in order.
    let mut folders: BTreeMap<String, Vec<(u32, &String)>> = BTreeMap::new();
    for key in sources.pictures.keys() {
        let Some((folder, file)) = key.rsplit_once("/file") else { continue };
        let Ok(n) = file.parse::<u32>() else { continue };
        if n != 0 && key.starts_with("stage") && !key.starts_with("stage0") {
            folders.entry(folder.to_string()).or_default().push((n, key));
        }
    }
    for (folder, mut files) in folders {
        files.sort();
        let main = format!("{folder}/file0");
        let blobs = sources.packages.get(&main).map_or(&[][..], Vec::as_slice);
        let room_tables: Vec<Vec<Sprite>> = blobs.iter().flat_map(|p| tables(&lzss(p, &mut 0))).collect();
        let blocks: Vec<Block> = sources
            .bundles
            .iter()
            .filter(|b| b.0 == main)
            .flat_map(|b| resources(&b.2, 2).into_iter().filter_map(image).collect::<Vec<_>>())
            .collect();
        let cameras: Vec<Camera> = files
            .iter()
            .map(|(_, key)| Camera {
                key: key.as_str(),
                columns: sources.pictures[*key]
                    .iter()
                    .flat_map(|p| picture(p))
                    .filter_map(|c| catalog.find(sheet_hash(&c.rows)).map(|sheet| (c, sheet)))
                    .collect(),
                background: sources
                    .backgrounds
                    .get(*key)
                    .and_then(|b| b.first())
                    .and_then(|b| super::files::background_pixels(b)),
            })
            .collect();
        for (t, table) in room_tables.iter().enumerate() {
            // Its camera and block: where its sprites match the background
            // best; when nothing tells (semi-transparent pieces), its place
            // in the order.
            let mut best: Option<(u64, usize, usize)> = None;
            for (c, camera) in cameras.iter().enumerate() {
                for (b, block) in blocks.iter().enumerate() {
                    let (e, n) = walk(table, camera, block, 3, |_, _, _, _, _, _| {});
                    if n < 20 {
                        continue;
                    }
                    let score = e * 100 / n;
                    let ordered = (c, b) == (t, t);
                    if best.is_none_or(|(s, ..)| score < s || (score == s && ordered)) {
                        best = Some((score, c, b));
                    }
                }
            }
            let (c, b) = match best {
                Some((_, c, b)) => (c, b),
                None if t < cameras.len() && t < blocks.len() => (t, t),
                None => continue,
            };
            let (e, n) = walk(table, &cameras[c], &blocks[b], 1, |sheet, depth, width, height, texel, colour| {
                let p =
                    painted.entry(sheet).or_insert_with(|| Painted { depth, width, rgba: vec![0; width * height * 4] });
                if p.depth == depth {
                    let ch = |sh: u16| ((colour >> sh & 31) << 3) as u8;
                    let rgba = [ch(0), ch(5), ch(10), if colour == 0 { 0 } else { 255 }];
                    p.rgba[texel * 4..texel * 4 + 4].copy_from_slice(&rgba);
                }
            });
            if n > 50 {
                checked += 1;
                // About 1 of 31 per channel is the backgrounds' compression.
                if e / n > 6 {
                    mismatched += 1;
                    if crate::runtime::trace("rooms") {
                        eprintln!("[rooms] {}: sprites differ from the background ({})", cameras[c].key, e / n);
                    }
                }
            }
        }
    }
    (painted, checked, mismatched)
}
