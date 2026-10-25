//! The game's pictures of text redrawn from built-in fonts, with
//! [video] hd_fonts (the default): the menus' font (DejaVu Sans Bold) and
//! the loading screen's captions (Courier Prime, a typewriter face), as
//! tools/hd_fonts.py draws them for mods, made here when the game starts.
//!
//! Each sheet becomes an indexed replacement K times finer (indexed.rs),
//! which the renderer draws instead of the sheet, through the palette the
//! game sets: first the sheet enlarged smoothly (its indices' greys
//! Lanczos-enlarged, each finer texel taking, of its own texel's index and
//! its 8 neighbours', the nearest), so what is not redrawn stays as it
//! was; then each glyph of its map (glyphmaps.rs) drawn from the font into
//! its cell, in the indices the game's glyphs use:
//!
//! - menus: 5 the page, 10 the outline ring, 15 the fill, those between
//!   the edges; letters of a face share one scale (its capitals') and
//!   stand on the baseline their record gives; the pad's buttons drawn as
//!   outlines;
//! - loading: 0 clear, 1 the ring, 15 the fill, those between the edges.
//!
//! A mod's indexed replacement of the same sheet wins (mods::indexed_texture).

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use ab_glyph::{Font as _, FontRef, PxScale, point};

use super::glyphmaps::{Glyph, Map};
use super::indexed::Indexed;
use super::sheets::Catalog;

/// The loading captions' font (its license: res/fonts/LICENSE-CourierPrime.txt).
const COURIER_PRIME: &[u8] = include_bytes!("../../res/fonts/CourierPrime-Regular.ttf");

/// Finer texels per sheet texel, and the supersampling of each glyph's
/// coverage within one.
const K: usize = 8;
const SS: usize = 4;
/// Size the fonts' letters are first drawn at, in pixels.
const INK_PX: f32 = 400.0;

/// Whether the built-in fonts redraw the game's text ([video] hd_fonts).
pub fn enabled() -> bool {
    crate::config::get().video.hd_fonts.unwrap_or(true)
}

static SHEETS: OnceLock<HashMap<(u64, u32), Arc<Indexed>>> = OnceLock::new();

/// The redrawn `sheet` at `depth` (0: 4 bits, 1: 8), once made.
pub fn get(sheet: u64, depth: u32) -> Option<Arc<Indexed>> {
    SHEETS.get()?.get(&(sheet, depth)).cloned()
}

/// Redraws the menus' font and the loading captions, from the sheets of
/// `catalog` and the maps the game's executable `exe` gives.
pub fn build(catalog: &Catalog, exe: &[u8]) {
    let since = std::time::Instant::now();
    let mut out = HashMap::new();
    let ui = Font::new(super::capfont::BUILT_IN);
    let typewriter = Font::new(COURIER_PRIME);
    if let Some(map) = super::glyphmaps::ui_font(catalog, exe) {
        // The small face is wide capitals: stretched into its cells.
        let faces = [(Some("medium"), &ui, None), (Some("large"), &ui, None), (Some("small"), &ui, Some(3.0))];
        let scheme = Scheme { clear: 5, fill_from: 13, ink_from: 8, keep_zero: true, levels: ui_levels, weight: -0.25 };
        if let Some(sheet) = redraw(catalog, &map, &faces, &scheme) {
            out.insert((sheet.sheet, sheet.depth), Arc::new(sheet));
        }
    }
    if let Some(map) = super::glyphmaps::loading(catalog, exe) {
        let faces = [(None, &typewriter, None)];
        let scheme =
            Scheme { clear: 0, fill_from: 10, ink_from: 1, keep_zero: false, levels: loading_levels, weight: 0.15 };
        if let Some(sheet) = redraw(catalog, &map, &faces, &scheme) {
            out.insert((sheet.sheet, sheet.depth), Arc::new(sheet));
        }
    }
    eprintln!(
        "pe2: video: menus' font and loading captions redrawn ({} sheets) in {:.1} s",
        out.len(),
        since.elapsed().as_secs_f32()
    );
    // PE2_FONTSHEETS=<dir> (debugging): saves them as indexed replacements.
    if let Some(dir) = std::env::var_os("PE2_FONTSHEETS") {
        for sheet in out.values() {
            if let Err(e) =
                super::indexed::write(std::path::Path::new(&dir), sheet, Some("redrawn from a built-in font"))
            {
                eprintln!("pe2: video: {e}");
            }
        }
    }
    let _ = SHEETS.set(out);
}

