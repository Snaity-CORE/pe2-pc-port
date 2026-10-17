//! The CD-ROM drive as libcd presents it: primitive commands, completion
//! ("sync") and data-ready interrupts with their callbacks, and CdGetSector.
//!
//! Sectors are read from the player's disc image in real time (75 sectors/s,
//! 150 at double speed) and each one raises a data-ready interrupt.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::{call_psx, irq};
use crate::disc::{self, Disc, RawSector};

// libcd command codes.
const CDL_NOP: u8 = 0x01;
const CDL_SETLOC: u8 = 0x02;
const CDL_PLAY: u8 = 0x03;
const CDL_READ_N: u8 = 0x06;
const CDL_STANDBY: u8 = 0x07;
const CDL_STOP: u8 = 0x08;
const CDL_PAUSE: u8 = 0x09;
const CDL_INIT: u8 = 0x0A;
const CDL_SETFILTER: u8 = 0x0D;
const CDL_SETMODE: u8 = 0x0E;
const CDL_GETPARAM: u8 = 0x0F;
const CDL_GETLOC_L: u8 = 0x10;
const CDL_GETLOC_P: u8 = 0x11;
const CDL_GET_TN: u8 = 0x13;
const CDL_GET_TD: u8 = 0x14;
const CDL_SEEK_L: u8 = 0x15;
const CDL_SEEK_P: u8 = 0x16;
const CDL_READ_S: u8 = 0x1B;

// Interrupt / sync status.
const CDL_NO_INTR: u8 = 0x00;
const CDL_DATA_READY: u8 = 0x01;
const CDL_COMPLETE: u8 = 0x02;
const CDL_DATA_END: u8 = 0x04;

// Drive status bits.
const STAT_STANDBY: u8 = 0x02; // spindle motor on
const STAT_SHELL_OPEN: u8 = 0x10; // lid open, or opened since the last Getstat
const STAT_READ: u8 = 0x20;
const STAT_PLAY: u8 = 0x80;

// Mode bits.
const MODE_DA: u8 = 0x01;
const MODE_SF: u8 = 0x08;
const MODE_SIZE0: u8 = 0x10;
const MODE_SIZE1: u8 = 0x20;
const MODE_RT: u8 = 0x40;
const MODE_SPEED: u8 = 0x80;

const CDL_CDROM_FORMAT: i32 = 2;

/// Time from a read command to its first sector.
const SEEK_TIME: Duration = Duration::from_millis(20);
/// Completion time of Pause / Stop / Seek / Init.
const SLOW_COMMAND_TIME: Duration = Duration::from_millis(5);

/// How long the lid stays open during a disc change, and how long the disc
/// then takes to spin up.
const LID_OPEN_TIME: Duration = Duration::from_millis(1000);
const SPIN_UP_TIME: Duration = Duration::from_millis(500);
/// Consecutive Getstat (CdlNop) polls with the motor stopped after which
/// the game is taken to be waiting for the lid to open (a disc change), and
/// the quiet time after a change before that can trigger again.
const AUTO_SWAP_POLLS: u32 = 50;
const AUTO_SWAP_COOLDOWN: Duration = Duration::from_secs(3);

/// Set by the window (F10) to change discs, as opening the lid would.
pub static SWAP_REQUEST: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The lid during a disc change.
#[derive(Clone, Copy, PartialEq)]
enum Lid {
    Closed,
    /// Open until `closes_at`; disc `next` goes in.
    Open {
        closes_at: Instant,
        next: usize,
    },
    /// Closed, disc spinning up until the instant.
    SpinningUp(Instant),
}

struct Drive {
    discs: Vec<Box<dyn Disc>>,
    current: usize,
    mode: u8,
    stat: u8,
    setloc: Option<u32>,
    /// Next sector the read head delivers.
    pos: u32,
    /// Incremented to cancel scheduled sector / completion events.
    read_gen: u64,
    /// The read started by CdRead2, whose sectors go to the stream ring.
    /// Any later read or pause ends streaming, as on the console where it
    /// replaces libcd's streaming callback.
    stream_gen: Option<u64>,
    cmd_gen: u64,
    sector: Box<RawSector>,
    read_off: usize,
    last_header: [u8; 8],
    filter: (u8, u8),
    xa: super::spu::xa::XaDecoder,
    sync_status: u8,
    sync_result: [u8; 8],
    ready_status: u8,
    ready_result: [u8; 8],
    sync_cb: u32,
    ready_cb: u32,
    lid: Lid,
    /// STAT_SHELL_OPEN stays set after the lid closes until a Getstat.
    shell_latch: bool,
    nop_polls: u32,
    last_change: Option<Instant>,
    /// Mods: each disc's replaced files and file starts, and the replaced
    /// file being read, by its first sector (see mods::files).
    overrides: Vec<(crate::mods::files::Overrides, crate::mods::files::Starts)>,
    redirect: Option<(u32, std::sync::Arc<Vec<u8>>)>,
}

