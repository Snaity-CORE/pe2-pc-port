//! Dialogue in a mod's font (docs/MODDING.md).
//!
//! A mod's `fonts/captions.ttf` (or `.otf`) draws the dialogue instead of
//! the game's glyph pictures: every CAP file the game loads gets a copy
//! (translate.rs) whose lines use glyphs drawn here from that font, put in
//! the file's glyph page below the game's own, as the translations' are.
//! The game still lays the text out by its glyph table (now the font's
//! widths), colours it by its palettes and draws its shadow; each glyph is
//! drawn in the indices the game's glyphs use (OUTLINE around, FILL inside,
//! the shades between for the edges), once at the page's resolution, in
//! VRAM, and once SCALE times finer, which the renderer draws instead
//! (runtime/gpu/textures.rs `host`).
//!
//! What character each of a file's glyphs is comes from its glyph page on
//! the disc (dialogue::cap_texts), collected when the game starts; glyphs
//! not known (and the speakers' name labels) stay the game's.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use ab_glyph::{Font as _, FontVec, PxScale, ScaleFont as _, point};
use serde::Deserialize;

/// Palette indices of the dialogue glyphs: the outline, and the fill. In
/// the captions' palette 8 is darkest and 1 lightest; the shades between
/// outline and fill, by their grey there.
const OUTLINE: u8 = 8;
const SHADES: [(u8, f32); 7] = [(8, 16.0), (7, 40.0), (6, 72.0), (5, 104.0), (4, 128.0), (3, 160.0), (2, 192.0)];

/// Finer texels per page texel of the drawn glyphs.
pub const SCALE: usize = 8;

/// The game's glyph cells: 14 rows, bottom at the pen. The font's baseline
/// is 3 rows above the bottom (its descenders below, then the outline).
const CELL_ROWS: usize = 14;
const BELOW_BASELINE: usize = 3;

/// `fonts/captions.toml`, all optional.
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Settings {
    /// Height of a capital's fill, in page texels (the game's: 9).
    cap_height: Option<f32>,
    /// Texels added between letters (may be negative).
    tracking: Option<i32>,
    /// Texels the letters' strokes are made thicker (negative: thinner).
    weight: Option<f32>,
}

struct Loaded {
    font: FontVec,
    /// The font's size (px) whose capitals' fill is `cap_height` texels.
    px: f32,
    tracking: i32,
    weight: f32,
    path: PathBuf,
}

fn loaded() -> Option<&'static Loaded> {
    static LOADED: OnceLock<Option<Loaded>> = OnceLock::new();
    LOADED
        .get_or_init(|| {
            if !super::enabled() {
                return None;
            }
            // The highest priority mod's.
            let dir = super::installed()
                .into_iter()
                .filter(|m| m.enabled)
                .map(|m| m.dir.join("fonts"))
                .find(|d| d.join("captions.ttf").is_file() || d.join("captions.otf").is_file())?;
            let path = ["captions.ttf", "captions.otf"].iter().map(|n| dir.join(n)).find(|p| p.is_file())?;
            let settings: Settings = match std::fs::read_to_string(dir.join("captions.toml")) {
                Ok(t) => toml::from_str(&t).unwrap_or_else(|e| {
                    eprintln!("pe2: mods: {}: {e}", dir.join("captions.toml").display());
                    Settings::default()
                }),
                Err(_) => Settings::default(),
            };
            let font = match std::fs::read(&path)
                .map_err(|e| e.to_string())
                .and_then(|b| FontVec::try_from_vec(b).map_err(|e| e.to_string()))
            {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("pe2: mods: {}: {e}", path.display());
                    return None;
                }
            };
            let probe = font.glyph_id('H').with_scale(100.0);
            let cap = font.outline_glyph(probe)?.px_bounds().height();
            let px = 100.0 * settings.cap_height.unwrap_or(9.0) / cap.max(1.0);
            eprintln!("pe2: mods: dialogue drawn in {}", path.display());
            Some(Loaded {
                font,
                px,
                tracking: settings.tracking.unwrap_or(0),
                weight: settings.weight.unwrap_or(0.0),
                path,
            })
        })
        .as_ref()
}

/// Whether a mod's font draws the dialogue.
pub fn active() -> bool {
    loaded().is_some()
}

type Texts = HashMap<u64, Vec<Option<String>>>;
static TEXTS: OnceLock<Texts> = OnceLock::new();

/// Each CAP file's glyphs' texts, collected from the discs (mods::start).
pub fn set_texts(texts: Texts) {
    let _ = TEXTS.set(texts);
}

/// What glyph `index` of the CAP file of hash `cap` shows, if a character
/// the font draws (not a name label); None for the whole file while not
/// collected.
pub fn texts(cap: u64) -> Option<impl Fn(usize) -> Option<&'static str>> {
    let texts = TEXTS.get()?.get(&cap)?;
    Some(move |index: usize| texts.get(index).and_then(|t| t.as_deref()).filter(|t| !t.starts_with('[')))
}

/// A glyph drawn from the font: its cell size, its texels (4-bit palette
/// indices, rows top to bottom) and the same SCALE times finer.
pub struct Drawn {
    pub w: usize,
    pub h: usize,
    pub texels: Vec<u8>,
    pub fine: Vec<u8>,
}

/// The glyph of `c` (cached), or None when the font has none.
pub fn glyph(c: char) -> Option<Arc<Drawn>> {
    static CACHE: OnceLock<Mutex<HashMap<char, Option<Arc<Drawn>>>>> = OnceLock::new();
    let font = loaded()?;
    let cache = CACHE.get_or_init(Default::default);
    if let Some(g) = cache.lock().unwrap().get(&c) {
        return g.clone();
    }
    let drawn = draw(font, c).map(Arc::new);
    cache.lock().unwrap().insert(c, drawn.clone());
    drawn
}