/// A sheet's indices: what its page is, where a glyph's fill and ink begin,
/// whether its zero texels stay clear, and how coverage becomes indices.
struct Scheme {
    clear: u8,
    fill_from: u8,
    ink_from: u8,
    keep_zero: bool,
    /// (fill, ring coverage) -> index.
    levels: fn(f32, f32) -> u8,
    /// Texels the strokes grow (negative: thinner).
    weight: f32,
}

fn ui_levels(fill: f32, ring: f32) -> u8 {
    (5.0 + 5.0 * ring + 5.0 * fill).round_ties_even().clamp(5.0, 15.0) as u8
}

fn loading_levels(fill: f32, ring: f32) -> u8 {
    if ring >= 0.5 { (1.0 + 14.0 * fill).round_ties_even().clamp(1.0, 15.0) as u8 } else { 0 }
}

/// A face's font: (face name, font, horizontal stretch for boxed faces).
type Face<'a> = (Option<&'static str>, &'a Font, Option<f32>);

/// A grey picture, `w` x `h`, rows top to bottom.
struct Grey {
    w: usize,
    h: usize,
    px: Vec<f32>,
}

struct Font {
    font: FontRef<'static>,
    /// A capital H's height at INK_PX.
    cap: f32,
}

impl Font {
    fn new(bytes: &'static [u8]) -> Font {
        let font = FontRef::try_from_slice(bytes).expect("a built-in font loads");
        let cap = font
            .outline_glyph(font.glyph_id('H').with_scale_and_position(INK_PX, point(0.0, 0.0)))
            .map_or(INK_PX * 0.7, |g| g.px_bounds().height());
        Font { font, cap }
    }

    /// `text`'s ink (0..255) at INK_PX, cropped to where it is at least
    /// half, and the baseline's row in it.
    fn ink(&self, text: &str) -> Option<(Grey, f32)> {
        let scale = PxScale::from(INK_PX);
        let mut pen = 0.0;
        let mut glyphs = Vec::new();
        for c in text.chars() {
            let id = self.font.glyph_id(c);
            if id.0 == 0 {
                return None;
            }
            let g = id.with_scale_and_position(scale, point(pen, 0.0));
            pen += ab_glyph::ScaleFont::h_advance(&self.font.as_scaled(scale), id);
            if let Some(o) = self.font.outline_glyph(g) {
                glyphs.push(o);
            }
        }
        if glyphs.is_empty() {
            return None;
        }
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for o in &glyphs {
            let b = o.px_bounds();
            (x0, y0, x1, y1) = (x0.min(b.min.x), y0.min(b.min.y), x1.max(b.max.x), y1.max(b.max.y));
        }
        let (w, h) = ((x1 - x0).ceil() as usize + 2, (y1 - y0).ceil() as usize + 2);
        let mut px = vec![0.0f32; w * h];
        for o in &glyphs {
            let b = o.px_bounds();
            let (ox, oy) = ((b.min.x - x0) as usize + 1, (b.min.y - y0) as usize + 1);
            o.draw(|x, y, c| {
                let i = (oy + y as usize) * w + ox + x as usize;
                px[i] = (px[i] + c * 255.0).min(255.0);
            });
        }
        // Cropped to the half-covered texels, as the tool does.
        let (mut cx0, mut cy0, mut cx1, mut cy1) = (w, h, 0, 0);
        for y in 0..h {
            for x in 0..w {
                if px[y * w + x] >= 128.0 {
                    (cx0, cy0, cx1, cy1) = (cx0.min(x), cy0.min(y), cx1.max(x + 1), cy1.max(y + 1));
                }
            }
        }
        if cx1 <= cx0 {
            return None;
        }
        let (cw, ch) = (cx1 - cx0, cy1 - cy0);
        let crop =
            (0..ch).flat_map(|y| (0..cw).map(move |x| (x, y))).map(|(x, y)| px[(cy0 + y) * w + cx0 + x]).collect();
        // The baseline is y = 0 of the glyphs: row 1 - y0 of the picture.
        let baseline = (1.0 - y0) - cy0 as f32;
        Some((Grey { w: cw, h: ch, px: crop }, baseline))
    }
}

