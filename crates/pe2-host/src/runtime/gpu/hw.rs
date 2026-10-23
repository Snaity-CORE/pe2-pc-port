//! The renderer, on the graphics card: the VRAM drawn with wgpu into a
//! texture `scale` times its size, which holds the whole VRAM as the
//! console would (everything the game draws, loads, fills and copies).
//!
//! Operations are recorded in order (draws as vertices, uploads as data,
//! copies) and run on the card at each `flush`. Textures and CLUTs are
//! sampled, in the shader, from a copy of the CPU's VRAM (vram.rs: what the
//! game sent from the CPU): it is sent again before a primitive samples an
//! area the CPU's VRAM changed since the last copy, and before the CPU's
//! VRAM changes an area that recorded primitives sample, so each primitive
//! sees the VRAM as it was when the game drew it.
//!
//! What only the card drew is tracked by 16x16 tiles: `stale`, where the
//! CPU's VRAM lacks it, and `hires`, where the card has it finer than the
//! CPU's VRAM can. The CPU's VRAM gets the card's pixels back (`sync`, one
//! console pixel per `scale` x `scale` block) when the game reads an area
//! (StoreImage) or uses one as a 4 or 8-bit texture or as a palette;
//! 15-bit textures in `hires` areas are sampled from the card itself, at
//! its resolution (a copy of the area in the scratch texture, as it was
//! when the primitive was drawn). The mask bit is kept in the alpha
//! channel.
//!
//! With frame interpolation, everything run for a frame is also kept, so
//! the frame can be drawn again with its model vertices moved (interp.rs):
//! the operations, and the native VRAM sent before each flush, so that the
//! drawing again samples textures as each part of the frame did (the game
//! may load other glyphs into the same place between two lines of text).
//!
//! The game never tests the mask bit when drawing (it only sets it): the
//! card sets it, and does not test it.

use std::sync::atomic::{AtomicU64, Ordering};

use super::interp;
use super::textures;
use super::vram::{DrawState, Shading, VRAM_HEIGHT, VRAM_WIDTH, Vertex as NativeVertex, Vram};
use crate::gfx::Gfx;

/// The key a screen image read back is kept by among the pictures: not
/// its hash, which a mod's picture of the same image is known by (the game
/// reads back screens that are a background as it is).
fn kept_key(hash: u64) -> u64 {
    hash.rotate_left(17) ^ 0x6B65_7074_5F73_6372
}

/// PE2_TRACE=cpu: flushes, and native VRAM halfwords sent to the card.
pub static FLUSHES: AtomicU64 = AtomicU64::new(0);
pub static VRAM_SENT: AtomicU64 = AtomicU64::new(0);
/// PE2_TRACE=cpu: console pixels read back from the card.
pub static SYNCED: AtomicU64 = AtomicU64::new(0);
/// PE2_TRACE=cpu: draw calls, render passes and vertices sent.
pub static DRAWS: AtomicU64 = AtomicU64::new(0);
pub static PASSES: AtomicU64 = AtomicU64::new(0);
pub static VERTICES: AtomicU64 = AtomicU64::new(0);

const SHADER: &str = r#"
struct Uniforms { size: vec2<f32>, scale: f32, _pad: f32 };
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var vram: texture_2d<u32>;
@group(0) @binding(2) var<storage, read> uploads: array<u32>;
@group(0) @binding(3) var scratch: texture_2d<f32>;
// Mods' textures (atlas.rs): the layers they are packed in, and per
// texture its place (x | y << 16), size (w | h << 16), layer, and the
// texels it replaces (w | h << 16).
@group(1) @binding(0) var atlas: texture_2d_array<f32>;
@group(1) @binding(1) var<storage, read> atlas_table: array<vec4<u32>>;
// Pages primitives draw: per page column, the texture replacing its sheet
// (table index + 1, 0: none), then the VRAM row the sheet's top is at.
@group(1) @binding(2) var<storage, read> atlas_pages: array<vec4<u32>>;
// Indexed replacements' values (a table entry's layer with bit 31 set):
// palette indices, read as stored and looked up in the primitive's CLUT.
@group(1) @binding(3) var atlas_indices: texture_2d_array<u32>;

struct In {
    @location(0) pos: vec2<f32>,
    @location(1) color: u32,
    @location(2) uv: vec2<f32>,
    @location(3) mode: u32,
    @location(4) clut: u32,
    @location(5) window: u32,
    @location(6) w: f32,
    @location(7) limits: u32,
    @location(8) repl: u32,
};
struct Out {
    @builtin(position) pos: vec4<f32>,
    // Perspective-correct with the vertices' depths in w; with w = 1 (no
    // depths), plain linear interpolation as on the console.
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) @interpolate(flat) mode: u32,
    @location(3) @interpolate(flat) clut: u32,
    @location(4) @interpolate(flat) window: u32,
    @location(5) @interpolate(flat) limits: u32,
    @location(6) @interpolate(flat) repl: u32,
};

@vertex
fn vs(i: In) -> Out {
    var o: Out;
    // Pixel (x, y) is covered when its integer position is inside, as in
    // the console's GPU: shift by half a pixel to the card's centres.
    let p = (i.pos + vec2<f32>(0.5, 0.5)) / u.size * 2.0 - 1.0;
    let w = select(1.0, i.w, i.w > 0.0);
    o.pos = vec4<f32>(p.x * w, -p.y * w, 0.0, w);
    o.color = vec3<f32>(f32(i.color & 0xFFu), f32((i.color >> 8u) & 0xFFu), f32((i.color >> 16u) & 0xFFu));
    o.uv = i.uv;
    o.mode = i.mode;
    o.clut = i.clut;
    o.window = i.window;
    o.limits = i.limits;
    o.repl = i.repl;
    return o;
}

fn vram_at(x: u32, y: u32) -> u32 {
    return textureLoad(vram, vec2<u32>(x & 1023u, y & 511u), 0).r;
}

