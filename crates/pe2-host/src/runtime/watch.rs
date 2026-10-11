//! Debugging aid: hardware write watchpoints on PSX addresses.
//!
//! `PE2_WATCH=0x801D49C4[,addr...]` (up to four) arms the x86 debug
//! registers on the game and IRQ threads; every write to a watched word logs
//! the writing instruction's address and the new value. Map the address to a
//! function with tools/symbolize.py.

use std::sync::OnceLock;

use windows_sys::Win32::Foundation::{EXCEPTION_SINGLE_STEP, HANDLE, NTSTATUS};
use windows_sys::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, CONTEXT, CONTEXT_DEBUG_REGISTERS_X86, EXCEPTION_POINTERS, GetThreadContext,
    SetThreadContext,
};
use windows_sys::Win32::System::Threading::{ResumeThread, SuspendThread};

static WATCHED: OnceLock<Vec<u32>> = OnceLock::new();

fn watched() -> &'static [u32] {
    WATCHED.get_or_init(|| {
        std::env::var("PE2_WATCH")
            .map(|v| {
                v.split(',')
                    .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
                    .take(4)
                    .collect()
            })
            .unwrap_or_default()
    })
}

pub fn enabled() -> bool {
    !watched().is_empty()
}

/// Arms the watchpoints on `thread` (not the calling thread).
pub fn arm(thread: HANDLE) {
    let addrs = watched();
    if addrs.is_empty() {
        return;
    }
    // SAFETY: suspends another thread, edits its debug registers, resumes.
    unsafe {
        SuspendThread(thread);
        let mut ctx: CONTEXT = std::mem::zeroed();
        ctx.ContextFlags = CONTEXT_DEBUG_REGISTERS_X86;
        GetThreadContext(thread, &mut ctx);
        let mut dr7 = 0u32;
        for (i, &a) in addrs.iter().enumerate() {
            match i {
                0 => ctx.Dr0 = a & !3,
                1 => ctx.Dr1 = a & !3,
                2 => ctx.Dr2 = a & !3,
                _ => ctx.Dr3 = a & !3,
            }
            // Local enable, break on write (01), length 4 bytes (11).
            dr7 |= 1 << (i * 2);
            dr7 |= 0b1101 << (16 + i * 4);
        }
        ctx.Dr7 = dr7;
        SetThreadContext(thread, &ctx);
        ResumeThread(thread);
    }
}

pub fn install_handler() {
    if enabled() {
        // SAFETY: registering a handler with the documented signature.
        unsafe { AddVectoredExceptionHandler(1, Some(handler)) };
    }
}

unsafe extern "system" fn handler(info: *mut EXCEPTION_POINTERS) -> i32 {
    // SAFETY: the OS passes valid exception pointers.
    let (rec, ctx) = unsafe { (&*(*info).ExceptionRecord, &mut *(*info).ContextRecord) };
    if rec.ExceptionCode != EXCEPTION_SINGLE_STEP as NTSTATUS {
        return 0; // EXCEPTION_CONTINUE_SEARCH
    }
    let hit = ctx.Dr6 & 0xF;
    for (i, &a) in watched().iter().enumerate() {
        if hit & (1 << i) != 0 {
            // SAFETY: the watched word is in mapped PSX RAM.
            let value = unsafe { ((a & !3) as *const u32).read_volatile() };
            let mips = super::mips::CUR_PC.load(std::sync::atomic::Ordering::Relaxed);
            eprintln!("[watch] write to 0x{a:08X} = 0x{value:08X} by eip 0x{:08X} (last MIPS pc 0x{mips:08X})", ctx.Eip);
        }
    }
    ctx.Dr6 = 0;
    -1 // EXCEPTION_CONTINUE_EXECUTION
}
