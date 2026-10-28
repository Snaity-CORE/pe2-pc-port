//! Settings from `pe2.toml` in the folder of the player's files (paths.rs;
//! see pe2.example.toml). Command-line arguments and
//! `PE2_*` environment variables take precedence over the file; anything
//! it leaves out keeps its default. Relative paths in the file are relative
//! to the file itself.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub general: General,
    pub discs: Discs,
    pub video: Video,
    pub input: Input,
    pub memcard: Memcard,
    pub mods: Mods,
    pub cheats: Cheats,
    pub debug: Debug,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Debug {
    /// Record each session's input into recordings/ (runtime/replay.rs;
    /// default off).
    pub record_input: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Cheats {
    /// Every weapon hit is a critical hit (default off).
    pub always_critical: Option<bool>,
    /// The player takes no damage (default off).
    pub infinite_hp: Option<bool>,
    /// Every hit of the player's kills (default off).
    pub one_hit_kill: Option<bool>,
    /// The player's MP never goes down (default off).
    pub infinite_mp: Option<bool>,
    /// What a won battle's EXP is multiplied by (default 1).
    pub exp_multiplier: Option<u32>,
    /// What a won battle's BP is multiplied by (default 1).
    pub bp_multiplier: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Mods {
    /// Use the mods in the data folder's mods/ (default on, with
    /// [general] experimental).
    pub enabled: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Input {
    /// Player 1's controller: "auto" (the one last used, the default) or a
    /// controller's name.
    pub controller: Option<String>,
    /// Controller vibration (default on).
    pub vibration: Option<bool>,
    /// PlayStation button name -> key name (see controls.rs).
    pub keyboard: std::collections::BTreeMap<String, String>,
    /// PlayStation button name -> controller button name.
    pub gamepad: std::collections::BTreeMap<String, String>,
    /// "tank" (the original, the default) or "modern" (the stick or d-pad
    /// points where to go, relative to the camera).
    pub movement: Option<String>,
    /// Back + run turns around, with tank movement (default on).
    pub quick_turn: Option<bool>,
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
    /// Exact (sub-pixel) vertex positions at higher resolutions (default on).
    pub subpixel: Option<bool>,
    /// Perspective-correct textures and colours (with subpixel; default on).
    pub perspective: Option<bool>,
    /// The graphics API: "auto" (the default: Vulkan, else OpenGL),
    /// "vulkan" or "opengl".
    pub graphics_api: Option<String>,
    /// The window's size in pixels, "<width>x<height>", or "auto" (the
    /// default: 960x720 scaled with the desktop's DPI).
    pub window_size: Option<String>,
    /// Start in full screen (Alt+Enter toggles it; default off).
    pub fullscreen: Option<bool>,
    /// Hide the mouse cursor over the picture in full screen (default on).
    pub hide_cursor: Option<bool>,
    /// 3D textures: "nearest" (the default) or "bilinear".
    pub texture_filter: Option<String>,
    /// The game's text drawn from high-definition fonts instead of its
    /// pictures of text: the dialogue (the built-in font, or a mod's), the
    /// menus and the loading captions (default on; only above scale 1).
    pub hd_fonts: Option<bool>,
    /// Pre-rendered backgrounds and full-screen pictures: "nearest" (the
    /// default) or "bicubic".
    pub background_filter: Option<String>,
    /// Frames in between at 60 fps (with subpixel; default on).
    pub frame_interpolation: Option<bool>,
    /// The look of a CRT television: scanlines and an aperture grille
    /// (graphics card presentation; default off).
    pub crt: Option<bool>,
    /// The picture at whole multiples of its height, with black borders
    /// (graphics card presentation; default off).
    pub integer_scaling: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct General {
    /// Ask before Esc quits (default on).
    pub confirm_exit: Option<bool>,
    /// The launcher's language: "en" (the default) or "pt".
    pub language: Option<String>,
    /// Experimental features: mods (default off).
    pub experimental: Option<bool>,
    /// Discord shows where the player is, with their EXP and BP (default
    /// on; nothing happens without Discord running).
    pub discord: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Memcard {
    pub slot1: Option<PathBuf>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// The settings (loaded on first use; defaults when there is no file).
pub fn get() -> &'static Config {
    CONFIG.get_or_init(load)
}

/// The settings file in use, or where a new one goes.
pub fn file_path() -> PathBuf {
    crate::paths::settings()
}

fn load() -> Config {
    let path = file_path();
    if !path.is_file() {
        return Config::default();
    }
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
            for p in [&mut c.discs.disc1, &mut c.discs.disc2, &mut c.memcard.slot1].into_iter().flatten() {
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