// mode: bits 0-9 texture page x, 10-18 page y, 19 a 15-bit texture read
// from the scratch texture (the card's own pixels), 20-21 depth, 22
// textured, 23 raw texture, 24-25 pass (0 all texels, 1 those without bit
// 15, 2 those with it), 26-27 kind (0 primitive, 1 upload, 2 copy), 28
// bilinear filtering (a copy: the game's grey negative of it), 29 texel
// clamping (limits: the polygon's u, v minimum and maximum, a byte each),
// 30 set the mask bit. Alpha is the mask bit written.
@fragment
fn fs(o: Out) -> @location(0) vec4<f32> {
    let kind = (o.mode >> 26u) & 3u;
    if (kind == 1u) {
        // Upload: clut = first halfword in `uploads`, window = row width,
        // limits = rows; bit 28: smoothed (bicubic) rather than blocks.
        let n = vec2<u32>(floor(o.uv));
        let index = o.clut + n.y * o.window + n.x;
        let word = uploads[index >> 1u];
        let px = (word >> ((index & 1u) * 16u)) & 0xFFFFu;
        let mask = f32(px >> 15u);
        if (((o.mode >> 28u) & 1u) != 0u) {
            return vec4<f32>(upload_bicubic(o.clut, o.window, o.limits, o.uv), mask);
        }
        return vec4<f32>(rgb15(px), mask);
    }
    if (kind == 2u) {
        let c = textureLoad(scratch, vec2<u32>(floor(o.uv)), 0);
        if (((o.mode >> 28u) & 1u) != 0u) {
            // As the game's screen negative: 31 - (3R + 4G + B) / 8 in
            // 5-bit colour, grey (shared/screen_negative_filter.inc.c).
            let v = c.rgb * 255.0 / 8.0;
            let grey = 31.0 - floor((3.0 * v.r + 4.0 * v.g + v.b) / 8.0);
            return vec4<f32>(vec3<f32>(max(grey, 0.0) * 8.0 / 255.0), 0.0);
        }
        return c;
    }
    var c = floor(o.color);
    var written_mask = (o.mode >> 30u) & 1u;
    let from_layer = ((o.mode >> 19u) & 1u) != 0u && ((o.mode >> 20u) & 3u) >= 2u;
    if (((o.mode >> 22u) & 1u) != 0u) {
        var t: u32;
        var tc: vec3<f32>;
        // A mod's texture for the sheet this texel is of, if any (repl:
        // the page's index + 1 in atlas_pages).
        var texture = 0u;
        var sheet_texel = vec2<f32>(0.0);
        if (o.repl != 0u) {
            let per = 4u >> ((o.mode >> 20u) & 3u);
            let width = f32(64u * per);
            // The texel the game reads (the polygon's clamp, the texture
            // window), and where in it this pixel is: the finer texels of a
            // larger replacement keep the game's texel coordinates.
            var at = vec2<i32>(floor(o.uv));
            if (((o.mode >> 29u) & 1u) != 0u) {
                let lo = vec2<i32>(i32(o.limits & 0xFFu), i32((o.limits >> 8u) & 0xFFu));
                let hi = vec2<i32>(i32((o.limits >> 16u) & 0xFFu), i32((o.limits >> 24u) & 0xFFu));
                at = clamp(at, lo, hi);
            }
            let within = clamp(o.uv - floor(o.uv), vec2<f32>(0.0), vec2<f32>(0.999));
            let uv = vec2<f32>(windowed(o.window, at)) + within;
            let column = min(u32(uv.x / width), 3u);
            texture = atlas_pages[(o.repl - 1u) * 2u][column];
            let top = atlas_pages[(o.repl - 1u) * 2u + 1u][column];
            sheet_texel = vec2<f32>(uv.x - f32(column) * width, f32((o.mode >> 10u) & 511u) + uv.y - f32(top));
        }
        let d = atlas_table[max(texture, 1u) - 1u];
        let texels = vec2<f32>(f32(d.w & 0xFFFFu), f32(d.w >> 16u));
        let size = vec2<u32>(max(d.y & 0xFFFFu, 1u), max(d.y >> 16u, 1u));
        let q = min(vec2<u32>(max(sheet_texel / texels * vec2<f32>(size), vec2<f32>(0.0))), size - vec2<u32>(1u));
        let place = vec2<u32>(d.x & 0xFFFFu, d.x >> 16u) + q;
        if (texture != 0u && (d.z & 0x80000000u) != 0u) {
            // An indexed replacement: its value is the texel's palette
            // index, looked up in the CLUT the game set for this
            // primitive, as the console looks the game's own up (colour 0
            // transparent, the colour's bit 15 its mask bit).
            let depth = (o.mode >> 20u) & 3u;
            let index = textureLoad(atlas_indices, place, d.z & 0x7FFFFFFFu, 0).r & select(0xFFu, 0xFu, depth == 0u);
            t = vram_at((o.clut & 1023u) + index, (o.clut >> 10u) & 511u);
            if (t == 0u) {
                discard;
            }
            tc = texel_rgb(t);
        } else if (texture != 0u) {
            // Its colours and transparency are drawn; the game's own texel
            // still decides the mask bit.
            let s = textureLoad(atlas, place, d.z, 0);
            if (s.a < 0.5) {
                discard;
            }
            tc = floor(s.rgb * 255.0);
            var at = vec2<i32>(floor(o.uv));
            if (((o.mode >> 29u) & 1u) != 0u) {
                let lo = vec2<i32>(i32(o.limits & 0xFFu), i32((o.limits >> 8u) & 0xFFu));
                let hi = vec2<i32>(i32((o.limits >> 16u) & 0xFFu), i32((o.limits >> 24u) & 0xFFu));
                at = clamp(at, lo, hi);
            }
            t = texel(o.mode, o.clut, o.window, at.x, at.y);
        } else if (((o.mode >> 28u) & 1u) != 0u && !from_layer) {
            // Bilinear: the four texels around the sample point, kept inside
            // the polygon's own texture rectangle (`limits`, so neighbours
            // in the texture page do not bleed in). Transparent texels
            // (value 0) weigh nothing: colour is premultiplied by coverage,
            // and a pixel less than half covered is not drawn.
            let lo = vec2<i32>(i32(o.limits & 0xFFu), i32((o.limits >> 8u) & 0xFFu));
            let hi = vec2<i32>(i32((o.limits >> 16u) & 0xFFu), i32((o.limits >> 24u) & 0xFFu));
            let p = o.uv - vec2<f32>(0.5, 0.5);
            let base = vec2<i32>(floor(p));
            let f = p - floor(p);
            var sum = vec3<f32>(0.0);
            var cover = 0.0;
            var best = 0.0;
            t = 0u;
            for (var k = 0u; k < 4u; k++) {
                let d = vec2<i32>(i32(k & 1u), i32(k >> 1u));
                let at = clamp(base + d, lo, hi);
                let w = select(1.0 - f.x, f.x, d.x == 1) * select(1.0 - f.y, f.y, d.y == 1);
                let s = texel(o.mode, o.clut, o.window, at.x, at.y);
                if (s != 0u) {
                    sum += w * texel_rgb(s);
                    cover += w;
                    // The heaviest opaque texel decides the mask bit.
                    if (w > best) {
                        best = w;
                        t = s;
                    }
                }
            }
            if (cover < 0.5) {
                discard;
            }
            tc = sum / cover;
        } else {
            var at = vec2<i32>(floor(o.uv));
            if (((o.mode >> 29u) & 1u) != 0u) {
                let lo = vec2<i32>(i32(o.limits & 0xFFu), i32((o.limits >> 8u) & 0xFFu));
                let hi = vec2<i32>(i32((o.limits >> 16u) & 0xFFu), i32((o.limits >> 24u) & 0xFFu));
                at = clamp(at, lo, hi);
            }
            if (from_layer) {
                // The card's pixel under this point of the texel: finer
                // than the console's.
                let w = windowed(o.window, at);
                let base = vec2<f32>(f32(((o.mode & 1023u) + w.x) & 1023u), f32((((o.mode >> 10u) & 511u) + w.y) & 511u));
                let within = clamp(o.uv - floor(o.uv), vec2<f32>(0.0), vec2<f32>(0.999));
                let s = textureLoad(scratch, vec2<u32>((base + within) * u.scale), 0);
                t = word15(s);
                tc = floor(s.rgb * 255.0);
            } else {
                t = texel(o.mode, o.clut, o.window, at.x, at.y);
                tc = texel_rgb(t);
            }
            if (t == 0u) {
                discard;
            }
        }
        let pass_ = (o.mode >> 24u) & 3u;
        let mask = (t >> 15u) & 1u;
        if ((pass_ == 1u && mask == 1u) || (pass_ == 2u && mask == 0u)) {
            discard;
        }
        written_mask = written_mask | mask;
        if (((o.mode >> 23u) & 1u) != 0u) {
            c = tc;
        } else {
            c = floor(tc * c / 128.0);
        }
    }
    return vec4<f32>(min(c, vec3<f32>(255.0)) / 255.0, f32(written_mask));
}

// A pixel of the card as the console's 15-bit value (5 bits a channel,
// the mask bit from alpha).
fn word15(s: vec4<f32>) -> u32 {
    let c = vec3<u32>(s.rgb * 255.0 + 0.5) >> vec3<u32>(3u);
    return c.r | (c.g << 5u) | (c.b << 10u) | select(0u, 0x8000u, s.a > 0.5);
}

// Texel coordinates (u, v) through the texture window.
fn windowed(window: u32, at: vec2<i32>) -> vec2<u32> {
    let mx = window & 31u;
    let my = (window >> 5u) & 31u;
    let ox = (window >> 10u) & 31u;
    let oy = (window >> 15u) & 31u;
    return vec2<u32>(
        ((u32(at.x) & ~(mx * 8u)) | ((ox & mx) * 8u)) & 0xFFu,
        ((u32(at.y) & ~(my * 8u)) | ((oy & my) * 8u)) & 0xFFu,
    );
}

// The 15-bit texel at (u, v) of the primitive's texture page, through its
// texture window and CLUT.
fn texel(mode: u32, clut: u32, window: u32, u_in: i32, v_in: i32) -> u32 {
    let w = windowed(window, vec2<i32>(u_in, v_in));
    let tu = w.x;
    let tv = w.y;
    let px = mode & 1023u;
    let py = (mode >> 10u) & 511u;
    let depth = (mode >> 20u) & 3u;
    let cx = clut & 1023u;
    let cy = (clut >> 10u) & 511u;
    if (depth == 0u) {
        let word = vram_at(px + tu / 4u, py + tv);
        return vram_at(cx + ((word >> ((tu & 3u) * 4u)) & 0xFu), cy);
    }
    if (depth == 1u) {
        let word = vram_at(px + tu / 2u, py + tv);
        return vram_at(cx + ((word >> ((tu & 1u) * 8u)) & 0xFFu), cy);
    }
    if (((mode >> 19u) & 1u) != 0u) {
        let at = vec2<f32>(f32((px + tu) & 1023u), f32((py + tv) & 511u));
        return word15(textureLoad(scratch, vec2<u32>(at * u.scale), 0));
    }
    return vram_at(px + tu, py + tv);
}

// Pixel (x, y) of an upload `w` x `h` starting at halfword `first`, the
// nearest one inside it.
fn upload_px(first: u32, w: u32, h: u32, x: i32, y: i32) -> vec3<f32> {
    let cx = u32(clamp(x, 0, i32(w) - 1));
    let cy = u32(clamp(y, 0, i32(h) - 1));
    let index = first + cy * w + cx;
    let word = uploads[index >> 1u];
    return rgb15((word >> ((index & 1u) * 16u)) & 0xFFFFu);
}

// Catmull-Rom weights of the four pixels around a sample `t` past the
// second one: sharp, and exact at the pixels themselves.
fn catmull_rom(t: f32) -> vec4<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    return vec4<f32>(
        -0.5 * t3 + t2 - 0.5 * t,
        1.5 * t3 - 2.5 * t2 + 1.0,
        -1.5 * t3 + 2.0 * t2 + 0.5 * t,
        0.5 * t3 - 0.5 * t2,
    );
}

// An upload sampled bicubically at `uv` (in its pixels), its edges
// extended: pre-rendered backgrounds enlarged smoothly instead of in blocks.
fn upload_bicubic(first: u32, w: u32, h: u32, uv: vec2<f32>) -> vec3<f32> {
    let p = uv - vec2<f32>(0.5, 0.5);
    let base = vec2<i32>(floor(p));
    let f = p - floor(p);
    let wx = catmull_rom(f.x);
    let wy = catmull_rom(f.y);
    var sum = vec3<f32>(0.0);
    for (var j = 0; j < 4; j++) {
        var row = vec3<f32>(0.0);
        for (var i = 0; i < 4; i++) {
            row += wx[i] * upload_px(first, w, h, base.x + i - 1, base.y + j - 1);
        }
        sum += wy[j] * row;
    }
    return clamp(sum, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn texel_rgb(t: u32) -> vec3<f32> {
    return vec3<f32>(f32((t & 31u) << 3u), f32(((t >> 5u) & 31u) << 3u), f32(((t >> 10u) & 31u) << 3u));
}

fn rgb15(px: u32) -> vec3<f32> {
    return vec3<f32>(f32((px & 31u) << 3u), f32(((px >> 5u) & 31u) << 3u), f32(((px >> 10u) & 31u) << 3u)) / 255.0;
}
"#;

/// The card's pixels read back as the console's: one per `scale` x
/// `scale` block, its top-left one (where the console samples coverage).
const DOWN_SHADER: &str = r#"
struct Uniforms { size: vec2<f32>, scale: f32, _pad: f32 };
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var card: texture_2d<f32>;

@vertex
fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn fs(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    return textureLoad(card, vec2<u32>(floor(pos.xy)) * u32(u.scale), 0);
}
"#;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GpuVertex {
    pos: [f32; 2],
    color: u32,
    uv: [f32; 2],
    mode: u32,
    clut: u32,
    window: u32,
    /// Depth for perspective-correct interpolation (0: none).
    w: f32,
    /// Bilinear filtering: the polygon's texture rectangle (u0, v0, u1, v1).
    limits: u32,
    /// Mods' textures: the page's index + 1 in the atlas's page table (0:
    /// none).
    repl: u32,
}

const FILTER_BILINEAR: u32 = 1 << 28;
const UV_CLAMP: u32 = 1 << 29;
/// A 15-bit texture sampled from the card's pixels (in the scratch texture).
const FROM_CARD: u32 = 1 << 19;
const SET_MASK: u32 = 1 << 30;

/// Whether 3D polygons' textures are filtered: PE2_TEXTURE_FILTER, else
/// pe2.toml's [video] texture_filter ("nearest", the default, or
/// "bilinear"). Sprites (text, interface) are never filtered.
fn bilinear() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let choice =
            std::env::var("PE2_TEXTURE_FILTER").ok().or_else(|| crate::config::get().video.texture_filter.clone());
        match choice.as_deref().map(str::trim) {
            None | Some("nearest") => false,
            Some("bilinear") => true,
            Some(other) => {
                eprintln!("pe2: unknown texture filter {other:?} (nearest or bilinear); using nearest");
                false
            }
        }
    })
}