static DRIVE: Mutex<Option<Drive>> = Mutex::new(None);

pub fn start(mut discs: Vec<Box<dyn Disc>>) {
    let overrides = discs.iter_mut().enumerate().map(|(n, d)| crate::mods::files::overrides(d.as_mut(), n)).collect();
    *DRIVE.lock().unwrap() = Some(Drive {
        discs,
        current: 0,
        mode: 0,
        stat: STAT_STANDBY,
        setloc: None,
        pos: 0,
        read_gen: 0,
        stream_gen: None,
        cmd_gen: 0,
        sector: Box::new([0; disc::RAW_SECTOR_SIZE]),
        read_off: 0,
        last_header: [0; 8],
        filter: (0, 0),
        xa: Default::default(),
        sync_status: CDL_COMPLETE,
        sync_result: [0; 8],
        ready_status: CDL_NO_INTR,
        ready_result: [0; 8],
        sync_cb: 0,
        ready_cb: 0,
        lid: Lid::Closed,
        shell_latch: false,
        nop_polls: 0,
        last_change: None,
        overrides,
        redirect: None,
    });
}

fn with_drive<R>(f: impl FnOnce(&mut Drive) -> R) -> R {
    let mut d = DRIVE.lock().unwrap();
    f(d.as_mut().expect("cdrom not started"))
}

fn bcd(v: u8) -> u8 {
    ((v / 10) << 4) | (v % 10)
}

fn from_bcd(b: u8) -> u8 {
    (b >> 4) * 10 + (b & 0xF)
}

/// Calls a libcd callback `void (*)(u_char intr, u_char *result)`.
fn call_cb(cb: u32, intr: u8, result: [u8; 8]) {
    if cb == 0 {
        return;
    }
    static mut CB_RESULT: [u8; 8] = [0; 8];
    // SAFETY: only the IRQ thread calls callbacks, one at a time.
    let ptr = unsafe {
        CB_RESULT = result;
        std::ptr::addr_of_mut!(CB_RESULT) as u32
    };
    call_psx(cb, &[u32::from(intr), ptr]);
    // The game decompresses code packages inside its CD callbacks and may
    // call into one as soon as the interrupt returns, so recognize newly
    // completed images right away.
    super::scan_images();
}

impl Drive {
    fn sector_period(&self) -> Duration {
        if self.mode & MODE_SPEED != 0 { Duration::from_micros(6_667) } else { Duration::from_micros(13_333) }
    }

    fn track_of(&self, lba: u32) -> (u8, u32) {
        let tracks = self.discs[self.current].tracks();
        tracks.iter().rev().find(|t| lba >= t.start_lba).map_or((1, 0), |t| (t.number, t.start_lba))
    }

    /// Starts a completion for the current command: immediate status, and an
    /// interrupt (sync callback) after `delay`.
    fn complete_after(&mut self, delay: Duration, result: [u8; 8]) {
        self.cmd_gen += 1;
        let generation = self.cmd_gen;
        // The interrupt is always raised: games often install the sync
        // callback right after issuing the command (CdControlF +
        // CdSyncCallback), and it is read when the interrupt is delivered.
        if delay.is_zero() {
            self.sync_status = CDL_COMPLETE;
            self.sync_result = result;
        } else {
            self.sync_status = CDL_NO_INTR;
        }
        irq::schedule(
            Instant::now() + delay.max(Duration::from_micros(100)),
            Box::new(move || {
                let cb = with_drive(|d| {
                    if d.cmd_gen != generation {
                        return None;
                    }
                    d.sync_status = CDL_COMPLETE;
                    d.sync_result = result;
                    Some(d.sync_cb)
                });
                if let Some(cb) = cb {
                    call_cb(cb, CDL_COMPLETE, result);
                }
            }),
        );
    }

