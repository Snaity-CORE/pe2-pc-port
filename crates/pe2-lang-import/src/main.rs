//! pe2-lang-import: makes a PE2PC language mod from your own discs of
//! another release of Parasite Eve II (French, German, Spanish, Italian or
//! Japanese). PE2PC plays the USA discs; this puts the release's dialogue,
//! menus, items, pictures of text, loading captions and movies with text
//! (of the Japanese one, its dialogue and movies with text) over them, as a
//! mod in PE2PC's mods folder. Nothing of the game is in
//! this program: all comes from the player's discs, through PE2PC's
//! extract (`pe2 --extract`).
//!
//! Without arguments (or with `--pe2 <PE2PC's program>`), a window asks for
//! the discs. From a terminal, `run` does the same, and `import` makes the
//! mod from two extracts already made (each `pe2 --extract
//! files,textures,texts` in a data folder of its own, PE2_HOME).

mod dialogue;
mod dumps;
mod gui;
mod i18n;
mod import;
mod japanese;
mod lang;
mod pe2pc;
mod pipeline;
mod py;
mod release;
mod strings;

use std::path::PathBuf;
use std::process::ExitCode;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const USAGE: &str = "usage: pe2-lang-import [--pe2 <PE2PC's program>]   (the window)
       pe2-lang-import run [--pe2 <PE2PC's program>] --usa <disc 1> <disc 2> --release <disc 1> <disc 2>
       pe2-lang-import import <USA dumps> <release's dumps> <mod folder> [--name NAME]";

/// The arguments: those alone, and each `--name`'s values.
fn parse(args: &[String]) -> (Vec<PathBuf>, std::collections::HashMap<&str, Vec<PathBuf>>) {
    let (mut alone, mut named) = (Vec::new(), std::collections::HashMap::<&str, Vec<PathBuf>>::new());
    let mut current = None;
    for a in args {
        if let Some(name) = a.strip_prefix("--") {
            current = Some(name);
            named.entry(name).or_default();
        } else if let Some(name) = current {
            named.entry(name).or_default().push(PathBuf::from(a));
        } else {
            alone.push(PathBuf::from(a));
        }
    }
    (alone, named)
}

fn fail(e: impl std::fmt::Display) -> ExitCode {
    eprintln!("pe2-lang-import: {e}");
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).filter(|a| !a.starts_with("--"));
    let (alone, named) = parse(&args[usize::from(command.is_some())..]);
    let one = |name: &str| named.get(name).and_then(|v| v.first()).cloned();
    match command {
        None if alone.is_empty() => {
            drop_own_console();
            gui::run(one("pe2")).map_or_else(fail, |()| ExitCode::SUCCESS)
        }
        Some("import") if alone.len() == 3 => {
            let name = one("name").map(|n| n.to_string_lossy().into_owned());
            match import::make(&alone[0], &alone[1], &alone[2], name.as_deref()) {
                Ok(m) => {
                    println!(
                        "{}: {} lines, {} texts, {} items' details, {} loading captions, {} pictures (notes in report.txt)",
                        m.release.name, m.lines, m.texts, m.details, m.captions, m.pictures
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => fail(e),
            }
        }
        Some("run") => {
            let Some(pe2) = one("pe2").or_else(pe2pc::next_to_me) else {
                return fail("PE2PC's program not found: --pe2");
            };
            let (usa, release) =
                (named.get("usa").cloned().unwrap_or_default(), named.get("release").cloned().unwrap_or_default());
            if usa.len() != 2 || release.len() != 2 {
                return fail(USAGE);
            }
            let job = pipeline::Job {
                data: pe2pc::data_dir(&pe2),
                pe2,
                usa,
                release,
                work: std::env::temp_dir().join("pe2-lang-import"),
            };
            let code = std::cell::Cell::new(ExitCode::SUCCESS);
            pipeline::run(&job, &|event| match event {
                pipeline::Event::Step(_) => {}
                pipeline::Event::Line(line) => eprintln!("{line}"),
                pipeline::Event::Done(Ok(done)) => {
                    let m = &done.made;
                    println!(
                        "{} in {}: {} lines, {} texts, {} items' details, {} loading captions, {} pictures; movies: {}",
                        m.release.name,
                        done.folder.display(),
                        m.lines,
                        m.texts,
                        m.details,
                        m.captions,
                        m.pictures,
                        done.movies.as_ref().map_or_else(|e| e.clone(), |n| n.to_string())
                    );
                }
                pipeline::Event::Done(Err(e)) => code.set(fail(e)),
            });
            code.into_inner()
        }
        _ => fail(USAGE),
    }
}

/// Started from Explorer, the program is alone on a console of its own: let
/// it go, so no empty console window stays behind the window (as PE2PC
/// does). Started from a terminal, the console is shared.
fn drop_own_console() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{FreeConsole, GetConsoleProcessList};
        let mut ids = [0u32; 2];
        // SAFETY: a two-entry buffer for the list.
        unsafe {
            if GetConsoleProcessList(ids.as_mut_ptr(), ids.len() as u32) == 1 {
                FreeConsole();
            }
        }
    }
}
