//! Settings from `pe2.toml`, looked for in the current directory, then next
//! to the executable (see pe2.example.toml). Command-line arguments and
//! `PE2_*` environment variables take precedence over the file; anything
//! it leaves out keeps its default. Relative paths in the file are relative
//! to the file itself.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

pub const FILE_NAME: &str = "pe2.toml";

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub discs: Discs,
    pub video: Video,
    pub memcard: Memcard,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Discs {
    pub disc1: Option<PathBuf>,
    pub disc2: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Video {
    /// Internal resolution, a multiple of the console's (1-8).
    pub scale: Option<i32>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Memcard {
    pub slot1: Option<PathBuf>,
    pub slot2: Option<PathBuf>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// The settings (loaded on first use; defaults when there is no file).
pub fn get() -> &'static Config {
    CONFIG.get_or_init(load)
}

fn candidates() -> Vec<PathBuf> {
    let mut v = vec![PathBuf::from(FILE_NAME)];
    if let Some(dir) = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)) {
        v.push(dir.join(FILE_NAME));
    }
    v
}

fn load() -> Config {
    let Some(path) = candidates().into_iter().find(|p| p.is_file()) else {
        return Config::default();
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("pe2: {}: {err}; using defaults", path.display());
            return Config::default();
        }
    };
    match toml::from_str::<Config>(&text) {
        Ok(mut c) => {
            let base = path.parent().map(Path::to_path_buf).unwrap_or_default();
            for p in [&mut c.discs.disc1, &mut c.discs.disc2, &mut c.memcard.slot1, &mut c.memcard.slot2]
                .into_iter()
                .flatten()
            {
                if p.is_relative() {
                    *p = base.join(&*p);
                }
            }
            eprintln!("pe2: settings from {}", path.display());
            c
        }
        Err(err) => {
            eprintln!("pe2: {}: {err}; using defaults", path.display());
            if text.lines().any(|l| l.contains('"') && l.contains('\\')) {
                eprintln!(
                    "pe2: hint: inside \"double quotes\" a backslash starts an escape; write Windows paths \
                     in 'single quotes' or with forward slashes (E:/Games/...)"
                );
            }
            Config::default()
        }
    }
}