    fn start_reading(&mut self) {
        let target = self.setloc.take().unwrap_or(self.pos);
        self.retarget(target);
        self.pos = target;
        self.stat = (self.stat | STAT_STANDBY | STAT_READ) & !STAT_PLAY;
        self.read_gen += 1;
        self.ready_status = CDL_NO_INTR;
        schedule_sector(self.read_gen, Instant::now() + SEEK_TIME);
        super::spu::clear_cd_audio();
    }

    fn stop_reading(&mut self) {
        self.read_gen += 1;
        self.stat &= !(STAT_READ | STAT_PLAY);
        // XA stops sounding when the drive stops.
        super::spu::clear_cd_audio();
    }

    /// Opens the lid: the drive stops, and after LID_OPEN_TIME the next disc
    /// is in and spins up.
    fn open_lid(&mut self) {
        if self.discs.len() < 2 || self.lid != Lid::Closed {
            return;
        }
        self.stop_reading();
        self.stream_gen = None;
        self.stat &= !STAT_STANDBY;
        self.shell_latch = true;
        let next = (self.current + 1) % self.discs.len();
        self.lid = Lid::Open { closes_at: Instant::now() + LID_OPEN_TIME, next };
        eprintln!("pe2: disc lid opened");
    }

    /// Advances the lid through a disc change.
    fn update_lid(&mut self) {
        if SWAP_REQUEST.swap(false, std::sync::atomic::Ordering::SeqCst) {
            self.open_lid();
        }
        let now = Instant::now();
        match self.lid {
            Lid::Open { closes_at, next } if now >= closes_at => {
                self.current = next;
                self.redirect = None;
                self.pos = 0;
                self.lid = Lid::SpinningUp(now + SPIN_UP_TIME);
                eprintln!("pe2: disc image {} inserted, lid closed", next + 1);
            }
            Lid::SpinningUp(at) if now >= at => {
                self.stat |= STAT_STANDBY;
                self.lid = Lid::Closed;
                self.last_change = Some(now);
            }
            _ => {}
        }
        let shell = self.shell_latch || matches!(self.lid, Lid::Open { .. });
        self.stat = if shell { self.stat | STAT_SHELL_OPEN } else { self.stat & !STAT_SHELL_OPEN };
    }

    /// A game that stopped the motor and polls Getstat over and over waits
    /// for the lid to open: change discs for it.
    fn watch_for_disc_change(&mut self, com: u8) {
        if com != CDL_NOP {
            self.nop_polls = 0;
            return;
        }
        self.nop_polls += 1;
        let quiet = self.last_change.is_none_or(|t| t.elapsed() >= AUTO_SWAP_COOLDOWN);
        if self.nop_polls >= AUTO_SWAP_POLLS && self.stat & STAT_STANDBY == 0 && self.lid == Lid::Closed && quiet {
            self.nop_polls = 0;
            self.open_lid();
        }
    }

