//! Native implementations of the libspu calls the game makes, on top of the
//! SPU model. Written from the library's documented interface (structures
//! and constants as the game uses them); no library code is run.
//!
//! Structure offsets (checked against the decomp's headers):
//! - SpuLVoiceAttr (68 bytes): voiceNum i16 @0; SpuVoiceAttr @4 with mask
//!   @4, volume.left/right @8/@10, pitch @20, addr @28, loop_addr @32,
//!   adsr1 @58, adsr2 @60 (offsets within the voice attribute).
//! - SpuCommonAttr: mask @0, mvol @4/@6, cd.volume @16/@18, cd.reverb @20,
//!   cd.mix @24.
//! - SpuReverbAttr: mask @0, mode @4, depth @8/@10.

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use super::core::{
    CONTROL_CD_ENABLE, CONTROL_CD_REVERB, CONTROL_ENABLE, CONTROL_IRQ_ENABLE, CONTROL_REVERB, CONTROL_UNMUTE,
    RAM_SIZE, ReverbRegs, VOICES,
};
use super::{IRQ_CALLBACK, with_spu};
use crate::runtime::{call_psx, irq, trace};

const SPU_OFF: i32 = 0;
const SPU_ON: i32 = 1;
const SPU_RESET: i32 = 3;

static TRANSFER_ADDR: AtomicU32 = AtomicU32::new(0);
static TRANSFER_CALLBACK: AtomicU32 = AtomicU32::new(0);

fn rd16(p: *const u8, off: usize) -> u16 {
    // SAFETY: callers pass libspu structures in PSX RAM.
    unsafe { p.add(off).cast::<u16>().read_unaligned() }
}
fn rd32(p: *const u8, off: usize) -> u32 {
    // SAFETY: as above.
    unsafe { p.add(off).cast::<u32>().read_unaligned() }
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuInit() {
    let _g = irq::guard();
    with_spu(|s| {
        // The state libspu's initialization leaves the hardware in: every
        // voice silent at the highest pitch, playing a silent looping block
        // at 0x1000. The game relies on the pitch: its own start-up keys
        // voices on without setting one, and they only free themselves by
        // running to the end of their sample.
        s.ram[0x1000..0x1010].fill(0);
        s.ram[0x1001] = 0x07; // loop start, end, repeat
        for v in s.voices.iter_mut() {
            *v = Default::default();
            v.pitch = 0x3FFF;
            v.start = 0x1000 / 8;
        }
        s.control = CONTROL_ENABLE | CONTROL_UNMUTE;
        s.main_vol = [0, 0];
        s.reverb_vol = [0, 0];
        s.cd_vol = [0, 0];
        s.reverb_on = 0;
        s.endx = 0;
        s.irq_flag = false;
        s.cd_input.clear();
        s.key_on(0xFF_FFFF);
    });
    super::start();
}

/// SpuSetCommonAttr: master volume and CD input. A zero mask sets every
/// attribute. Volumes are stored as libspu stores direct-mode volumes.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetCommonAttr(attr: *const u8) {
    let _g = irq::guard();
    let mask = rd32(attr, 0);
    let all = mask == 0;
    let has = |bit: u32| all || mask & bit != 0;
    with_spu(|s| {
        if has(1 << 0) {
            s.main_vol[0] = rd16(attr, 4) & 0x7FFF;
        }
        if has(1 << 1) {
            s.main_vol[1] = rd16(attr, 6) & 0x7FFF;
        }
        if has(1 << 6) {
            s.cd_vol[0] = rd16(attr, 16) as i16;
        }
        if has(1 << 7) {
            s.cd_vol[1] = rd16(attr, 18) as i16;
        }
        if has(1 << 8) {
            set_bit(&mut s.control, CONTROL_CD_REVERB, rd32(attr, 20) as i32 == SPU_ON);
        }
        if has(1 << 9) {
            set_bit(&mut s.control, CONTROL_CD_ENABLE, rd32(attr, 24) as i32 == SPU_ON);
        }
    });
}

fn set_bit(reg: &mut u16, bit: u16, on: bool) {
    if on {
        *reg |= bit;
    } else {
        *reg &= !bit;
    }
}

