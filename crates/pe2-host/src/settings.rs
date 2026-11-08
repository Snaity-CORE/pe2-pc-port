//! The settings the pause menu's Options change while the game runs (the
//! fork's src/main/pc/menu.c draws them in the game's own panels): each with
//! its label, choices, place in pe2.toml and current value, which the code
//! that uses it reads here instead of from config.rs. Most take effect at
//! once; those that cannot (the internal resolution, the fonts) show
//! "(restart)" once changed. Changes are written to pe2.toml when a
//! category's panel closes, keeping everything else in it.
//!
//! The menu asks for categories, rows, labels and choices through the
//! pc_options_* functions; their texts pass through the menus'
//! translations like the game's (mods/strings.rs).

use std::ffi::CString;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A choice: its label and the value pe2.toml holds for it.
#[derive(Clone, Copy)]
enum Value {
    Bool(bool),
    Int(i64),
    Str(&'static str),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Id {
    Scale,
    Interpolation,
    TextureFilter,
    BackgroundFilter,
    Crt,
    IntegerScaling,
    Fullscreen,
    HdFonts,
    Volume,
    Movement,
    QuickTurn,
    Vibration,
    AlwaysCritical,
    InfiniteHp,
    OneHitKill,
    InfiniteMp,
    ExpMultiplier,
    BpMultiplier,
    FastCameraChanges,
    SkipIntro,
    InstantText,
    FastDoors,
    UnifiedBoxes,
    BiggerInventory,
    FullToBox,
    StackItems,
    SaveAnywhere,
}

struct Def {
    id: Id,
    category: usize,
    label: &'static str,
    section: &'static str,
    key: &'static str,
    choices: &'static [(&'static str, Value)],
    /// The choice when pe2.toml has none.
    default: usize,
    /// Takes effect at once (else at the next start).
    live: bool,
}

const ON_OFF: &[(&str, Value)] = &[("Off", Value::Bool(false)), ("On", Value::Bool(true))];
const MULTIPLIERS: &[(&str, Value)] = &[
    ("x1", Value::Int(1)),
    ("x2", Value::Int(2)),
    ("x3", Value::Int(3)),
    ("x5", Value::Int(5)),
    ("x10", Value::Int(10)),
];

const CATEGORIES: [&str; 5] = ["Video", "Audio", "Controls", "Gameplay", "Cheats"];

