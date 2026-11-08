//! The PE2PC screen shown before the game starts (before the publisher's
//! picture): the launcher's art, the background filling a 4:3 screen with
//! the logo in its middle, faded in and out. The game waits for it. Not with
//! [gameplay] skip_intro, nor in unattended runs (PE2_INPUT, replays), whose
//! timing it would shift.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const FADE: Duration = Duration::from_millis(400);
const HOLD: Duration = Duration::from_millis(1400);
/// The game starts anyway after this long (a window that never draws).
const TIMEOUT: Duration = Duration::from_secs(6);

const WIDTH: u32 = 960;
const HEIGHT: u32 = 720;
/// Where the logo goes, and how wide (the launcher's place, on the screen's
/// wider crop of the background).
const LOGO_AT: (f32, f32) = (40.0, 45.0);
const LOGO_WIDTH: f32 = 430.0;

static DONE: AtomicBool = AtomicBool::new(false);
static STARTED: OnceLock<Instant> = OnceLock::new();

fn wanted() -> bool {
    static WANTED: OnceLock<bool> = OnceLock::new();
    *WANTED.get_or_init(|| {
        !crate::settings::flag(crate::settings::Id::SkipIntro)
            && ["PE2_INPUT", "PE2_REPLAY", "PE2_LIBTEST", "PE2_SELFTEST"].iter().all(|v| std::env::var_os(v).is_none())
    })
}

/// On the game thread, before the game starts: waits for the screen to end.
pub fn wait() {
    if !wanted() {
        return;
    }
    let since = Instant::now();
    while !DONE.load(Ordering::SeqCst) && since.elapsed() < TIMEOUT {
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// The screen's brightness now (0-1), its time counted from the first call;
/// None once it is over (or not wanted).
pub fn level() -> Option<f32> {
    if !wanted() || DONE.load(Ordering::SeqCst) {
        return None;
    }
    let t = STARTED.get_or_init(Instant::now).elapsed();
    let ramp = |d: Duration| d.as_secs_f32() / FADE.as_secs_f32();
    if t < FADE {
        Some(ramp(t))
    } else if t < FADE + HOLD {
        Some(1.0)
    } else if t < FADE * 2 + HOLD {
        Some(1.0 - ramp(t - FADE - HOLD))
    } else {
        DONE.store(true, Ordering::SeqCst);
        None
    }
}

/// The screen at full brightness, `WIDTH` x `HEIGHT` 0x00RRGGBB words.
pub fn picture() -> Option<&'static (Vec<u32>, u32, u32)> {
    static PICTURE: OnceLock<Option<(Vec<u32>, u32, u32)>> = OnceLock::new();
    PICTURE.get_or_init(compose).as_ref()
}

/// RGBA pixels of a WebP picture, and its size.
fn decode(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let mut decoder = image_webp::WebPDecoder::new(std::io::Cursor::new(bytes)).ok()?;
    let (w, h) = decoder.dimensions();
    let mut buf = vec![0u8; decoder.output_buffer_size()?];
    decoder.read_image(&mut buf).ok()?;
    if !decoder.has_alpha() {
        buf = buf.chunks(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect();
    }
    Some((buf, w, h))
}

/// Bilinear sample of RGBA `src` (`w` x `h`) at (x, y) in its pixels.
fn sample(src: &[u8], w: u32, h: u32, x: f32, y: f32) -> [f32; 4] {
    let (x, y) = (x.clamp(0.0, (w - 1) as f32), y.clamp(0.0, (h - 1) as f32));
    let (x0, y0) = (x.floor() as u32, y.floor() as u32);
    let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
    let (fx, fy) = (x - x0 as f32, y - y0 as f32);
    let px = |x: u32, y: u32| {
        let i = ((y * w + x) * 4) as usize;
        [src[i], src[i + 1], src[i + 2], src[i + 3]].map(f32::from)
    };
    let (a, b, c, d) = (px(x0, y0), px(x1, y0), px(x0, y1), px(x1, y1));
    std::array::from_fn(|k| (a[k] * (1.0 - fx) + b[k] * fx) * (1.0 - fy) + (c[k] * (1.0 - fx) + d[k] * fx) * fy)
}

fn compose() -> Option<(Vec<u32>, u32, u32)> {
    let (bg, bw, bh) = decode(crate::launcher::BACKGROUND)?;
    let (logo, lw, lh) = decode(crate::launcher::LOGO)?;
    // The background covers the screen (cropped at the sides), the logo at
    // the top left as the launcher has it, clear of the background's own.
    let scale = (WIDTH as f32 / bw as f32).max(HEIGHT as f32 / bh as f32);
    let (ox, oy) = ((bw as f32 * scale - WIDTH as f32) / 2.0, (bh as f32 * scale - HEIGHT as f32) / 2.0);
    let logo_scale = LOGO_WIDTH / lw as f32;
    let (lx, ly) = LOGO_AT;
    let mut out = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut c = sample(&bg, bw, bh, (x as f32 + ox) / scale, (y as f32 + oy) / scale);
            let (u, v) = ((x as f32 - lx) / logo_scale, (y as f32 - ly) / logo_scale);
            if (0.0..lw as f32).contains(&u) && (0.0..lh as f32).contains(&v) {
                let l = sample(&logo, lw, lh, u, v);
                let a = l[3] / 255.0;
                for k in 0..3 {
                    c[k] = c[k] * (1.0 - a) + l[k] * a;
                }
            }
            let [r, g, b] = [c[0], c[1], c[2]].map(|v| v.round().clamp(0.0, 255.0) as u32);
            out.push((r << 16) | (g << 8) | b);
        }
    }
    Some((out, WIDTH, HEIGHT))
}