/// `src` resampled to `nw` x `nh` with a Lanczos (a = 3) filter, widened
/// when shrinking, separably.
fn resize(src: &Grey, nw: usize, nh: usize) -> Grey {
    fn lanczos(x: f32) -> f32 {
        let x = x.abs();
        if x < 1e-6 {
            1.0
        } else if x < 3.0 {
            let p = std::f32::consts::PI * x;
            3.0 * p.sin() * (p / 3.0).sin() / (p * p)
        } else {
            0.0
        }
    }
    // One axis: for each output position, the source taps and weights.
    fn taps(n: usize, m: usize) -> Vec<Vec<(usize, f32)>> {
        let scale = n as f32 / m as f32;
        let support = 3.0 * scale.max(1.0);
        (0..m)
            .map(|i| {
                let centre = (i as f32 + 0.5) * scale;
                let (lo, hi) =
                    ((centre - support).floor().max(0.0) as usize, ((centre + support).ceil() as usize).min(n));
                let mut t: Vec<(usize, f32)> =
                    (lo..hi).map(|j| (j, lanczos((j as f32 + 0.5 - centre) / scale.max(1.0)))).collect();
                let sum: f32 = t.iter().map(|&(_, w)| w).sum();
                if sum.abs() > 1e-6 {
                    t.iter_mut().for_each(|e| e.1 /= sum);
                }
                t
            })
            .collect()
    }
    let (tx, ty) = (taps(src.w, nw), taps(src.h, nh));
    let mut rows = vec![0.0f32; nw * src.h];
    for y in 0..src.h {
        for (x, t) in tx.iter().enumerate() {
            rows[y * nw + x] = t.iter().map(|&(j, w)| src.px[y * src.w + j] * w).sum();
        }
    }
    let mut px = vec![0.0f32; nw * nh];
    for (y, t) in ty.iter().enumerate() {
        for x in 0..nw {
            px[y * nw + x] = t.iter().map(|&(j, w)| rows[j * nw + x] * w).sum();
        }
    }
    Grey { w: nw, h: nh, px }
}

/// A boolean picture, `w` x `h`.
struct Mask {
    w: usize,
    h: usize,
    on: Vec<bool>,
}

impl Mask {
    fn new(w: usize, h: usize) -> Mask {
        Mask { w, h, on: vec![false; w * h] }
    }

    fn from_grey(g: &Grey) -> Mask {
        Mask { w: g.w, h: g.h, on: g.px.iter().map(|&v| v >= 128.0).collect() }
    }

    /// `m` put at (`x`, `y`).
    fn paste(&mut self, m: &Mask, x: usize, y: usize) {
        for j in 0..m.h.min(self.h.saturating_sub(y)) {
            for i in 0..m.w.min(self.w.saturating_sub(x)) {
                self.on[(y + j) * self.w + x + i] = m.on[j * m.w + i];
            }
        }
    }
}

