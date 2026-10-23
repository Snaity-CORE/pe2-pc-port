//! Frames in between (60 fps from the game's 30). The game draws each new
//! frame into the framebuffer not on screen and shows it two vertical
//! blanks later; at the blank in between, the window shows that frame drawn
//! again with its models halfway from their positions in the frame on
//! screen. The game is unchanged and runs at its own rate, and nothing is
//! shown later than on the console.
//!
//! Model vertices are matched between frames by their keys (Vertex::key:
//! the model part and the vertex, from the game's TMD renderer). The
//! pre-rendered backgrounds, the interface and anything drawn by other
//! code stay as the new frame has them. Needs sub-pixel positions (the keys travel with the exact positions).

use std::collections::HashMap;

use super::hw::{self, Hw};

/// Whether frames in between are drawn: PE2_INTERPOLATION=0/1, else
/// pe2.toml's [video] frame_interpolation, else on.
pub fn enabled() -> bool {
    std::env::var("PE2_INTERPOLATION")
        .ok()
        .map(|v| v.trim() != "0")
        .or(crate::config::get().video.frame_interpolation)
        .unwrap_or(true)
}

/// How far between the two frames the frame in between is: halfway, or
/// PE2_INTERP_T for checks (1 draws the new frame itself again, which must
/// come out the same as the frame).
fn between_t() -> f32 {
    static T: std::sync::OnceLock<f32> = std::sync::OnceLock::new();
    *T.get_or_init(|| std::env::var("PE2_INTERP_T").ok().and_then(|v| v.trim().parse().ok()).unwrap_or(0.5))
}

/// A displayed region of the layer: x, y, width, height in its pixels.
pub type Region = (u32, u32, u32, u32);

#[derive(Default)]
pub struct Interp {
    /// Vertical blanks so far, and the one the last new frame came at.
    blank: u64,
    last_frame: u64,
    /// The last frame's keyed vertex positions, relative to its
    /// framebuffer (the game alternates between two).
    before: HashMap<u32, [f32; 2]>,
    /// Where `hw.between` holds the frame for the next blank.
    pending: Option<Region>,
    /// New frames so far (PE2_TRACE=interp).
    traced: u64,
}

impl Interp {
    /// Each vertical blank, before anything else.
    pub fn blank(&mut self) {
        self.blank += 1;
    }

    /// At a blank with no new frame: the region of `hw.between` to show,
    /// once after each new frame.
    pub fn take_pending(&mut self) -> Option<Region> {
        self.pending.take()
    }

    /// A new frame not shown from the layer (a 24-bit movie).
    pub fn new_frame_hidden(&mut self) {
        self.pending = None;
        self.last_frame = self.blank;
        self.before.clear();
    }

    /// A new frame, `frame` (as hw.rs kept it), drawn into `region` of the
    /// layer (None: not to be shown from it) while `shown` is on screen:
    /// draws it again into `hw.between`, halfway from the last frame, for
    /// the next blank. Returns whether it did.
    pub fn new_frame(&mut self, hw: &mut Hw, frame: &[hw::Chunk], region: Option<Region>, shown: Region) -> bool {
        self.pending = None;
        let origin = region.map_or([0.0; 2], |r| [r.0 as f32, r.1 as f32]);
        let now: HashMap<u32, [f32; 2]> =
            hw::positions(frame).into_iter().map(|(k, p)| (k, [p[0] - origin[0], p[1] - origin[1]])).collect();
        // At 60 fps (menus) the next blank brings a frame of its own: no
        // use drawing one in between.
        let steady = self.blank - self.last_frame >= 2;
        self.last_frame = self.blank;
        let before = std::mem::replace(&mut self.before, now);
        let moves: Vec<f32> = self
            .before
            .iter()
            .filter_map(|(k, p)| before.get(k).map(|q| (p[0] - q[0]).abs().max((p[1] - q[1]).abs())))
            .collect();
        let matched = moves.len();
        if crate::runtime::trace("interp") {
            self.traced += 1;
            if self.traced.is_multiple_of(30) {
                eprintln!(
                    "[interp] blank {}: {} keyed vertices, {matched} matched (moved: mean {:.2}, max {:.2} px), steady {steady}, shown {}",
                    self.blank,
                    self.before.len(),
                    moves.iter().sum::<f32>() / matched.max(1) as f32,
                    moves.iter().copied().fold(0.0, f32::max),
                    region.is_some()
                );
            }
        }
        let Some(region) = region else { return false };
        let drawn = steady && matched > 0;
        if drawn {
            // Further than this in one frame (1/30 s) is a cut, not movement.
            let limit = 32.0 * hw.scale as f32;
            hw.draw_between(frame, &before, origin, between_t(), limit);
            self.pending = Some(region);
        }
        // The next frame is drawn where this one is shown.
        hw.keep_before(shown);
        drawn
    }
}
