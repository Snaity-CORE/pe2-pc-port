//! Host executable: owns the window, renderer, audio, input and the virtual
//! PSX runtime, and drives the decompiled game code (linked as a C static
//! library, see build.rs).

mod config;
mod controls;
mod disc;
mod gamepad;
mod gfx;
mod icon;
mod launcher;
mod mods;
mod paths;
mod present;
mod runtime;
mod video;

use std::path::PathBuf;
use std::process::ExitCode;

mod ffi {
    unsafe extern "C" {
        pub fn pc_boot_load(exe: *const u8, size: usize) -> i32;
        pub fn pc_boot_run() -> i32;
        pub fn pc_libtest(spec: *const std::ffi::c_char) -> i32;
    }
}

const USAGE: &str = "usage: pe2 [disc 1 image] [disc 2 image]\n\
                     Images are your own Parasite Eve II (USA) dumps, as .cue or .chd.\n\
                     Without arguments, the launcher opens; with --play, pe2.toml's [discs]\n\
                     disc1 and disc2 are played. --extract [images] saves the discs' files\n\
                     to dumps/files in the data folder, for making mods.\n\
                     --indexed-check <file.idx.png>... checks indexed texture replacements;\n\
                     --indexed-scale <file.idx.png> <factor> <folder> enlarges one, each\n\
                     texel repeated (its values stay the same palette indices).";

/// Started from Explorer, the process is alone on a console of its own:
/// let it go, so no empty console window stays behind the game. Started
/// from a terminal, the console is shared and keeps the logs.
fn drop_own_console() {
    use windows_sys::Win32::System::Console::{FreeConsole, GetConsoleProcessList};
    let mut ids = [0u32; 2];
    // SAFETY: a two-entry buffer for the list.
    unsafe {
        if GetConsoleProcessList(ids.as_mut_ptr(), ids.len() as u32) == 1 {
            FreeConsole();
        }
    }
}

/// Whether the process has a console (for the game the launcher starts).
pub fn has_console() -> bool {
    // SAFETY: plain query.
    unsafe { !windows_sys::Win32::System::Console::GetConsoleWindow().is_null() }
}

/// `--indexed-check` and `--indexed-scale` (mods/indexed.rs): answers the
/// exit code, or None for other arguments.
fn indexed_tool(args: &[PathBuf]) -> Option<ExitCode> {
    use mods::indexed;
    match args.first()?.to_str()? {
        "--indexed-check" if args.len() > 1 => {
            let mut failed = false;
            for path in &args[1..] {
                match indexed::read(path) {
                    Ok(t) => println!(
                        "{}: ok, sheet {:016x}, {}-bit, {}x{} texels, {}x{} ({}x)",
                        path.display(),
                        t.sheet,
                        [4, 8][t.depth as usize],
                        t.native.0,
                        t.native.1,
                        t.size.0,
                        t.size.1,
                        t.scale()
                    ),
                    Err(e) => {
                        println!("{}: {e}", path.display());
                        failed = true;
                    }
                }
            }
            Some(if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS })
        }
        "--indexed-scale" if args.len() == 4 => {
            let factor = args[2].to_str().and_then(|f| f.parse::<u32>().ok());
            let result = indexed::read(&args[1]).and_then(|t| match factor {
                Some(f) => indexed::upscale(&t, f),
                None => Err(format!("{} is no whole factor", args[2].display())),
            });
            match result.and_then(|t| indexed::write(&args[3], &t, None)) {
                Ok(out) => {
                    println!("{}", out.display());
                    Some(ExitCode::SUCCESS)
                }
                Err(e) => {
                    eprintln!("pe2: {}: {e}", args[1].display());
                    Some(ExitCode::FAILURE)
                }
            }
        }
        "--indexed-check" | "--indexed-scale" => {
            eprintln!("{USAGE}");
            Some(ExitCode::FAILURE)
        }
        _ => None,
    }
}

fn main() -> ExitCode {
    drop_own_console();
    let mut images: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if let Some(code) = indexed_tool(&images) {
        return code;
    }
    let visuals = images.first().is_some_and(|a| a.as_os_str() == "--extract-visuals");
    let extract = visuals || images.first().is_some_and(|a| a.as_os_str() == "--extract");
    if extract {
        images.remove(0);
    }
    // No arguments: the launcher, except for the unattended and test runs
    // the PE2_* variables drive. It plays with `--play`.
    let unattended = ["PE2_INPUT", "PE2_LIBTEST", "PE2_SELFTEST"].iter().any(|v| std::env::var_os(v).is_some());
    if images.is_empty() && !unattended && !extract {
        return launcher::run();
    }
    if images.len() == 1 && images[0].as_os_str() == "--play" {
        images.clear();
    }
    eprintln!("pe2: data folder: {}", paths::user_dir().display());
    if images.is_empty() {
        let discs = &config::get().discs;
        images.extend(discs.disc1.iter().chain(discs.disc2.iter()).cloned());
    }
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
    for (n, (image, d)) in images.iter().zip(&discs).enumerate() {
        let tracks = d.tracks().len();
        eprintln!(
            "pe2: disc {}: {} ({tracks} track{}, {} sectors)",
            n + 1,
            image.file_name().map_or_else(|| image.display().to_string(), |f| f.to_string_lossy().into_owned()),
            if tracks == 1 { "" } else { "s" },
            d.total_sectors()
        );
    }
    let disc1 = &mut discs[0];
    let (exe_name, exe) = match disc::iso9660::boot_executable(disc1.as_mut()) {
        Ok(found) => found,
        Err(err) => {
            eprintln!("pe2: {}: {err}", images[0].display());
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
    // The extract runs the game's sound code for its songs (music.rs).
    if extract {
        let out = paths::user_file("dumps");
        return match mods::files::extract(&mut discs, &out, !visuals) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("pe2: extract: {}: {e}", out.display());
                ExitCode::FAILURE
            }
        };
    }
    if let Ok(spec) = std::env::var("PE2_LIBTEST") {
        let spec = std::ffi::CString::new(spec).unwrap_or_default();
        // SAFETY: the executable is loaded; `spec` is NUL-terminated.
        let failures = unsafe { ffi::pc_libtest(spec.as_ptr()) };
        return if failures == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE };
    }
    if std::env::var_os("PE2_SELFTEST").is_some() {
        let failures = runtime::selftest::run(runtime::library_address);
        return if failures == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE };
    }
    // The disc's executable supplies the game's data (tables, text, initial
    // values) at its PlayStation addresses; its MIPS code is never run.
    eprintln!("pe2: game data: {exe_name} from disc 1 ({} KB), loaded into PlayStation memory", exe.len() / 1024);
    eprintln!("pe2: game code: native, starting the game");
    mods::start(&images);
    runtime::start(discs);
    video::start();
    gamepad::start();

    // SAFETY: memory and thunks are set up by pc_boot_load.
    let ret = unsafe { ffi::pc_boot_run() };
    eprintln!("pe2: main() returned {ret}");
    ExitCode::SUCCESS
}
