//! The application icon for the windows: the player's local/icon.png, built
//! in when it is there (see build.rs).

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