/// SpuSetKey(on_off, voices): keys the voices in the bit mask on or off.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetKey(on: i32, voices: u32) {
    let _g = irq::guard();
    with_spu(|s| {
        if on == SPU_ON {
            s.key_on(voices & 0xFF_FFFF);
        } else {
            s.key_off(voices & 0xFF_FFFF);
        }
    });
    if trace("spu") {
        let t = super::stamp();
        eprintln!("[spu {t:8.3}] key {} {voices:06X}", if on == SPU_ON { "on" } else { "off" });
    }
}

/// SpuLSetVoiceAttr(count, list): applies each entry's attributes to its
/// voice. Volumes are in direct mode, as the game uses them.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuLSetVoiceAttr(count: i32, list: *const u8) {
    let _g = irq::guard();
    with_spu(|s| {
        for n in 0..count.max(0) as usize {
            // SAFETY: `list` holds `count` SpuLVoiceAttr entries.
            let e = unsafe { list.add(n * 68) };
            let voice = rd16(e, 0) as usize;
            if voice >= VOICES {
                continue;
            }
            let a = e.wrapping_add(4);
            let mask = rd32(a, 4);
            let all = mask == 0;
            let has = |bit: u32| all || mask & bit != 0;
            let v = &mut s.voices[voice];
            if has(1 << 0) {
                v.vol_l = rd16(a, 8) & 0x7FFF;
            }
            if has(1 << 1) {
                v.vol_r = rd16(a, 10) & 0x7FFF;
            }
            if has(1 << 4) {
                v.pitch = rd16(a, 20).min(0x3FFF);
            }
            if has(1 << 7) {
                v.start = (rd32(a, 28) >> 3) as u16;
            }
            if has(1 << 16) {
                v.repeat = (rd32(a, 32) >> 3) as u16;
            }
            if has(1 << 17) {
                v.adsr1 = rd16(a, 58);
            }
            if has(1 << 18) {
                v.adsr2 = rd16(a, 60);
            }
        }
    });
}

/// SpuGetAllKeysStatus(status): one SPU_ON / SPU_OFF / SPU_OFF_ENV_ON /
/// SPU_ON_ENV_OFF byte per voice.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuGetAllKeysStatus(status: *mut u8) {
    let _g = irq::guard();
    let line = with_spu(|s| {
        let mut line = String::new();
        for i in 0..VOICES {
            let st = s.key_status(i);
            // SAFETY: the game passes a 24-byte array.
            unsafe { status.add(i).write(st) };
            line.push(char::from(b'0' + st));
        }
        line
    });
    if trace("spukeys") {
        static LAST: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
        let mut last = LAST.lock().unwrap();
        if *last != line {
            eprintln!("[spu] key status {line}");
            *last = line;
        }
    }
}

// ---- reverb ----------------------------------------------------------------

