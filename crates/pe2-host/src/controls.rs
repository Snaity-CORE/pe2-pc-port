//! Controls: the PlayStation buttons, their default keys and controller
//! buttons, and the mapping pe2.toml's [input] section changes.
//!
//! Keys are named as winit names them ("KeyZ", "ArrowUp", "Enter"),
//! controller buttons as gilrs does, by position ("South", "DPadUp",
//! "LeftTrigger2"). An empty name leaves a button unmapped.

use gilrs::Button;
use winit::keyboard::KeyCode;

use crate::video::button;

/// A PlayStation button: its pe2.toml name, its bit, its default key and
/// controller button, and its label.
pub struct PsxButton {
    pub name: &'static str,
    pub bit: u16,
    pub key: &'static str,
    pub pad: &'static str,
    pub label: &'static str,
}

pub const BUTTONS: [PsxButton; 16] = [
    PsxButton { name: "up", bit: button::UP, key: "ArrowUp", pad: "DPadUp", label: "Up" },
    PsxButton { name: "down", bit: button::DOWN, key: "ArrowDown", pad: "DPadDown", label: "Down" },
    PsxButton { name: "left", bit: button::LEFT, key: "ArrowLeft", pad: "DPadLeft", label: "Left" },
    PsxButton { name: "right", bit: button::RIGHT, key: "ArrowRight", pad: "DPadRight", label: "Right" },
    PsxButton { name: "cross", bit: button::CROSS, key: "KeyZ", pad: "South", label: "Cross" },
    PsxButton { name: "circle", bit: button::CIRCLE, key: "KeyX", pad: "East", label: "Circle" },
    PsxButton { name: "square", bit: button::SQUARE, key: "KeyA", pad: "West", label: "Square" },
    PsxButton { name: "triangle", bit: button::TRIANGLE, key: "KeyS", pad: "North", label: "Triangle" },
    PsxButton { name: "start", bit: button::START, key: "Enter", pad: "Start", label: "Start" },
    PsxButton { name: "select", bit: button::SELECT, key: "Backspace", pad: "Select", label: "Select" },
    PsxButton { name: "l1", bit: button::L1, key: "KeyQ", pad: "LeftTrigger", label: "L1" },
    PsxButton { name: "r1", bit: button::R1, key: "KeyW", pad: "RightTrigger", label: "R1" },
    PsxButton { name: "l2", bit: button::L2, key: "KeyE", pad: "LeftTrigger2", label: "L2" },
    PsxButton { name: "r2", bit: button::R2, key: "KeyR", pad: "RightTrigger2", label: "R2" },
    PsxButton { name: "l3", bit: button::L3, key: "", pad: "LeftThumb", label: "L3" },
    PsxButton { name: "r3", bit: button::R3, key: "", pad: "RightThumb", label: "R3" },
];

/// Keys with a job of their own (quit, pause, screenshot, disc swap,
/// marker, GPU dump, and Alt for full screen): never mapped to buttons.
pub const RESERVED_KEYS: [&str; 8] = ["Escape", "KeyP", "F9", "F10", "F11", "F12", "AltLeft", "AltRight"];

/// Keys a button can be mapped to.
const KEYS: &[KeyCode] = {
    use KeyCode::*;
    &[
        KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ,
        KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ, Digit0, Digit1, Digit2, Digit3, Digit4, Digit5,
        Digit6, Digit7, Digit8, Digit9, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7,
        Numpad8, Numpad9, NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide, NumpadDecimal, NumpadEnter,
        ArrowUp, ArrowDown, ArrowLeft, ArrowRight, Enter, Space, Backspace, Tab, ShiftLeft, ShiftRight,
        ControlLeft, ControlRight, Comma, Period, Slash, Semicolon, Quote, BracketLeft, BracketRight, Minus,
        Equal, Backquote, Backslash, Insert, Delete, Home, End, PageUp, PageDown, F1, F2, F3, F4, F5, F6, F7, F8,
    ]
};

/// The key a name stands for.
pub fn key_code(name: &str) -> Option<KeyCode> {
    KEYS.iter().copied().find(|k| format!("{k:?}") == name)
}