/// `c` drawn from `font` in a cell of the game's (its bottom at the pen),
/// at the page's resolution and SCALE times finer.
fn draw(font: &Loaded, c: char) -> Option<Drawn> {
    let id = font.font.glyph_id(c);
    if id.0 == 0 && !c.is_whitespace() {
        if crate::runtime::trace("text") {
            eprintln!("[text] {}: no glyph for {c:?}", font.path.display());
        }
        return None;
    }
    let Some(outline) = font.font.outline_glyph(id.with_scale(font.px)) else {
        // Nothing drawn (a space): as wide as it advances.
        let advance = font.font.as_scaled(PxScale::from(font.px)).h_advance(id);
        let w = (advance.round() as i32 + font.tracking).max(1) as usize + 1;
        return Some(Drawn {
            w,
            h: CELL_ROWS,
            texels: vec![0; w * CELL_ROWS],
            fine: vec![0; w * CELL_ROWS * SCALE * SCALE],
        });
    };
    let b = outline.px_bounds();
    // The ink from x = 1 (its outline at 0); rows above the baseline for
    // the ink and its outline; a descender too long is raised (up to a
    // texel), as the game's own are.
    let w = ((b.max.x - b.min.x).ceil() as i32 + 2 + font.tracking).max(1) as usize;
    let above = (-b.min.y).ceil().max(0.0) as usize + 1;
    let h = CELL_ROWS.max(above + BELOW_BASELINE);
    let raise = (b.max.y - (BELOW_BASELINE as f32 - 1.0)).clamp(0.0, 1.0);
    let origin = (1.0 - b.min.x, (h - BELOW_BASELINE) as f32 - raise);
    let at = |scale: usize| {
        let s = scale as f32;
        let g = id.with_scale_and_position(font.px * s, point(origin.0 * s, origin.1 * s));
        let (cw, ch) = (w * scale, h * scale);
        let mut cover = vec![0f32; cw * ch];
        if let Some(o) = font.font.outline_glyph(g) {
            let m = o.px_bounds().min;
            o.draw(|x, y, v| {
                let (px, py) = (m.x as i32 + x as i32, m.y as i32 + y as i32);
                if px >= 0 && py >= 0 && (px as usize) < cw && (py as usize) < ch {
                    cover[py as usize * cw + px as usize] = v.min(1.0);
                }
            });
        }
        indices(&cover, cw, ch, scale, font.weight)
    };
    Some(Drawn { w, h, texels: at(1), fine: at(SCALE) })
}

/// The largest value of `cover` within `radius` of each texel (`radius`
/// negative: the smallest, within -radius).
fn spread(cover: &[f32], w: usize, h: usize, radius: f32) -> Vec<f32> {
    let r = radius.abs();
    let reach = r.floor() as i32;
    let offsets: Vec<(i32, i32)> = (-reach..=reach)
        .flat_map(|dy| (-reach..=reach).map(move |dx| (dx, dy)))
        .filter(|&(dx, dy)| (dx * dx + dy * dy) as f32 <= r * r)
        .collect();
    let grow = radius > 0.0;
    let mut out = vec![0f32; w * h];
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let mut v = if grow { 0f32 } else { 1f32 };
            for &(dx, dy) in &offsets {
                let (nx, ny) = (x + dx, y + dy);
                let c = if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
                    cover[ny as usize * w + nx as usize]
                } else {
                    0.0
                };
                v = if grow { v.max(c) } else { v.min(c) };
            }
            out[y as usize * w + x as usize] = v;
        }
    }
    out
}

/// A glyph's palette indices from its ink's coverage at `scale` finer
/// texels per page texel: an outline a page texel wide around it (clear
/// beyond), the fill inside, the shades between where the ink partly
/// covers.
fn indices(cover: &[f32], w: usize, h: usize, scale: usize, weight: f32) -> Vec<u8> {
    let s = scale as f32;
    let fill = if weight.abs() * s >= 0.5 { spread(cover, w, h, weight * s) } else { cover.to_vec() };
    let ring = spread(&fill, w, h, s + 0.5);
    fill.iter()
        .zip(&ring)
        .map(|(&f, &o)| {
            if o < 0.5 {
                return 0;
            }
            let grey = 16.0 + 176.0 * f;
            SHADES.iter().min_by(|a, b| (a.1 - grey).abs().total_cmp(&(b.1 - grey).abs())).map_or(OUTLINE, |s| s.0)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Coverage becomes the game's indices: full ink the fill, none the
    /// outline next to it, clear beyond.
    #[test]
    fn coverage_takes_the_games_indices() {
        // 5 x 1: ink in the middle texel only.
        let cover = [0.0, 0.0, 1.0, 0.0, 0.0];
        assert_eq!(indices(&cover, 5, 1, 1, 0.0), vec![0, 8, 2, 8, 0]);
        // Half covered: a shade between.
        let half = [0.0, 0.5, 1.0];
        let out = indices(&half, 3, 1, 1, 0.0);
        assert_eq!(out[2], 2);
        assert!((3..=7).contains(&out[1]), "{out:?}");
    }

    /// Thicker and thinner strokes.
    #[test]
    fn weight_grows_and_shrinks_the_ink() {
        let cover = [0.0, 0.0, 1.0, 0.0, 0.0];
        assert_eq!(spread(&cover, 5, 1, 1.0), vec![0.0, 1.0, 1.0, 1.0, 0.0]);
        // 5 x 3 of ink: a texel in from every side is left.
        let block = [1.0; 15];
        assert_eq!(spread(&block, 5, 3, -1.0)[5..10], [0.0, 1.0, 1.0, 1.0, 0.0]);
    }
}