/// Reverb presets (register values at 0x1F801DC0 and work area size in
/// bytes), from the public hardware documentation, indexed by libspu mode:
/// off, room, studio A/B/C (small, medium, large), hall, space echo, echo
/// (chaos echo), delay, pipe (half echo).
#[rustfmt::skip]
const PRESETS: [([u16; 32], u32); 10] = [
    ([0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0001,0x0001,0x0001,0x0001,0x0001,0x0001,
      0x0000,0x0000,0x0001,0x0001,0x0001,0x0001,0x0001,0x0001,0x0000,0x0000,0x0001,0x0001,0x0001,0x0001,0x0000,0x0000], 0x10),
    ([0x007D,0x005B,0x6D80,0x54B8,0xBED0,0x0000,0x0000,0xBA80,0x5800,0x5300,0x04D6,0x0333,0x03F0,0x0227,0x0374,0x01EF,
      0x0334,0x01B5,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x01B4,0x0136,0x00B8,0x005C,0x8000,0x8000], 0x26C0),
    ([0x0033,0x0025,0x70F0,0x4FA8,0xBCE0,0x4410,0xC0F0,0x9C00,0x5280,0x4EC0,0x03E4,0x031B,0x03A4,0x02AF,0x0372,0x0266,
      0x031C,0x025D,0x025C,0x018E,0x022F,0x0135,0x01D2,0x00B7,0x018F,0x00B5,0x00B4,0x0080,0x004C,0x0026,0x8000,0x8000], 0x1F40),
    ([0x00B1,0x007F,0x70F0,0x4FA8,0xBCE0,0x4510,0xBEF0,0xB4C0,0x5280,0x4EC0,0x0904,0x076B,0x0824,0x065F,0x07A2,0x0616,
      0x076C,0x05ED,0x05EC,0x042E,0x050F,0x0305,0x0462,0x02B7,0x042F,0x0265,0x0264,0x01B2,0x0100,0x0080,0x8000,0x8000], 0x4840),
    ([0x00E3,0x00A9,0x6F60,0x4FA8,0xBCE0,0x4510,0xBEF0,0xA680,0x5680,0x52C0,0x0DFB,0x0B58,0x0D09,0x0A3C,0x0BD9,0x0973,
      0x0B59,0x08DA,0x08D9,0x05E9,0x07EC,0x04B0,0x06EF,0x03D2,0x05EA,0x031D,0x031C,0x0238,0x0154,0x00AA,0x8000,0x8000], 0x6FE0),
    ([0x01A5,0x0139,0x6000,0x5000,0x4C00,0xB800,0xBC00,0xC000,0x6000,0x5C00,0x15BA,0x11BB,0x14C2,0x10BD,0x11BC,0x0DC1,
      0x11C0,0x0DC3,0x0DC0,0x09C1,0x0BC4,0x07C1,0x0A00,0x06CD,0x09C2,0x05C1,0x05C0,0x041A,0x0274,0x013A,0x8000,0x8000], 0xADE0),
    ([0x033D,0x0231,0x7E00,0x5000,0xB400,0xB000,0x4C00,0xB000,0x6000,0x5400,0x1ED6,0x1A31,0x1D14,0x183B,0x1BC2,0x16B2,
      0x1A32,0x15EF,0x15EE,0x1055,0x1334,0x0F2D,0x11F6,0x0C5D,0x1056,0x0AE1,0x0AE0,0x07A2,0x0464,0x0232,0x8000,0x8000], 0xF6C0),
    ([0x0001,0x0001,0x7FFF,0x7FFF,0x0000,0x0000,0x0000,0x8100,0x0000,0x0000,0x1FFF,0x0FFF,0x1005,0x0005,0x0000,0x0000,
      0x1005,0x0005,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x1004,0x1002,0x0004,0x0002,0x8000,0x8000], 0x18040),
    ([0x0001,0x0001,0x7FFF,0x7FFF,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x1FFF,0x0FFF,0x1005,0x0005,0x0000,0x0000,
      0x1005,0x0005,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x0000,0x1004,0x1002,0x0004,0x0002,0x8000,0x8000], 0x18040),
    ([0x0017,0x0013,0x70F0,0x4FA8,0xBCE0,0x4510,0xBEF0,0x8500,0x5F80,0x54C0,0x0371,0x02AF,0x02E5,0x01DF,0x02B0,0x01D7,
      0x0358,0x026A,0x01D6,0x011E,0x012D,0x00B1,0x011F,0x0059,0x01A0,0x00E3,0x0058,0x0040,0x0028,0x0014,0x8000,0x8000], 0x3C00),
];

/// libspu flag on a reverb mode: also clear the work area.
const SPU_REV_MODE_CLEAR_WA: u32 = 0x100;

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverb(on: i32) -> i32 {
    let _g = irq::guard();
    with_spu(|s| set_bit(&mut s.control, CONTROL_REVERB, on == SPU_ON));
    on
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuReserveReverbWorkArea(on: i32) -> i32 {
    on
}

/// SpuSetReverbModeParam(attr): selects a preset (and work area) and/or the
/// output depth. A zero mask sets every attribute.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverbModeParam(attr: *const u8) -> i32 {
    let _g = irq::guard();
    let mask = rd32(attr, 0);
    let all = mask == 0;
    with_spu(|s| {
        if all || mask & 1 != 0 {
            let mode = rd32(attr, 4);
            let (regs, size) = PRESETS[(mode & 0xFF).min(9) as usize];
            s.reverb = ReverbRegs::from_words(&regs);
            let base = (0x80000 - size) & !7;
            s.set_reverb_base((base / 8) as u16);
            if mode & SPU_REV_MODE_CLEAR_WA != 0 {
                s.ram[base as usize..RAM_SIZE].fill(0);
            }
        }
        if all || mask & 2 != 0 {
            s.reverb_vol[0] = rd16(attr, 8) as i16;
        }
        if all || mask & 4 != 0 {
            s.reverb_vol[1] = rd16(attr, 10) as i16;
        }
    });
    0
}

/// SpuSetReverbDepth(attr): reverb output volume.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverbDepth(attr: *const u8) {
    let _g = irq::guard();
    let mask = rd32(attr, 0);
    with_spu(|s| {
        if mask == 0 || mask & 2 != 0 {
            s.reverb_vol[0] = rd16(attr, 8) as i16;
        }
        if mask == 0 || mask & 4 != 0 {
            s.reverb_vol[1] = rd16(attr, 10) as i16;
        }
    });
}

