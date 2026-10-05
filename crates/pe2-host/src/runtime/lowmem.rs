//! Accesses to the first 64 KiB of the address space.
//!
//! On the PlayStation, addresses from 0 mirror RAM (the kernel's area), so a
//! read through a null or near-null pointer quietly returns kernel data. The
//! game has such latent bugs (e.g. `CdCmd_Enqueue(cmd, NULL, NULL)` reads
//! `paramA[3]`). Windows never maps that range, so the access faults. This
//! handler emulates the faulting instruction against the RAM mirror at
//! 0x80000000 and resumes after it.

use std::sync::atomic::{AtomicU32, Ordering};

use iced_x86::{Code, Decoder, DecoderOptions, Instruction, MemorySize, OpKind, Register};
use windows_sys::Win32::Foundation::{EXCEPTION_ACCESS_VIOLATION, NTSTATUS};
use windows_sys::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, CONTEXT, EXCEPTION_POINTERS,
};

const LOW_LIMIT: u32 = 0x1_0000;
const RAM_BASE: u32 = 0x8000_0000;
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;

static REPORTED: AtomicU32 = AtomicU32::new(0);

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
    if REPORTED.fetch_add(1, Ordering::Relaxed) < 8 {
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

/// Emulates loads into a register and stores of a register or immediate.
fn emulate(insn: &Instruction, addr: u32, ctx: &mut CONTEXT) -> bool {
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