const DEFS: &[Def] = &[
    Def {
        id: Id::Scale,
        category: 0,
        label: "Resolution",
        section: "video",
        key: "scale",
        choices: &[
            ("1x", Value::Int(1)),
            ("2x", Value::Int(2)),
            ("3x", Value::Int(3)),
            ("4x", Value::Int(4)),
            ("5x", Value::Int(5)),
            ("6x", Value::Int(6)),
            ("7x", Value::Int(7)),
            ("8x", Value::Int(8)),
        ],
        default: 3,
        live: false,
    },
    Def {
        id: Id::Interpolation,
        category: 0,
        label: "60 fps motion",
        section: "video",
        key: "frame_interpolation",
        choices: ON_OFF,
        default: 1,
        live: true,
    },
    Def {
        id: Id::TextureFilter,
        category: 0,
        label: "3D textures",
        section: "video",
        key: "texture_filter",
        choices: &[("Sharp", Value::Str("nearest")), ("Smooth", Value::Str("bilinear"))],
        default: 0,
        live: true,
    },
    Def {
        id: Id::BackgroundFilter,
        category: 0,
        label: "Backgrounds",
        section: "video",
        key: "background_filter",
        choices: &[("Sharp", Value::Str("nearest")), ("Smooth", Value::Str("bicubic"))],
        default: 0,
        live: true,
    },
    Def {
        id: Id::Crt,
        category: 0,
        label: "CRT look",
        section: "video",
        key: "crt",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::IntegerScaling,
        category: 0,
        label: "Integer scaling",
        section: "video",
        key: "integer_scaling",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::Fullscreen,
        category: 0,
        label: "Full screen",
        section: "video",
        key: "fullscreen",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::HdFonts,
        category: 0,
        label: "HD fonts",
        section: "video",
        key: "hd_fonts",
        choices: ON_OFF,
        default: 1,
        live: false,
    },
    Def {
        id: Id::Volume,
        category: 1,
        label: "Volume",
        section: "audio",
        key: "volume",
        choices: &[
            ("0%", Value::Int(0)),
            ("10%", Value::Int(10)),
            ("20%", Value::Int(20)),
            ("30%", Value::Int(30)),
            ("40%", Value::Int(40)),
            ("50%", Value::Int(50)),
            ("60%", Value::Int(60)),
            ("70%", Value::Int(70)),
            ("80%", Value::Int(80)),
            ("90%", Value::Int(90)),
            ("100%", Value::Int(100)),
        ],
        default: 10,
        live: true,
    },
    Def {
        id: Id::Movement,
        category: 2,
        label: "Movement",
        section: "input",
        key: "movement",
        choices: &[("Tank", Value::Str("tank")), ("Modern", Value::Str("modern"))],
        default: 0,
        live: true,
    },
    Def {
        id: Id::QuickTurn,
        category: 2,
        label: "Quick turn",
        section: "input",
        key: "quick_turn",
        choices: ON_OFF,
        default: 1,
        live: true,
    },
    Def {
        id: Id::Vibration,
        category: 2,
        label: "Vibration",
        section: "input",
        key: "vibration",
        choices: ON_OFF,
        default: 1,
        live: true,
    },
    Def {
        id: Id::FastCameraChanges,
        category: 3,
        label: "Fast camera changes",
        section: "gameplay",
        key: "fast_camera_changes",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::SkipIntro,
        category: 3,
        label: "Skip intro",
        section: "gameplay",
        key: "skip_intro",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::InstantText,
        category: 3,
        label: "Instant text",
        section: "gameplay",
        key: "instant_text",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::FastDoors,
        category: 3,
        label: "Fast doors",
        section: "gameplay",
        key: "fast_doors",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::UnifiedBoxes,
        category: 3,
        label: "Unified item boxes",
        section: "gameplay",
        key: "unified_boxes",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::BiggerInventory,
        category: 3,
        label: "Bigger inventory",
        section: "gameplay",
        key: "bigger_inventory",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::FullToBox,
        category: 3,
        label: "Full inventory to box",
        section: "gameplay",
        key: "full_to_box",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::StackItems,
        category: 3,
        label: "Stack items",
        section: "gameplay",
        key: "stack_items",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::SaveAnywhere,
        category: 3,
        label: "Save anywhere",
        section: "gameplay",
        key: "save_anywhere",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::AlwaysCritical,
        category: 4,
        label: "Always critical",
        section: "cheats",
        key: "always_critical",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::InfiniteHp,
        category: 4,
        label: "Infinite HP",
        section: "cheats",
        key: "infinite_hp",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::OneHitKill,
        category: 4,
        label: "One-hit kills",
        section: "cheats",
        key: "one_hit_kill",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::InfiniteMp,
        category: 4,
        label: "Infinite MP",
        section: "cheats",
        key: "infinite_mp",
        choices: ON_OFF,
        default: 0,
        live: true,
    },
    Def {
        id: Id::ExpMultiplier,
        category: 4,
        label: "EXP",
        section: "cheats",
        key: "exp_multiplier",
        choices: MULTIPLIERS,
        default: 0,
        live: true,
    },
    Def {
        id: Id::BpMultiplier,
        category: 4,
        label: "BP",
        section: "cheats",
        key: "bp_multiplier",
        choices: MULTIPLIERS,
        default: 0,
        live: true,
    },
];

/// Each setting's choice now, and at start (what a restart setting runs with).
struct State {
    now: Vec<AtomicUsize>,
    start: Vec<usize>,
}

fn state() -> &'static State {
    static STATE: OnceLock<State> = OnceLock::new();
    STATE.get_or_init(|| {
        let start: Vec<usize> = DEFS.iter().map(from_config).collect();
        State { now: start.iter().map(|&c| AtomicUsize::new(c)).collect(), start }
    })
}