    /// Runs `com`; returns its first-response result.
    fn command(&mut self, com: u8, param: Option<&[u8]>) -> [u8; 8] {
        self.update_lid();
        self.watch_for_disc_change(com);
        self.update_lid();
        if super::trace("cd") {
            eprintln!(
                "[cd {:8.3}] cmd 0x{com:02X} param {param:02X?} mode 0x{:02X} pos {}",
                super::spu::stamp(),
                self.mode,
                self.pos
            );
        }
        let mut r = [0u8; 8];
        let param_byte = |i: usize| param.and_then(|p| p.get(i).copied()).unwrap_or(0);
        match com {
            CDL_NOP => {
                // Getstat reports the shell-open bit, then clears it if the
                // lid has closed since.
                let r0 = self.stat;
                if !matches!(self.lid, Lid::Open { .. }) {
                    self.shell_latch = false;
                    self.stat &= !STAT_SHELL_OPEN;
                }
                r[0] = r0;
                self.complete_after(Duration::ZERO, r);
                return r;
            }
            CDL_SETLOC => {
                if let Some(p) = param {
                    self.setloc = Some(disc::bcd_msf_to_lba([p[0], p[1], p[2]]));
                }
            }
            CDL_SETMODE => self.mode = param_byte(0),
            CDL_SETFILTER => {
                let filter = (param_byte(0), param_byte(1));
                if filter != self.filter {
                    super::spu::clear_cd_audio();
                }
                self.filter = filter;
            }
            CDL_GETPARAM => {
                r[1] = self.mode;
                r[3] = self.filter.0;
                r[4] = self.filter.1;
            }
            CDL_GETLOC_L => {
                r = self.last_header;
                self.complete_after(Duration::ZERO, r);
                return r;
            }
            CDL_GETLOC_P => {
                let lba = self.pos.saturating_sub(1);
                let (track, start) = self.track_of(lba);
                let rel = disc::lba_to_bcd_msf((lba - start).wrapping_sub(disc::LBA_MSF_OFFSET));
                let abs = disc::lba_to_bcd_msf(lba);
                r = [bcd(track), 1, rel[0], rel[1], rel[2], abs[0], abs[1], abs[2]];
                self.complete_after(Duration::ZERO, r);
                return r;
            }
            CDL_GET_TN => {
                let tracks = self.discs[self.current].tracks();
                r[1] = bcd(tracks.first().map_or(1, |t| t.number));
                r[2] = bcd(tracks.last().map_or(1, |t| t.number));
            }
            CDL_GET_TD => {
                let n = from_bcd(param_byte(0));
                let d = &self.discs[self.current];
                let lba = if n == 0 {
                    d.total_sectors()
                } else {
                    d.tracks().iter().find(|t| t.number == n).map_or(0, |t| t.start_lba)
                };
                let msf = disc::lba_to_bcd_msf(lba);
                r[1] = msf[0];
                r[2] = msf[1];
            }
            CDL_READ_N | CDL_READ_S => {
                if let Some(p) = param {
                    self.setloc = Some(disc::bcd_msf_to_lba([p[0], p[1], p[2]]));
                }
                self.start_reading();
            }
            CDL_PAUSE | CDL_STOP | CDL_STANDBY | CDL_INIT => {
                self.stop_reading();
                match com {
                    CDL_STOP => self.stat &= !STAT_STANDBY,
                    CDL_INIT => self.mode = 0,
                    _ => self.stat |= STAT_STANDBY,
                }
                r[0] = self.stat;
                self.complete_after(SLOW_COMMAND_TIME, r);
                return r;
            }
            CDL_SEEK_L | CDL_SEEK_P => {
                self.stop_reading();
                let target = self.setloc.take().unwrap_or(self.pos);
                self.retarget(target);
                self.pos = target;
                r[0] = self.stat;
                self.complete_after(SEEK_TIME, r);
                return r;
            }
            CDL_PLAY => {
                // CD-DA playback is not implemented yet (phase 3).
                eprintln!("[cdrom] CdlPlay (CD-DA) not implemented");
                self.stop_reading();
                self.stat |= STAT_PLAY;
            }
            other => eprintln!("[cdrom] unhandled command 0x{other:02X}"),
        }
        r[0] = self.stat;
        self.complete_after(Duration::ZERO, r);
        r
    }

    /// Mods: a read or seek from `lba` (before `pos` moves there). The game
    /// reads a file from its first sector on, so one starting at a replaced
    /// file serves the replacement from there; a read resuming inside it
    /// (a retry, where the last read stopped) keeps it, and a read of
    /// anything else ends it.
    fn retarget(&mut self, lba: u32) {
        let Some((files, starts)) = self.overrides.get(self.current) else { return };
        if let Some((start, data)) = &self.redirect {
            let end = start + (data.len() / disc::SECTOR_SIZE) as u32;
            if (*start + 1..end).contains(&lba) && (lba == self.pos || !starts.contains_key(&lba)) {
                return;
            }
        }
        self.redirect = files.get(&lba).map(|data| (lba, data.clone()));
        if crate::runtime::trace("files")
            && let Some(name) = starts.get(&lba)
        {
            let from = if self.redirect.is_some() { "a mod" } else { "the disc" };
            eprintln!("[files] {name} (sector {lba}) from {from}");
        }
    }

    /// The replacement's sector `lba`, as a raw mode 2 form 1 data sector.
    fn replaced_sector(&self, lba: u32, raw: &mut disc::RawSector) -> bool {
        let Some((start, data)) = &self.redirect else { return false };
        let at = (lba.wrapping_sub(*start) as usize).wrapping_mul(disc::SECTOR_SIZE);
        let Some(src) = data.get(at..at + disc::SECTOR_SIZE) else { return false };
        let msf = disc::lba_to_bcd_msf(lba);
        raw[12..16].copy_from_slice(&[msf[0], msf[1], msf[2], 2]);
        raw[16..24].copy_from_slice(&[0, 0, 0x08, 0, 0, 0, 0x08, 0]);
        raw[24..24 + disc::SECTOR_SIZE].copy_from_slice(src);
        true
    }

