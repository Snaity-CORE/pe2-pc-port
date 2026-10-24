//! A small MIPS R3000A interpreter for library code the game calls but that
//! has no C body: Psy-Q libraries (GTE math, GPU packet builders...) and the
//! hand-written assembly routines. Their original code is in PSX RAM, loaded
//! from the player's disc, so running it gives exactly the console's results.
//!
//! Functions that drive hardware get native implementations instead, except
//! thin layers over a device: their register accesses go to the device
//! models in `hw.rs` (which report registers they do not handle).
//!
//! Calls from interpreted code into game functions (addresses holding a
//! native thunk) run the native body with a0-a3 as arguments.
//!
//! Interpreted code is interruptible, like game code on the console; hardware
//! register accesses and logging hold interrupts off while they run.

/// eprintln! while holding off interrupts: interpreted code runs
/// interruptibly, and stderr has a lock an interrupt handler may also take.
macro_rules! log {
    ($($t:tt)*) => {{
        let _g = super::irq::guard();
        eprintln!($($t)*);
    }};
}

unsafe extern "C" {
    fn pcgte_read_data(reg: u32) -> u32;
    fn pcgte_write_data(reg: u32, value: u32);
    fn pcgte_read_ctrl(reg: u32) -> u32;
    fn pcgte_write_ctrl(reg: u32, value: u32);
    fn pcgte_op(instruction: u32);
    fn pc_thunk_at(addr: u32) -> i32;
    fn pc_code_word(addr: u32) -> u32;
}

/// Return address planted in `ra`: reaching it ends the call.
const RETURN_MAGIC: u32 = 0xFFFF_FFF0;
const RAM_BASE: u32 = 0x8000_0000;
const RAM_SIZE: u32 = 0x0020_0000;
const IO_START: u32 = 0x1F80_1000;
const IO_END: u32 = 0x1F80_3000;
const STACK_WORDS: usize = 16 * 1024;

/// PSX address of the instruction the interpreter is executing (for the
/// write watchpoints in watch.rs; per-process, last writer wins).
pub static CUR_PC: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

struct Cpu {
    r: [u32; 32],
    hi: u32,
    lo: u32,
    pc: u32,
    next_pc: u32,
    /// Load scheduled by the previous instruction (register, value).
    pending_load: (usize, u32),
    next_load: (usize, u32),
    entry: u32,
}

fn is_ram(addr: u32) -> bool {
    (RAM_BASE..RAM_BASE + RAM_SIZE).contains(&addr)
}

/// Host pointer for a PSX address. Most addresses are identity-mapped;
/// KSEG1 and the unmappable low 64 KiB go to the KSEG0 RAM view.
fn host(addr: u32) -> Option<*mut u8> {
    match addr {
        0..0x1_0000 => Some((RAM_BASE + addr) as *mut u8),
        0xA000_0000..0xA020_0000 => Some((addr - 0x2000_0000) as *mut u8),
        IO_START..IO_END => None,
        _ => Some(addr as *mut u8),
    }
}

impl Cpu {
    fn read8(&self, addr: u32) -> u32 {
        match host(addr) {
            // SAFETY: game code computes addresses into mapped memory.
            Some(p) => unsafe { u32::from(p.read()) },
            None => (super::hw::read32(addr & !3) >> ((addr & 3) * 8)) & 0xFF,
        }
    }
    fn read16(&self, addr: u32) -> u32 {
        match host(addr) {
            // SAFETY: as in read8.
            Some(p) => unsafe { u32::from(p.cast::<u16>().read_unaligned()) },
            None => (super::hw::read32(addr & !3) >> ((addr & 2) * 8)) & 0xFFFF,
        }
    }
    fn read32(&self, addr: u32) -> u32 {
        match host(addr) {
            // SAFETY: as in read8.
            Some(p) => unsafe { p.cast::<u32>().read_unaligned() },
            None => super::hw::read32(addr),
        }
    }
    fn write8(&self, addr: u32, v: u32) {
        match host(addr) {
            // SAFETY: as in read8.
            Some(p) => unsafe { p.write(v as u8) },
            None => self.io_partial(addr, v & 0xFF, 0xFF),
        }
    }
    fn write16(&self, addr: u32, v: u32) {
        match host(addr) {
            // SAFETY: as in read8.
            Some(p) => unsafe { p.cast::<u16>().write_unaligned(v as u16) },
            None => self.io_partial(addr, v & 0xFFFF, 0xFFFF),
        }
    }
    fn write32(&self, addr: u32, v: u32) {
        match host(addr) {
            // SAFETY: as in read8.
            Some(p) => unsafe { p.cast::<u32>().write_unaligned(v) },
            None => super::hw::write32(addr, v),
        }
    }