/// Each texel's distance to the nearest texel where `target` is true
/// (exact Euclidean, Felzenszwalb and Huttenlocher's two passes).
fn distance(target: &[bool], w: usize, h: usize) -> Vec<f32> {
    const INF: f32 = 1e20;
    fn pass(f: &[f32], d: &mut [f32]) {
        let n = f.len();
        let (mut v, mut z) = (vec![0usize; n], vec![0f32; n + 1]);
        let mut k = 0;
        z[0] = -INF;
        z[1] = INF;
        // Where the parabola from q overtakes the one from p.
        let cross =
            |q: usize, p: usize| ((f[q] + (q * q) as f32) - (f[p] + (p * p) as f32)) / (2.0 * (q as f32 - p as f32));
        for q in 1..n {
            let mut s = cross(q, v[k]);
            while s <= z[k] {
                k -= 1;
                s = cross(q, v[k]);
            }
            k += 1;
            v[k] = q;
            z[k] = s;
            z[k + 1] = INF;
        }
        k = 0;
        for (q, out) in d.iter_mut().enumerate() {
            while z[k + 1] < q as f32 {
                k += 1;
            }
            let p = v[k];
            *out = (q as f32 - p as f32).powi(2) + f[p];
        }
    }
    let mut grid: Vec<f32> = target.iter().map(|&t| if t { 0.0 } else { INF }).collect();
    let mut col = vec![0.0f32; h];
    let mut out = vec![0.0f32; h];
    for x in 0..w {
        for y in 0..h {
            col[y] = grid[y * w + x];
        }
        pass(&col, &mut out);
        for y in 0..h {
            grid[y * w + x] = out[y];
        }
    }
    let mut row = vec![0.0f32; w];
    for y in 0..h {
        pass(&grid[y * w..(y + 1) * w], &mut row);
        grid[y * w..(y + 1) * w].copy_from_slice(&row);
    }
    grid.into_iter().map(f32::sqrt).collect()
}

/// Fill and ring (a sheet texel wide) coverage, K per texel, of `fill`
/// drawn SS times finer still; its strokes grown by `weight` texels.
fn coverage(fill: &Mask, w: usize, h: usize, weight: f32) -> (Vec<f32>, Vec<f32>) {
    let s = (K * SS) as f32;
    let off: Vec<bool> = fill.on.iter().map(|&b| !b).collect();
    let fill: Vec<bool> = if weight < 0.0 {
        // Distance of each texel of the fill to the nearest outside.
        distance(&off, fill.w, fill.h).iter().map(|&d| d > -weight * s).collect()
    } else if weight > 0.0 {
        distance(&fill.on, fill.w, fill.h).iter().map(|&d| d <= weight * s).collect()
    } else {
        fill.on.clone()
    };
    let (bw, bh) = (w * K * SS, h * K * SS);
    let near = distance(&fill, bw, bh);
    let ring: Vec<bool> = fill.iter().zip(&near).map(|(&f, &d)| f || d <= s).collect();
    let down = |m: &[bool]| -> Vec<f32> {
        let mut out = vec![0.0f32; w * K * h * K];
        for y in 0..h * K {
            for x in 0..w * K {
                let mut n = 0;
                for j in 0..SS {
                    for i in 0..SS {
                        n += usize::from(m[(y * SS + j) * bw + x * SS + i]);
                    }
                }
                out[y * w * K + x] = n as f32 / (SS * SS) as f32;
            }
        }
        out
    };
    (down(&fill), down(&ring))
}

/// The pad's buttons, outlined, `w` x `h` with strokes `stroke` wide.
fn icon(name: &str, w: usize, h: usize, stroke: f32) -> Option<Mask> {
    let s = stroke.round().max(1.0);
    let mut m = Mask::new(w, h);
    let (fw, fh) = (w as f32, h as f32);
    let inside_triangle = |px: f32, py: f32, a: (f32, f32), b: (f32, f32), c: (f32, f32)| {
        let side = |p: (f32, f32), q: (f32, f32)| (q.0 - p.0) * (py - p.1) - (q.1 - p.1) * (px - p.0);
        let (d1, d2, d3) = (side(a, b), side(b, c), side(c, a));
        !((d1 < 0.0 || d2 < 0.0 || d3 < 0.0) && (d1 > 0.0 || d2 > 0.0 || d3 > 0.0))
    };
    let segment = |px: f32, py: f32, a: (f32, f32), b: (f32, f32)| {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let t = (((px - a.0) * dx + (py - a.1) * dy) / (dx * dx + dy * dy).max(1e-6)).clamp(0.0, 1.0);
        ((px - a.0 - t * dx).powi(2) + (py - a.1 - t * dy).powi(2)).sqrt()
    };
    for y in 0..h {
        for x in 0..w {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let on = match name {
                "circle" => {
                    let (rx, ry) = (fw / 2.0, fh / 2.0);
                    let e = |rx: f32, ry: f32| ((px - fw / 2.0) / rx).powi(2) + ((py - fh / 2.0) / ry).powi(2) <= 1.0;
                    e(rx, ry) && !e((rx - s).max(0.1), (ry - s).max(0.1))
                }
                "square" => px < s || py < s || px > fw - s || py > fh - s,
                "triangle" => {
                    let t = s * 1.9;
                    inside_triangle(px, py, (fw / 2.0, 0.0), (fw, fh), (0.0, fh))
                        && !inside_triangle(px, py, (fw / 2.0, t * 1.1), (fw - t, fh - s), (t, fh - s))
                }
                "cross" => {
                    let half = s / 2.0;
                    segment(px, py, (half, half), (fw - half, fh - half)) <= half
                        || segment(px, py, (fw - half, half), (half, fh - half)) <= half
                }
                _ => return None,
            };
            m.on[y * w + x] = on;
        }
    }
    Some(m)
}

