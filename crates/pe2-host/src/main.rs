//! Host executable: owns the window, renderer, audio, input and the virtual
//! PSX runtime, and drives the decompiled game code (linked as a C static
//! library, see build.rs).

mod disc;
mod runtime;
mod video;

use std::path::PathBuf;
use std::process::ExitCode;

mod ffi {
    unsafe extern "C" {
        pub fn pc_boot_load(exe: *const u8, size: usize) -> i32;
        pub fn pc_boot_run() -> i32;
    }
}

const USAGE: &str = "usage: pe2 <disc 1 image> [disc 2 image]\n\
                     Images are your own Parasite Eve II (USA) dumps, as .cue or .chd.";

fn main() -> ExitCode {
    let images: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if images.is_empty() || images.len() > 2 {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    }

    let mut discs = Vec::new();
    for image in &images {
        match disc::open(image) {
            Ok(d) => discs.push(d),
            Err(err) => {
                eprintln!("pe2: {}: {err}", image.display());
                return ExitCode::FAILURE;
            }
        }
    }
    let disc1 = &mut discs[0];
    let (exe_name, exe) = match disc::iso9660::boot_executable(disc1.as_mut()) {
        Ok(found) => found,
        Err(err) => {
            eprintln!("pe2: {}: {err}", images[0].display());
            return ExitCode::FAILURE;
        }
    };
    eprintln!(
        "pe2: disc 1: {} tracks, {} sectors; booting {exe_name} ({} bytes)",
        disc1.tracks().len(),
        disc1.total_sectors(),
        exe.len()
    );

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
    runtime::start(discs);
    video::start();

    // SAFETY: memory and thunks are set up by pc_boot_load.
    let ret = unsafe { ffi::pc_boot_run() };
    eprintln!("pe2: main() returned {ret}");
    ExitCode::SUCCESS
}
