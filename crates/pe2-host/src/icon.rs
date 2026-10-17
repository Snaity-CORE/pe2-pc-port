//! The application icon for the windows: res/icon.png, or the player's own
//! local/icon.png, built in (see build.rs).

/// The icon's pixels (RGBA) and side, if pe2.exe was built with one.
pub fn rgba() -> Option<(Vec<u8>, u32)> {
    #[cfg(pe2_icon)]
    {
        const PIXELS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icon.rgba"));
        let side = (PIXELS.len() as f64 / 4.0).sqrt() as u32;
        Some((PIXELS.to_vec(), side))
    }
    #[cfg(not(pe2_icon))]
    None
}
