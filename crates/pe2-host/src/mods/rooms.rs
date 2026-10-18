//! Rooms' sprites: the colours of the texture sheets of the pieces of rooms
//! drawn over their backgrounds (what passes in front of the characters).
//!
//! Each camera of a room with such pieces has a file of their picture
//! (`.pe2img`, at the texture pages of VRAM columns 896 and 960), and its
//! view has a table of sprites (`SpriteSource` in the decompilation's
//! gameplay/sprites.h, 20 bytes each): a rectangle of a texture page, the
//! palette it is drawn with, and where on the screen. The game finds a
//! view's table as the extract does (`view_tables`), through its stage's
//! map overlay into the room's overlay; the view's index there is also its
//! camera's file, and the slot of its palettes (a block of 256-colour rows
//! from VRAM row 255) in the room's bundle. So every sprite tells which
//! palette colours which texels of which sheet, and the extract colours
//! sheets so, exactly. The sprites are checked against the camera's
//! background, which they are cut from (unless the room changed since).

use std::collections::{BTreeMap, HashMap};

use super::dialogue::{Column, Sources, lzss, picture, slots};
use super::sheets::{Catalog, sheet_hash};

/// A sheet coloured from the sprites that draw it: its depth, width in
/// texels, and texels (RGBA; those no sprite draws transparent).
pub struct Painted {
    pub depth: u32,
    pub width: usize,
    pub rgba: Vec<u8>,
    /// What coloured it (for the extract's index).
    pub source: &'static str,
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
    /// The colour the GPU multiplies its texels by (128: as they are).
    shade: [u8; 3],
}

fn i16le(b: &[u8], at: usize) -> i32 {
    i32::from(i16::from_le_bytes([b[at], b[at + 1]]))
}

/// A sprite record as a `SpriteView` points to it: anything a sprite can
/// be.
fn any_sprite(r: &[u8]) -> Option<Sprite> {
    let tpage = u16::from_le_bytes([r[0], r[1]]);
    let clut = u16::from_le_bytes([r[2], r[3]]);
    let (w, h, x0, y0) = (i16le(r, 4), i16le(r, 6), i16le(r, 8), i16le(r, 10));
    let depth = u32::from((tpage >> 7) & 3);
    let ok = tpage >> 9 == 0 && depth < 2 && (1..=256).contains(&w) && (1..=256).contains(&h);
    ok.then(|| Sprite {
        page_x: i32::from(tpage & 0xF) * 64,
        page_y: i32::from((tpage >> 4) & 1) * 256,
        depth,
        clut_x: i32::from(clut & 0x3F) * 16,
        clut_y: i32::from(clut >> 6),
        w,
        h,
        x0,
        y0,
        u: i32::from(r[14]),
        v: i32::from(r[15]),
        semi: r[19] & 2 != 0,
        shade: [r[16], r[17], r[18]],
    })
}

/// Where the stages' map overlays (STAGE0 file 900000 + stage) and the
/// rooms' overlays (their folder's file 0) load.
const MAP_BASE: u32 = 0x8017_9950;
const ROOM_BASE: u32 = 0x8017_D5C0;

/// Per stage, where its map overlay keeps its directories of views
/// (decompilation: gameplay/load_screen.c Gp_ViewCountTables,
/// Gp_ViewIndexTables, Gp_SprtTables): rooms' view counts, rooms' maps of
/// view to sprite view, and areas' sprite views.
const MAP_TABLES: [(u32, u32, u32); 5] = [
    (0x8017_ABC0, 0x8017_AC68, 0x8017_AB1C),
    (0x8017_AA28, 0x8017_ABFC, 0x8017_AC98),
    (0x8017_A93C, 0x8017_AB10, 0x8017_ABAC),
    (0x8017_B110, 0x8017_B548, 0x8017_B610),
    (0x8017_AB88, 0x8017_ADB0, 0x8017_AE38),
];

/// The map of views to themselves many rooms use, and its length
/// (gameplay/direction_input.c gViewIdentityMap: 1, 2, 3...).
const IDENTITY_MAP: u32 = 0x8010_CAF8;
const IDENTITY_VIEWS: u8 = 50;

/// PlayStation memory as the overlays loaded fill it.
struct Memory(Vec<(u32, Vec<u8>)>);

impl Memory {
    fn bytes(&self, addr: u32, n: usize) -> Option<&[u8]> {
        self.0.iter().find_map(|(base, blob)| {
            let at = usize::try_from(addr.checked_sub(*base)?).ok()?;
            blob.get(at..at + n)
        })
    }

