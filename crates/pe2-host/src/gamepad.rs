//! Game controllers (Xbox, DualShock 4/5 and others, through gilrs), read on
//! their own thread into the state the controller emulation picks up.
//!
//! The controller pe2.toml's [input] controller names drives port 1, or by
//! default the one last used. Its buttons map as [input.gamepad] says
//! (by default by position: south = Cross, east = Circle, west = Square,
//! north = Triangle, bumpers = L1/R1, triggers = L2/R2, back = Select,
//! stick clicks = L3/R3; see controls.rs), and its sticks become the
//! DualShock's. [input] vibration = false turns rumble off.

use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};
use std::time::Duration;

use gilrs::ff::{BaseEffect, BaseEffectType, Effect, EffectBuilder, Replay, Ticks};
use gilrs::{Axis, EventType, GamepadId, Gilrs};

/// Buttons held on the controller, PSX bit layout (1 = pressed).
pub static BUTTONS: AtomicU16 = AtomicU16::new(0);
/// Stick positions in DualShock wire order (right X, right Y, left X,
/// left Y), one byte each, 0x00 = left/up, 0x80 = centre, 0xFF = right/down.
pub static STICKS: AtomicU32 = AtomicU32::new(0x8080_8080);
/// True while a controller is connected.
pub static CONNECTED: AtomicBool = AtomicBool::new(false);
/// Requested vibration: large motor strength (0-255) in bits 8-15, small
/// motor on in bit 0.
pub static RUMBLE: AtomicU32 = AtomicU32::new(0);

/// A stick axis (-1.0..1.0, up positive for Y) as a DualShock byte.
fn axis_byte(v: f32, invert: bool) -> u8 {
    let v = if invert { -v } else { v };
    (v.clamp(-1.0, 1.0) * 127.5 + 127.5).round() as u8
}

pub fn start() {
    std::thread::Builder::new().name("gamepad".into()).spawn(run).expect("spawn gamepad thread");
}

fn run() {
    let mut gilrs = match Gilrs::new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("pe2: no controller support ({e}); keyboard only");
            return;
        }
    };
    let map = crate::controls::gamepad();
    let input = &crate::config::get().input;
    let wanted = input.controller.as_deref().filter(|n| !n.is_empty() && *n != "auto");
    let vibration = input.vibration.unwrap_or(true);
    let mut active: Option<GamepadId> = None;
    let mut rumble: (u32, Option<Effect>) = (0, None);
    loop {
        while let Some(ev) = gilrs.next_event() {
            match ev.event {
                EventType::Connected => {
                    eprintln!("pe2: input: controller connected: {}", gilrs.gamepad(ev.id).name());
                    active.get_or_insert(ev.id);
                }
                EventType::Disconnected if active == Some(ev.id) => active = None,
                // The controller last used takes over port 1, unless one
                // is chosen.
                EventType::ButtonPressed(..) if wanted.is_none() => active = Some(ev.id),
                _ => {}
            }
        }
        // The chosen controller whenever it is connected.
        if let Some(name) = wanted
            && let Some((id, _)) = gilrs.gamepads().find(|(_, g)| g.name() == name)
        {
            active = Some(id);
        }
        if active.is_none_or(|id| gilrs.connected_gamepad(id).is_none()) {
            active = gilrs.gamepads().next().map(|(id, _)| id);
        }

        match active.and_then(|id| gilrs.connected_gamepad(id)) {
            Some(pad) => {
                let held = map.iter().filter(|(b, _)| pad.is_pressed(*b)).fold(0, |a, (_, bit)| a | bit);
                let sticks = u32::from_le_bytes([
                    axis_byte(pad.value(Axis::RightStickX), false),
                    axis_byte(pad.value(Axis::RightStickY), true),
                    axis_byte(pad.value(Axis::LeftStickX), false),
                    axis_byte(pad.value(Axis::LeftStickY), true),
                ]);
                BUTTONS.store(held, Ordering::SeqCst);
                STICKS.store(sticks, Ordering::SeqCst);
                CONNECTED.store(true, Ordering::SeqCst);
            }
            None => {
                BUTTONS.store(0, Ordering::SeqCst);
                STICKS.store(0x8080_8080, Ordering::SeqCst);
                CONNECTED.store(false, Ordering::SeqCst);
            }
        }

        let want = if vibration { RUMBLE.load(Ordering::SeqCst) } else { 0 };
        if want != rumble.0 {
            rumble = (want, active.and_then(|id| start_rumble(&mut gilrs, id, want)));
        }
        std::thread::sleep(Duration::from_millis(4));
    }
}

/// Starts a vibration effect for `want` (see RUMBLE); dropping it stops it.
fn start_rumble(gilrs: &mut Gilrs, id: GamepadId, want: u32) -> Option<Effect> {
    let large = ((want >> 8) & 0xFF) as u16;
    let small = want & 1 != 0;
    if large == 0 && !small {
        return None;
    }
    // Plays until replaced; the game turns the motors off itself.
    let scheduling = Replay { play_for: Ticks::from_ms(60_000), ..Default::default() };
    let mut builder = EffectBuilder::new();
    if large != 0 {
        builder.add_effect(BaseEffect {
            kind: BaseEffectType::Strong { magnitude: large * 257 },
            scheduling,
            envelope: Default::default(),
        });
    }
    if small {
        builder.add_effect(BaseEffect {
            kind: BaseEffectType::Weak { magnitude: u16::MAX },
            scheduling,
            envelope: Default::default(),
        });
    }
    let effect = builder.gamepads(&[id]).finish(gilrs).ok()?;
    effect.play().ok()?;
    Some(effect)
}
