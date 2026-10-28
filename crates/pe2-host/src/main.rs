//! Host executable: owns the window, renderer, audio, input and the virtual
//! PSX runtime, and drives the decompiled game code (linked as a C static
//! library, see build.rs).

mod config;
mod controls;
mod disc;
mod discord;
mod gamepad;
mod gfx;
mod icon;
mod launcher;
mod logfile;
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
                     texel repeated (its values stay the same palette indices);\n\
                     --indexed-from-upscale <picture or folder> <folder> turns enlarged\n\
                     colour previews (dumps/textures/<name>.png made larger, by an AI\n\
                     upscaler say) into indexed replacements.";

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
        "--indexed-from-upscale" if args.len() == 3 => {
            let (done, failed) = from_upscale(&args[1], &args[2]);
            println!("{done} indexed replacements written in {}, {failed} failed", args[2].display());
            Some(if failed > 0 || done == 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS })
        }
        "--indexed-check" | "--indexed-scale" | "--indexed-from-upscale" => {
            eprintln!("{USAGE}");
            Some(ExitCode::FAILURE)
        }
        _ => None,
    }
}

/// `--indexed-from-upscale`: each enlarged picture at `source` (a picture,
/// or a folder of them) made an indexed replacement in `out`. A picture
/// named as a ramp picture of `dumps/textures-ramp/` (`<sheet>-<4|8>bit.png`)
/// is read against that ramp; one named as a colour preview of
/// `dumps/textures/` (`<sheet>.png`, `<sheet>-<palette>.png`) against that
/// preview, at its depth (its width: 256 4-bit, 128 8-bit). The sheet's
/// indices come from `dumps/textures-indexed/`. One replacement per sheet
/// and depth: the first picture of it wins. Answers how many were written
/// and how many failed.
fn from_upscale(source: &std::path::Path, out: &std::path::Path) -> (usize, usize) {
    use mods::indexed;
    let dumps = paths::user_dir().join("dumps");
    let mut pictures: Vec<PathBuf> = if source.is_dir() {
        std::fs::read_dir(source).map(|d| d.flatten().map(|e| e.path()).collect()).unwrap_or_default()
    } else {
        vec![source.to_path_buf()]
    };
    pictures.retain(|p| {
        p.extension().is_some_and(|x| x.eq_ignore_ascii_case("png"))
            && !p.to_string_lossy().to_ascii_lowercase().ends_with(".idx.png")
    });
    pictures.sort();
    let (mut done, mut failed) = (0, 0);
    let mut sheets = std::collections::HashSet::new();
    for picture in pictures {
        let Some(name) = picture.file_stem().and_then(|s| s.to_str()) else { continue };
        let result = (|| -> Result<Option<PathBuf>, String> {
            let read_native = |sheet: u64, depth: u32| {
                indexed::read(&dumps.join("textures-indexed").join(format!("{}.idx.png", indexed::stem(sheet, depth))))
            };
            // A ramp picture's colours are its sheet's indices' greys; a
            // colour preview's are read from it.
            let (native, preview) = match indexed::parse_stem(name) {
                Some((sheet, depth)) => {
                    let native = read_native(sheet, depth)?;
                    let ramp = indexed::ramp(&native);
                    (native, ramp)
                }
                None => {
                    let sheet = u64::from_str_radix(name.split('-').next().unwrap_or(""), 16).map_err(|_| {
                        "not named as a sheet (<sheet>-<4|8>bit.png, <sheet>.png or <sheet>-<palette>.png)".to_string()
                    })?;
                    let preview = mods::read_png(&dumps.join("textures").join(format!("{name}.png")), true)
                        .map_err(|e| format!("its preview dumps/textures/{name}.png: {e} (extract first)"))?;
                    let depth = match preview.width {
                        256 => 0,
                        128 => 1,
                        w => return Err(format!("its preview is {w} wide: not a 4- or 8-bit sheet")),
                    };
                    (read_native(sheet, depth)?, preview.rgba)
                }
            };
            let key = (native.sheet, native.depth);
            if sheets.contains(&key) {
                return Ok(None);
            }
            let upscaled = mods::read_png(&picture, true)?;
            let made = indexed::from_upscale(
                &native,
                &indexed::Rgba { width: native.size.0, height: native.size.1, pixels: &preview },
                &indexed::Rgba { width: upscaled.width, height: upscaled.height, pixels: &upscaled.rgba },
            )?;
            sheets.insert(key);
            indexed::write(out, &made, Some(&format!("from the enlarged {name}.png"))).map(Some)
        })();
        match result {
            Ok(Some(path)) => {
                done += 1;
                println!("{}", path.display());
            }
            Ok(None) => println!("{}: skipped, its sheet is already done", picture.display()),
            Err(e) => {
                failed += 1;
                println!("{}: {e}", picture.display());
            }
        }
    }
    (done, failed)
}

fn main() -> ExitCode {
    drop_own_console();
    logfile::start();
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
    let unattended =
        ["PE2_INPUT", "PE2_REPLAY", "PE2_LIBTEST", "PE2_SELFTEST"].iter().any(|v| std::env::var_os(v).is_some());
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
    runtime::replay::start();
    runtime::start(discs);
    video::start();
    gamepad::start();
    discord::start();

    // SAFETY: memory and thunks are set up by pc_boot_load.
    let ret = unsafe { ffi::pc_boot_run() };
    eprintln!("pe2: main() returned {ret}");
    ExitCode::SUCCESS
}