/// Whether image transfers (the pre-rendered backgrounds, full-screen
/// pictures) are enlarged smoothly: PE2_BACKGROUND_FILTER, else pe2.toml's
/// [video] background_filter ("nearest", the default: blocks of whole
/// console pixels, or "bicubic").
fn smooth_uploads() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let choice = std::env::var("PE2_BACKGROUND_FILTER")
            .ok()
            .or_else(|| crate::config::get().video.background_filter.clone());
        match choice.as_deref().map(str::trim) {
            None | Some("nearest") => false,
            Some("bicubic") => true,
            Some(other) => {
                eprintln!("pe2: unknown background filter {other:?} (nearest or bicubic); using nearest");
                false
            }
        }
    })
}

/// Smaller transfers stay in blocks: bits of interface, not pictures.
const SMOOTH_UPLOAD_MIN: i32 = 64;

const KIND_UPLOAD: u32 = 1 << 26;
/// The atlas key's palette for indexed replacements, less the depth (no
/// palette's hash).
const INDEXED_KEY: u64 = u64::MAX;
const UPLOAD_SMOOTH: u32 = 1 << 28;
const KIND_COPY: u32 = 2 << 26;
/// A copy drawn as the game's grey screen negative of what it copies.
const COPY_NEGATIVE: u32 = 1 << 28;

/// Fixed-function blending for the four semi-transparency modes, or none.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Blend {
    None,
    Average,
    Add,
    Subtract,
    AddQuarter,
}

impl Blend {
    fn index(self) -> usize {
        self as usize
    }

    fn constant(self) -> f64 {
        match self {
            Blend::Average => 0.5,
            Blend::AddQuarter => 0.25,
            _ => 1.0,
        }
    }

    fn state(self) -> Option<wgpu::BlendState> {
        use wgpu::{BlendComponent, BlendFactor as F, BlendOperation as Op};
        let color = match self {
            Blend::None => return None,
            Blend::Average => BlendComponent { src_factor: F::Constant, dst_factor: F::Constant, operation: Op::Add },
            Blend::Add => BlendComponent { src_factor: F::One, dst_factor: F::One, operation: Op::Add },
            Blend::Subtract => {
                BlendComponent { src_factor: F::One, dst_factor: F::One, operation: Op::ReverseSubtract }
            }
            Blend::AddQuarter => BlendComponent { src_factor: F::Constant, dst_factor: F::One, operation: Op::Add },
        };
        Some(wgpu::BlendState { color, alpha: BlendComponent::REPLACE })
    }
}

/// A recorded operation.
#[derive(Clone, Copy)]
enum Op {
    /// Vertices `first..first + count` with one blend mode and scissor.
    Draw { blend: Blend, scissor: [u32; 4], first: u32, count: u32 },
    /// Copies a rectangle of the target to the same place in the scratch
    /// texture (for a VRAM-to-VRAM copy drawn by the next Draw).
    ToScratch { x: u32, y: u32, w: u32, h: u32 },
    /// Copies a mod's picture (by its screen image's hash) to (x, y).
    Picture { hash: u64, x: u32, y: u32 },
    /// Copies a picture to (x, y) of the scratch texture (for the Draw
    /// after it).
    PictureToScratch { hash: u64, x: u32, y: u32 },
}

/// A native VRAM rectangle, inclusive.
#[derive(Clone, Copy)]
struct Area {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Area {
    const EMPTY: Area = Area { x0: i32::MAX, y0: i32::MAX, x1: i32::MIN, y1: i32::MIN };

    fn new(x: i32, y: i32, w: i32, h: i32) -> Area {
        Area { x0: x, y0: y, x1: x + w - 1, y1: y + h - 1 }
    }

    fn union(self, o: Area) -> Area {
        Area { x0: self.x0.min(o.x0), y0: self.y0.min(o.y0), x1: self.x1.max(o.x1), y1: self.y1.max(o.y1) }
    }

    fn hits(self, o: Area) -> bool {
        self.x0 <= o.x1 && o.x0 <= self.x1 && self.y0 <= o.y1 && o.y0 <= self.y1
    }

    fn contains(self, o: Area) -> bool {
        self.x0 <= o.x0 && o.x1 <= self.x1 && self.y0 <= o.y0 && o.y1 <= self.y1
    }
}

/// A few rectangles: texture pages and CLUTs lie far apart, and one
/// bounding box of them would cover the framebuffers too.
#[derive(Default)]
struct Areas(Vec<Area>);

impl Areas {
    const MAX: usize = 32;

    fn add(&mut self, a: Area) {
        if a.x0 > a.x1 || a.y0 > a.y1 || self.0.iter().any(|b| b.contains(a)) {
            return;
        }
        self.0.retain(|b| !a.contains(*b));
        if self.0.len() == Self::MAX {
            // Too many: merge everything (correct, if coarse).
            let all = self.0.iter().fold(a, |u, b| u.union(*b));
            self.0.clear();
            self.0.push(all);
        } else {
            self.0.push(a);
        }
    }

    fn hits(&self, a: Area) -> bool {
        self.0.iter().any(|b| b.hits(a))
    }

    fn covers(&self, a: Area) -> bool {
        self.0.iter().any(|b| b.contains(a))
    }

    fn remove_hits(&mut self, a: Area) {
        self.0.retain(|b| !b.hits(a));
    }