/// `text` fitted to `w` x `h`: its height, at most `stretch` times its own
/// width, centred.
fn boxed(font: &Font, text: &str, w: usize, h: usize, stretch: f32) -> Option<Mask> {
    let (a, _) = font.ink(text)?;
    let gw = ((a.w as f32 * h as f32 / a.h as f32 * stretch).round_ties_even() as usize).max(1).min(w);
    let g = Mask::from_grey(&resize(&a, gw, h));
    let mut full = Mask::new(w, h);
    full.paste(&g, (w - gw) / 2, 0);
    Some(full)
}

/// `text` at the scale whose capitals' fill is `cap` texels, standing on
/// cell row boundary `base`, centred between fx0 and fx1; squeezed about
/// the baseline only as much as leaves a texel above and below for the
/// ring, a mark too low for its cell lifted whole. None when it cannot fit.
#[allow(clippy::too_many_arguments)]
fn on_baseline(
    font: &Font,
    text: &str,
    cap: f32,
    base: usize,
    h: usize,
    fx0: usize,
    fx1: usize,
    descends: bool,
) -> Option<(usize, usize, Mask)> {
    let s = (K * SS) as f32;
    let (a, b0) = font.ink(text)?;
    let scale = cap * s / font.cap;
    let gw = ((a.w as f32 * scale).round_ties_even() as usize).max(1).min((fx1 - fx0) * K * SS);
    let (mut above, mut below) = (b0 * scale, (a.h as f32 - b0) * scale);
    if below > s / 3.0 && !descends {
        (above, below) = (above + below, 0.0);
    }
    let base_at = base as f32 * s;
    let (room_up, room_down) = (base_at - s, (h as f32 - 1.0) * s - base_at);
    let mut squeeze = 1.0f32.min(if above > 0.0 { room_up / above } else { 1.0 }).min(if below > s / 3.0 {
        room_down / below
    } else {
        1.0
    });
    if squeeze < 0.75 {
        squeeze = 1.0f32.min((h as f32 - 2.0) * s / (above + below));
    }
    let gh = (((above + below) * squeeze).round_ties_even() as usize).max(1);
    let top = (base_at - (above * squeeze).round_ties_even()).min((h as f32 - 1.0) * s - gh as f32).max(s);
    if top < 0.0 || top as usize + gh > h * K * SS {
        return None;
    }
    let g = Mask::from_grey(&resize(&a, gw, gh));
    let left = fx0 * K * SS + ((fx1 - fx0) * K * SS - gw) / 2;
    Some((top as usize, left, g))
}

fn median(v: &mut [f32]) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f32::total_cmp);
    let n = v.len();
    Some(if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 })
}