/// A setting's choice as pe2.toml has it (the nearest number for numbers).
fn from_config(def: &Def) -> usize {
    let c = crate::config::get();
    let found: Option<Value> = match def.id {
        Id::Scale => c.video.scale.map(|v| Value::Int(v.into())),
        Id::Interpolation => c.video.frame_interpolation.map(Value::Bool),
        Id::TextureFilter => c.video.texture_filter.as_deref().map(|s| Value::Str(leak(s))),
        Id::BackgroundFilter => c.video.background_filter.as_deref().map(|s| Value::Str(leak(s))),
        Id::Crt => c.video.crt.map(Value::Bool),
        Id::IntegerScaling => c.video.integer_scaling.map(Value::Bool),
        Id::Fullscreen => c.video.fullscreen.map(Value::Bool),
        Id::HdFonts => c.video.hd_fonts.map(Value::Bool),
        Id::Volume => c.audio.volume.map(|v| Value::Int(v.into())),
        Id::Movement => c.input.movement.as_deref().map(|s| Value::Str(leak(s.trim()))),
        Id::QuickTurn => c.input.quick_turn.map(Value::Bool),
        Id::Vibration => c.input.vibration.map(Value::Bool),
        Id::AlwaysCritical => c.cheats.always_critical.map(Value::Bool),
        Id::InfiniteHp => c.cheats.infinite_hp.map(Value::Bool),
        Id::OneHitKill => c.cheats.one_hit_kill.map(Value::Bool),
        Id::InfiniteMp => c.cheats.infinite_mp.map(Value::Bool),
        Id::ExpMultiplier => c.cheats.exp_multiplier.map(|v| Value::Int(v.into())),
        Id::BpMultiplier => c.cheats.bp_multiplier.map(|v| Value::Int(v.into())),
        Id::FastCameraChanges => c.gameplay.fast_camera_changes.map(Value::Bool),
        Id::SkipIntro => c.gameplay.skip_intro.map(Value::Bool),
        Id::InstantText => c.gameplay.instant_text.map(Value::Bool),
        Id::FastDoors => c.gameplay.fast_doors.map(Value::Bool),
        Id::UnifiedBoxes => c.gameplay.unified_boxes.map(Value::Bool),
        Id::BiggerInventory => c.gameplay.bigger_inventory.map(Value::Bool),
        Id::FullToBox => c.gameplay.full_to_box.map(Value::Bool),
        Id::StackItems => c.gameplay.stack_items.map(Value::Bool),
        Id::SaveAnywhere => c.gameplay.save_anywhere.map(Value::Bool),
    };
    let Some(found) = found else { return def.default };
    let distance = |v: &Value| -> i64 {
        match (v, &found) {
            (Value::Bool(a), Value::Bool(b)) => i64::from(a != b),
            (Value::Int(a), Value::Int(b)) => (a - b).abs(),
            (Value::Str(a), Value::Str(b)) => i64::from(a != b),
            _ => i64::MAX,
        }
    };
    let best = (0..def.choices.len()).min_by_key(|&i| distance(&def.choices[i].1)).unwrap_or(def.default);
    if distance(&def.choices[best].1) == i64::MAX { def.default } else { best }
}

/// A text read once from pe2.toml, kept for as long as the game runs.
fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

fn def(id: Id) -> (usize, &'static Def) {
    DEFS.iter().enumerate().find(|(_, d)| d.id == id).expect("every setting is defined")
}

fn value(id: Id) -> Value {
    let (i, d) = def(id);
    d.choices[state().now[i].load(Ordering::Relaxed).min(d.choices.len() - 1)].1
}

/// A setting that is on or off.
pub fn flag(id: Id) -> bool {
    matches!(value(id), Value::Bool(true))
}

/// A setting that is a number.
pub fn number(id: Id) -> i64 {
    match value(id) {
        Value::Int(n) => n,
        _ => 0,
    }
}

/// A setting that is a word ("bilinear", "modern").
pub fn word(id: Id) -> &'static str {
    match value(id) {
        Value::Str(s) => s,
        _ => "",
    }
}

/// Sets a setting from elsewhere (Alt+Enter's full screen).
pub fn set_flag(id: Id, on: bool) {
    let (i, d) = def(id);
    if let Some(c) = d.choices.iter().position(|ch| matches!(ch.1, Value::Bool(b) if b == on)) {
        state().now[i].store(c, Ordering::Relaxed);
    }
}

/// The rows of category `category`: indices into DEFS.
fn rows(category: i32) -> Vec<usize> {
    DEFS.iter().enumerate().filter(|(_, d)| d.category as i32 == category).map(|(i, _)| i).collect()
}

/// Texts handed to the game: made once each, kept while it runs (the menu
/// draws them every frame).
fn text(s: &str) -> *const u8 {
    use std::collections::HashMap;
    use std::sync::Mutex;
    static TEXTS: OnceLock<Mutex<HashMap<String, CString>>> = OnceLock::new();
    let mut texts = TEXTS.get_or_init(Default::default).lock().unwrap();
    texts
        .entry(s.to_string())
        .or_insert_with(|| CString::new(crate::mods::strings::encode(s)).unwrap_or_default())
        .as_ptr()
        .cast()
}

