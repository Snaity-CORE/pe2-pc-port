//! Host executable: owns the window, renderer, audio, input and the virtual
//! PSX runtime, and drives the decompiled game code (linked as a C static
//! library, see build.rs).

use std::process::ExitCode;

mod ffi {
    unsafe extern "C" {
        pub fn pc_boot_load(exe: *const u8, size: usize) -> i32;
        pub fn pc_boot_run() -> i32;
    }
}

fn main() -> ExitCode {
    let Some(exe_path) = std::env::args().nth(1) else {
        eprintln!("usage: pe2 <path to SLUS_010.42 from your Parasite Eve II disc 1>");
        return ExitCode::FAILURE;
    };
    let exe = match std::fs::read(&exe_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("pe2: cannot read {exe_path}: {err}");
            return ExitCode::FAILURE;
        }
    };

    // SAFETY: `exe` outlives the call, which copies it into PSX memory.
    let status = unsafe { ffi::pc_boot_load(exe.as_ptr(), exe.len()) };
    if status != 0 {
        let reason = match status {
            -1 => "could not map PSX memory at 0x80000000 / 0x1F800000",
            -2 => "not a valid PS-X EXE",
            -3 => "no thunk table for the main image",
            _ => "unknown error",
        };
        eprintln!("pe2: boot failed ({status}): {reason}");
        return ExitCode::FAILURE;
    }
    eprintln!("pe2: main executable loaded, entering main()");

    // SAFETY: memory and thunks are set up by pc_boot_load.
    let ret = unsafe { ffi::pc_boot_run() };
    eprintln!("pe2: main() returned {ret}");
    ExitCode::SUCCESS
}
