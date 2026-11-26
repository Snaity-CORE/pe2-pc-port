//! Where PE2PC is and keeps the player's files, and what its settings
//! say: PE2PC's paths.rs and config.rs, as far as this needs them.

use std::path::{Path, PathBuf};

/// PE2PC's program's name.
pub const PROGRAM: &str = if cfg!(windows) { "pe2.exe" } else { "pe2" };

/// PE2PC's program next to this one, if it is there.
pub fn next_to_me() -> Option<PathBuf> {
    let me = std::env::current_exe().ok()?;
    Some(me.parent()?.join(PROGRAM)).filter(|p| p.is_file())
}

/// PE2PC's data folder for its program `pe2`, the first of: `PE2_HOME`;
/// the program's folder, with a `portable.txt` or a `pe2.toml` there; the
/// system's folder for application data.
pub fn data_dir(pe2: &Path) -> PathBuf {
    if let Some(dir) = std::env::var_os("PE2_HOME").filter(|d| !d.is_empty()) {
        return PathBuf::from(dir);
    }
    let dir = pe2.parent().map(Path::to_path_buf).unwrap_or_default();
    if dir.join("portable.txt").is_file() || dir.join("pe2.toml").is_file() {
        return dir;
    }
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    let system = if cfg!(windows) {
        var("APPDATA").map(|d| d.join("PE2PC"))
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|d| d.join("Library/Application Support/PE2PC"))
    } else {
        var("XDG_DATA_HOME").or_else(|| var("HOME").map(|d| d.join(".local/share"))).map(|d| d.join("pe2pc"))
    };
    system.unwrap_or(dir)
}

/// What the settings (the data folder's pe2.toml) say.
#[derive(Default)]
pub struct Settings {
    /// [discs] disc1 and disc2, those set.
    pub discs: Vec<PathBuf>,
    /// Whether PE2PC uses mods ([general] experimental and [mods] enabled).
    pub mods_on: bool,
}

pub fn settings(data: &Path) -> Settings {
    let path = data.join("pe2.toml");
    let Some(table) = std::fs::read_to_string(&path).ok().and_then(|t| t.parse::<toml::Table>().ok()) else {
        return Settings::default();
    };
    let get = |section: &str, key: &str| table.get(section).and_then(|s| s.get(key)).cloned();
    let discs = ["disc1", "disc2"]
        .iter()
        .filter_map(|k| get("discs", k)?.as_str().map(PathBuf::from))
        .map(|p| if p.is_relative() { data.join(p) } else { p })
        .collect();
    let experimental = get("general", "experimental").and_then(|v| v.as_bool()).unwrap_or(false);
    let mods = get("mods", "enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    Settings { discs, mods_on: experimental && mods }
}