    /// Bytes of the current sector the game reads with CdGetSector.
    fn data_area(&self) -> &[u8] {
        let s = &self.sector[..];
        if self.mode & MODE_SIZE1 != 0 {
            &s[12..12 + 2340]
        } else if self.mode & MODE_SIZE0 != 0 {
            &s[24..24 + 2328]
        } else {
            &s[24..24 + 2048]
        }
    }
}

fn schedule_sector(generation: u64, at: Instant) {
    irq::schedule(at, Box::new(move || deliver_sector(generation, at)));
}

fn deliver_sector(generation: u64, at: Instant) {
    let delivery = with_drive(|d| {
        if d.read_gen != generation {
            return None;
        }
        let lba = d.pos;
        let mut raw = [0u8; disc::RAW_SECTOR_SIZE];
        let replaced = d.replaced_sector(lba, &mut raw);
        if lba >= d.discs[d.current].total_sectors() && !replaced {
            d.stop_reading();
            d.ready_status = CDL_DATA_END;
            d.ready_result[0] = d.stat;
            return Some((d.ready_cb, CDL_DATA_END, d.ready_result));
        }
        if !replaced && let Err(e) = d.discs[d.current].read_sector(lba, &mut raw) {
            eprintln!("[cdrom] read error at LBA {lba}: {e}");
        }
        d.pos += 1;
        schedule_sector(generation, at + d.sector_period());

        // Mode 2 subheader: file, channel, submode, coding.
        let (file, chan, submode) = (raw[16], raw[17], raw[18]);
        let is_audio = submode & 0x04 != 0;
        if d.mode & MODE_RT != 0 && is_audio {
            // XA-ADPCM goes to the sound processor, not to the host. With
            // the filter on, only the selected file and channel play.
            if d.mode & MODE_SF == 0 || (file, chan) == d.filter {
                if super::trace("xa") {
                    eprintln!(
                        "[xa] disc {} lba {lba} file {file} chan {chan} submode 0x{submode:02X} coding 0x{:02X}",
                        d.current + 1,
                        raw[19]
                    );
                }
                let samples = d.xa.decode_sector(&raw);
                super::spu::push_cd_audio(samples);
            }
            return None;
        }
        if d.mode & MODE_DA != 0 && d.track_of(lba).0 > 1 {
            return None;
        }
        if d.stream_gen == Some(generation) && super::stream::active() {
            // CdRead2: the sector goes into the stream ring, not to the
            // game's ready callback.
            d.last_header.copy_from_slice(&raw[12..20]);
            super::stream::push_sector(&raw[24..24 + 2048], [raw[12], raw[13], raw[14]]);
            return None;
        }
        if super::trace("cdsector") {
            eprintln!("[cd] sector {lba} submode 0x{submode:02X} -> callback 0x{:08X}", d.ready_cb);
        }
        d.sector.copy_from_slice(&raw);
        d.read_off = 0;
        d.last_header.copy_from_slice(&raw[12..20]);
        d.ready_status = CDL_DATA_READY;
        d.ready_result = [d.stat, 0, 0, 0, 0, 0, 0, 0];
        Some((d.ready_cb, CDL_DATA_READY, d.ready_result))
    });
    if let Some((cb, intr, result)) = delivery {
        call_cb(cb, intr, result);
    }
}

fn param_slice<'a>(param: *const u8) -> Option<&'a [u8]> {
    // SAFETY: libcd parameters are at most 4 bytes (CdlLOC); a non-null
    // pointer from the game points at least that far into valid memory.
    (!param.is_null()).then(|| unsafe { std::slice::from_raw_parts(param, 4) })
}

fn write_result(result: *mut u8, r: &[u8; 8]) {
    if !result.is_null() {
        // SAFETY: libcd result buffers are 8 bytes.
        unsafe { std::ptr::copy_nonoverlapping(r.as_ptr(), result, 8) };
    }
}