    /// A narrow write to a hardware register, as a read-modify-write.
    fn io_partial(&self, addr: u32, v: u32, mask: u32) {
        let shift = (addr & 3) * 8;
        let word = super::hw::read32(addr & !3);
        super::hw::write32(addr & !3, (word & !(mask << shift)) | (v << shift));
    }

    fn set(&mut self, reg: usize, v: u32) {
        if reg != 0 {
            self.r[reg] = v;
            if self.pending_load.0 == reg {
                self.pending_load.0 = 0; // the instruction's write wins
            }
        }
    }

    fn load(&mut self, reg: usize, v: u32) {
        self.next_load = (reg, v);
    }

    fn branch(&mut self, target: u32) {
        self.next_pc = target;
    }

    fn run(&mut self) {
        // (Counted for PE2_LIBTEST's cost reports: see `executed`.)
        while self.pc != RETURN_MAGIC {
            if matches!(self.pc, 0xA0 | 0xB0 | 0xC0) {
                // BIOS function table call: function number in t1.
                log!(
                    "[mips] function 0x{:08X} calls BIOS {:02X}:{:02X} (not available, returns 0)",
                    self.entry,
                    self.pc,
                    self.r[9]
                );
                self.r[2] = 0;
                self.pc = self.r[31];
                self.next_pc = self.pc.wrapping_add(4);
                continue;
            }
            // SAFETY: plain query of the thunk bitmap.
            if is_ram(self.pc) && unsafe { pc_thunk_at(self.pc) } != 0 {
                self.call_native();
                continue;
            }
            // Library routines keep a thunk over their first two words;
            // the original instructions come from pc_code_word.
            // SAFETY: plain lookup into PSX RAM / the saved words.
            CUR_PC.store(self.pc, std::sync::atomic::Ordering::Relaxed);
            let insn = if is_ram(self.pc) { unsafe { pc_code_word(self.pc) } } else { self.read32(self.pc) };
            self.pending_load = std::mem::take(&mut self.next_load);
            self.pc = self.next_pc;
            self.next_pc = self.pc.wrapping_add(4);
            INSNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if matches!(insn >> 26, 0x20..=0x26) {
                // Main RAM only: the scratchpad answers in a cycle.
                let addr = self.r[((insn >> 21) & 31) as usize].wrapping_add(insn as i16 as i32 as u32);
                if addr & 0x1FFF_FC00 != 0x1F80_0000 {
                    LOADS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            self.execute(insn);
            let (reg, v) = self.pending_load;
            if reg != 0 {
                self.r[reg] = v;
            }
        }
        // A load in the last instruction still lands.
        let (reg, v) = std::mem::take(&mut self.next_load);
        if reg != 0 {
            self.r[reg] = v;
        }
    }

    /// The current pc is a game function with a native body: call it like a
    /// `jal` would (arguments in a0-a3 and on the MIPS stack), then return
    /// to `ra`.
    fn call_native(&mut self) {
        type Native = extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
        let sp = self.r[29];
        let stack_arg = |i: u32| self.read32(sp.wrapping_add(16 + 4 * i));
        let args = [self.r[4], self.r[5], self.r[6], self.r[7], stack_arg(0), stack_arg(1), stack_arg(2), stack_arg(3)];
        // SAFETY: a thunk at this address jumps to the native body.
        let f: Native = unsafe { std::mem::transmute(self.pc as usize) };
        let ret = f(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
        self.r[2] = ret;
        self.pc = self.r[31];
        self.next_pc = self.pc.wrapping_add(4);
    }

    fn execute(&mut self, i: u32) {
        let op = i >> 26;
        let rs = ((i >> 21) & 31) as usize;
        let rt = ((i >> 16) & 31) as usize;
        let rd = ((i >> 11) & 31) as usize;
        let sa = (i >> 6) & 31;
        let imm = i & 0xFFFF;
        let simm = i32::from(i as u16 as i16) as u32;
        let s = self.r[rs];
        let t = self.r[rt];
        let addr = s.wrapping_add(simm);
        let br_target = self.pc.wrapping_add(simm << 2);
        match op {
            0x00 => match i & 0x3F {
                0x00 => self.set(rd, t << sa),
                0x02 => self.set(rd, t >> sa),
                0x03 => self.set(rd, ((t as i32) >> sa) as u32),
                0x04 => self.set(rd, t << (s & 31)),
                0x06 => self.set(rd, t >> (s & 31)),
                0x07 => self.set(rd, ((t as i32) >> (s & 31)) as u32),
                0x08 => self.branch(s),
                0x09 => {
                    let ret = self.next_pc;
                    self.set(rd, ret);
                    self.branch(s);
                }
                0x0C | 0x0D => {
                    log!("[mips] syscall/break in 0x{:08X} at 0x{:08X}", self.entry, self.pc.wrapping_sub(4))
                }
                0x10 => self.set(rd, self.hi),
                0x11 => self.hi = s,
                0x12 => self.set(rd, self.lo),
                0x13 => self.lo = s,
                0x18 => {
                    let p = i64::from(s as i32) * i64::from(t as i32);
                    (self.lo, self.hi) = (p as u32, (p >> 32) as u32);
                }
                0x19 => {
                    let p = u64::from(s) * u64::from(t);
                    (self.lo, self.hi) = (p as u32, (p >> 32) as u32);
                }
                0x1A => {
                    let (n, d) = (s as i32, t as i32);
                    (self.lo, self.hi) = if d == 0 {
                        (if n >= 0 { 0xFFFF_FFFF } else { 1 }, n as u32)
                    } else if n == i32::MIN && d == -1 {
                        (0x8000_0000, 0)
                    } else {
                        ((n / d) as u32, (n % d) as u32)
                    };
                }
                0x1B => {
                    (self.lo, self.hi) = match (s.checked_div(t), s.checked_rem(t)) {
                        (Some(q), Some(r)) => (q, r),
                        // Division by zero: what the R3000 leaves.
                        _ => (0xFFFF_FFFF, s),
                    };
                }
                0x20 | 0x21 => self.set(rd, s.wrapping_add(t)),
                0x22 | 0x23 => self.set(rd, s.wrapping_sub(t)),
                0x24 => self.set(rd, s & t),
                0x25 => self.set(rd, s | t),
                0x26 => self.set(rd, s ^ t),
                0x27 => self.set(rd, !(s | t)),
                0x2A => self.set(rd, u32::from((s as i32) < (t as i32))),
                0x2B => self.set(rd, u32::from(s < t)),
                f => log!("[mips] unknown SPECIAL 0x{f:02X} in 0x{:08X}", self.entry),
            },
            0x01 => {
                let link = rt & 0x1E == 0x10;
                let taken = if rt & 1 != 0 { (s as i32) >= 0 } else { (s as i32) < 0 };
                if link {
                    let ret = self.next_pc;
                    self.set(31, ret);
                }
                if taken {
                    self.branch(br_target);
                }
            }
            0x02 => self.branch((self.pc & 0xF000_0000) | ((i & 0x03FF_FFFF) << 2)),
            0x03 => {
                let ret = self.next_pc;
                self.set(31, ret);
                self.branch((self.pc & 0xF000_0000) | ((i & 0x03FF_FFFF) << 2));
            }
            0x04 => {
                if s == t {
                    self.branch(br_target)
                }
            }
            0x05 => {
                if s != t {
                    self.branch(br_target)
                }
            }
            0x06 => {
                if (s as i32) <= 0 {
                    self.branch(br_target)
                }
            }
            0x07 => {
                if (s as i32) > 0 {
                    self.branch(br_target)
                }
            }
            0x08 | 0x09 => self.set(rt, s.wrapping_add(simm)),
            0x0A => self.set(rt, u32::from((s as i32) < (simm as i32))),
            0x0B => self.set(rt, u32::from(s < simm)),
            0x0C => self.set(rt, s & imm),
            0x0D => self.set(rt, s | imm),
            0x0E => self.set(rt, s ^ imm),
            0x0F => self.set(rt, imm << 16),
            0x10 => match rs {
                0x00 => self.load(rt, 0), // mfc0: report a plain status
                0x04 | 0x10 => {}         // mtc0 / rfe: ignored
                _ => {}
            },
            0x12 => {
                if i & (1 << 25) != 0 {
                    // SAFETY: software GTE.
                    unsafe { pcgte_op(i) };
                } else {
                    match rs {
                        // SAFETY (all four): software GTE register access.
                        0x00 => {
                            let v = unsafe { pcgte_read_data(rd as u32) };
                            self.load(rt, v)
                        }
                        0x02 => {
                            let v = unsafe { pcgte_read_ctrl(rd as u32) };
                            self.load(rt, v)
                        }
                        0x04 => unsafe { pcgte_write_data(rd as u32, t) },
                        0x06 => unsafe { pcgte_write_ctrl(rd as u32, t) },
                        _ => log!("[mips] unknown COP2 op in 0x{:08X}", self.entry),
                    }
                }
            }
            0x20 => {
                let v = self.read8(addr) as u8 as i8 as i32 as u32;
                self.load(rt, v)
            }
            0x21 => {
                let v = self.read16(addr) as u16 as i16 as i32 as u32;
                self.load(rt, v)
            }
            0x22 => {
                // lwl: merge into the value the register will have.
                let cur = if self.pending_load.0 == rt && rt != 0 { self.pending_load.1 } else { t };
                let w = self.read32(addr & !3);
                let v = match addr & 3 {
                    0 => (cur & 0x00FF_FFFF) | (w << 24),
                    1 => (cur & 0x0000_FFFF) | (w << 16),
                    2 => (cur & 0x0000_00FF) | (w << 8),
                    _ => w,
                };
                self.load(rt, v);
            }
            0x23 => {
                let v = self.read32(addr);
                self.load(rt, v)
            }
            0x24 => {
                let v = self.read8(addr);
                self.load(rt, v)
            }
            0x25 => {
                let v = self.read16(addr);
                self.load(rt, v)
            }
            0x26 => {
                let cur = if self.pending_load.0 == rt && rt != 0 { self.pending_load.1 } else { t };
                let w = self.read32(addr & !3);
                let v = match addr & 3 {
                    0 => w,
                    1 => (cur & 0xFF00_0000) | (w >> 8),
                    2 => (cur & 0xFFFF_0000) | (w >> 16),
                    _ => (cur & 0xFFFF_FF00) | (w >> 24),
                };
                self.load(rt, v);
            }
            0x28 => self.write8(addr, t),
            0x29 => self.write16(addr, t),
            0x2A => {
                let a = addr & !3;
                let w = self.read32(a);
                let v = match addr & 3 {
                    0 => (w & 0xFFFF_FF00) | (t >> 24),
                    1 => (w & 0xFFFF_0000) | (t >> 16),
                    2 => (w & 0xFF00_0000) | (t >> 8),
                    _ => t,
                };
                self.write32(a, v);
            }
            0x2B => self.write32(addr, t),
            0x2E => {
                let a = addr & !3;
                let w = self.read32(a);
                let v = match addr & 3 {
                    0 => t,
                    1 => (w & 0x0000_00FF) | (t << 8),
                    2 => (w & 0x0000_FFFF) | (t << 16),
                    _ => (w & 0x00FF_FFFF) | (t << 24),
                };
                self.write32(a, v);
            }
            // SAFETY (both): software GTE register access.
            0x32 => {
                let v = self.read32(addr);
                unsafe { pcgte_write_data(rt as u32, v) }
            }
            0x3A => {
                let v = unsafe { pcgte_read_data(rt as u32) };
                self.write32(addr, v)
            }
            other => {
                log!("[mips] unknown opcode 0x{other:02X} in 0x{:08X} at 0x{:08X}", self.entry, self.pc.wrapping_sub(4))
            }
        }
    }
}

/// Runs the MIPS function at `addr` with up to eight word arguments (o32:
/// four in registers, the rest on the stack) and returns v0.
///
/// # Safety
/// `args` must point at eight readable words; `addr` must be MIPS code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pc_mips_call(addr: u32, args: *const u32) -> u32 {
    // SAFETY: the caller passes eight words (unused ones are its own frame).
    let args = unsafe { std::slice::from_raw_parts(args, 8) };
    if super::trace("interp") {
        count_entry(addr);
    }
    let mut stack = {
        let _g = super::irq::guard();
        vec![0u32; STACK_WORDS]
    };
    let sp_index = STACK_WORDS - 64;
    stack[sp_index..sp_index + 8].copy_from_slice(args);
    let sp = stack[sp_index..].as_ptr() as u32;

    let mut cpu = Cpu {
        r: [0; 32],
        hi: 0,
        lo: 0,
        pc: addr,
        next_pc: addr.wrapping_add(4),
        pending_load: (0, 0),
        next_load: (0, 0),
        entry: addr,
    };
    cpu.r[4..8].copy_from_slice(&args[..4]);
    cpu.r[29] = sp;
    cpu.r[31] = RETURN_MAGIC;
    cpu.run();
    {
        let _g = super::irq::guard();
        drop(stack);
    }
    cpu.r[2]
}

/// PE2_TRACE=interp: how often each function is entered through the
/// interpreter, printed every 20 seconds (most called first).
fn count_entry(addr: u32) {
    use std::collections::HashMap;
    use std::sync::{Mutex, Once};
    static COUNTS: Mutex<Option<HashMap<u32, u64>>> = Mutex::new(None);
    static REPORTER: Once = Once::new();
    let _g = super::irq::guard();
    *COUNTS.lock().unwrap().get_or_insert_with(HashMap::new).entry(addr).or_insert(0) += 1;
    REPORTER.call_once(|| {
        std::thread::spawn(|| {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(20));
                let mut v: Vec<(u32, u64)> = COUNTS
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map(|m| m.iter().map(|(&a, &n)| (a, n)).collect())
                    .unwrap_or_default();
                v.sort_by_key(|e| std::cmp::Reverse(e.1));
                let line: Vec<String> = v.iter().map(|(a, n)| format!("{a:08X}:{n}")).collect();
                eprintln!("[interp] {}", line.join(" "));
            }
        });
    });
}

/// Instructions the interpreter has run, and how many of them were loads from main RAM
/// (PE2_LIBTEST=vlcfile reports what the original library code costs).
static INSNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static LOADS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Instructions and loads run so far (for the C side's tests).
#[unsafe(no_mangle)]
pub extern "C" fn pc_mips_executed(loads: *mut u64) -> u64 {
    // SAFETY: the caller passes a writable u64.
    unsafe { *loads = LOADS.load(std::sync::atomic::Ordering::Relaxed) };
    INSNS.load(std::sync::atomic::Ordering::Relaxed)
}
