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
use std::time::{Duration, Instant};

use super::{call_psx, irq, trace};

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
    *PAD_BUFFERS.lock().unwrap() = (buf1, buf2);
    refresh_pads();
}

/// Writes the current controller state into the game's buffers: port 1 a
/// digital pad (0x41) with the keyboard's buttons (active-low), port 2
/// empty.
pub fn refresh_pads() {
    let (b1, b2) = *PAD_BUFFERS.lock().unwrap();
    let held = crate::video::PAD1_BUTTONS.load(Ordering::SeqCst);
    // SAFETY: the game registered buffers of at least 34 bytes.
    unsafe {
        if b1 != 0 {
            let p = b1 as *mut u8;
            *p = 0x00;
            *p.add(1) = 0x41;
            *p.add(2) = !(held as u8);
            *p.add(3) = !((held >> 8) as u8);
        }
        if b2 != 0 {
            *(b2 as *mut u8) = 0xFF;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadStartCom() {}

/// PadGetState(port): 6 (stable) for the pad on port 1, 0 (disconnected)
/// otherwise.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadGetState(port: i32) -> i32 {
    refresh_pads();
    if port == 0x00 { 6 } else { 0 }
}

/// PadInfoMode(port, term, offs): current controller ID 4 (digital pad).
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadInfoMode(port: i32, term: i32, _offs: i32) -> i32 {
    if port == 0x00 && term == 1 { 4 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadSetAct(_port: i32, _table: u32, _len: i32) {}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadSetActAlign(_port: i32, _table: u32) -> i32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_PadSetMainMode(_port: i32, _offs: i32, _lock: i32) -> i32 {
    1
}

// ---- memory card: no card inserted ---------------------------------------

const MC_ERR_CARD_NOT_EXIST: i32 = 1;

static MC_LAST: Mutex<(i32, i32)> = Mutex::new((0, 0));

fn mc_command(func: i32) -> i32 {
    *MC_LAST.lock().unwrap() = (func, MC_ERR_CARD_NOT_EXIST);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardInit(_val: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardStart() {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardClose() {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardExist(_chan: i32) -> i32 {
    mc_command(1)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardAccept(_chan: i32) -> i32 {
    mc_command(2)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardOpen(_chan: i32, _file: u32, _flag: i32) -> i32 {
    MC_ERR_CARD_NOT_EXIST
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardReadData(_adrs: u32, _ofs: i32, _bytes: i32) -> i32 {
    mc_command(5)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardWriteData(_adrs: u32, _ofs: i32, _bytes: i32) -> i32 {
    mc_command(6)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardWriteFile(_chan: i32, _file: u32, _adrs: u32, _ofs: i32, _bytes: i32) -> i32 {
    mc_command(4)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardCreateFile(_chan: i32, _file: u32, _blocks: i32) -> i32 {
    MC_ERR_CARD_NOT_EXIST
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardFormat(_chan: i32) -> i32 {
    MC_ERR_CARD_NOT_EXIST
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardGetDirentry(
    _chan: i32,
    _name: u32,
    _dir: u32,
    files: *mut i32,
    _ofs: i32,
    _max: i32,
) -> i32 {
    if !files.is_null() {
        // SAFETY: the game passes a valid out pointer.
        unsafe { *files = 0 };
    }
    MC_ERR_CARD_NOT_EXIST
}

/// MemCardSync(mode, cmds, rslt): the last command has always finished.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardSync(_mode: i32, cmds: *mut i32, rslt: *mut i32) -> i32 {
    let (cmd, result) = *MC_LAST.lock().unwrap();
    // SAFETY: the game passes valid out pointers (or null).
    unsafe {
        if !cmds.is_null() {
            *cmds = cmd;
        }
        if !rslt.is_null() {
            *rslt = result;
        }
    }
    1
}

// ---- sound: accepted and dropped ------------------------------------------

static SPU_TRANSFER_CB: AtomicU32 = AtomicU32::new(0);
static SPU_IRQ_CB: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuInit() {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetCommonAttr(_attr: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuReserveReverbWorkArea(on: i32) -> i32 {
    on
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverbVoice(_on: i32, voices: u32) -> u32 {
    voices
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverb(on: i32) -> i32 {
    on
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverbModeParam(_attr: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverbDepth(_attr: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetTransferStartAddr(addr: u32) -> u32 {
    addr
}

fn spu_transfer(size: u32) -> u32 {
    let cb = SPU_TRANSFER_CB.load(Ordering::SeqCst);
    if cb != 0 {
        irq::schedule(Instant::now() + Duration::from_micros(200), Box::new(move || {
            call_psx(cb, &[]);
        }));
    }
    size
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuWrite(_addr: u32, size: u32) -> u32 {
    spu_transfer(size)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuWritePartly(_addr: u32, size: u32) -> u32 {
    spu_transfer(size)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuIsTransferCompleted(_flag: i32) -> i32 {
    1
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetTransferCallback(func: u32) -> u32 {
    SPU_TRANSFER_CB.swap(func, Ordering::SeqCst)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetIRQ(on: i32) -> i32 {
    on
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetIRQAddr(addr: u32) -> u32 {
    addr
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetIRQCallback(func: u32) -> u32 {
    SPU_IRQ_CB.swap(func, Ordering::SeqCst)
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuGetAllKeysStatus(status: *mut u8) {
    if !status.is_null() {
        // SAFETY: the game passes a 24-voice status array.
        unsafe { std::ptr::write_bytes(status, 0, 24) };
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetKey(_on: i32, _voices: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuLSetVoiceAttr(_count: i32, _attrs: u32) {}
