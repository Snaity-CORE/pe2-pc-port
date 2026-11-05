//! Controllers through XInput, where gilrs cannot start: under Wine and
//! Proton, whose Windows.Gaming.Input lacks what gilrs asks of it. There
//! every controller is an XInput one (the Steam Deck's too, through Steam
//! Input). Read by polling, up to four, named by their slot.

use gilrs::{Axis, Button};
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
struct Gamepad {
    buttons: u16,
    left_trigger: u8,
    right_trigger: u8,
    thumb_lx: i16,
    thumb_ly: i16,
    thumb_rx: i16,
    thumb_ry: i16,
}

#[repr(C)]
#[derive(Default)]
struct State {
    packet: u32,
    gamepad: Gamepad,
}

#[repr(C)]
struct Vibration {
    left: u16,
    right: u16,
}

type GetState = unsafe extern "system" fn(u32, *mut State) -> u32;
type SetState = unsafe extern "system" fn(u32, *mut Vibration) -> u32;

/// XInput's button bits (Guide only through XInputGetStateEx).
const BUTTON_BITS: [(Button, u16); 15] = [
    (Button::DPadUp, 0x0001),
    (Button::DPadDown, 0x0002),
    (Button::DPadLeft, 0x0004),
    (Button::DPadRight, 0x0008),
    (Button::Start, 0x0010),
    (Button::Select, 0x0020),
    (Button::LeftThumb, 0x0040),
    (Button::RightThumb, 0x0080),
    (Button::LeftTrigger, 0x0100),
    (Button::RightTrigger, 0x0200),
    (Button::Mode, 0x0400),
    (Button::South, 0x1000),
    (Button::East, 0x2000),
    (Button::West, 0x4000),
    (Button::North, 0x8000),
];

/// How far a trigger goes before it counts as pressed (XInput's own
/// threshold).
const TRIGGER_THRESHOLD: u8 = 30;

pub struct XInput {
    get_state: GetState,
    set_state: Option<SetState>,
}

/// One controller's reading.
#[derive(Clone, Copy, PartialEq)]
pub struct Pad(Gamepad);

/// A slot's name, for pe2.toml's [input] controller and the launcher.
pub fn name(slot: usize) -> String {
    format!("XInput controller {}", slot + 1)
}

impl XInput {
    pub fn open() -> Option<XInput> {
        for dll in [c"xinput1_4.dll", c"xinput1_3.dll", c"xinput9_1_0.dll"] {
            let lib = unsafe { LoadLibraryA(dll.as_ptr().cast()) };
            if lib.is_null() {
                continue;
            }
            // XInputGetStateEx (ordinal 100) also reports the Guide button.
            let get = unsafe { GetProcAddress(lib, 100usize as *const u8) }
                .or_else(|| unsafe { GetProcAddress(lib, c"XInputGetState".as_ptr().cast()) })?;
            let set = unsafe { GetProcAddress(lib, c"XInputSetState".as_ptr().cast()) };
            // SAFETY: both have the signatures XInput documents.
            return Some(unsafe {
                XInput {
                    get_state: std::mem::transmute::<unsafe extern "system" fn() -> isize, GetState>(get),
                    set_state: set.map(|f| std::mem::transmute::<unsafe extern "system" fn() -> isize, SetState>(f)),
                }
            });
        }
        None
    }

    /// The controller in a slot (0-3), if one is connected.
    pub fn read(&self, slot: usize) -> Option<Pad> {
        let mut state = State::default();
        (unsafe { (self.get_state)(slot as u32, &mut state) } == 0).then_some(Pad(state.gamepad))
    }

    /// The four slots' controllers.
    pub fn read_all(&self) -> [Option<Pad>; 4] {
        std::array::from_fn(|slot| self.read(slot))
    }

    /// Sets a controller's motors (0 stops them).
    pub fn rumble(&self, slot: usize, large: u16, small: u16) {
        if let Some(set) = self.set_state {
            let mut v = Vibration { left: large, right: small };
            unsafe { set(slot as u32, &mut v) };
        }
    }
}

impl Pad {
    pub fn is_pressed(&self, b: Button) -> bool {
        match b {
            Button::LeftTrigger2 => self.0.left_trigger > TRIGGER_THRESHOLD,
            Button::RightTrigger2 => self.0.right_trigger > TRIGGER_THRESHOLD,
            _ => BUTTON_BITS.iter().any(|&(x, bit)| x == b && self.0.buttons & bit != 0),
        }
    }

    /// A stick axis, -1.0..1.0, up positive for Y (as gilrs reports it).
    pub fn value(&self, axis: Axis) -> f32 {
        let v = match axis {
            Axis::LeftStickX => self.0.thumb_lx,
            Axis::LeftStickY => self.0.thumb_ly,
            Axis::RightStickX => self.0.thumb_rx,
            Axis::RightStickY => self.0.thumb_ry,
            _ => 0,
        };
        (f32::from(v) / 32767.0).max(-1.0)
    }

    /// The buttons pressed here and not in `before`.
    pub fn pressed_since(&self, before: Option<Pad>) -> impl Iterator<Item = Button> + '_ {
        BUTTON_BITS
            .iter()
            .map(|&(b, _)| b)
            .chain([Button::LeftTrigger2, Button::RightTrigger2])
            .filter(move |&b| self.is_pressed(b) && !before.is_some_and(|p| p.is_pressed(b)))
    }
}
