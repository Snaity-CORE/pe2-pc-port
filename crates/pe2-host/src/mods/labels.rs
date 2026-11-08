//! The speakers' name labels of the dialogue, redrawn: each CAP file's copy
//! (translate.rs) points its labels' cells at new ones drawn here from the
//! built-in font (DejaVu Sans Bold, slanted as the game's), white on the
//! game's green box, in the label palette's indices. So they are sharp at
//! any scale, take a mod's translation of the name, and no longer depend on
//! the glyph page: a translation that replaces a room's page (its own
//! letters elsewhere) would otherwise show pieces of them.
//!
//! A mod translates the names in `text/speakers.toml` (the extract lists
//! them in `dumps/text/speakers.toml`):
//!
//! ```toml
//! "POLICE OFFICER" = "POLICIAL"
//! ```

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use ab_glyph::{Font as _, FontRef, PxScale, ScaleFont as _, point};

use super::capfont::{BUILT_IN, Drawn, SCALE};

/// The file of a mod's translations of the names, under its `text/`.
pub const FILE: &str = "speakers.toml";

/// The game's labels: the box is index 1 (dark green), the letters 15
/// (white) with the indices between for their edges (a ramp from the box's
/// green to white, but 8: the dialogue's outline, near black).
const BOX: u8 = 1;
const LETTERS: u8 = 15;
const NOT_A_SHADE: u8 = 8;

/// A capital's height, in page texels; the box's margin left and right of
/// the name; how far the letters lean (texels right per texel up).
const CAP: f32 = 7.0;
const MARGIN: usize = 4;
const SLANT: f32 = 0.21;

/// The translations of the speakers' names the enabled mods give (the
/// highest priority's wins).
fn translations() -> &'static HashMap<String, String> {
    static NAMES: OnceLock<HashMap<String, String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let mut out = HashMap::new();
        if !super::enabled() {
            return out;
        }
        for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
            let path = m.dir.join("text").join(FILE);
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            match toml::from_str::<HashMap<String, String>>(&text) {
                Ok(names) => out.extend(names.into_iter().filter(|(_, t)| !t.is_empty())),
                Err(e) => eprintln!("pe2: mods: {}: {e}", path.display()),
            }
        }
        out
    })
}

/// The label of the speaker `name` (as the game's label reads, BALDWIN),
/// in a mod's translation if any, `h` texels high (the game's cell's):
/// cached.
pub fn label(name: &str, h: usize) -> Option<Arc<Drawn>> {
    type Cache = std::sync::Mutex<HashMap<(String, usize), Option<Arc<Drawn>>>>;
    static CACHE: OnceLock<Cache> = OnceLock::new();
    let shown = translations().get(name).map_or(name, String::as_str);
    let key = (shown.to_string(), h);
    let cache = CACHE.get_or_init(Default::default);
    if let Some(d) = cache.lock().unwrap().get(&key) {
        return d.clone();
    }
    let drawn = draw(shown, h).map(Arc::new);
    cache.lock().unwrap().insert(key, drawn.clone());
    drawn
}

/// `name` drawn in a box `h` texels high: at the page's resolution and
/// SCALE times finer.
fn draw(name: &str, h: usize) -> Option<Drawn> {
    let font = FontRef::try_from_slice(BUILT_IN).ok()?;
    if h < 4 || name.trim().is_empty() {
        return None;
    }
    let k = SCALE as f32;
    // The size whose capitals are CAP texels high (finer texels).
    let probe = font.outline_glyph(font.glyph_id('H').with_scale(100.0))?.px_bounds();
    let px = 100.0 * CAP * k / (probe.max.y - probe.min.y);
    let scaled = font.as_scaled(PxScale::from(px));
    // The letters' pen positions, and how wide they are with the lean.
    let mut pens = Vec::new();
    let mut x = 0.0f32;
    let mut last = None;
    for c in name.chars() {
        let id = font.glyph_id(c);
        if let Some(prev) = last {
            x += scaled.kern(prev, id);
        }
        pens.push((id, x));
        x += scaled.h_advance(id);
        last = Some(id);
    }
    let lean = SLANT * CAP * k;
    let text_w = x + lean;
    let w = (text_w / k).ceil() as usize + 2 * MARGIN;
    let (fw, fh) = (w * SCALE, h * SCALE);
    // The baseline, the capitals centred in the box.
    let base = (h as f32 + CAP) / 2.0 * k;
    let left = MARGIN as f32 * k;
    let mut cover = vec![0f32; fw * fh];
    for (id, pen) in pens {
        let Some(o) = font.outline_glyph(id.with_scale_and_position(px, point(left + pen, base))) else { continue };
        let m = o.px_bounds().min;
        o.draw(|gx, gy, v| {
            let y = m.y + gy as f32;
            // Leaning: further right the higher above the baseline.
            let x = m.x + gx as f32 + SLANT * (base - y);
            let (px, py) = (x.round() as i32, y as i32);
            if px >= 0 && py >= 0 && (px as usize) < fw && (py as usize) < fh {
                let at = &mut cover[py as usize * fw + px as usize];
                *at = (*at + v).min(1.0);
            }
        });
    }
    // The page's texels: each the mean of its finer ones.
    let mut coarse = vec![0f32; w * h];
    for (i, c) in cover.iter().enumerate() {
        let (x, y) = (i % fw / SCALE, i / fw / SCALE);
        coarse[y * w + x] += c / (SCALE * SCALE) as f32;
    }
    Some(Drawn {
        w,
        h,
        texels: coarse.iter().map(|&c| index(c)).collect(),
        fine: cover.iter().map(|&c| index(c)).collect(),
    })
}

/// The palette index of a texel the letters cover by `c` (0 to 1).
fn index(c: f32) -> u8 {
    let i = BOX + (c.clamp(0.0, 1.0) * f32::from(LETTERS - BOX)).round() as u8;
    if i == NOT_A_SHADE { if c * 14.0 < 7.0 { i - 1 } else { i + 1 } } else { i }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A label is the box with the name in it: wider for a longer name, the
    /// letters white somewhere, nothing clear, never the outline's index.
    #[test]
    fn labels_are_boxes_with_white_letters() {
        let aya = draw("AYA", 10).unwrap();
        let baldwin = draw("BALDWIN", 10).unwrap();
        assert!(baldwin.w > aya.w);
        assert_eq!((aya.h, aya.texels.len(), aya.fine.len()), (10, aya.w * 10, aya.w * 10 * SCALE * SCALE));
        for d in [&aya, &baldwin] {
            assert!(d.texels.iter().all(|&t| (BOX..=LETTERS).contains(&t) && t != NOT_A_SHADE));
            assert!(d.fine.contains(&LETTERS));
            // The margins and the top and bottom rows are the box.
            assert!((0..d.h).all(|y| d.texels[y * d.w] == BOX && d.texels[y * d.w + d.w - 1] == BOX));
            assert!((0..d.w).all(|x| d.texels[x] == BOX && d.texels[(d.h - 1) * d.w + x] == BOX));
        }
    }

    #[test]
    fn coverage_skips_the_outline_index() {
        assert_eq!(index(0.0), BOX);
        assert_eq!(index(1.0), LETTERS);
        assert!((0..=100).all(|i| index(i as f32 / 100.0) != NOT_A_SHADE));
    }
}