    fn bounds(&self) -> Area {
        self.0.iter().fold(Area::EMPTY, |u, b| u.union(*b))
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

/// 16x16 tiles of the VRAM, one bit each (a row of 64 tiles per word).
#[derive(Clone, Default)]
struct Tiles([u64; VRAM_HEIGHT / 16]);

impl Tiles {
    /// The tiles `a` touches, clipped to the VRAM: rows, and the column
    /// mask.
    fn touched(a: Area) -> Option<(std::ops::RangeInclusive<usize>, u64)> {
        let (x0, y0) = (a.x0.max(0), a.y0.max(0));
        let (x1, y1) = (a.x1.min(VRAM_WIDTH as i32 - 1), a.y1.min(VRAM_HEIGHT as i32 - 1));
        if x0 > x1 || y0 > y1 {
            return None;
        }
        Some(((y0 >> 4) as usize..=(y1 >> 4) as usize, columns(x0 >> 4, x1 >> 4)))
    }

    fn mark(&mut self, a: Area) {
        if let Some((rows, mask)) = Self::touched(a) {
            for r in rows {
                self.0[r] |= mask;
            }
        }
    }

    fn hits(&self, a: Area) -> bool {
        Self::touched(a).is_some_and(|(rows, mask)| rows.into_iter().any(|r| self.0[r] & mask != 0))
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self.0[(y as usize >> 4) % self.0.len()] >> ((x as u32 >> 4) & 63) & 1 != 0
    }

    /// Clears the tiles `a` covers whole.
    fn clear_covered(&mut self, a: Area) {
        let (x0, y0) = ((a.x0.max(0) + 15) >> 4, (a.y0.max(0) + 15) >> 4);
        let (x1, y1) = ((a.x1.min(VRAM_WIDTH as i32 - 1) + 1) >> 4, (a.y1.min(VRAM_HEIGHT as i32 - 1) + 1) >> 4);
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        let mask = columns(x0, x1 - 1);
        for r in y0 as usize..y1 as usize {
            self.0[r] &= !mask;
        }
    }
}

/// Tile columns `c0..=c1` as a mask.
fn columns(c0: i32, c1: i32) -> u64 {
    let n = (c1 - c0 + 1) as u32;
    if n >= 64 { u64::MAX } else { ((1u64 << n) - 1) << c0 }
}

/// Native VRAM sent to the card: x, y, width, height and the halfwords.
struct VramUpdate {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    rows: Vec<u16>,
}

/// Work run on the card in one flush, kept to draw the frame again.
pub struct Chunk {
    /// The native VRAM the flush sent first.
    vram: Option<VramUpdate>,
    ops: Vec<Op>,
    vertices: Vec<GpuVertex>,
    /// Each vertex's key (Vertex::key; 0: none).
    keys: Vec<u32>,
    uploads: Vec<u16>,
}

/// The positions of a frame's keyed vertices, in this layer's pixels.
pub fn positions(frame: &[Chunk]) -> std::collections::HashMap<u32, [f32; 2]> {
    let mut out = std::collections::HashMap::new();
    for c in frame {
        for (v, &k) in c.vertices.iter().zip(&c.keys) {
            if k != 0 {
                out.entry(k).or_insert(v.pos);
            }
        }
    }
    out
}

pub struct Hw {
    gfx: &'static Gfx,
    pub scale: i32,
    width: u32,
    height: u32,
    pub target: wgpu::Texture,
    scratch: wgpu::Texture,
    vram: wgpu::Texture,
    uniforms: wgpu::Buffer,
    vertex_buffer: Option<wgpu::Buffer>,
    upload_buffer: wgpu::Buffer,
    bind: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    pipelines: Vec<wgpu::RenderPipeline>,
    vertices: Vec<GpuVertex>,
    keys: Vec<u32>,
    uploads: Vec<u16>,
    ops: Vec<Op>,
    /// The flushes of the frame being drawn, when frames are kept.
    frame: Option<Vec<Chunk>>,
    /// Where frames are drawn again (created when first needed).
    pub between: Option<wgpu::Texture>,
    /// The framebuffer the next frame is drawn into, as the last frame left
    /// it (where it is in the layer): what the next frame starts from.
    before: Option<(wgpu::Texture, interp::Region)>,
    /// Mods' pictures at this layer's scale, by screen image hash, with
    /// when each was last used (the least recent go beyond PICTURES_KEPT).
    pictures: std::collections::HashMap<u64, (wgpu::Texture, u64)>,
    pictures_used: u64,
    /// The native VRAM copy as the last frame left it, and the copy frames
    /// are drawn again with (rebuilt from it flush by flush), with its bind
    /// group.
    vram_before: Option<wgpu::Texture>,
    vram_replay: Option<(wgpu::Texture, wgpu::BindGroup)>,
    /// Native VRAM written since the last copy sent to the card, and the
    /// native VRAM recorded primitives sample.
    dirty: Areas,
    sampled: Areas,
    /// What only the card drew (see the top of this file), and the areas
    /// the scratch texture holds a copy of the card's pixels of, for
    /// primitives sampling them.
    stale: Tiles,
    hires: Tiles,
    snapped: Areas,
    /// Reading the card back: the downscaling pass and what it draws into.
    down: (wgpu::RenderPipeline, wgpu::BindGroup, wgpu::Texture),
    /// Mods' textures: what primitives draw from, and the replacements on
    /// the card.
    tracker: super::textures::Tracker,
    atlas: super::atlas::Atlas,
}

impl Hw {
    pub fn new(gfx: &'static Gfx, scale: i32) -> Hw {
        let device = &gfx.device;
        let (width, height) = (VRAM_WIDTH as u32 * scale as u32, VRAM_HEIGHT as u32 * scale as u32);
        let texture = |label, w, h, format, usage| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
        };
        use wgpu::TextureUsages as U;
        let target = texture(
            "hw target",
            width,
            height,
            wgpu::TextureFormat::Rgba8Unorm,
            U::RENDER_ATTACHMENT | U::TEXTURE_BINDING | U::COPY_SRC | U::COPY_DST,
        );
        let scratch =
            texture("hw scratch", width, height, wgpu::TextureFormat::Rgba8Unorm, U::TEXTURE_BINDING | U::COPY_DST);
        let vram = texture(
            "hw vram",
            VRAM_WIDTH as u32,
            VRAM_HEIGHT as u32,
            wgpu::TextureFormat::R16Uint,
            U::TEXTURE_BINDING | U::COPY_DST | U::COPY_SRC,
        );
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hw uniforms"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let u: [f32; 4] = [width as f32, height as f32, scale as f32, 0.0];
        gfx.queue.write_buffer(&uniforms, 0, bytes(&u));
        let upload_buffer = Self::make_upload_buffer(device, 1 << 20);

        let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
            ty,
            count: None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("hw"),
            entries: &[
                entry(
                    0,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
                entry(
                    1,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
                entry(
                    2,
                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                ),
                entry(
                    3,
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                ),
            ],
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hw"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let atlas = super::atlas::Atlas::new(gfx, crate::mods::wants_textures());
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hw"),
            bind_group_layouts: &[Some(&layout), Some(&atlas.layout)],
            ..Default::default()
        });
        let attributes = wgpu::vertex_attr_array![0 => Float32x2, 1 => Uint32, 2 => Float32x2, 3 => Uint32, 4 => Uint32, 5 => Uint32, 6 => Float32, 7 => Uint32, 8 => Uint32];
        let pipelines = [Blend::None, Blend::Average, Blend::Add, Blend::Subtract, Blend::AddQuarter]
            .iter()
            .map(|b| {
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("hw"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: Some("vs"),
                        buffers: &[Some(wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<GpuVertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &attributes,
                        })],
                        compilation_options: Default::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: Some("fs"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            blend: b.state(),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: Default::default(),
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                    cache: None,
                })
            })
            .collect();
        let bind = Self::make_bind(device, &layout, &uniforms, &vram, &upload_buffer, &scratch);
        let down = Self::make_down(device, &uniforms, &target);
        Hw {
            gfx,
            scale,
            width,
            height,
            target,
            scratch,
            vram,
            uniforms,
            vertex_buffer: None,
            upload_buffer,
            bind,
            layout,
            pipelines,
            vertices: Vec::new(),
            keys: Vec::new(),
            uploads: Vec::new(),
            ops: Vec::new(),
            frame: None,
            between: None,
            before: None,
            pictures: std::collections::HashMap::new(),
            pictures_used: 0,
            vram_before: None,
            vram_replay: None,
            dirty: Areas(vec![Area::new(0, 0, VRAM_WIDTH as i32, VRAM_HEIGHT as i32)]),
            sampled: Areas::default(),
            stale: Tiles::default(),
            hires: Tiles::default(),
            snapped: Areas::default(),
            down,
            tracker: Default::default(),
            atlas,
        }
    }

    /// The pass reading the card back at the console's resolution.
    fn make_down(
        device: &wgpu::Device,
        uniforms: &wgpu::Buffer,
        target: &wgpu::Texture,
    ) -> (wgpu::RenderPipeline, wgpu::BindGroup, wgpu::Texture) {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hw down"),
            source: wgpu::ShaderSource::Wgsl(DOWN_SHADER.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("hw down"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hw down"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&target.create_view(&Default::default())),
                },
            ],
        });
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hw down"),
            size: wgpu::Extent3d { width: VRAM_WIDTH as u32, height: VRAM_HEIGHT as u32, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        (pipeline, bind, texture)
    }