/// SpuSetReverbVoice(on_off, voices): sends voices to the reverb or not;
/// returns the voices now sent.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetReverbVoice(on: i32, voices: u32) -> u32 {
    let _g = irq::guard();
    with_spu(|s| {
        if on == SPU_ON {
            s.reverb_on |= voices & 0xFF_FFFF;
        } else {
            s.reverb_on &= !voices;
        }
        s.reverb_on
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuGetReverbVoice() -> u32 {
    let _g = irq::guard();
    with_spu(|s| s.reverb_on)
}

/// SpuClearReverbWorkArea(mode): zeroes the work area of `mode`.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuClearReverbWorkArea(mode: i32) -> i32 {
    let _g = irq::guard();
    let (_, size) = PRESETS[(mode.max(0) as usize).min(9)];
    let base = ((0x80000 - size) & !7) as usize;
    with_spu(|s| s.ram[base..RAM_SIZE].fill(0));
    0
}

// ---- transfers -------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetTransferStartAddr(addr: u32) -> u32 {
    if addr as usize >= RAM_SIZE {
        return 0;
    }
    TRANSFER_ADDR.store(addr & !7, Ordering::SeqCst);
    addr
}

/// Copies `size` bytes from main RAM to sound RAM at the transfer address.
fn transfer(src: *const u8, size: u32, advance: bool) -> u32 {
    let _g = irq::guard();
    let dst = TRANSFER_ADDR.load(Ordering::SeqCst) as usize;
    let len = (size as usize).min(RAM_SIZE - dst.min(RAM_SIZE));
    with_spu(|s| {
        // SAFETY: the game passes `size` readable bytes in PSX RAM.
        let data = unsafe { std::slice::from_raw_parts(src, len) };
        s.ram[dst..dst + len].copy_from_slice(data);
        s.transfer_irq_check(dst as u32, len as u32);
    });
    if advance {
        TRANSFER_ADDR.store(((dst + len) & (RAM_SIZE - 1)) as u32, Ordering::SeqCst);
    }
    let irq_hit = with_spu(|s| std::mem::take(&mut s.irq_flag));
    if irq_hit {
        super::raise_irq();
    }
    let cb = TRANSFER_CALLBACK.load(Ordering::SeqCst);
    if cb != 0 {
        irq::schedule(Instant::now() + Duration::from_micros(200), Box::new(move || {
            call_psx(cb, &[]);
        }));
    }
    if trace("spu") {
        eprintln!("[spu {:8.3}] write {len} bytes to 0x{dst:05X}", super::stamp());
    }
    size
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuWrite(src: *const u8, size: u32) -> u32 {
    transfer(src, size, false)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuWritePartly(src: *const u8, size: u32) -> u32 {
    transfer(src, size, true)
}

/// Transfers complete immediately.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuIsTransferCompleted(_flag: i32) -> i32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetTransferCallback(func: u32) -> u32 {
    TRANSFER_CALLBACK.swap(func, Ordering::SeqCst)
}

// ---- IRQ -------------------------------------------------------------------

/// SpuSetIRQ(on_off): SPU_ON / SPU_OFF / SPU_RESET (off then on); returns
/// the resulting state.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetIRQ(on: i32) -> i32 {
    let _g = irq::guard();
    with_spu(|s| {
        match on {
            SPU_OFF => s.control &= !CONTROL_IRQ_ENABLE,
            SPU_ON | SPU_RESET => {
                s.irq_flag = false;
                s.control |= CONTROL_IRQ_ENABLE;
            }
            _ => {}
        }
        if trace("spu") {
            eprintln!("[spu {:8.3}] irq enable {on} at 0x{:05X}", super::stamp(), u32::from(s.irq_addr) * 8);
        }
        if s.control & CONTROL_IRQ_ENABLE != 0 { SPU_ON } else { SPU_OFF }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetIRQAddr(addr: u32) -> u32 {
    let _g = irq::guard();
    if addr as usize >= RAM_SIZE {
        return 0;
    }
    with_spu(|s| s.irq_addr = (addr >> 3) as u16);
    addr
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_SpuSetIRQCallback(func: u32) -> u32 {
    IRQ_CALLBACK.swap(func, Ordering::SeqCst)
}