    fn word(&self, addr: u32) -> Option<u32> {
        self.bytes(addr, 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn half(&self, addr: u32) -> Option<u16> {
        self.bytes(addr, 2).map(|b| u16::from_le_bytes([b[0], b[1]]))
    }

    /// The sprites of the `SpriteView` at `addr` (gameplay/sprites.h): as
    /// many as its batches reach (8 bytes each, the last with first sprite
    /// 0xFFFF).
    fn view(&self, addr: u32) -> Option<Vec<Sprite>> {
        let (sources, batches) = (self.word(addr)?, self.word(addr + 4)?);
        let mut count = 0u32;
        for b in 0..64 {
            let first = self.half(batches + b * 8)?;
            if first == 0xFFFF {
                break;
            }
            count = count.max(u32::from(first) + u32::from(self.half(batches + b * 8 + 2)?));
        }
        if count == 0 || sources == 0 || sources == batches {
            return Some(Vec::new());
        }
        self.bytes(sources, count as usize * 20)?.chunks(20).map(any_sprite).collect()
    }
}

/// The sprite tables of a room's views, as the game finds them: its
/// stage's map overlay maps the room's view to an index (gameplay/
/// sprite_link.c Gp_GetViewIndex), which picks one of its area's sprite
/// views, held by the room's overlay (Gp_LinkViewSprts). By index
/// (1-based; it is also the camera's file, and its palettes' place in the
/// room's bundle), for `folder` (`stage<s>/<area * 100 + room>`), with
/// whether the room's map has it from the start. Scripts change the maps
/// (a room after an event): the area's views for the `others` indices, of
/// cameras the map does not have, come too.
fn view_tables(sources: &Sources, folder: &str, others: &[u32]) -> Option<BTreeMap<u32, (Vec<Sprite>, bool)>> {
    let (stage, place) = folder.strip_prefix("stage")?.split_once('/')?;
    let (stage, place) = (stage.parse::<usize>().ok()?, place.parse::<u32>().ok()?);
    let (area, room) = (place / 100, place % 100);
    let (counts, maps, views) = *MAP_TABLES.get(stage.checked_sub(1)?)?;
    let overlay = |key: &str, base: u32| {
        let packages = sources.packages.get(key)?;
        packages.iter().find(|(b, _)| *b == base).map(|(b, p)| (*b, lzss(p, &mut 0)))
    };
    let memory = Memory(vec![
        overlay(&format!("stage0/file{}", 900_000 + stage), MAP_BASE)?,
        overlay(&format!("{folder}/file0"), ROOM_BASE)?,
        (IDENTITY_MAP, (1..=IDENTITY_VIEWS).collect()),
    ]);
    let (a, r) = (area.checked_sub(1)? * 4, room.checked_sub(1)?);
    let count = memory.half(memory.word(memory.word(counts)? + a)? + r * 2)?;
    let map = memory.word(memory.word(memory.word(maps)? + a)? + r * 4)?;
    let views = memory.word(memory.word(views)? + a)?;
    let mut out = BTreeMap::new();
    for v in 1..=u32::from(count) {
        let index = u32::from(*memory.bytes(map + v - 1, 1)?.first()?);
        if index > 0
            && !out.contains_key(&index)
            && let Some(table) = memory.view(views + (index - 1) * 12)
        {
            out.insert(index, (table, true));
        }
    }
    for &index in others {
        if index > 0
            && !out.contains_key(&index)
            && let Some(table) = memory.view(views + (index - 1) * 12)
        {
            out.insert(index, (table, false));
        }
    }
    Some(out)
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
                        let k = (sh / 5) as usize;
                        let shaded = (i32::from(colour >> sh & 31) * i32::from(s.shade[k]) / 128).min(31);
                        let diff = i32::from(b >> sh & 31) - shaded;
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
        // The room's palettes: its bundle's images, by slot (the game keeps
        // the last bundle loaded).
        let blocks: BTreeMap<usize, Block> = sources
            .bundles
            .iter()
            .rfind(|b| b.0 == main)
            .map(|b| slots(&b.2).into_iter().filter(|s| s.1 == 2).filter_map(|s| Some((s.0, image(s.2)?))).collect())
            .unwrap_or_default();
        // Palettes a view without its own keeps from before: the room's
        // other blocks, and those its package loads (`.pe2clut`).
        let kept: Vec<Block> = blocks
            .values()
            .cloned()
            .chain(sources.palettes.get(&main).into_iter().flatten().filter_map(|c| image(c)))
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
        // The tables, cameras and palettes as the game finds them: a view's
        // index is its camera's file and its palettes' slot, the ones
        // cd_loading.c Gp_LoadViewImages loads.
        let numbers: Vec<u32> = files.iter().map(|(n, _)| *n).collect();
        let Some(views) = view_tables(sources, &folder, &numbers) else {
            if crate::runtime::trace("rooms") {
                eprintln!("[rooms] {folder}: no view tables");
            }
            continue;
        };
        for (index, (table, mapped)) in &views {
            let key = format!("{folder}/file{index}");
            let Some(c) = cameras.iter().position(|c| c.key == key) else { continue };
            // A view without palettes of its own keeps those already loaded
            // (Gp_LoadViewImages loads none): the ones its sprites match the
            // background best with.
            let block = match blocks.get(&(*index as usize - 1)) {
                Some(block) => block,
                None => {
                    let best = kept
                        .iter()
                        .map(|block| (walk(table, &cameras[c], block, 3, |_, _, _, _, _, _| {}), block))
                        .filter(|((_, n), _)| *n >= 20)
                        .min_by_key(|((e, n), _)| e * 100 / n);
                    match best {
                        Some((_, block)) => block,
                        None => continue,
                    }
                }
            };
            // A view the map does not have at first is taken only when its
            // sprites are its camera's (they match its background).
            if !mapped {
                let (e, n) = walk(table, &cameras[c], block, 3, |_, _, _, _, _, _| {});
                let matches = n >= 20 && e / n <= 6;
                if crate::runtime::trace("rooms") {
                    eprintln!("[rooms] {key}: outside the map, {} sprites, matches {matches}", table.len());
                }
                if !matches {
                    continue;
                }
            }
            let (e, n) = walk(table, &cameras[c], block, 1, |sheet, depth, width, height, texel, colour| {
                let p = painted.entry(sheet).or_insert_with(|| Painted {
                    depth,
                    width,
                    rgba: vec![0; width * height * 4],
                    source: "rooms' sprites",
                });
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