    fn make_upload_buffer(device: &wgpu::Device, size: u64) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hw uploads"),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn make_bind(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        uniforms: &wgpu::Buffer,
        vram: &wgpu::Texture,
        uploads: &wgpu::Buffer,
        scratch: &wgpu::Texture,
    ) -> wgpu::BindGroup {
        let vram_view = vram.create_view(&Default::default());
        let scratch_view = scratch.create_view(&Default::default());
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("hw"),
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&vram_view) },
                wgpu::BindGroupEntry { binding: 2, resource: uploads.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&scratch_view) },
            ],
        })
    }

    // ---- ordering with the CPU's VRAM -----------------------------------

    /// Before the CPU's VRAM changes `x, y, w, h`: recorded primitives that
    /// sample it must run first.
    pub fn before_cpu_write(&mut self, vram: &Vram, x: i32, y: i32, w: i32, h: i32) {
        if self.sampled.hits(Area::new(x, y, w, h)) {
            self.flush(vram);
        }
    }

    /// After the CPU's VRAM changed `x, y, w, h` (the same is recorded for
    /// the card): it has what the card will draw there.
    pub fn after_cpu_write(&mut self, x: i32, y: i32, w: i32, h: i32) {
        let a = Area::new(x, y, w, h);
        self.dirty.add(a);
        self.stale.clear_covered(a);
        self.hires.clear_covered(a);
        self.snapped.remove_hits(a);
        if crate::mods::wants_textures() {
            self.tracker.write(x, y, w, h);
        }
    }

    /// A primitive recorded for the card changes `x, y, w, h`, which the
    /// CPU's VRAM does not get.
    pub fn drew(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if w <= 0 || h <= 0 {
            return;
        }
        let a = Area::new(x, y, w, h);
        self.stale.mark(a);
        self.hires.mark(a);
        self.snapped.remove_hits(a);
        if crate::mods::wants_textures() {
            self.tracker.write(x, y, w, h);
        }
    }

    /// A VRAM-to-VRAM copy (after `after_cpu_write` of its destination):
    /// what only the card had in the source, it has in the destination.
    pub fn copied(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        let (from, to) = (Area::new(sx, sy, w, h), Area::new(dx, dy, w, h));
        if self.stale.hits(from) {
            self.stale.mark(to);
        }
        if self.hires.hits(from) {
            self.hires.mark(to);
        }
    }

    /// The card shows `x, y, w, h` finer than the CPU's VRAM has it (a
    /// mod's picture, a screen kept at this resolution), though the CPU's
    /// VRAM has the game's own pixels.
    fn finer(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.hires.mark(Area::new(x, y, w, h));
    }

    /// Gives the CPU's VRAM the card's pixels in `x, y, w, h` where only
    /// the card has them (one per `scale` x `scale` block): the game reads
    /// them, or uses them as indexed texels or palettes.
    pub fn sync(&mut self, vram: &mut Vram, x: i32, y: i32, w: i32, h: i32) {
        let a =
            Area::new(x.max(0), y.max(0), w.min(VRAM_WIDTH as i32 - x.max(0)), h.min(VRAM_HEIGHT as i32 - y.max(0)));
        if a.x1 < a.x0 || a.y1 < a.y0 || !self.stale.hits(a) {
            return;
        }
        self.flush(vram);
        let Some(pixels) = self.read_down(a) else { return };
        let w = (a.x1 - a.x0 + 1) as usize;
        for (i, &px) in pixels.iter().enumerate() {
            let (px_x, px_y) = (a.x0 + (i % w) as i32, a.y0 + (i / w) as i32);
            if self.stale.get(px_x, px_y) {
                vram.set(px_x, px_y, px);
            }
        }
        SYNCED.fetch_add(pixels.len() as u64, Ordering::Relaxed);
        self.stale.clear_covered(a);
        self.dirty.add(a);
        if crate::mods::wants_textures() {
            self.tracker.write(a.x0, a.y0, a.x1 - a.x0 + 1, a.y1 - a.y0 + 1);
        }
    }

    /// The card's pixels in `a`, at the console's resolution (waits for
    /// the card).
    fn read_down(&self, a: Area) -> Option<Vec<u16>> {
        let gfx = self.gfx;
        let (x, y, w, h) = (a.x0 as u32, a.y0 as u32, (a.x1 - a.x0 + 1) as u32, (a.y1 - a.y0 + 1) as u32);
        let (pipeline, bind, texture) = &self.down;
        let row = (w * 4).next_multiple_of(256);
        let buffer = gfx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hw read back"),
            size: u64::from(row * h),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = gfx.device.create_command_encoder(&Default::default());
        {
            let view = texture.create_view(&Default::default());
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hw down"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bind, &[]);
            pass.set_scissor_rect(x, y, w, h);
            pass.draw(0..3, 0..1);
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row), rows_per_image: Some(h) },
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        gfx.queue.submit([encoder.finish()]);
        buffer.map_async(wgpu::MapMode::Read, .., |_| {});
        gfx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).ok()?;
        let data = buffer.get_mapped_range(..).ok()?;
        let mut out = Vec::with_capacity((w * h) as usize);
        for line in data.chunks(row as usize).take(h as usize) {
            out.extend(line[..w as usize * 4].as_chunks::<4>().0.iter().map(|p| {
                let c = |v: u8| u16::from(v >> 3);
                c(p[0]) | (c(p[1]) << 5) | (c(p[2]) << 10) | if p[3] >= 128 { 0x8000 } else { 0 }
            }));
        }
        Some(out)
    }

    /// The mods' textures a textured primitive with texel rectangle `uv`
    /// draws instead of its sheets, as its vertices' `repl` (0: none).
    fn replacement(&mut self, vram: &Vram, st: &DrawState, sh: &Shading, uv: (i32, i32, i32, i32)) -> u32 {
        if !sh.textured || !(crate::mods::wants_textures() || crate::mods::capfont::active()) {
            return 0;
        }
        let Some(f) = self.tracker.find(vram, st, sh, uv) else { return 0 };
        let catalog = crate::mods::sheets::catalog();
        let per = [4, 2, 1][f.depth as usize];
        let mut page = [0u32; 8];
        for part in &f.parts {
            if part.sheet & textures::HOSTED != 0 {
                if let Some(i) = self.hosted(vram, part.sheet & !textures::HOSTED, f.depth) {
                    page[part.column] = i;
                    page[4 + part.column] = part.top as u32;
                }
                continue;
            }
            let Some(sheet) = catalog.and_then(|c| c.sheets.get(part.sheet as usize)) else { continue };
            let (width, height) = (64 * per, sheet.rows.len() as i32);
            // An indexed replacement of the sheet (whatever its palette),
            // else this palette's picture of it, else the one for any.
            let mut texture = self.indexed(vram, sheet.hash, f.depth, (width as u32, height as u32));
            for key in [(sheet.hash, f.palette), (sheet.hash, 0)] {
                if texture.is_some() {
                    break;
                }
                let Some(picture) = crate::mods::texture(key) else { continue };
                // Exported at the wrong texel depth, a sheet has half/twice
                // the correct width. Do not stretch it onto this primitive.
                if !crate::mods::texture_dimensions_match(picture.width, picture.height, width, height) {
                    static WARNED: std::sync::Mutex<Option<std::collections::HashSet<crate::mods::TextureKey>>> =
                        std::sync::Mutex::new(None);
                    if WARNED.lock().unwrap().get_or_insert_with(Default::default).insert(key) {
                        eprintln!(
                            "pe2: mods: ignored texture {}: {}x{} does not match {}x{} sheet ({}-bit); check extraction depth",
                            crate::mods::texture_name(key),
                            picture.width,
                            picture.height,
                            width,
                            height,
                            [4, 8, 15][f.depth as usize]
                        );
                    }
                    continue;
                }
                if let Some(i) = self.atlas.index(key) {
                    texture = Some(i);
                    break;
                }
                texture = match self.atlas.add(key, &picture, width, height) {
                    Some(i) => Some(i),
                    None => {
                        // Full: what was recorded runs with the textures it
                        // uses, then the atlas starts over.
                        self.flush(vram);
                        self.atlas.clear();
                        self.atlas.add(key, &picture, width, height)
                    }
                };
                break;
            }
            if let Some(i) = texture {
                page[part.column] = i;
                page[4 + part.column] = part.top as u32;
            }
        }
        if page[..4].iter().all(|&i| i == 0) {
            return 0;
        }
        self.atlas.page(page).unwrap_or(0)
    }

    /// The atlas entry (index + 1) of the indexed replacement of `sheet`, if
    /// a mod has one that fits how the game draws it: at `depth` (0: 4
    /// bits, 1: 8), `native` texels.
    fn indexed(&mut self, vram: &Vram, sheet: u64, depth: u32, native: (u32, u32)) -> Option<u32> {
        if depth > 1 {
            return None;
        }
        let replacement = crate::mods::indexed_texture(sheet, depth)?;
        if replacement.depth != depth || replacement.native != native {
            static WARNED: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
            if WARNED.lock().unwrap().get_or_insert_with(Default::default).insert(sheet) {
                eprintln!(
                    "pe2: mods: ignored indexed texture {sheet:016x}: {}-bit {}x{}, but the game draws it {}-bit {}x{}",
                    [4, 8][replacement.depth as usize],
                    replacement.native.0,
                    replacement.native.1,
                    [4, 8][depth as usize],
                    native.0,
                    native.1
                );
            }
            return None;
        }
        self.atlas_indexed(vram, &replacement)
    }

    /// The atlas entry (index + 1) of hosted texture `id` (textures.rs
    /// `host`), drawn at `depth` (only 4 bits are hosted).
    fn hosted(&mut self, vram: &Vram, id: u32, depth: u32) -> Option<u32> {
        let replacement = textures::hosted(id)?;
        if depth != replacement.depth {
            return None;
        }
        self.atlas_indexed(vram, &replacement)
    }

    /// The atlas entry (index + 1) of `replacement`, added if not there yet.
    fn atlas_indexed(&mut self, vram: &Vram, replacement: &crate::mods::indexed::Indexed) -> Option<u32> {
        // Its own key: one replacement serves every palette.
        let key = (replacement.sheet, INDEXED_KEY - u64::from(replacement.depth));
        if let Some(i) = self.atlas.index(key) {
            return Some(i);
        }
        match self.atlas.add_indexed(key, replacement) {
            Some(i) => Some(i),
            None => {
                // Full: what was recorded runs with the textures it uses,
                // then the atlas starts over.
                self.flush(vram);
                self.atlas.clear();
                self.atlas.add_indexed(key, replacement)
            }
        }
    }

    /// A textured primitive samples `page` and `clut`: they must be
    /// current. True when its 15-bit texels are the card's own pixels.
    fn sample(&mut self, vram: &mut Vram, st: &DrawState, sh: &Shading) -> bool {
        let page_w = match st.tex_depth {
            0 => 64,
            1 => 128,
            _ => 256,
        };
        let page = Area::new(st.tex_x, st.tex_y, page_w, 256);
        let clut_w = if st.tex_depth == 0 { 16 } else { 256 };
        let clut = Area::new(sh.clut_x, sh.clut_y, clut_w, 1);
        if st.tex_depth < 2 {
            self.sync(vram, st.tex_x, st.tex_y, page_w, 256);
            self.sync(vram, sh.clut_x, sh.clut_y, clut_w, 1);
        }
        if self.dirty.hits(page) || (st.tex_depth < 2 && self.dirty.hits(clut)) {
            self.flush(vram);
        }
        self.sampled.add(page);
        if st.tex_depth < 2 {
            self.sampled.add(clut);
            return false;
        }
        if !self.hires.hits(page) {
            return false;
        }
        // The card's pixels as they are now, for this primitive.
        if !self.snapped.covers(page) {
            let s = self.scale;
            let (x0, y0) = (page.x0 * s, page.y0 * s);
            let (x1, y1) = (((page.x1 + 1) * s).min(self.width as i32), ((page.y1 + 1) * s).min(self.height as i32));
            self.ops.push(Op::ToScratch { x: x0 as u32, y: y0 as u32, w: (x1 - x0) as u32, h: (y1 - y0) as u32 });
            self.snapped.add(page);
        }
        true
    }

    // ---- recording ----------------------------------------------------------

    /// Recorded work beyond this runs at once (the card's buffers have
    /// size limits, and nothing is gained by waiting).
    const PENDING_LIMIT: usize = 8 << 20;

    fn too_much(&self) -> bool {
        self.uploads.len() * 2 + self.vertices.len() * std::mem::size_of::<GpuVertex>() > Self::PENDING_LIMIT
    }

    /// Records vertices drawn with one blend mode and scissor; `keys`, when
    /// not empty, has each vertex's key.
    fn push_draw(&mut self, blend: Blend, scissor: [u32; 4], verts: &[GpuVertex], keys: &[u32]) {
        let first = self.vertices.len() as u32;
        self.vertices.extend_from_slice(verts);
        if keys.len() == verts.len() {
            self.keys.extend_from_slice(keys);
        } else {
            self.keys.resize(self.vertices.len(), 0);
        }
        if let Some(Op::Draw { blend: b, scissor: s, count, .. }) = self.ops.last_mut()
            && *b == blend
            && *s == scissor
        {
            *count += verts.len() as u32;
            return;
        }
        self.ops.push(Op::Draw { blend, scissor, first, count: verts.len() as u32 });
    }

    fn full(&self) -> [u32; 4] {
        [0, 0, self.width, self.height]
    }

    /// The drawing area as a scissor rectangle.
    fn scissor(&self, st: &DrawState) -> [u32; 4] {
        let s = self.scale;
        let x0 = (st.area_x1 * s).clamp(0, self.width as i32);
        let y0 = (st.area_y1 * s).clamp(0, self.height as i32);
        let x1 = ((st.area_x2 + 1) * s).clamp(x0, self.width as i32);
        let y1 = ((st.area_y2 + 1) * s).clamp(y0, self.height as i32);
        [x0 as u32, y0 as u32, (x1 - x0) as u32, (y1 - y0) as u32]
    }

    /// A rectangle in this layer's pixels as two triangles; `uv` at its
    /// top-left and bottom-right corners.
    fn quad(x0: f32, y0: f32, x1: f32, y1: f32, u0: [f32; 2], u1: [f32; 2], template: GpuVertex) -> [GpuVertex; 6] {
        let v = |x, y, u, v| GpuVertex { pos: [x, y], uv: [u, v], ..template };
        let (a, b, c, d) =
            (v(x0, y0, u0[0], u0[1]), v(x1, y0, u1[0], u0[1]), v(x0, y1, u0[0], u1[1]), v(x1, y1, u1[0], u1[1]));
        [a, b, c, b, d, c]
    }

    /// The vertex fields a primitive's shading selects.
    fn template(&self, st: &DrawState, sh: &Shading, pass: u32) -> GpuVertex {
        let mut mode = 0;
        if sh.textured {
            mode = (st.tex_x as u32 & 1023)
                | ((st.tex_y as u32 & 511) << 10)
                | ((st.tex_depth & 3) << 20)
                | (1 << 22)
                | (u32::from(sh.raw_texture) << 23)
                | (pass << 24);
        }
        if st.set_mask {
            mode |= SET_MASK;
        }
        GpuVertex {
            mode,
            clut: (sh.clut_x as u32 & 1023) | ((sh.clut_y as u32 & 511) << 10),
            window: (st.win_mask_x as u32 & 31)
                | ((st.win_mask_y as u32 & 31) << 5)
                | ((st.win_off_x as u32 & 31) << 10)
                | ((st.win_off_y as u32 & 31) << 15),
            ..Default::default()
        }
    }

    fn semi_blend(st: &DrawState) -> Blend {
        match st.semi_mode {
            0 => Blend::Average,
            1 => Blend::Add,
            2 => Blend::Subtract,
            _ => Blend::AddQuarter,
        }
    }

    /// Records a shaded primitive: textured semi-transparent ones in two
    /// passes, since only texels with bit 15 set are blended.
    fn shaded(
        &mut self,
        vram: &mut Vram,
        st: &DrawState,
        sh: &Shading,
        keys: &[u32],
        make: impl Fn(GpuVertex) -> Vec<GpuVertex>,
    ) {
        if self.too_much() {
            self.flush(vram);
        }
        let from_card = sh.textured && self.sample(vram, st, sh);
        let template = |pass| {
            let t = self.template(st, sh, pass);
            GpuVertex { mode: t.mode | if from_card { FROM_CARD } else { 0 }, ..t }
        };
        let (t0, t1, t2) = (template(0), template(1), template(2));
        let scissor = self.scissor(st);
        if !sh.semi_transparent {
            let verts = make(t0);
            self.push_draw(Blend::None, scissor, &verts, keys);
        } else if !sh.textured {
            let verts = make(t0);
            self.push_draw(Self::semi_blend(st), scissor, &verts, keys);
        } else {
            let opaque = make(t1);
            self.push_draw(Blend::None, scissor, &opaque, keys);
            let blended = make(t2);
            self.push_draw(Self::semi_blend(st), scissor, &blended, keys);
        }
    }

    pub fn triangle(&mut self, vram: &mut Vram, st: &DrawState, sh: &Shading, v: [NativeVertex; 3]) {
        // The GPU skips polygons larger than 1023x511.
        let (min_x, max_x) = (v[0].x.min(v[1].x).min(v[2].x), v[0].x.max(v[1].x).max(v[2].x));
        let (min_y, max_y) = (v[0].y.min(v[1].y).min(v[2].y), v[0].y.max(v[1].y).max(v[2].y));
        if max_x - min_x > 1023 || max_y - min_y > 511 {
            return;
        }
        let s = self.scale as f32;
        let gouraud = sh.gouraud;
        // Texels are kept inside the polygon's own rectangle (see
        // Shading::uv_limits), and filtered when asked.
        let (filter, limits) = match sh.uv_limits {
            Some((u0, v0, u1, v1)) if sh.textured => {
                let byte = |n: i32| n.clamp(0, 255) as u32;
                let flags = UV_CLAMP | if bilinear() { FILTER_BILINEAR } else { 0 };
                (flags, byte(u0) | (byte(v0) << 8) | (byte(u1) << 16) | (byte(v1) << 24))
            }
            _ => (0, 0),
        };
        let keys = v.map(|q| q.key);
        let repl = match sh.uv_limits {
            Some(uv) => self.replacement(vram, st, sh, uv),
            None => 0,
        };
        self.shaded(vram, st, sh, &keys, |t| {
            v.iter()
                .map(|q| {
                    let c = if gouraud { q } else { &v[0] };
                    let (x, y) = q.exact.unwrap_or((q.x as f32, q.y as f32));
                    GpuVertex {
                        pos: [x * s, y * s],
                        color: rgb(c.r, c.g, c.b),
                        uv: [q.u as f32, q.v as f32],
                        w: if v.iter().all(|p| p.w > 0.0) { q.w } else { 0.0 },
                        mode: t.mode | filter,
                        limits,
                        repl,
                        ..t
                    }
                })
                .collect()
        });
    }

    /// A sprite: each texel covers `scale` x `scale` pixels.
    #[allow(clippy::too_many_arguments)]
    pub fn rectangle(
        &mut self,
        vram: &mut Vram,
        st: &DrawState,
        sh: &Shading,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: (i32, i32, i32),
        u: i32,
        v: i32,
    ) {
        if w <= 0 || h <= 0 {
            return;
        }
        let s = self.scale as f32;
        // Texel coordinates at the corners: with a flip, counting down from
        // just below the next texel keeps whole texels per native pixel.
        const EDGE: f32 = 1.0 / 4096.0;
        let (u0, u1) = if st.rect_flip_x {
            (u as f32 + 1.0 - EDGE, (u - w) as f32 + 1.0 - EDGE)
        } else {
            (u as f32, (u + w) as f32)
        };
        let (v0, v1) = if st.rect_flip_y {
            (v as f32 + 1.0 - EDGE, (v - h) as f32 + 1.0 - EDGE)
        } else {
            (v as f32, (v + h) as f32)
        };
        let (x0, y0, x1, y1) = (x as f32 * s, y as f32 * s, (x + w) as f32 * s, (y + h) as f32 * s);
        // The quad's far edges sit on pixel boundaries: pull them in by a
        // half pixel so the last row and column are covered exactly once.
        let c = rgb(color.0, color.1, color.2);
        let uv = (
            if st.rect_flip_x { u - w + 1 } else { u },
            if st.rect_flip_y { v - h + 1 } else { v },
            if st.rect_flip_x { u } else { u + w - 1 },
            if st.rect_flip_y { v } else { v + h - 1 },
        );
        let repl = self.replacement(vram, st, sh, uv);
        self.shaded(vram, st, sh, &[], |t| {
            Self::quad(x0, y0, x1 - 0.0, y1 - 0.0, [u0, v0], [u1, v1], GpuVertex { color: c, repl, ..t })
                .iter()
                .map(|q| GpuVertex { pos: [q.pos[0] - 0.5, q.pos[1] - 0.5], ..*q })
                .collect()
        });
    }

    /// A line: the console's pixels, each a `scale` x `scale` block.
    pub fn line(&mut self, vram: &mut Vram, st: &DrawState, sh: &Shading, a: NativeVertex, b: NativeVertex) {
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        if dx.abs() > 1023 || dy.abs() > 511 {
            return;
        }
        let steps = dx.abs().max(dy.abs());
        let s = self.scale as f32;
        let gouraud = sh.gouraud;
        self.shaded(vram, st, sh, &[], |t| {
            let mut out = Vec::with_capacity(6 * (steps as usize + 1));
            for i in 0..=steps {
                let lerp = |p: i32, q: i32| if steps == 0 { p } else { p + (q - p) * i / steps };
                let (x, y) = (lerp(a.x, b.x) as f32 * s, lerp(a.y, b.y) as f32 * s);
                let c = if gouraud { rgb(lerp(a.r, b.r), lerp(a.g, b.g), lerp(a.b, b.b)) } else { rgb(a.r, a.g, a.b) };
                for q in Self::quad(
                    x - 0.5,
                    y - 0.5,
                    x + s - 0.5,
                    y + s - 0.5,
                    [0.0; 2],
                    [0.0; 2],
                    GpuVertex { color: c, ..t },
                ) {
                    out.push(q);
                }
            }
            out
        });
    }

    /// GP0(02h) fill, with the native command's coordinate rounding.
    pub fn fill_command(&mut self, x: i32, y: i32, w: i32, h: i32, rgb24: u32) {
        let w = ((w & 0x3FF) + 0xF) & !0xF;
        self.fill(x & 0x3F0, y & 0x1FF, w, h & 0x1FF, rgb24);
    }

    /// A rectangle of native pixels set to one colour (ClearImage).
    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, rgb24: u32) {
        if w <= 0 || h <= 0 {
            return;
        }
        let s = self.scale as f32;
        let c = rgb((rgb24 & 0xFF) as i32, ((rgb24 >> 8) & 0xFF) as i32, ((rgb24 >> 16) & 0xFF) as i32);
        let verts = Self::quad(
            x as f32 * s - 0.5,
            y as f32 * s - 0.5,
            (x + w) as f32 * s - 0.5,
            (y + h) as f32 * s - 0.5,
            [0.0; 2],
            [0.0; 2],
            GpuVertex { color: c, ..Default::default() },
        );
        let full = self.full();
        self.push_draw(Blend::None, full, &verts, &[]);
    }

    /// An image transfer of `w` x `h` native pixels (the native VRAM
    /// already has them: `vram` is only for flushing early).
    pub fn upload_rect(&mut self, vram: &Vram, x: i32, y: i32, w: i32, h: i32, pixels: &[u16]) {
        if w <= 0 || h <= 0 {
            return;
        }
        if crate::mods::wants_textures() {
            self.tracker.upload(x, y, w, h, pixels);
        }
        if self.too_much() {
            self.flush(vram);
        }
        let n = (w * h) as usize;
        let first = self.uploads.len() as u32;
        self.uploads.extend_from_slice(&pixels[..n.min(pixels.len())]);
        self.uploads.resize(first as usize + n, 0);
        let s = self.scale as f32;
        let smooth = smooth_uploads() && w >= SMOOTH_UPLOAD_MIN && h >= SMOOTH_UPLOAD_MIN;
        let template = GpuVertex {
            mode: KIND_UPLOAD | if smooth { UPLOAD_SMOOTH } else { 0 },
            clut: first,
            window: w as u32,
            limits: h as u32,
            ..Default::default()
        };
        let verts = Self::quad(
            x as f32 * s - 0.5,
            y as f32 * s - 0.5,
            (x + w) as f32 * s - 0.5,
            (y + h) as f32 * s - 0.5,
            [0.0, 0.0],
            [w as f32, h as f32],
            template,
        );
        let full = self.full();
        self.push_draw(Blend::None, full, &verts, &[]);
    }

    /// A screen image (`w` x `h` native pixels at x, y) shown as a mod's
    /// `picture` instead, enlarged or reduced to this layer's scale (the
    /// native VRAM has the game's own: `vram` is only for flushing early).
    #[allow(clippy::too_many_arguments)]
    pub fn upload_picture(
        &mut self,
        vram: &Vram,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        hash: u64,
        picture: &crate::mods::Picture,
    ) {
        let Some((px, py, pw, ph)) = self.screen_rect(x, y, w, h) else { return };
        if self.too_much() {
            self.flush(vram);
        }
        self.finer(x, y, w, h);
        if !self.pictures.contains_key(&hash) {
            let texture = self.picture_texture(pw, ph);
            self.gfx.queue.write_texture(
                texture.as_image_copy(),
                &crate::mods::resize(picture, pw, ph),
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(pw * 4), rows_per_image: Some(ph) },
                wgpu::Extent3d { width: pw, height: ph, depth_or_array_layers: 1 },
            );
            self.keep_picture(hash, texture);
        }
        self.show_picture(hash, px, py);
    }

    /// The game read the screen image at `x, y, w, h` back (StoreImage), to
    /// load it again later (the frozen picture behind its menus): this
    /// layer keeps it as it has it, by the hash of what was read.
    pub fn keep_screen(&mut self, vram: &Vram, x: i32, y: i32, w: i32, h: i32, hash: u64) {
        let Some((px, py, pw, ph)) = self.screen_rect(x, y, w, h) else { return };
        // What was recorded first, so the layer holds what the game read.
        self.flush(vram);
        let texture = self.picture_texture(pw, ph);
        let mut encoder =
            self.gfx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("hw keep") });
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.target,
                mip_level: 0,
                origin: wgpu::Origin3d { x: px, y: py, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            texture.as_image_copy(),
            wgpu::Extent3d { width: pw, height: ph, depth_or_array_layers: 1 },
        );
        self.gfx.queue.submit([encoder.finish()]);
        self.keep_picture(kept_key(hash), texture);
    }

    /// A screen image the game loads (`hash` of its pixels) that this layer
    /// kept when the game read it back: drawn as kept. False when not kept.
    #[allow(clippy::too_many_arguments)]
    pub fn load_kept(&mut self, vram: &Vram, x: i32, y: i32, w: i32, h: i32, pixels: &[u16], hash: u64) -> bool {
        let Some((px, py, pw, ph)) = self.screen_rect(x, y, w, h) else { return false };
        match self.pictures.get(&kept_key(hash)) {
            Some((t, _)) if t.width() == pw && t.height() == ph => {}
            _ => return false,
        }
        if crate::mods::wants_textures() {
            self.tracker.upload(x, y, w, h, pixels);
        }
        if self.too_much() {
            self.flush(vram);
        }
        self.finer(x, y, w, h);
        self.show_picture(kept_key(hash), px, py);
        true
    }

    /// A screen image the game loads that is its grey negative (the screen
    /// negative effect) of one this layer kept when the game read it back
    /// (`hash`, of what was read): drawn so from the kept one. False when
    /// not kept.
    #[allow(clippy::too_many_arguments)]
    pub fn load_kept_negative(
        &mut self,
        vram: &Vram,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        pixels: &[u16],
        hash: u64,
    ) -> bool {
        let Some((px, py, pw, ph)) = self.screen_rect(x, y, w, h) else { return false };
        let key = kept_key(hash);
        match self.pictures.get(&key) {
            Some((t, _)) if t.width() == pw && t.height() == ph => {}
            _ => return false,
        }
        if crate::mods::wants_textures() {
            self.tracker.upload(x, y, w, h, pixels);
        }
        if self.too_much() {
            self.flush(vram);
        }
        self.finer(x, y, w, h);
        // The scratch texture gets the picture, not the card's pixels.
        self.snapped.remove_hits(Area::new(x, y, w, h));
        self.pictures_used += 1;
        if let Some(entry) = self.pictures.get_mut(&key) {
            entry.1 = self.pictures_used;
        }
        self.ops.push(Op::PictureToScratch { hash: key, x: px, y: py });
        let (x0, y0) = (px as f32 - 0.5, py as f32 - 0.5);
        let verts = Self::quad(
            x0,
            y0,
            x0 + pw as f32,
            y0 + ph as f32,
            [px as f32, py as f32],
            [(px + pw) as f32, (py + ph) as f32],
            GpuVertex { mode: KIND_COPY | COPY_NEGATIVE, ..Default::default() },
        );
        let full = self.full();
        let first = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&verts);
        self.keys.resize(self.vertices.len(), 0);
        // Never merged with a previous draw: the scratch copy comes between.
        self.ops.push(Op::Draw { blend: Blend::None, scissor: full, first, count: 6 });
        true
    }

    /// `x, y, w, h` of the VRAM in this layer's texels, if inside it.
    fn screen_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Option<(u32, u32, u32, u32)> {
        let s = self.scale;
        let (px, py) = ((x * s).max(0) as u32, (y * s).max(0) as u32);
        let (pw, ph) = ((w * s) as u32, (h * s) as u32);
        (w > 0 && h > 0 && px + pw <= self.width && py + ph <= self.height).then_some((px, py, pw, ph))
    }

    fn picture_texture(&self, w: u32, h: u32) -> wgpu::Texture {
        self.gfx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hw picture"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        })
    }

    /// Keeps `texture` as the picture of `hash` (the least recently shown
    /// go beyond PICTURES_KEPT).
    fn keep_picture(&mut self, hash: u64, texture: wgpu::Texture) {
        const PICTURES_KEPT: usize = 24;
        if !self.pictures.contains_key(&hash)
            && self.pictures.len() >= PICTURES_KEPT
            && let Some(oldest) = self.pictures.iter().min_by_key(|(_, e)| e.1).map(|(k, _)| *k)
        {
            self.pictures.remove(&oldest);
        }
        self.pictures_used += 1;
        self.pictures.insert(hash, (texture, self.pictures_used));
    }

    fn show_picture(&mut self, hash: u64, px: u32, py: u32) {
        self.pictures_used += 1;
        if let Some(entry) = self.pictures.get_mut(&hash) {
            entry.1 = self.pictures_used;
        }
        self.ops.push(Op::Picture { hash, x: px, y: py });
    }

    /// A VRAM-to-VRAM copy (no wrapping at the VRAM's edges).
    pub fn copy(&mut self, sx: i32, sy: i32, dx: i32, dy: i32, w: i32, h: i32) {
        let s = self.scale;
        let clip = |p: i32, len: i32, max: i32| (p.clamp(0, max), (p + len).clamp(0, max));
        let (sx0, sx1) = clip(sx * s, w * s, self.width as i32);
        let (sy0, sy1) = clip(sy * s, h * s, self.height as i32);
        if sx1 <= sx0 || sy1 <= sy0 {
            return;
        }
        self.ops.push(Op::ToScratch { x: sx0 as u32, y: sy0 as u32, w: (sx1 - sx0) as u32, h: (sy1 - sy0) as u32 });
        let (cw, ch) = ((sx1 - sx0) as f32, (sy1 - sy0) as f32);
        let (x0, y0) = ((dx * s) as f32 - 0.5, (dy * s) as f32 - 0.5);
        let verts = Self::quad(
            x0,
            y0,
            x0 + cw,
            y0 + ch,
            [sx0 as f32, sy0 as f32],
            [sx1 as f32, sy1 as f32],
            GpuVertex { mode: KIND_COPY, ..Default::default() },
        );
        let full = self.full();
        let first = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&verts);
        self.keys.resize(self.vertices.len(), 0);
        // Never merged with a previous draw: the scratch copy comes between.
        self.ops.push(Op::Draw { blend: Blend::None, scissor: full, first, count: 6 });
    }

    // ---- running --------------------------------------------------------------

    /// Runs everything recorded on the card.
    pub fn flush(&mut self, vram: &Vram) {
        let gfx = self.gfx;
        // The part of the native VRAM that changed since the last copy.
        let d = self.dirty.bounds();
        let (x0, y0) = (d.x0.max(0), d.y0.max(0));
        let (x1, y1) = (d.x1.min(VRAM_WIDTH as i32 - 1), d.y1.min(VRAM_HEIGHT as i32 - 1));
        let mut sent = None;
        if x0 <= x1 && y0 <= y1 {
            let (w, h) = ((x1 - x0 + 1) as usize, (y1 - y0 + 1) as usize);
            let mut rows = Vec::with_capacity(w * h);
            for y in y0 as usize..=y1 as usize {
                rows.extend_from_slice(&vram.pixels[y * VRAM_WIDTH + x0 as usize..][..w]);
            }
            let update = VramUpdate { x: x0 as u32, y: y0 as u32, w: w as u32, h: h as u32, rows };
            write_vram(gfx, &self.vram, &update);
            VRAM_SENT.fetch_add((w * h) as u64, Ordering::Relaxed);
            sent = Some(update);
        }
        self.dirty.clear();
        self.sampled.clear();
        self.snapped.clear();
        if self.ops.is_empty() {
            // Kept even without work: later flushes of the frame sample it.
            if let (Some(frame), Some(update)) = (self.frame.as_mut(), sent) {
                frame.push(Chunk {
                    vram: Some(update),
                    ops: Vec::new(),
                    vertices: Vec::new(),
                    keys: Vec::new(),
                    uploads: Vec::new(),
                });
            }
            return;
        }
        FLUSHES.fetch_add(1, Ordering::Relaxed);
        let target = self.target.clone();
        let (ops, vertices, mut uploads) =
            (std::mem::take(&mut self.ops), std::mem::take(&mut self.vertices), std::mem::take(&mut self.uploads));
        let keys = std::mem::take(&mut self.keys);
        // Whole words for the shader's storage buffer.
        if uploads.len() % 2 == 1 {
            uploads.push(0);
        }
        self.run(&ops, &vertices, &uploads, &target, false);
        match self.frame.as_mut() {
            Some(frame) => frame.push(Chunk { vram: sent, ops, vertices, keys, uploads }),
            None => {
                // Keep the allocations.
                (self.ops, self.vertices, self.uploads, self.keys) = (ops, vertices, uploads, keys);
                self.ops.clear();
                self.vertices.clear();
                self.uploads.clear();
                self.keys.clear();
            }
        }
    }

    /// Runs recorded operations on the card, drawing into `into`, with
    /// textures from the CPU's VRAM copy (`replay`: the one frames are drawn
    /// again with). Copies read the layer itself.
    fn run(&mut self, ops: &[Op], vertices: &[GpuVertex], uploads: &[u16], into: &wgpu::Texture, replay: bool) {
        let gfx = self.gfx;
        let device = &gfx.device;
        let vbytes = bytes(vertices);
        if self.vertex_buffer.as_ref().is_none_or(|b| b.size() < vbytes.len() as u64) {
            self.vertex_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("hw vertices"),
                size: (vbytes.len() as u64).next_power_of_two().max(1 << 16),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));
        }
        let vertex_buffer = self.vertex_buffer.as_ref().unwrap();
        gfx.queue.write_buffer(vertex_buffer, 0, vbytes);
        if !uploads.is_empty() {
            let ubytes = bytes(uploads);
            if self.upload_buffer.size() < ubytes.len() as u64 {
                self.upload_buffer = Self::make_upload_buffer(device, (ubytes.len() as u64).next_power_of_two());
                self.bind = Self::make_bind(
                    device, &self.layout, &self.uniforms, &self.vram, &self.upload_buffer, &self.scratch,
                );
                if let Some((texture, bind)) = self.vram_replay.as_mut() {
                    *bind = Self::make_bind(
                        device, &self.layout, &self.uniforms, texture, &self.upload_buffer, &self.scratch,
                    );
                }
            }
            gfx.queue.write_buffer(&self.upload_buffer, 0, ubytes);
        }

        let vertex_buffer = self.vertex_buffer.as_ref().unwrap();
        let view = into.create_view(&Default::default());
        let mut encoder = device.create_command_encoder(&Default::default());
        let mut i = 0;
        while i < ops.len() {
            if let Op::ToScratch { x, y, w, h } = ops[i] {
                let at = |texture| wgpu::TexelCopyTextureInfo {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x, y, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                };
                encoder.copy_texture_to_texture(
                    at(&self.target),
                    at(&self.scratch),
                    wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                );
                i += 1;
                continue;
            }
            if let Op::PictureToScratch { hash, x, y } = ops[i] {
                if let Some((picture, _)) = self.pictures.get(&hash) {
                    encoder.copy_texture_to_texture(
                        picture.as_image_copy(),
                        wgpu::TexelCopyTextureInfo {
                            texture: &self.scratch,
                            mip_level: 0,
                            origin: wgpu::Origin3d { x, y, z: 0 },
                            aspect: wgpu::TextureAspect::All,
                        },
                        picture.size(),
                    );
                }
                i += 1;
                continue;
            }
            if let Op::Picture { hash, x, y } = ops[i] {
                if let Some((picture, _)) = self.pictures.get(&hash) {
                    encoder.copy_texture_to_texture(
                        picture.as_image_copy(),
                        wgpu::TexelCopyTextureInfo {
                            texture: into,
                            mip_level: 0,
                            origin: wgpu::Origin3d { x, y, z: 0 },
                            aspect: wgpu::TextureAspect::All,
                        },
                        picture.size(),
                    );
                }
                i += 1;
                continue;
            }
            // A render pass over the run of draws up to the next copy.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hw"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                })],
                ..Default::default()
            });
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            let bind = match (&self.vram_replay, replay) {
                (Some((_, bind)), true) => bind,
                _ => &self.bind,
            };
            pass.set_bind_group(0, bind, &[]);
            pass.set_bind_group(1, &self.atlas.bind, &[]);
            PASSES.fetch_add(1, Ordering::Relaxed);
            // State is set only when it changes (draws mostly share it).
            let (mut blend_now, mut scissor_now) = (None, None);
            while let Some(Op::Draw { blend, scissor, first, count }) = ops.get(i) {
                if scissor[2] > 0 && scissor[3] > 0 {
                    DRAWS.fetch_add(1, Ordering::Relaxed);
                    VERTICES.fetch_add(u64::from(*count), Ordering::Relaxed);
                    if blend_now != Some(*blend) {
                        pass.set_pipeline(&self.pipelines[blend.index()]);
                        let k = blend.constant();
                        pass.set_blend_constant(wgpu::Color { r: k, g: k, b: k, a: k });
                        blend_now = Some(*blend);
                    }
                    if scissor_now != Some(*scissor) {
                        pass.set_scissor_rect(scissor[0], scissor[1], scissor[2], scissor[3]);
                        scissor_now = Some(*scissor);
                    }
                    pass.draw(*first..*first + *count, 0..1);
                }
                i += 1;
            }
        }
        gfx.queue.submit([encoder.finish()]);
    }

    // ---- frames in between ------------------------------------------------

    /// Keeps what each frame runs, from now on (see `take_frame`).
    pub fn keep_frames(&mut self) {
        self.frame.get_or_insert_with(Vec::new);
    }

    /// What was run since the last call (flush first).
    pub fn take_frame(&mut self) -> Vec<Chunk> {
        self.frame.as_mut().map(std::mem::take).unwrap_or_default()
    }

    /// A texture the size of the layer.
    fn layer_texture(&self, label: &str, usage: wgpu::TextureUsages) -> wgpu::Texture {
        self.gfx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: self.target.size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage,
            view_formats: &[],
        })
    }

    /// Copies a region of one layer-sized texture to the same place in
    /// another (clipped to the layer).
    fn copy_region(&self, from: &wgpu::Texture, to: &wgpu::Texture, (x, y, w, h): interp::Region) {
        let at = |texture| wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x, y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        };
        let (w, h) = (w.min(self.width.saturating_sub(x)), h.min(self.height.saturating_sub(y)));
        if w == 0 || h == 0 {
            return;
        }
        let mut encoder = self.gfx.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_texture(
            at(from),
            at(to),
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.gfx.queue.submit([encoder.finish()]);
    }

    /// Keeps `region` of the layer, where the next frame is drawn, as that
    /// frame starts from it (flush first).
    pub fn keep_before(&mut self, region: interp::Region) {
        let texture = match self.before.take() {
            Some((t, _)) => t,
            None => self.layer_texture("hw before", wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST),
        };
        self.copy_region(&self.target, &texture, region);
        self.before = Some((texture, region));
    }

    /// Keeps the native VRAM copy as the next frame starts from it (flush
    /// first).
    pub fn keep_vram(&mut self) {
        let texture = self.vram_before.get_or_insert_with(|| {
            self.gfx.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("hw vram before"),
                size: self.vram.size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R16Uint,
                usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        });
        let mut encoder = self.gfx.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_texture(self.vram.as_image_copy(), texture.as_image_copy(), self.vram.size());
        self.gfx.queue.submit([encoder.finish()]);
    }

    /// The texture frames are drawn again with, set to the native VRAM as
    /// the last frame left it.
    fn start_vram_replay(&mut self) {
        if self.vram_replay.is_none() {
            let texture = self.gfx.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("hw vram replay"),
                size: self.vram.size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R16Uint,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let bind = Self::make_bind(
                &self.gfx.device, &self.layout, &self.uniforms, &texture, &self.upload_buffer, &self.scratch,
            );
            self.vram_replay = Some((texture, bind));
        }
        let (texture, _) = self.vram_replay.as_ref().unwrap();
        // Without a copy yet (the first frame), the current VRAM.
        let from = self.vram_before.as_ref().unwrap_or(&self.vram);
        let mut encoder = self.gfx.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_texture(from.as_image_copy(), texture.as_image_copy(), self.vram.size());
        self.gfx.queue.submit([encoder.finish()]);
    }

    /// Draws `frame` again into `between`, its keyed vertices `t` of the
    /// way from their positions in `before` (0, relative to `origin`) to
    /// their own (1). A vertex that moved further than `limit` pixels is
    /// left where it is: the camera cut away.
    pub fn draw_between(
        &mut self,
        frame: &[Chunk],
        before: &std::collections::HashMap<u32, [f32; 2]>,
        origin: [f32; 2],
        t: f32,
        limit: f32,
    ) {
        if self.between.is_none() {
            use wgpu::TextureUsages as U;
            self.between = Some(self.layer_texture("hw between", U::RENDER_ATTACHMENT | U::COPY_SRC | U::COPY_DST));
        }
        let into = self.between.clone().unwrap();
        // Start from what the frame started from (effects blend over it),
        // with the textures as they were then.
        if let Some((before, region)) = self.before.as_ref() {
            self.copy_region(before, &into, *region);
        }
        self.start_vram_replay();
        for c in frame {
            if let (Some(update), Some((texture, _))) = (&c.vram, &self.vram_replay) {
                write_vram(self.gfx, texture, update);
            }
            if c.ops.is_empty() {
                continue;
            }
            let mut vertices = c.vertices.clone();
            for (v, k) in vertices.iter_mut().zip(&c.keys) {
                let Some(p) = before.get(k).filter(|_| *k != 0) else { continue };
                let p = [p[0] + origin[0], p[1] + origin[1]];
                let (dx, dy) = (v.pos[0] - p[0], v.pos[1] - p[1]);
                if dx.abs() <= limit && dy.abs() <= limit {
                    v.pos = [p[0] + dx * t, p[1] + dy * t];
                }
            }
            self.run(&c.ops, &vertices, &c.uploads, &into, true);
        }
    }
}

/// Sends native VRAM halfwords to a VRAM-sized texture.
fn write_vram(gfx: &Gfx, texture: &wgpu::Texture, u: &VramUpdate) {
    gfx.queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x: u.x, y: u.y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        },
        bytes(&u.rows),
        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(u.w * 2), rows_per_image: Some(u.h) },
        wgpu::Extent3d { width: u.w, height: u.h, depth_or_array_layers: 1 },
    );
}

fn rgb(r: i32, g: i32, b: i32) -> u32 {
    (r.clamp(0, 255) as u32) | ((g.clamp(0, 255) as u32) << 8) | ((b.clamp(0, 255) as u32) << 16)
}

fn bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: plain-old-data slices (u16, u32, f32, GpuVertex: repr(C),
    // no padding) viewed as bytes.
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}