/// The controller button a name stands for.
pub fn pad_button(name: &str) -> Option<Button> {
    use Button::*;
    [
        South, East, North, West, C, Z, LeftTrigger, LeftTrigger2, RightTrigger, RightTrigger2, Select, Start,
        Mode, LeftThumb, RightThumb, DPadUp, DPadDown, DPadLeft, DPadRight,
    ]
    .into_iter()
    .find(|b| format!("{b:?}") == name)
}

/// A key's name for display ("KeyZ" -> "Z", "Digit1" -> "1").
pub fn key_label(name: &str) -> String {
    if name.is_empty() {
        return "-".into();
    }
    name.strip_prefix("Key")
        .or_else(|| name.strip_prefix("Digit"))
        .map(str::to_string)
        .unwrap_or_else(|| name.replacen("Arrow", "Arrow ", 1))
}

/// A controller button's name for display, with the Xbox and PlayStation
/// buttons at that position.
pub fn pad_label(name: &str) -> String {
    match name {
        "" => "-".into(),
        "South" => "A / \u{2715}".into(),
        "East" => "B / \u{25CB}".into(),
        "West" => "X / \u{25A1}".into(),
        "North" => "Y / \u{25B3}".into(),
        "LeftTrigger" => "LB / L1".into(),
        "RightTrigger" => "RB / R1".into(),
        "LeftTrigger2" => "LT / L2".into(),
        "RightTrigger2" => "RT / R2".into(),
        "LeftThumb" => "Left stick click".into(),
        "RightThumb" => "Right stick click".into(),
        "Select" => "Back / Share".into(),
        "Start" => "Start / Options".into(),
        other => other.replace("DPad", "D-pad "),
    }
}

/// Whether movement is camera-relative ([input] movement = "modern").
pub fn modern_movement() -> bool {
    match crate::config::get().input.movement.as_deref().map(str::trim) {
        None | Some("tank") => false,
        Some("modern") => true,
        Some(other) => {
            eprintln!("pe2: unknown movement {other:?} (tank or modern); using tank");
            false
        }
    }
}

/// The game's movement options (pad_input.c in the fork): bit 0 modern
/// movement, bit 1 quick turn.
#[unsafe(no_mangle)]
pub extern "C" fn pc_input_options() -> u32 {
    static OPTIONS: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *OPTIONS.get_or_init(|| {
        let quick_turn = crate::config::get().input.quick_turn.unwrap_or(true);
        u32::from(modern_movement()) | (u32::from(quick_turn) << 1)
    })
}

/// PE2_TRACE=move: the stick (or d-pad) direction, the yaw it asks for,
/// the player's yaw and position, each tick of modern movement.
#[unsafe(no_mangle)]
pub extern "C" fn pc_trace_movement(sx: i32, sy: i32, want: i32, yaw: i32, x: i32, z: i32) {
    if crate::runtime::trace("move") {
        eprintln!("[move] stick ({sx}, {sy}) want {want} yaw {yaw} at ({x}, {z})");
    }
}

/// The keyboard mapping in use: each key and the buttons it presses.
pub fn keyboard() -> Vec<(KeyCode, u16)> {
    let config = &crate::config::get().input.keyboard;
    BUTTONS
        .iter()
        .filter_map(|b| {
            let name = config.get(b.name).map_or(b.key, String::as_str);
            key_code(name).filter(|_| !RESERVED_KEYS.contains(&name)).map(|k| (k, b.bit))
        })
        .collect()
}

/// The controller mapping in use.
pub fn gamepad() -> Vec<(Button, u16)> {
    let config = &crate::config::get().input.gamepad;
    BUTTONS
        .iter()
        .filter_map(|b| pad_button(config.get(b.name).map_or(b.pad, String::as_str)).map(|p| (p, b.bit)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every default key and controller button is one the mapping knows,
    /// and no default uses a reserved key.
    #[test]
    fn defaults_are_known() {
        for b in &BUTTONS {
            assert!(b.key.is_empty() || key_code(b.key).is_some(), "{}", b.key);
            assert!(!RESERVED_KEYS.contains(&b.key), "{}", b.key);
            assert!(pad_button(b.pad).is_some(), "{}", b.pad);
        }
        assert_eq!(key_label("KeyZ"), "Z");
        assert_eq!(key_label("ArrowUp"), "Arrow Up");
    }
}
