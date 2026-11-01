//! The pause menu's host side: Esc and a controller's Guide button ask for
//! it, and the game's main loop opens it when it can (the fork's
//! src/main/pc/menu.c, drawn with the game's own dialogs); its Quit game
//! ends the process here.
//!
//! The game cannot open it while it does not draw its own frames (movies,
//! the intro), as it tells each frame: there Esc asks to quit at once,
//! with the window's prompt (video.rs). Where it is only held up a moment
//! (a scene, a load), a second ask after a while, unanswered, falls back
//! to the prompt too, and an ask left unanswered lapses, so the menu does
//! not open by surprise later.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// When the menu was asked for and not yet opened.
static ASKED: Mutex<Option<Instant>> = Mutex::new(None);

/// Whether the game draws its own frames, where the menu can open.
static AVAILABLE: AtomicBool = AtomicBool::new(false);

/// How long an ask may wait before another one falls back to the prompt,
/// and before it lapses.
const PATIENCE: Duration = Duration::from_secs(1);
const LAPSE: Duration = Duration::from_secs(2);

/// Asks for the menu (or, open, closes it as Cancel does). Returns true when
/// the game cannot open it now: it does not draw its own frames, or an
/// earlier ask has waited unanswered too long.
pub fn ask() -> bool {
    let mut asked = ASKED.lock().unwrap();
    if !AVAILABLE.load(Ordering::SeqCst) {
        *asked = None;
        return true;
    }
    match *asked {
        Some(at) if at.elapsed() >= LAPSE => {
            *asked = Some(Instant::now());
            false
        }
        Some(at) if at.elapsed() >= PATIENCE => {
            *asked = None;
            true
        }
        Some(_) => false,
        None => {
            *asked = Some(Instant::now());
            false
        }
    }
}

/// Whether the game draws its own frames (it tells each frame).
#[unsafe(no_mangle)]
pub extern "C" fn pc_menu_available(available: i32) {
    AVAILABLE.store(available != 0, Ordering::SeqCst);
}

/// Whether the menu is asked for (the game polls it each frame).
#[unsafe(no_mangle)]
pub extern "C" fn pc_menu_requested() -> i32 {
    // Not interrupted while holding the lock.
    let _g = crate::runtime::irq::guard();
    let mut asked = ASKED.lock().unwrap();
    if asked.is_some_and(|at| at.elapsed() >= LAPSE) {
        *asked = None;
    }
    i32::from(asked.is_some())
}

/// The game took the ask.
#[unsafe(no_mangle)]
pub extern "C" fn pc_menu_taken() {
    let _g = crate::runtime::irq::guard();
    *ASKED.lock().unwrap() = None;
}

/// Whether Quit game asks first ([general] confirm_exit, default on).
#[unsafe(no_mangle)]
pub extern "C" fn pc_menu_confirms_quit() -> i32 {
    i32::from(crate::config::get().general.confirm_exit.unwrap_or(true))
}

/// Quit game.
#[unsafe(no_mangle)]
pub extern "C" fn pc_menu_quit() {
    crate::logfile::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A second ask soon after the first waits; one after a while falls
    /// back; one left longer lapses.
    #[test]
    fn asks_fall_back_when_unanswered() {
        // Where the game cannot open it: the prompt at once.
        pc_menu_available(0);
        assert!(ask());
        assert_eq!(pc_menu_requested(), 0);
        pc_menu_available(1);
        *ASKED.lock().unwrap() = None;
        assert!(!ask());
        assert!(!ask());
        *ASKED.lock().unwrap() = Some(Instant::now() - PATIENCE);
        assert!(ask());
        assert_eq!(pc_menu_requested(), 0);
        *ASKED.lock().unwrap() = Some(Instant::now() - LAPSE);
        assert_eq!(pc_menu_requested(), 0);
        assert!(!ask());
        assert_eq!(pc_menu_requested(), 1);
    }
}
