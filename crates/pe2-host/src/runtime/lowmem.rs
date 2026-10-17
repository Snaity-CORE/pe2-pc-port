//! Accesses to the first 64 KiB of the address space.
//!
//! On the PlayStation, addresses from 0 mirror RAM (the kernel's area), so a
//! read through a null or near-null pointer quietly returns kernel data. The
//! game has such latent bugs (e.g. `CdCmd_Enqueue(cmd, NULL, NULL)` reads
//! `paramA[3]`). Windows never maps that range, so the access faults. This
//! handler emulates the faulting instruction against the RAM mirror at
//! 0x80000000 and resumes after it.

use iced_x86::{Code, Decoder, DecoderOptions, Instruction, MemorySize, OpKind, Register};
use windows_sys::Win32::Foundation::{EXCEPTION_ACCESS_VIOLATION, NTSTATUS};
use windows_sys::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, CONTEXT, EXCEPTION_POINTERS,
};

const LOW_LIMIT: u32 = 0x1_0000;
const RAM_BASE: u32 = 0x8000_0000;
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;

pub fn install() {
    // SAFETY: registering a handler with the documented signature.
    unsafe { AddVectoredExceptionHandler(1, Some(handler)) };
}

unsafe extern "system" fn handler(info: *mut EXCEPTION_POINTERS) -> i32 {
    // SAFETY: the OS passes valid exception pointers.
    let (rec, ctx) = unsafe { (&*(*info).ExceptionRecord, &mut *(*info).ContextRecord) };
    if rec.ExceptionCode != EXCEPTION_ACCESS_VIOLATION as NTSTATUS || rec.NumberParameters < 2 {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let addr = rec.ExceptionInformation[1] as u32;
    if addr >= LOW_LIMIT {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    // SAFETY: EIP points at the faulting instruction in mapped code.
    let bytes = unsafe { std::slice::from_raw_parts(ctx.Eip as *const u8, 15) };
    let mut decoder = Decoder::with_ip(32, bytes, u64::from(ctx.Eip), DecoderOptions::NONE);
    let insn = decoder.decode();
    if insn.is_invalid() || !emulate(&insn, addr, ctx) {
        eprintln!("[lowmem] cannot emulate {:?} at 0x{:08X} (address 0x{addr:08X})", insn.code(), ctx.Eip);
        return EXCEPTION_CONTINUE_SEARCH;
    }
    // Expected (see above): reported only when asked, PE2_TRACE=lowmem.
    if super::trace("lowmem") {
        eprintln!("[lowmem] emulated {:?} at 0x{:08X} (PSX address 0x{addr:08X})", insn.code(), ctx.Eip);
    }
    ctx.Eip += insn.len() as u32;
    EXCEPTION_CONTINUE_EXECUTION
}

fn mirror(addr: u32) -> *mut u8 {
    (RAM_BASE + addr) as *mut u8
}

fn read_mem(addr: u32, size: MemorySize) -> Option<u32> {
    let p = mirror(addr);
    // SAFETY: the RAM mirror at 0x80000000 is mapped (2 MiB > 64 KiB).
    unsafe {
        Some(match size {
            MemorySize::UInt8 | MemorySize::Int8 => u32::from(p.read()),
            MemorySize::UInt16 | MemorySize::Int16 => u32::from(p.cast::<u16>().read_unaligned()),
            MemorySize::UInt32 | MemorySize::Int32 | MemorySize::DwordOffset => p.cast::<u32>().read_unaligned(),
            _ => return None,
        })
    }
}

fn write_mem(addr: u32, size: MemorySize, value: u32) -> bool {
    let p = mirror(addr);
    // SAFETY: as in read_mem.
    unsafe {
        match size {
            MemorySize::UInt8 | MemorySize::Int8 => p.write(value as u8),
            MemorySize::UInt16 | MemorySize::Int16 => p.cast::<u16>().write_unaligned(value as u16),
            MemorySize::UInt32 | MemorySize::Int32 => p.cast::<u32>().write_unaligned(value),
            _ => return false,
        }
    }
    true
}

fn reg_slot(ctx: &mut CONTEXT, full: Register) -> Option<&mut u32> {
    Some(match full {
        Register::EAX => &mut ctx.Eax,
        Register::ECX => &mut ctx.Ecx,
        Register::EDX => &mut ctx.Edx,
        Register::EBX => &mut ctx.Ebx,
        Register::ESP => &mut ctx.Esp,
        Register::EBP => &mut ctx.Ebp,
        Register::ESI => &mut ctx.Esi,
        Register::EDI => &mut ctx.Edi,
        _ => return None,
    })
}

fn get_reg(ctx: &mut CONTEXT, reg: Register) -> Option<u32> {
    let full = reg.full_register32();
    let v = *reg_slot(ctx, full)?;
    Some(match reg.size() {
        4 => v,
        2 => v & 0xFFFF,
        1 if is_high_byte(reg) => (v >> 8) & 0xFF,
        1 => v & 0xFF,
        _ => return None,
    })
}

fn set_reg(ctx: &mut CONTEXT, reg: Register, value: u32) -> bool {
    let size = reg.size();
    let high = is_high_byte(reg);
    let Some(slot) = reg_slot(ctx, reg.full_register32()) else { return false };
    *slot = match size {
        4 => value,
        2 => (*slot & 0xFFFF_0000) | (value & 0xFFFF),
        1 if high => (*slot & 0xFFFF_00FF) | ((value & 0xFF) << 8),
        1 => (*slot & 0xFFFF_FF00) | (value & 0xFF),
        _ => return false,
    };
    true
}

fn is_high_byte(reg: Register) -> bool {
    matches!(reg, Register::AH | Register::CH | Register::DH | Register::BH)
}

fn sign_extend(v: u32, size: MemorySize) -> u32 {
    match size {
        MemorySize::Int8 | MemorySize::UInt8 => v as u8 as i8 as i32 as u32,
        MemorySize::Int16 | MemorySize::UInt16 => v as u16 as i16 as i32 as u32,
        _ => v,
    }
}

const CF: u32 = 1 << 0;
const PF: u32 = 1 << 2;
const AF: u32 = 1 << 4;
const ZF: u32 = 1 << 6;
const SF: u32 = 1 << 7;
const OF: u32 = 1 << 11;

#[derive(Clone, Copy, PartialEq)]
enum Alu {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Inc,
    Dec,
    Neg,
    Not,
    Cmp,
    Test,
}

/// Computes `a op b` at `bits` width and the resulting EFLAGS.
fn alu(op: Alu, a: u32, b: u32, bits: u32, eflags: u32) -> (u32, u32) {
    let mask = if bits == 32 { u32::MAX } else { (1 << bits) - 1 };
    let sign = 1u32 << (bits - 1);
    let (a, b) = (a & mask, b & mask);
    let (res, mut flags) = match op {
        Alu::Add | Alu::Inc => {
            let b = if op == Alu::Inc { 1 } else { b };
            let r = a.wrapping_add(b) & mask;
            let mut f = 0;
            if op == Alu::Add && u64::from(a) + u64::from(b) > u64::from(mask) {
                f |= CF;
            }
            if (!(a ^ b) & (a ^ r)) & sign != 0 {
                f |= OF;
            }
            if (a ^ b ^ r) & 0x10 != 0 {
                f |= AF;
            }
            (r, f)
        }
        Alu::Sub | Alu::Cmp | Alu::Dec | Alu::Neg => {
            let (a, b) = match op {
                Alu::Dec => (a, 1),
                Alu::Neg => (0, a),
                _ => (a, b),
            };
            let r = a.wrapping_sub(b) & mask;
            let mut f = 0;
            if op != Alu::Dec && b > a {
                f |= CF;
            }
            if ((a ^ b) & (a ^ r)) & sign != 0 {
                f |= OF;
            }
            if (a ^ b ^ r) & 0x10 != 0 {
                f |= AF;
            }
            (r, f)
        }
        Alu::And | Alu::Test => (a & b, 0),
        Alu::Or => (a | b, 0),
        Alu::Xor => (a ^ b, 0),
        Alu::Not => return (!a & mask, eflags),
    };
    if res == 0 {
        flags |= ZF;
    }
    if res & sign != 0 {
        flags |= SF;
    }
    if (res as u8).count_ones() % 2 == 0 {
        flags |= PF;
    }
    // INC and DEC leave CF as it was.
    if matches!(op, Alu::Inc | Alu::Dec) {
        flags = (flags & !CF) | (eflags & CF);
    }
    (res, (eflags & !(CF | PF | AF | ZF | SF | OF)) | flags)
}

fn mem_bits(size: MemorySize) -> Option<u32> {
    match size {
        MemorySize::UInt8 | MemorySize::Int8 => Some(8),
        MemorySize::UInt16 | MemorySize::Int16 => Some(16),
        MemorySize::UInt32 | MemorySize::Int32 => Some(32),
        _ => None,
    }
}

/// ALU instructions with a memory operand: `op [m], reg/imm`,
/// `op reg, [m]`, and the one-operand `inc/dec/neg/not [m]`.
fn emulate_alu(insn: &Instruction, addr: u32, ctx: &mut CONTEXT) -> bool {
    use iced_x86::Mnemonic as M;
    let op = match insn.mnemonic() {
        M::Add => Alu::Add,
        M::Sub => Alu::Sub,
        M::And => Alu::And,
        M::Or => Alu::Or,
        M::Xor => Alu::Xor,
        M::Inc => Alu::Inc,
        M::Dec => Alu::Dec,
        M::Neg => Alu::Neg,
        M::Not => Alu::Not,
        M::Cmp => Alu::Cmp,
        M::Test => Alu::Test,
        _ => return false,
    };
    let Some(bits) = mem_bits(insn.memory_size()) else { return false };
    let size = insn.memory_size();
    let Some(mem) = read_mem(addr, size) else { return false };
    let writes = !matches!(op, Alu::Cmp | Alu::Test);
    let other = |ctx: &mut CONTEXT, kind: OpKind, reg: Register| -> Option<u32> {
        match kind {
            OpKind::Register => get_reg(ctx, reg),
            OpKind::Immediate8 | OpKind::Immediate16 | OpKind::Immediate32
            | OpKind::Immediate8to16 | OpKind::Immediate8to32 => Some(insn.immediate32()),
            _ => None,
        }
    };
    if insn.op0_kind() == OpKind::Memory {
        let b = if insn.op_count() > 1 {
            match other(ctx, insn.op1_kind(), insn.op1_register()) {
                Some(v) => v,
                None => return false,
            }
        } else {
            0
        };
        let (res, flags) = alu(op, mem, b, bits, ctx.EFlags);
        ctx.EFlags = flags;
        if writes {
            return write_mem(addr, size, res);
        }
        true
    } else if insn.op_count() > 1 && insn.op1_kind() == OpKind::Memory && insn.op0_kind() == OpKind::Register {
        let reg = insn.op0_register();
        let Some(a) = get_reg(ctx, reg) else { return false };
        let (res, flags) = alu(op, a, mem, bits, ctx.EFlags);
        ctx.EFlags = flags;
        if writes {
            return set_reg(ctx, reg, res);
        }
        true
    } else {
        false
    }
}

/// Emulates loads into a register and stores of a register or immediate.
fn emulate(insn: &Instruction, addr: u32, ctx: &mut CONTEXT) -> bool {
    if emulate_alu(insn, addr, ctx) {
        return true;
    }
    let size = insn.memory_size();
    match insn.code() {
        // mov r, [m]
        Code::Mov_r8_rm8 | Code::Mov_r16_rm16 | Code::Mov_r32_rm32
            if insn.op1_kind() == OpKind::Memory =>
        {
            read_mem(addr, size).is_some_and(|v| set_reg(ctx, insn.op0_register(), v))
        }
        Code::Mov_AL_moffs8 | Code::Mov_AX_moffs16 | Code::Mov_EAX_moffs32 => {
            read_mem(addr, size).is_some_and(|v| set_reg(ctx, insn.op0_register(), v))
        }
        // movzx / movsx r, [m]
        Code::Movzx_r16_rm8 | Code::Movzx_r32_rm8 | Code::Movzx_r16_rm16 | Code::Movzx_r32_rm16
            if insn.op1_kind() == OpKind::Memory =>
        {
            read_mem(addr, size).is_some_and(|v| set_reg(ctx, insn.op0_register(), v))
        }
        Code::Movsx_r16_rm8 | Code::Movsx_r32_rm8 | Code::Movsx_r16_rm16 | Code::Movsx_r32_rm16
            if insn.op1_kind() == OpKind::Memory =>
        {
            read_mem(addr, size).is_some_and(|v| set_reg(ctx, insn.op0_register(), sign_extend(v, size)))
        }
        // mov [m], r / imm
        Code::Mov_rm8_r8 | Code::Mov_rm16_r16 | Code::Mov_rm32_r32
            if insn.op0_kind() == OpKind::Memory =>
        {
            get_reg(ctx, insn.op1_register()).is_some_and(|v| write_mem(addr, size, v))
        }
        Code::Mov_rm8_imm8 | Code::Mov_rm16_imm16 | Code::Mov_rm32_imm32
            if insn.op0_kind() == OpKind::Memory =>
        {
            write_mem(addr, size, insn.immediate32())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_flags_match_x86() {
        // dec byte 1 -> 0: ZF, PF set; CF preserved.
        let (r, f) = alu(Alu::Dec, 1, 0, 8, CF);
        assert_eq!(r, 0);
        assert_eq!(f & (ZF | PF | CF | SF), ZF | PF | CF);
        // dec byte 0 -> 0xFF: SF, AF set, PF (8 bits) set, no ZF.
        let (r, f) = alu(Alu::Dec, 0, 0, 8, 0);
        assert_eq!(r, 0xFF);
        assert_eq!(f & (ZF | SF | AF | PF | CF), SF | AF | PF);
        // add 0x7F + 1 (8-bit): signed overflow.
        let (r, f) = alu(Alu::Add, 0x7F, 1, 8, 0);
        assert_eq!(r, 0x80);
        assert_ne!(f & OF, 0);
        // add 0xFFFF + 1 (16-bit): carry, zero.
        let (r, f) = alu(Alu::Add, 0xFFFF, 1, 16, 0);
        assert_eq!(r, 0);
        assert_eq!(f & (CF | ZF), CF | ZF);
        // cmp 3, 5 (32-bit): borrow and negative.
        let (_, f) = alu(Alu::Cmp, 3, 5, 32, 0);
        assert_eq!(f & (CF | SF | ZF), CF | SF);
        // test with zero result clears CF/OF and sets ZF.
        let (_, f) = alu(Alu::Test, 0xF0, 0x0F, 8, CF | OF);
        assert_eq!(f & (CF | OF | ZF), ZF);
        // neg 0x80 (8-bit): overflow and carry.
        let (r, f) = alu(Alu::Neg, 0x80, 0, 8, 0);
        assert_eq!(r, 0x80);
        assert_eq!(f & (CF | OF), CF | OF);
    }
}