// ---- libcd entry points ---------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdInit() -> i32 {
    let _g = irq::guard();
    with_drive(|d| {
        d.stop_reading();
        d.mode = 0;
        d.stat = STAT_STANDBY;
        d.sync_status = CDL_COMPLETE;
        d.ready_status = CDL_NO_INTR;
        d.sync_cb = 0;
        d.ready_cb = 0;
    });
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdReset(_mode: i32) -> i32 {
    pcrt_CdInit()
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdFlush() {}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdControl(com: u8, param: *const u8, result: *mut u8) -> i32 {
    let _g = irq::guard();
    let r = with_drive(|d| d.command(com, param_slice(param)));
    write_result(result, &r);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdControlF(com: u8, param: *const u8) -> i32 {
    let _g = irq::guard();
    with_drive(|d| d.command(com, param_slice(param)));
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdControlB(com: u8, param: *const u8, result: *mut u8) -> i32 {
    let _g = irq::guard();
    let ack = with_drive(|d| d.command(com, param_slice(param)));
    irq::wait_until(|| with_drive(|d| d.sync_status != CDL_NO_INTR));
    let (status, r) = with_drive(|d| (d.sync_status, d.sync_result));
    write_result(result, if status == CDL_COMPLETE { &r } else { &ack });
    i32::from(status == CDL_COMPLETE)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdSync(mode: i32, result: *mut u8) -> i32 {
    let _g = irq::guard();
    if mode == 0 {
        irq::wait_until(|| with_drive(|d| d.sync_status != CDL_NO_INTR));
    }
    let (status, r) = with_drive(|d| (d.sync_status, d.sync_result));
    write_result(result, &r);
    i32::from(status)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdReady(mode: i32, result: *mut u8) -> i32 {
    let _g = irq::guard();
    if mode == 0 {
        irq::wait_until(|| with_drive(|d| d.ready_status != CDL_NO_INTR));
    }
    let (status, r) = with_drive(|d| (d.ready_status, d.ready_result));
    write_result(result, &r);
    i32::from(status)
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdSyncCallback(func: u32) -> u32 {
    let _g = irq::guard();
    with_drive(|d| std::mem::replace(&mut d.sync_cb, func))
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdReadyCallback(func: u32) -> u32 {
    let _g = irq::guard();
    with_drive(|d| std::mem::replace(&mut d.ready_cb, func))
}

/// CdGetSector(madr, size): copies `size` words of the current sector,
/// continuing where the previous call stopped.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdGetSector(madr: *mut u8, size: i32) -> i32 {
    let _g = irq::guard();
    with_drive(|d| {
        let area_len = d.data_area().len();
        let start = d.read_off.min(area_len);
        let n = (size.max(0) as usize * 4).min(area_len - start);
        // SAFETY: the game passes a buffer of at least `size` words.
        unsafe { std::ptr::copy_nonoverlapping(d.data_area()[start..].as_ptr(), madr, n) };
        d.read_off = start + n;
    });
    1
}

/// CdRead2(mode): sets the mode and streams sectors from the current
/// position into the St* ring (see stream.rs).
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdRead2(mode: u32) -> i32 {
    let _g = irq::guard();
    super::stream::set_active(true);
    with_drive(|d| {
        d.mode = mode as u8;
        d.start_reading();
        d.stream_gen = Some(d.read_gen);
    });
    if super::trace("cd") {
        eprintln!("[cd {:8.3}] CdRead2 mode 0x{mode:X}", super::spu::stamp());
    }
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdIntToPos(i: i32, p: *mut u8) -> *mut u8 {
    let msf = disc::lba_to_bcd_msf(i.max(0) as u32);
    // SAFETY: `p` is a CdlLOC (4 bytes).
    unsafe {
        std::ptr::copy_nonoverlapping(msf.as_ptr(), p, 3);
        *p.add(3) = 0;
    }
    p
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdPosToInt(p: *const u8) -> i32 {
    // SAFETY: `p` is a CdlLOC.
    let msf = unsafe { [*p, *p.add(1), *p.add(2)] };
    disc::bcd_msf_to_lba(msf) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdStatus() -> i32 {
    let _g = irq::guard();
    with_drive(|d| {
        d.update_lid();
        i32::from(d.stat)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdMode() -> i32 {
    let _g = irq::guard();
    with_drive(|d| i32::from(d.mode))
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdDiskReady(_mode: i32) -> i32 {
    let _g = irq::guard();
    let ready = with_drive(|d| {
        d.update_lid();
        d.lid == Lid::Closed
    });
    i32::from(if ready { CDL_COMPLETE } else { CDL_NO_INTR })
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdGetDiskType() -> i32 {
    CDL_CDROM_FORMAT
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_CdMix(vol: *const u8) -> i32 {
    // SAFETY: the game passes a CdlATV (four volume bytes).
    let v = unsafe { [*vol, *vol.add(1), *vol.add(2), *vol.add(3)] };
    *super::spu::xa::MIX.lock().unwrap() = v;
    1
}