/// `idx` (`w` x `h` indices) K times finer: its indices' greys
/// Lanczos-enlarged, each finer texel the nearest of its texel's index and
/// its neighbours'.
fn smooth(idx: &[u8], w: usize, h: usize) -> Vec<u8> {
    let grey = |i: u8| f32::from(i) * 17.0;
    let up = resize(&Grey { w, h, px: idx.iter().map(|&i| grey(i)).collect() }, w * K, h * K);
    let (bw, bh) = (w * K, h * K);
    let mut out = vec![0u8; bw * bh];
    for y in 0..bh {
        for x in 0..bw {
            let (sx, sy) = (x / K, y / K);
            let target = up.px[y * bw + x];
            let mut best = idx[sy * w + sx];
            let mut dist = (grey(best) - target).powi(2);
            // Neighbours in the tool's order (row by row), the first nearer one winning.
            for dy in [-1i32, 0, 1] {
                for dx in [-1i32, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = (sx as i32 + dx).clamp(0, w as i32 - 1) as usize;
                    let ny = (sy as i32 + dy).clamp(0, h as i32 - 1) as usize;
                    let cand = idx[ny * w + nx];
                    let d = (grey(cand) - target).powi(2);
                    if d < dist {
                        (best, dist) = (cand, d);
                    }
                }
            }
            out[y * bw + x] = best;
        }
    }
    out
}

