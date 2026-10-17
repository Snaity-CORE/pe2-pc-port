//! Native stand-ins for library functions that drive hardware or the BIOS
//! kernel, until each subsystem gets its real implementation:
//!
//! * interrupt and event management (libapi / libetc): native, minimal
//! * root counters: from the host clock
//! * controller (libpad): one digital pad, nothing pressed (input comes later)
//! * memory card (libmcrd): no card inserted (phase 3)
//! * sound (libspu): accepted and dropped, transfers complete at once (phase 3)
//!
//! Interpreting the original code of these would poke hardware registers
//! and BIOS tables the port does not have.

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use super::trace;

fn note(name: &str) {
    if trace("stubs") {
        eprintln!("[stub] {name}");
    }
}

// ---- interrupts, events, video mode --------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ResetCallback() -> i32 {
    note("ResetCallback");
    0
}

static VIDEO_MODE: AtomicU32 = AtomicU32::new(0);

/// SetVideoMode(mode): 0 NTSC, 1 PAL. Returns the previous mode.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SetVideoMode(mode: i32) -> i32 {
    VIDEO_MODE.swap(mode as u32, Ordering::SeqCst) as i32
}

static NEXT_EVENT: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_OpenEvent(desc: u32, spec: i32, mode: i32, func: u32) -> i32 {
    let handle = 0xF100_0000 | NEXT_EVENT.fetch_add(1, Ordering::SeqCst);
    eprintln!(
        "[stub] OpenEvent(desc 0x{desc:08X}, spec 0x{spec:X}, mode 0x{mode:X}, func 0x{func:08X}) -> 0x{handle:08X}"
    );
    handle as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CloseEvent(_event: i32) -> i32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_EnableEvent(_event: i32) -> i32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_DisableEvent(_event: i32) -> i32 {
    1
}

// ---- root counters --------------------------------------------------------

/// System clock / 8, the rate of root counter 2.
const SYSCLK_DIV8_HZ: f64 = 33_868_800.0 / 8.0;
const HSYNC_HZ: f64 = 15_734.264;

struct Counter {
    target: u16,
    started: Option<Instant>,
}

static COUNTERS: Mutex<[Counter; 4]> = Mutex::new([
    Counter { target: 0xFFFF, started: None },
    Counter { target: 0xFFFF, started: None },
    Counter { target: 0xFFFF, started: None },
    Counter { target: 0xFFFF, started: None },
]);

fn counter_index(spec: u32) -> Option<usize> {
    let i = (spec & 0xF) as usize;
    (spec & 0xFFFF_FFF0 == 0xF200_0000 && i < 4).then_some(i)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SetRCnt(spec: u32, target: u16, _mode: i32) -> i32 {
    let Some(i) = counter_index(spec) else { return 0 };
    let mut c = COUNTERS.lock().unwrap();
    c[i].target = target;
    c[i].started = Some(Instant::now());
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StartRCnt(spec: u32) -> i32 {
    let Some(i) = counter_index(spec) else { return 0 };
    let mut c = COUNTERS.lock().unwrap();
    c[i].started.get_or_insert_with(Instant::now);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_StopRCnt(spec: u32) -> i32 {
    let Some(i) = counter_index(spec) else { return 0 };
    COUNTERS.lock().unwrap()[i].started = None;
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_ResetRCnt(spec: u32) -> i32 {
    let Some(i) = counter_index(spec) else { return 0 };
    let mut c = COUNTERS.lock().unwrap();
    if c[i].started.is_some() {
        c[i].started = Some(Instant::now());
    }
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_GetRCnt(spec: u32) -> i32 {
    let Some(i) = counter_index(spec) else { return 0 };
    let c = COUNTERS.lock().unwrap();
    let Some(started) = c[i].started else { return 0 };
    let secs = started.elapsed().as_secs_f64();
    let ticks = match i {
        1 => secs * HSYNC_HZ,
        3 => secs * 59.94,
        _ => secs * SYSCLK_DIV8_HZ,
    } as u64;
    (ticks % (u64::from(c[i].target) + 1)) as i32
}

// ---- controller -----------------------------------------------------------

static PAD_BUFFERS: Mutex<(u32, u32)> = Mutex::new((0, 0));

/// PadInitDirect(buf1, buf2): the library fills these each frame with the
/// state of port 1 and port 2.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadInitDirect(buf1: u32, buf2: u32) {
    let _g = super::irq::guard();
    *PAD_BUFFERS.lock().unwrap() = (buf1, buf2);
    refresh_pads();
}

/// Vibration registered by PadSetAct / PadSetActAlign: the game's actuator
/// table, its length, and which motor each of its bytes drives.
struct Actuators {
    table: u32,
    len: usize,
    align: [u8; 6],
}

static ACTUATORS: Mutex<Actuators> = Mutex::new(Actuators { table: 0, len: 0, align: [0xFF; 6] });

/// Writes the current controller state into the game's buffers (active-low
/// buttons). Port 1 is a DualShock in analog mode (0x73, sticks after the
/// buttons) while a controller is connected, a digital pad (0x41) otherwise;
/// port 2 is empty. Also passes the game's vibration request on.
pub fn refresh_pads() {
    let (b1, b2) = *PAD_BUFFERS.lock().unwrap();
    let held = crate::video::PAD1_BUTTONS.load(Ordering::SeqCst) | crate::gamepad::BUTTONS.load(Ordering::SeqCst);
    let analog = crate::gamepad::CONNECTED.load(Ordering::SeqCst);
    // SAFETY: the game registered buffers of at least 34 bytes.
    unsafe {
        if b1 != 0 {
            let p = b1 as *mut u8;
            *p = 0x00;
            *p.add(1) = if analog { 0x73 } else { 0x41 };
            *p.add(2) = !(held as u8);
            *p.add(3) = !((held >> 8) as u8);
            if analog {
                let sticks = crate::gamepad::STICKS.load(Ordering::SeqCst).to_le_bytes();
                std::ptr::copy_nonoverlapping(sticks.as_ptr(), p.add(4), 4);
            }
        }
        if b2 != 0 {
            *(b2 as *mut u8) = 0xFF;
        }
    }
    update_rumble(analog);
}

/// Motor 0 (small) is on or off, motor 1 (large) has a strength 0-255.
fn update_rumble(analog: bool) {
    let a = ACTUATORS.lock().unwrap();
    let mut want = 0u32;
    if analog && a.table != 0 {
        for i in 0..a.len.min(a.align.len()) {
            // SAFETY: PadSetAct registered `len` readable bytes.
            let v = unsafe { *(a.table as *const u8).add(i) };
            match a.align[i] {
                0 if v != 0 => want |= 1,
                1 => want |= u32::from(v) << 8,
                _ => {}
            }
        }
    }
    crate::gamepad::RUMBLE.store(want, Ordering::SeqCst);
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadStartCom() {}

/// PadGetState(port): 6 (stable) for the pad on port 1, 0 (disconnected)
/// otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadGetState(port: i32) -> i32 {
    let _g = super::irq::guard();
    refresh_pads();
    if port == 0x00 { 6 } else { 0 }
}

/// PadInfoMode(port, term, offs). InfoModeCurID (1): 7 for a DualShock in
/// analog mode, 4 for a digital pad. InfoModeCurExID (2): 7 when the pad
/// has an analog mode (a DualShock), else 0.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadInfoMode(port: i32, term: i32, _offs: i32) -> i32 {
    if port != 0x00 {
        return 0;
    }
    let analog = crate::gamepad::CONNECTED.load(Ordering::SeqCst);
    match term {
        1 => if analog { 7 } else { 4 },
        2 if analog => { 7 },
        _ => 0,
    }
}

/// PadSetAct(port, table, len): the game's actuator values, read each frame.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadSetAct(port: i32, table: u32, len: i32) {
    let _g = super::irq::guard();
    if port == 0x00 {
        let mut a = ACTUATORS.lock().unwrap();
        a.table = table;
        a.len = len.max(0) as usize;
    }
}

/// PadSetActAlign(port, align): which motor each actuator byte drives
/// (0xFF: none).
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadSetActAlign(port: i32, align: u32) -> i32 {
    let _g = super::irq::guard();
    if port != 0x00 || align == 0 {
        return 0;
    }
    let mut a = ACTUATORS.lock().unwrap();
    for (i, slot) in a.align.iter_mut().enumerate() {
        // SAFETY: the game passes a 6-byte alignment table.
        *slot = unsafe { *(align as *const u8).add(i) };
    }
    1
}

/// PadSetMainMode(port, offs, lock): the pad follows the controller
/// connected, so the request is accepted as is.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadSetMainMode(_port: i32, _offs: i32, _lock: i32) -> i32 {
    1
}

// Memory cards (libmcrd) live in memcard.rs.
// Sound (libspu) lives in spu/libspu.rs.