/// Every text the menu's options show, for translators (the extract's
/// dumps/strings/pc/options.toml).
pub fn texts() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = vec!["OPTIONS", "(restart)"];
    out.extend(CATEGORIES);
    out.extend(CATEGORIES.iter().map(|c| leak(&c.to_uppercase())));
    for d in DEFS {
        out.push(d.label);
        out.extend(d.choices.iter().map(|c| c.0));
    }
    let mut seen = std::collections::HashSet::new();
    out.retain(|t| seen.insert(*t));
    out
}

// ---- the menu's side (src/main/pc/menu.c) ----

/// How many categories there are.
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_categories() -> i32 {
    CATEGORIES.len() as i32
}

/// Category `category`'s name: in the list ("Video"), or as a panel's title
/// ("VIDEO").
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_category(category: i32, title: i32) -> *const u8 {
    let _g = crate::runtime::irq::guard();
    let name = CATEGORIES.get(category as usize).copied().unwrap_or("");
    if title != 0 { text(&name.to_uppercase()) } else { text(name) }
}

/// How many rows category `category` has.
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_count(category: i32) -> i32 {
    let _g = crate::runtime::irq::guard();
    rows(category).len() as i32
}

/// A row's label.
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_label(category: i32, row: i32) -> *const u8 {
    let _g = crate::runtime::irq::guard();
    rows(category).get(row as usize).map_or_else(|| text(""), |&i| text(DEFS[i].label))
}

/// A row's choice `choice` (its current one for -1), or NULL past the last:
/// for measuring them all.
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_choice(category: i32, row: i32, choice: i32) -> *const u8 {
    let _g = crate::runtime::irq::guard();
    let Some(&i) = rows(category).get(row as usize) else { return std::ptr::null() };
    let d = &DEFS[i];
    let c = if choice < 0 { state().now[i].load(Ordering::Relaxed) } else { choice as usize };
    d.choices.get(c).map_or(std::ptr::null(), |ch| text(ch.0))
}

/// Whether a row's change waits for a restart; with `ever`, whether one
/// could (to lay the row out).
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_needs_restart(category: i32, row: i32, ever: i32) -> i32 {
    let _g = crate::runtime::irq::guard();
    let Some(&i) = rows(category).get(row as usize) else { return 0 };
    i32::from(!DEFS[i].live && (ever != 0 || state().now[i].load(Ordering::Relaxed) != state().start[i]))
}

/// "(restart)", as the menu shows it beside such a change.
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_restart_text() -> *const u8 {
    let _g = crate::runtime::irq::guard();
    text("(restart)")
}

/// Moves a row's choice by `delta`: two choices take turns, a scale (the
/// volume, the resolution) stops at its ends.
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_change(category: i32, row: i32, delta: i32) {
    let _g = crate::runtime::irq::guard();
    let Some(&i) = rows(category).get(row as usize) else { return };
    let n = DEFS[i].choices.len() as i32;
    let now = state().now[i].load(Ordering::Relaxed) as i32;
    let next = if n == 2 { (now + delta).rem_euclid(n) } else { (now + delta).clamp(0, n - 1) };
    state().now[i].store(next as usize, Ordering::Relaxed);
}

/// Writes the settings into pe2.toml (a category's panel closed).
#[unsafe(no_mangle)]
pub extern "C" fn pc_options_save() {
    let _g = crate::runtime::irq::guard();
    if let Err(e) = save() {
        eprintln!("pe2: settings: {e}");
    }
}

fn save() -> Result<(), String> {
    let path = crate::config::file_path();
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    let mut doc: toml_edit::DocumentMut = text.parse().map_err(|e| format!("{}: {e}", path.display()))?;
    // Only what was changed here, in sections of their own.
    for (i, d) in DEFS.iter().enumerate() {
        let now = state().now[i].load(Ordering::Relaxed);
        if now == state().start[i] && doc.get(d.section).and_then(|t| t.get(d.key)).is_none() {
            continue;
        }
        if !doc.contains_table(d.section) {
            doc[d.section] = toml_edit::table();
        }
        let v = d.choices[now].1;
        doc[d.section][d.key] = match v {
            Value::Bool(b) => toml_edit::value(b),
            Value::Int(n) => toml_edit::value(n),
            Value::Str(s) => toml_edit::value(s),
        };
    }
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    std::fs::write(&path, doc.to_string()).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every setting has choices and a default among them, and every
    /// category rows.
    #[test]
    fn definitions_hold_together() {
        for d in DEFS {
            assert!(d.default < d.choices.len(), "{:?}", d.id);
            assert!(d.category < CATEGORIES.len(), "{:?}", d.id);
        }
        for c in 0..CATEGORIES.len() as i32 {
            assert!(!rows(c).is_empty());
        }
    }
}