/// The sheet of `map` redrawn from `faces` in `scheme`'s indices.
fn redraw(catalog: &Catalog, map: &Map, faces: &[Face], scheme: &Scheme) -> Option<Indexed> {
    let [sheet_hash] = map.sheets[..] else { return None };
    let sheet = catalog.sheets.iter().find(|s| s.hash == sheet_hash)?;
    let depth = if map.depth == 4 { 0 } else { 1 };
    let (w, idx) = super::indexed::sheet_indices(&sheet.rows, depth);
    let (w, h) = (w as usize, sheet.rows.len());
    let mut out = smooth(&idx, w, h);
    let bw = w * K;
    let cell_of = |g: &Glyph| -> Option<(usize, usize, usize, usize)> {
        let [u, v, cw, ch] = g.cell.map(usize::from);
        (u + cw <= w && v + ch <= h).then_some((u, v, cw, ch))
    };
    let fill_rows = |u: usize, v: usize, cw: usize, ch: usize| -> Vec<usize> {
        (0..ch).filter(|&y| (0..cw).any(|x| idx[(v + y) * w + u + x] >= scheme.fill_from)).collect()
    };
    // Each face's capitals' fill height, and the sheet's baseline (where a
    // glyph's record gives none).
    let mut caps: HashMap<Option<&str>, Vec<f32>> = HashMap::new();
    let mut bases = Vec::new();
    for g in &map.glyphs {
        let Some((u, v, cw, ch)) = cell_of(g) else { continue };
        if g.text.chars().count() == 1 && "ABDEFHIKLMNPRTUVWXZ".contains(g.text.as_str()) {
            let rows = fill_rows(u, v, cw, ch);
            if let (Some(&lo), Some(&hi)) = (rows.first(), rows.last()) {
                caps.entry(g.face).or_default().push((hi - lo + 1) as f32);
                bases.push((hi + 1) as f32);
            }
        }
    }
    let caps: HashMap<Option<&str>, f32> =
        caps.into_iter().filter_map(|(f, mut c)| Some((f, median(&mut c)?))).collect();
    let page_base = median(&mut bases).map(|b| b as usize);
    // A texel around each cell first: the old ring may spill past it.
    for g in &map.glyphs {
        let Some((u, v, cw, ch)) = cell_of(g) else { continue };
        for y in v.saturating_sub(1) * K..((v + ch + 1) * K).min(h * K) {
            for x in u.saturating_sub(1) * K..((u + cw + 1) * K).min(bw) {
                if out[y * bw + x] != 0 {
                    out[y * bw + x] = scheme.clear;
                }
            }
        }
    }
    let s = K * SS;
    let mut drawn = 0;
    for g in &map.glyphs {
        if g.text.is_empty() {
            continue;
        }
        let Some(&(_, font, stretch)) = faces.iter().find(|f| f.0 == g.face) else { continue };
        let Some((u, v, cw, ch)) = cell_of(g) else { continue };
        let ink: Vec<(usize, usize)> = (0..ch)
            .flat_map(|y| (0..cw).map(move |x| (x, y)))
            .filter(|&(x, y)| idx[(v + y) * w + u + x] >= scheme.ink_from)
            .collect();
        if ink.is_empty() {
            continue;
        }
        let (x0, y0) = (ink.iter().map(|p| p.0).min()?, ink.iter().map(|p| p.1).min()?);
        let (x1, y1) = (ink.iter().map(|p| p.0).max()? + 1, ink.iter().map(|p| p.1).max()? + 1);
        let (fx0, fy0, fx1, fy1) = (x0 + 1, y0 + 1, x1.saturating_sub(1), y1.saturating_sub(1));
        if fx1 <= fx0 || fy1 <= fy0 {
            continue;
        }
        let mut big = Mask::new(cw * s, ch * s);
        if let Some(name) = g.text.strip_prefix('{').and_then(|t| t.strip_suffix('}')) {
            let stroke = if g.face == Some("large") { 2.0 } else { 1.6 } * s as f32;
            let Some(shape) = icon(name, (fx1 - fx0) * s, (fy1 - fy0) * s, stroke) else { continue };
            big.paste(&shape, fx0 * s, fy0 * s);
        } else {
            let base = g.baseline.map(usize::from).or(page_base);
            let mut placed = None;
            if let (Some(&cap), Some(base), None) = (caps.get(&g.face), base, stretch) {
                let rows = fill_rows(u, v, cw, ch);
                let descends = rows.last().is_some_and(|&r| r + 1 > base);
                placed = on_baseline(font, &g.text, cap, base, ch, fx0, fx1, descends);
            }
            match placed {
                Some((top, left, shape)) => big.paste(&shape, left, top),
                None => {
                    let Some(shape) = boxed(font, &g.text, (fx1 - fx0) * s, (fy1 - fy0) * s, stretch.unwrap_or(1.25))
                    else {
                        continue;
                    };
                    big.paste(&shape, fx0 * s, fy0 * s);
                }
            }
        }
        let (fill, ring) = coverage(&big, cw, ch, scheme.weight);
        for y in 0..ch * K {
            for x in 0..cw * K {
                let keep_zero = scheme.keep_zero && idx[(v + y / K) * w + u + x / K] == 0;
                let i = y * cw * K + x;
                out[(v * K + y) * bw + u * K + x] = if keep_zero { 0 } else { (scheme.levels)(fill[i], ring[i]) };
            }
        }
        drawn += 1;
    }
    if crate::runtime::trace("text") {
        eprintln!("[text] {}: {drawn} glyphs redrawn", map.kind);
    }
    Some(Indexed {
        sheet: sheet_hash,
        depth,
        native: (w as u32, h as u32),
        size: ((w * K) as u32, (h * K) as u32),
        indices: out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Distances to the nearest target texel, exact.
    #[test]
    fn distances_are_euclidean() {
        let (w, h) = (7, 5);
        let mut t = vec![false; w * h];
        t[2 * w + 3] = true;
        let d = distance(&t, w, h);
        assert_eq!(d[2 * w + 3], 0.0);
        assert!((d[2 * w + 6] - 3.0).abs() < 1e-4);
        assert!((d[0] - (9.0f32 + 4.0).sqrt()).abs() < 1e-4);
    }

    /// Enlarging keeps a flat picture flat.
    #[test]
    fn resizing_keeps_flat_areas() {
        let g = resize(&Grey { w: 4, h: 3, px: vec![100.0; 12] }, 32, 24);
        assert!(g.px.iter().all(|&v| (v - 100.0).abs() < 0.5));
    }

    /// The built-in fonts draw letters, with a capital's height.
    #[test]
    fn fonts_draw_letters() {
        for bytes in [super::super::capfont::BUILT_IN, COURIER_PRIME] {
            let f = Font::new(bytes);
            let (a, base) = f.ink("H").unwrap();
            assert!((a.h as f32 - f.cap).abs() <= 2.0, "{} vs {}", a.h, f.cap);
            assert!((base - a.h as f32).abs() <= 2.0);
        }
    }

    /// The buttons' outlines are hollow.
    #[test]
    fn icons_are_outlines() {
        let m = icon("circle", 64, 64, 6.0).unwrap();
        assert!(!m.on[32 * 64 + 32] && m.on[32 * 64 + 2]);
        assert!(icon("star", 8, 8, 1.0).is_none());
    }
}
