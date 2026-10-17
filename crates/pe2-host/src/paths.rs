//! Where the player's files go: the settings (pe2.toml), the default memory
//! cards, screenshots and the game's log.
//!
//! They live in one folder, the first of:
//!
//! 1. `PE2_HOME`, when set;
//! 2. the executable's folder, when a `portable.txt` (any content) sits next
//!    to it: a portable setup, everything beside pe2.exe;
//! 3. the folder pe2 runs from, or the executable's, when it already holds
//!    a pe2.toml (setups from before the data folder, kept as they are);
//! 4. the system's folder for application data: `%APPDATA%\PE2PC` on
//!    Windows, `~/Library/Application Support/PE2PC` on macOS, and
//!    `$XDG_DATA_HOME/pe2pc` (by default `~/.local/share/pe2pc`) elsewhere.
//!
//! The folder is created when something is first written there.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The settings file's name.
pub const SETTINGS: &str = "pe2.toml";

/// Next to pe2.exe, makes the executable's folder the data folder.
pub const PORTABLE: &str = "portable.txt";

/// The folder of the player's files.
pub fn user_dir() -> &'static Path {
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        if let Some(dir) = std::env::var_os("PE2_HOME").filter(|d| !d.is_empty()) {
            return PathBuf::from(dir);
        }
        if let Some(dir) = exe_dir().filter(|d| d.join(PORTABLE).is_file()) {
            return dir;
        }
        let existing = [std::env::current_dir().ok(), exe_dir()];
        if let Some(dir) = existing.into_iter().flatten().find(|d| d.join(SETTINGS).is_file()) {
            return dir;
        }
        system_dir().or_else(exe_dir).unwrap_or_default()
    })
}

/// The executable's folder.
fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe().ok()?.parent().map(Path::to_path_buf)
}

/// The system's folder for this program's data, if the system names one.
fn system_dir() -> Option<PathBuf> {
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(windows) {
        var("APPDATA").map(|d| d.join("PE2PC"))
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|d| d.join("Library/Application Support/PE2PC"))
    } else {
        var("XDG_DATA_HOME").or_else(|| var("HOME").map(|d| d.join(".local/share"))).map(|d| d.join("pe2pc"))
    }
}

/// `name` inside the folder of the player's files, creating the folder.
pub fn user_file(name: &str) -> PathBuf {
    let _ = std::fs::create_dir_all(user_dir());
    user_dir().join(name)
}

/// The settings file (which may not exist yet).
pub fn settings() -> PathBuf {
    user_dir().join(SETTINGS)
}

/// Opens the folder of the player's files in the system's file manager,
/// starting it on a thread of its own (starting a program can take a
/// moment, which the launcher's window would otherwise hang for).
/// `failed` gets why, if it could not be started.
pub fn open_user_dir(failed: impl FnOnce(String) + Send + 'static) {
    let _ = std::fs::create_dir_all(user_dir());
    open_dir(user_dir().to_path_buf(), failed);
}

/// Opens `dir` in the system's file manager, as open_user_dir does.
pub fn open_dir(dir: PathBuf, failed: impl FnOnce(String) + Send + 'static) {
    let program = if cfg!(windows) {
        "explorer"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };
    std::thread::spawn(move || {
        use std::process::Stdio;
        // Not the standard handles: a launcher without a console has none
        // that are valid to pass on, and starting the program would fail.
        let started = std::process::Command::new(program)
            .arg(&dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        if let Err(e) = started {
            failed(format!("{program} {}: {e}", dir.display()));
        }
    });
}
