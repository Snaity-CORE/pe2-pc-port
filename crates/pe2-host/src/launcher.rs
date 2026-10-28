//! The launcher: what pe2.exe shows when started without arguments.
//!
//! Setup (on first run, or while a disc is missing) picks the disc images,
//! checking them, the memory cards and a quality; Settings edits every
//! option of pe2.toml (keeping the file's comments); Play runs the game as a
//! child process (`pe2 --play`) and the launcher comes back when it quits.
//! English or Portuguese.
//!
//! The main screen is the PE2PC art (res/launcher_bg.webp, the logo in
//! res/launcher_logo.webp) with a column of buttons; the other screens show
//! on a dark panel over the same background.
//!
//! A controller works it too: the d-pad or left stick moves between
//! buttons and settings (left and right change a choice), A (Cross)
//! presses or ticks, B (Circle) goes back without saving. The arrow keys,
//! Enter and Esc do the same (outside text fields).

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use eframe::egui;

use crate::controls::{BUTTONS, RESERVED_KEYS, key_code, key_label, pad_button, pad_label};

/// The disc images' boot executables: Parasite Eve II (USA).
const DISC_SERIALS: [&str; 2] = ["SLUS_010.42", "SLUS_010.55"];

/// The template of a new pe2.toml: the documented example.
const EXAMPLE: &str = include_str!("../../../pe2.example.toml");

/// The launcher's art, made for the project (nothing from the game).
const BACKGROUND: &[u8] = include_bytes!("../res/launcher_bg.webp");
const LOGO: &[u8] = include_bytes!("../res/launcher_logo.webp");

/// The window's size, the background's.
const WINDOW: [f32; 2] = [960.0, 640.0];

/// The game window sizes the launcher offers, in pixels (4:3, the
/// picture's shape), after "auto" (960x720 scaled with the desktop's DPI).
const WINDOW_SIZES: [&str; 6] = ["640x480", "960x720", "1280x960", "1440x1080", "1600x1200", "1920x1440"];

/// The art's accent colour, and darker shades of it.
const TEAL: egui::Color32 = egui::Color32::from_rgb(38, 200, 186);
const TEAL_DARK: egui::Color32 = egui::Color32::from_rgb(10, 92, 88);
const TEAL_DEEP: egui::Color32 = egui::Color32::from_rgb(6, 52, 50);

/// The font family of Windows' icon font (Segoe Fluent Icons or Segoe MDL2
/// Assets), when there is one.
const ICON_FAMILY: &str = "icons";

/// Decodes a WebP picture to an egui image.
fn decode_webp(bytes: &[u8]) -> Option<egui::ColorImage> {
    let mut decoder = image_webp::WebPDecoder::new(std::io::Cursor::new(bytes)).ok()?;
    let (w, h) = decoder.dimensions();
    let mut buf = vec![0u8; decoder.output_buffer_size()?];
    decoder.read_image(&mut buf).ok()?;
    let size = [w as usize, h as usize];
    Some(if decoder.has_alpha() {
        egui::ColorImage::from_rgba_unmultiplied(size, &buf)
    } else {
        egui::ColorImage::from_rgb(size, &buf)
    })
}

/// The background and the logo, on the graphics card.
struct Art {
    background: egui::TextureHandle,
    logo: egui::TextureHandle,
}

impl Art {
    fn load(ctx: &egui::Context) -> Option<Art> {
        let texture =
            |name: &str, bytes: &[u8]| Some(ctx.load_texture(name, decode_webp(bytes)?, egui::TextureOptions::LINEAR));
        Some(Art { background: texture("background", BACKGROUND)?, logo: texture("logo", LOGO)? })
    }
}

/// What a controller asks of the launcher this frame.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Nav {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Back,
}

/// The controller's focus on the Settings, Setup and Controls screens:
/// each interactive item registers itself as it is drawn, in order, and
/// the focused one is outlined, kept in view and gets the controller's
/// presses.
#[derive(Default)]
struct Focus {
    /// The focused item, the items counted last frame and so far this one.
    at: usize,
    count: usize,
    counting: usize,
    /// This frame's presses, for the focused item.
    accept: bool,
    left: bool,
    right: bool,
    /// The focus moved this frame (scroll to it); a move asked for by an
    /// item, for the next frame.
    moved: bool,
    pending: i32,
}

impl Focus {
    /// Starts a frame with the controller's moves.
    fn frame(&mut self, nav: &[Nav]) {
        self.count = self.counting;
        self.counting = 0;
        self.moved = false;
        let mut step = std::mem::take(&mut self.pending);
        step += nav.iter().map(|n| matches!(n, Nav::Down) as i32 - matches!(n, Nav::Up) as i32).sum::<i32>();
        if step != 0 && self.count > 0 {
            self.at = (self.at as i32 + step).clamp(0, self.count as i32 - 1) as usize;
            self.moved = true;
        }
        self.at = self.at.min(self.count.saturating_sub(1));
        self.accept = nav.contains(&Nav::Accept);
        self.left = nav.contains(&Nav::Left);
        self.right = nav.contains(&Nav::Right);
    }

    /// Back to the first item (a new screen).
    fn reset(&mut self) {
        *self = Focus::default();
    }

    /// Registers the next item (drawn as `response`): whether it has the
    /// focus. Left and right move to the neighbouring item unless `choice`
    /// (they change its value then).
    fn item(&mut self, ui: &egui::Ui, response: &egui::Response, choice: bool) -> bool {
        let k = self.counting;
        self.counting += 1;
        if k != self.at {
            return false;
        }
        ui.painter().rect_stroke(
            response.rect.expand(3.0),
            4.0,
            egui::Stroke::new(2.0, TEAL),
            egui::StrokeKind::Outside,
        );
        if self.moved {
            response.scroll_to_me(Some(egui::Align::Center));
        }
        if !choice && (self.left || self.right) {
            self.pending = if self.right { 1 } else { -1 };
            self.left = false;
            self.right = false;
        }
        true
    }

    /// A checkbox; A ticks it.
    fn checkbox(&mut self, ui: &mut egui::Ui, value: &mut bool, text: &str, enabled: bool) {
        let response = ui.add_enabled(enabled, egui::Checkbox::new(value, text));
        if self.item(ui, &response, false) && enabled && std::mem::take(&mut self.accept) {
            *value = !*value;
        }
    }

    /// A button: whether it was pressed (clicked, or A while focused).
    fn button(&mut self, ui: &mut egui::Ui, text: &str, enabled: bool) -> bool {
        let response = ui.add_enabled(enabled, egui::Button::new(text));
        let pressed = self.item(ui, &response, false) && enabled && std::mem::take(&mut self.accept);
        response.clicked() || pressed
    }

    /// A choice among `options`, shown as "◀ name ▶" after `label`: the
    /// arrows (or left and right, or A for the next) change it.
    fn choice<T: PartialEq + Clone>(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        value: &mut T,
        options: &[(T, String)],
        enabled: bool,
    ) {
        let at = options.iter().position(|(v, _)| v == value).unwrap_or(0);
        let mut to = at;
        let row = ui.add_enabled_ui(enabled, |ui| {
            ui.horizontal(|ui| {
                if !label.is_empty() {
                    ui.label(label);
                }
                if ui.small_button("◀").clicked() {
                    to = at.checked_sub(1).unwrap_or(options.len() - 1);
                }
                let name = options.get(at).map_or("", |(_, n)| n.as_str());
                // As wide as there is room for, up to 230, with the arrow.
                let width = (ui.available_width() - 48.0).clamp(120.0, 230.0);
                ui.add_sized([width, 20.0], egui::Label::new(egui::RichText::new(name).strong()).truncate());
                if ui.small_button("▶").clicked() {
                    to = (at + 1) % options.len();
                }
            })
            .response
        });
        if self.item(ui, &row.inner, true) && enabled && !options.is_empty() {
            if std::mem::take(&mut self.left) {
                to = at.checked_sub(1).unwrap_or(options.len() - 1);
            }
            if std::mem::take(&mut self.right) || std::mem::take(&mut self.accept) {
                to = (at + 1) % options.len();
            }
        }
        if to != at && enabled {
            *value = options[to].0.clone();
        }
    }
}

impl Focus {
    /// A row of tabs, `value` the one shown: one item, whose left and right
    /// (or A, for the next) change the tab.
    fn tabs(&mut self, ui: &mut egui::Ui, value: &mut usize, names: &[&str]) {
        let mut to = *value;
        let row = ui.horizontal(|ui| {
            for (k, name) in names.iter().enumerate() {
                let text = egui::RichText::new(*name).size(16.0);
                let text = if k == *value { text.color(TEAL).strong() } else { text };
                let tab = ui.add(egui::Button::selectable(k == *value, text).min_size(egui::vec2(150.0, 32.0)));
                if tab.clicked() {
                    to = k;
                }
            }
        });
        if self.item(ui, &row.response, true) && !names.is_empty() {
            if std::mem::take(&mut self.left) {
                to = value.checked_sub(1).unwrap_or(names.len() - 1);
            }
            if std::mem::take(&mut self.right) || std::mem::take(&mut self.accept) {
                to = (*value + 1) % names.len();
            }
        }
        *value = to;
    }
}

/// Pushes what follows (a screen's Save and Cancel) to the bottom of the
/// panel.
fn to_bottom(ui: &mut egui::Ui) {
    let rest = ui.available_height() - 48.0;
    if rest > 0.0 {
        ui.add_space(rest);
    }
    ui.separator();
}

/// A group of a screen's settings: a framed card with its title in the
/// accent colour, and room between its rows.
fn card(ui: &mut egui::Ui, title: &str, add: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 6))
        .stroke(egui::Stroke::new(1.0, TEAL_DEEP))
        .corner_radius(8.0)
        .inner_margin(egui::Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new(title).color(TEAL).size(16.0).strong());
            ui.add_space(4.0);
            ui.spacing_mut().item_spacing.y = 10.0;
            add(ui);
        });
}

/// The main screen's buttons.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Icon {
    Play,
    Settings,
    Controls,
    Mods,
    Setup,
    Exit,
}

impl Icon {
    /// The icon's glyph in Windows' icon font (Play is drawn).
    fn glyph(self) -> char {
        match self {
            Icon::Play => '\u{E768}',
            Icon::Settings => '\u{E713}',
            Icon::Controls => '\u{E7FC}',
            Icon::Mods => '\u{EA86}',
            Icon::Setup => '\u{E7F4}',
            Icon::Exit => '\u{F3B1}',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    En,
    Pt,
}

impl Lang {
    fn from_code(code: Option<&str>) -> Lang {
        if code == Some("pt") { Lang::Pt } else { Lang::En }
    }

    fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Pt => "pt",
        }
    }

    /// The text in this language.
    fn t(self, en: &'static str, pt: &'static str) -> &'static str {
        match self {
            Lang::En => en,
            Lang::Pt => pt,
        }
    }
}

/// pe2.toml's options, as the launcher edits them.
#[derive(Clone)]
struct Settings {
    language: Lang,
    confirm_exit: bool,
    /// [general] experimental: mods.
    experimental: bool,
    /// [general] discord: the Discord presence.
    discord: bool,
    discs: [String; 2],
    scale: i64,
    subpixel: bool,
    perspective: bool,
    bilinear: bool,
    smooth_backgrounds: bool,
    interpolation: bool,
    crt: bool,
    integer_scaling: bool,
    /// [video] hd_fonts.
    hd_fonts: bool,
    /// [video] graphics_api: "auto", "vulkan" or "opengl".
    graphics_api: String,
    /// [video] window_size: "auto" or "<width>x<height>" in pixels.
    window_size: String,
    /// [mods] enabled and dump.
    mods_enabled: bool,
    fullscreen: bool,
    hide_cursor: bool,
    /// Empty: the default card in memcards/ in the folder of the player's
    /// files (paths.rs).
    card: String,
    /// Player 1's controller by name; empty: the one last used.
    controller: String,
    vibration: bool,
    /// Camera-relative movement ([input] movement = "modern"), and the
    /// tank movement's quick turn.
    modern: bool,
    quick_turn: bool,
    /// Key and controller button names, in controls::BUTTONS order.
    keys: [String; 16],
    pads: [String; 16],
}

impl Settings {
    fn load(doc: &toml_edit::DocumentMut) -> Settings {
        let get = |table: &str, key: &str| doc.get(table).and_then(|t| t.get(key));
        let text = |table: &str, key: &str| get(table, key).and_then(|v| v.as_str()).unwrap_or("").to_string();
        let flag = |table: &str, key: &str, default: bool| get(table, key).and_then(|v| v.as_bool()).unwrap_or(default);
        Settings {
            language: Lang::from_code(get("general", "language").and_then(|v| v.as_str())),
            confirm_exit: flag("general", "confirm_exit", true),
            experimental: flag("general", "experimental", false),
            discord: flag("general", "discord", true),
            discs: [text("discs", "disc1"), text("discs", "disc2")],
            scale: get("video", "scale").and_then(|v| v.as_integer()).unwrap_or(1).clamp(1, 8),
            subpixel: flag("video", "subpixel", true),
            perspective: flag("video", "perspective", true),
            bilinear: get("video", "texture_filter").and_then(|v| v.as_str()) == Some("bilinear"),
            smooth_backgrounds: get("video", "background_filter").and_then(|v| v.as_str()) == Some("bicubic"),
            interpolation: flag("video", "frame_interpolation", true),
            crt: flag("video", "crt", false),
            integer_scaling: flag("video", "integer_scaling", false),
            hd_fonts: flag("video", "hd_fonts", true),
            graphics_api: get("video", "graphics_api")
                .and_then(|v| v.as_str())
                .filter(|n| crate::gfx::APIS.iter().any(|(a, _)| a == n))
                .unwrap_or("auto")
                .to_string(),
            window_size: get("video", "window_size")
                .and_then(|v| v.as_str())
                .and_then(crate::video::parse_size)
                .map_or_else(|| "auto".to_string(), |(w, h)| format!("{w}x{h}")),
            mods_enabled: flag("mods", "enabled", true),
            fullscreen: flag("video", "fullscreen", false),
            hide_cursor: flag("video", "hide_cursor", true),
            card: text("memcard", "slot1"),
            controller: get("input", "controller")
                .and_then(|v| v.as_str())
                .filter(|n| *n != "auto")
                .unwrap_or("")
                .to_string(),
            vibration: flag("input", "vibration", true),
            modern: get("input", "movement").and_then(|v| v.as_str()) == Some("modern"),
            quick_turn: flag("input", "quick_turn", true),
            keys: std::array::from_fn(|i| {
                let b = &BUTTONS[i];
                doc.get("input")
                    .and_then(|t| t.get("keyboard"))
                    .and_then(|t| t.get(b.name))
                    .and_then(|v| v.as_str())
                    .unwrap_or(b.key)
                    .to_string()
            }),
            pads: std::array::from_fn(|i| {
                let b = &BUTTONS[i];
                doc.get("input")
                    .and_then(|t| t.get("gamepad"))
                    .and_then(|t| t.get(b.name))
                    .and_then(|v| v.as_str())
                    .unwrap_or(b.pad)
                    .to_string()
            }),
        }
    }

    /// Writes the options into `doc`, keeping everything else in it.
    fn store(&self, doc: &mut toml_edit::DocumentMut) {
        use toml_edit::value;
        doc["general"]["language"] = value(self.language.code());
        doc["general"]["confirm_exit"] = value(self.confirm_exit);
        doc["general"]["experimental"] = value(self.experimental);
        doc["general"]["discord"] = value(self.discord);
        let mut path = |table: &str, key: &str, text: &str| {
            if text.trim().is_empty() {
                if let Some(t) = doc.get_mut(table).and_then(|t| t.as_table_like_mut()) {
                    t.remove(key);
                }
            } else {
                doc[table][key] = value(text.trim());
            }
        };
        path("discs", "disc1", &self.discs[0]);
        path("discs", "disc2", &self.discs[1]);
        path("memcard", "slot1", &self.card);
        doc["video"]["scale"] = value(self.scale);
        doc["video"]["subpixel"] = value(self.subpixel);
        doc["video"]["perspective"] = value(self.perspective);
        doc["video"]["texture_filter"] = value(if self.bilinear { "bilinear" } else { "nearest" });
        doc["video"]["background_filter"] = value(if self.smooth_backgrounds { "bicubic" } else { "nearest" });
        doc["video"]["frame_interpolation"] = value(self.interpolation);
        doc["video"]["crt"] = value(self.crt);
        doc["video"]["integer_scaling"] = value(self.integer_scaling);
        doc["video"]["hd_fonts"] = value(self.hd_fonts);
        doc["video"]["graphics_api"] = value(self.graphics_api.as_str());
        doc["video"]["window_size"] = value(self.window_size.as_str());
        doc["mods"]["enabled"] = value(self.mods_enabled);
        doc["video"]["fullscreen"] = value(self.fullscreen);
        doc["video"]["hide_cursor"] = value(self.hide_cursor);
        doc["input"]["controller"] = value(if self.controller.is_empty() { "auto" } else { self.controller.as_str() });
        doc["input"]["vibration"] = value(self.vibration);
        doc["input"]["movement"] = value(if self.modern { "modern" } else { "tank" });
        doc["input"]["quick_turn"] = value(self.quick_turn);
        for (i, b) in BUTTONS.iter().enumerate() {
            doc["input"]["keyboard"][b.name] = value(self.keys[i].as_str());
            doc["input"]["gamepad"][b.name] = value(self.pads[i].as_str());
        }
    }
}

/// Whether `path` is the expected disc of Parasite Eve II (USA).
fn check_disc(path: &str, disc: usize, lang: Lang) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err(lang.t("No image chosen.", "Nenhuma imagem escolhida.").into());
    }
    let mut image = crate::disc::open(Path::new(path.trim())).map_err(|e| e.to_string())?;
    let (name, _) = crate::disc::iso9660::boot_executable(image.as_mut()).map_err(|e| e.to_string())?;
    if name.starts_with(DISC_SERIALS[disc]) {
        Ok(())
    } else if let Some(other) = DISC_SERIALS.iter().position(|s| name.starts_with(s)) {
        Err(format!("{} {}.", lang.t("This is disc", "Este é o disco"), other + 1))
    } else {
        Err(format!("{} ({name}).", lang.t("Not Parasite Eve II (USA)", "Não é o Parasite Eve II (USA)")))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Main,
    /// The first-run setup, at a step.
    Setup(u8),
    Settings,
    Controls,
    Mods,
}

/// A mod as the Mods screen edits it: on or off, and its place in the
/// order (the list's order; the first wins).
struct ModRow {
    dir: PathBuf,
    name: String,
    about: String,
    enabled: bool,
    /// As it was, to write only what changed: on or off, and where in the
    /// list.
    was_enabled: bool,
    was_at: usize,
}

struct Launcher {
    file: PathBuf,
    doc: toml_edit::DocumentMut,
    saved: Settings,
    /// What Settings or Setup edits, until saved.
    edit: Settings,
    screen: Screen,
    checks: [Option<Result<(), String>>; 2],
    running: Arc<AtomicBool>,
    message: Option<String>,
    /// Controls: the controller tab is shown (else the keyboard's), and the
    /// button waiting for a key or controller button (controller, index).
    pad_tab: bool,
    capture: Option<(bool, usize)>,
    /// Settings: the tab shown (general, graphics, discs and cards).
    settings_tab: usize,
    /// Controllers, for their names and to catch their buttons.
    gilrs: Option<gilrs::Gilrs>,
    art: Option<Art>,
    /// Windows' icon font was found.
    icons: bool,
    /// PE2_LAUNCHER_SHOT: frames drawn, to take the picture once settled.
    frames: u32,
    /// The game was started and has no window yet; why it could not start.
    starting: Arc<AtomicBool>,
    launch_error: Arc<std::sync::Mutex<Option<String>>>,
    /// This frame's controller moves, the controller button pressed (for
    /// the Controls screen), and the left stick's last direction (x, y).
    nav: Vec<Nav>,
    pad_pressed: Option<String>,
    stick: [i8; 2],
    /// The main screen's chosen button, and the other screens' focus.
    menu_focus: usize,
    focus: Focus,
    /// The Mods screen's list.
    mods: Vec<ModRow>,
}

/// The pe2.toml name of a key egui reports ("A" -> "KeyA", "Num1" ->
/// "Digit1"), when a button can be mapped to it.
fn egui_key_name(key: egui::Key) -> Option<String> {
    let name = format!("{key:?}");
    let name = match name.as_str() {
        n if n.len() == 1 && n.as_bytes()[0].is_ascii_uppercase() => format!("Key{n}"),
        n if n.starts_with("Num") && n.len() == 4 => format!("Digit{}", &n[3..]),
        "Equals" => "Equal".into(),
        "Backtick" => "Backquote".into(),
        "OpenBracket" => "BracketLeft".into(),
        "CloseBracket" => "BracketRight".into(),
        n => n.to_string(),
    };
    key_code(&name).map(|_| name)
}

impl Launcher {
    fn new(art: Option<Art>, icons: bool) -> Launcher {
        let file = crate::config::file_path();
        let doc = std::fs::read_to_string(&file)
            .ok()
            .and_then(|t| t.parse::<toml_edit::DocumentMut>().ok())
            .unwrap_or_else(|| EXAMPLE.parse().expect("pe2.example.toml parses"));
        let saved = Settings::load(&doc);
        let mut launcher = Launcher {
            file,
            doc,
            edit: saved.clone(),
            saved,
            screen: Screen::Main,
            checks: [None, None],
            running: Arc::new(AtomicBool::new(false)),
            message: None,
            pad_tab: false,
            settings_tab: 0,
            capture: None,
            gilrs: gilrs::Gilrs::new().ok(),
            art,
            icons,
            frames: 0,
            starting: Arc::new(AtomicBool::new(false)),
            launch_error: Arc::new(std::sync::Mutex::new(None)),
            nav: Vec::new(),
            pad_pressed: None,
            stick: [0; 2],
            menu_focus: 0,
            focus: Focus::default(),
            mods: Vec::new(),
        };
        launcher.recheck();
        if launcher.checks[0].as_ref().is_none_or(|c| c.is_err()) {
            launcher.screen = Screen::Setup(0);
        }
        launcher
    }

    fn lang(&self) -> Lang {
        self.edit.language
    }

    fn recheck(&mut self) {
        let lang = self.lang();
        self.checks = [0, 1].map(|d| Some(check_disc(&self.edit.discs[d], d, lang)));
    }

    fn save(&mut self) {
        self.edit.store(&mut self.doc);
        if let Some(dir) = self.file.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        match std::fs::write(&self.file, self.doc.to_string()) {
            Ok(()) => {
                self.saved = self.edit.clone();
                self.message = None;
            }
            Err(e) => self.message = Some(format!("{}: {e}", self.file.display())),
        }
    }

    /// Runs the game as a child process. Starting it, and waiting for its
    /// window, happen on a thread of their own, so the launcher keeps
    /// drawing ("Starting the game..."); it hides once the game's window
    /// shows, and comes back when the game quits.
    fn play(&mut self, ctx: &egui::Context, window: Option<isize>) {
        let exe = match std::env::current_exe() {
            Ok(e) => e,
            Err(e) => {
                self.message = Some(e.to_string());
                return;
            }
        };
        let mut command = std::process::Command::new(exe);
        command.arg("--play");
        if !crate::has_console() {
            // Without one to share, a console program would get a new
            // console window; and the standard handles left by the console
            // the launcher let go are not valid to pass on. The game's log
            // goes to pe2.log in the folder of the player's files instead.
            use std::os::windows::process::CommandExt;
            use std::process::Stdio;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.creation_flags(CREATE_NO_WINDOW).stdin(Stdio::null()).stdout(Stdio::null());
            match std::fs::File::create(crate::paths::user_file("pe2.log")).ok() {
                Some(file) => command.stderr(file),
                None => command.stderr(Stdio::null()),
            };
        }
        self.message = None;
        self.running.store(true, Ordering::SeqCst);
        self.starting.store(true, Ordering::SeqCst);
        let (running, starting, error, ctx) =
            (self.running.clone(), self.starting.clone(), self.launch_error.clone(), ctx.clone());
        std::thread::spawn(move || {
            let mut child = match command.spawn() {
                Ok(c) => c,
                Err(e) => {
                    *error.lock().unwrap() = Some(e.to_string());
                    starting.store(false, Ordering::SeqCst);
                    running.store(false, Ordering::SeqCst);
                    ctx.request_repaint();
                    return;
                }
            };
            // Until the game shows its window (or quits first).
            let since = std::time::Instant::now();
            while !has_window(child.id()) && since.elapsed() < std::time::Duration::from_secs(30) {
                if child.try_wait().ok().flatten().is_some() {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            starting.store(false, Ordering::SeqCst);
            show_window(window, false);
            let _ = child.wait();
            running.store(false, Ordering::SeqCst);
            show_window(window, true);
            ctx.request_repaint();
        });
    }

    /// One of the main screen's buttons at `rect`: a dark glass panel with
    /// an icon and a label, lit in the accent colour when `chosen` (pointed
    /// at, or picked with the controller or keyboard); `primary` (Play) is
    /// tinted always.
    #[allow(clippy::too_many_arguments)]
    fn menu_button(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        icon: Icon,
        label: &str,
        enabled: bool,
        primary: bool,
        chosen: bool,
    ) -> egui::Response {
        use egui::{Color32, FontFamily, FontId, Stroke, StrokeKind, pos2};
        let response = ui.allocate_rect(rect, if enabled { egui::Sense::click() } else { egui::Sense::hover() });
        let lit = enabled && chosen;
        let (fill, stroke) = match (primary && enabled, lit) {
            (true, false) => (Color32::from_rgba_unmultiplied(8, 78, 74, 215), Stroke::new(1.5, TEAL)),
            (true, true) => (Color32::from_rgba_unmultiplied(14, 112, 106, 230), Stroke::new(2.0, TEAL)),
            (false, false) => {
                (Color32::from_rgba_unmultiplied(12, 16, 18, 200), Stroke::new(1.0, Color32::from_gray(58)))
            }
            (false, true) => (Color32::from_rgba_unmultiplied(16, 40, 40, 220), Stroke::new(1.5, TEAL)),
        };
        let painter = ui.painter();
        painter.rect(rect, 4.0, fill, stroke, StrokeKind::Inside);
        let text = if enabled { Color32::from_gray(232) } else { Color32::from_gray(110) };
        let center = pos2(rect.left() + 44.0, rect.center().y);
        match icon {
            Icon::Play => {
                let colour = if enabled { TEAL } else { Color32::from_gray(90) };
                let (w, h) = (16.0, 18.0);
                painter.add(egui::Shape::convex_polygon(
                    vec![
                        pos2(center.x - w / 2.0, center.y - h),
                        pos2(center.x + w, center.y),
                        pos2(center.x - w / 2.0, center.y + h),
                    ],
                    colour,
                    Stroke::NONE,
                ));
            }
            _ if self.icons => {
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    icon.glyph(),
                    FontId::new(24.0, FontFamily::Name(ICON_FAMILY.into())),
                    text,
                );
            }
            _ => {}
        }
        painter.text(
            pos2(rect.left() + 84.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            FontId::proportional(21.0),
            text,
        );
        response
    }

    fn main_screen(&mut self, ui: &mut egui::Ui, window: Option<isize>) {
        use egui::{Rect, pos2, vec2};
        let lang = self.lang();
        let ready = self.checks[0].as_ref().is_some_and(|c| c.is_ok());
        let running = self.running.load(Ordering::SeqCst);
        let full = ui.max_rect();
        match &self.art {
            Some(art) => {
                let size = art.logo.size_vec2();
                let w = 380.0;
                let rect = Rect::from_min_size(full.min + vec2(48.0, 40.0), vec2(w, w * size.y / size.x));
                ui.painter().image(
                    art.logo.id(),
                    rect,
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
            None => {
                ui.painter().text(
                    full.min + vec2(73.0, 120.0),
                    egui::Align2::LEFT_CENTER,
                    "PE2PC",
                    egui::FontId::proportional(56.0),
                    egui::Color32::WHITE,
                );
            }
        }
        let mut buttons = vec![
            (Icon::Play, lang.t("Play", "Jogar"), ready && !running, true),
            (Icon::Settings, lang.t("Settings", "Configurações"), !running, false),
            (Icon::Controls, lang.t("Controls", "Controles"), !running, false),
            (Icon::Mods, lang.t("Mods", "Mods"), !running, false),
            (Icon::Setup, lang.t("Setup", "Configuração inicial"), !running, false),
            (Icon::Exit, lang.t("Exit", "Sair"), true, false),
        ];
        // Mods are an experimental feature.
        if !self.saved.experimental {
            buttons.retain(|b| !matches!(b.0, Icon::Mods));
        }
        // The controller's or keyboard's choice: up and down skip the
        // buttons that are off; Accept (or Enter) presses.
        let keys = ui.ctx().input(|i| {
            (i.key_pressed(egui::Key::ArrowUp), i.key_pressed(egui::Key::ArrowDown), i.key_pressed(egui::Key::Enter))
        });
        let enabled: Vec<usize> = (0..buttons.len()).filter(|&k| buttons[k].2).collect();
        if !enabled.contains(&self.menu_focus) {
            self.menu_focus = enabled.first().copied().unwrap_or(0);
        }
        let step = |focus: usize, down: bool| {
            let at = enabled.iter().position(|&k| k == focus).unwrap_or(0);
            let next = if down { (at + 1).min(enabled.len().saturating_sub(1)) } else { at.saturating_sub(1) };
            enabled.get(next).copied().unwrap_or(focus)
        };
        if self.nav.contains(&Nav::Up) || keys.0 {
            self.menu_focus = step(self.menu_focus, false);
        }
        if self.nav.contains(&Nav::Down) || keys.1 {
            self.menu_focus = step(self.menu_focus, true);
        }
        let pressed = (self.nav.contains(&Nav::Accept) || keys.2).then_some(self.menu_focus);
        let mut y = full.top() + 276.0;
        for (k, (icon, label, enabled, primary)) in buttons.into_iter().enumerate() {
            let rect = Rect::from_min_size(pos2(full.left() + 73.0, y), vec2(324.0, 46.0));
            let response = self.menu_button(ui, rect, icon, label, enabled, primary, self.menu_focus == k);
            if response.hovered() && enabled {
                self.menu_focus = k;
            }
            if response.clicked() || (enabled && pressed == Some(k)) {
                match icon {
                    Icon::Play => self.play(ui.ctx(), window),
                    Icon::Settings => {
                        self.edit = self.saved.clone();
                        self.go(Screen::Settings);
                    }
                    Icon::Controls => {
                        self.edit = self.saved.clone();
                        self.capture = None;
                        self.go(Screen::Controls);
                    }
                    Icon::Mods => {
                        self.edit = self.saved.clone();
                        self.load_mods();
                        self.go(Screen::Mods);
                    }
                    Icon::Setup => {
                        self.edit = self.saved.clone();
                        self.go(Screen::Setup(0));
                    }
                    Icon::Exit => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
                }
            }
            y += 52.0;
        }
        let status = if self.starting.load(Ordering::SeqCst) {
            Some((egui::Color32::from_gray(200), lang.t("Starting the game...", "Iniciando o jogo...")))
        } else if running {
            Some((egui::Color32::from_gray(200), lang.t("The game is running.", "O jogo está rodando.")))
        } else if !ready {
            Some((
                egui::Color32::from_rgb(255, 120, 110),
                lang.t("Choose the disc images in Setup.", "Escolha as imagens dos discos na Configuração inicial."),
            ))
        } else {
            None
        };
        if let Some((colour, text)) = status {
            ui.painter().text(
                pos2(full.left() + 75.0, y + 6.0),
                egui::Align2::LEFT_TOP,
                text,
                egui::FontId::proportional(15.0),
                colour,
            );
        }
        if let Some(m) = &self.message {
            ui.painter().text(
                pos2(full.left() + 75.0, y + 28.0),
                egui::Align2::LEFT_TOP,
                m,
                egui::FontId::proportional(14.0),
                egui::Color32::from_rgb(255, 120, 110),
            );
        }
    }

    /// A disc's path, or the memory card's (`disc` None): a text field and a
    /// Browse button (the item the controller focuses; A browses).
    fn path_row(&mut self, ui: &mut egui::Ui, disc: Option<usize>) {
        let lang = self.lang();
        ui.horizontal(|ui| {
            let (text, hint) = match disc {
                None => (&mut self.edit.card, lang.t("default (memcards folder)", "padrão (pasta memcards)")),
                Some(d) => (&mut self.edit.discs[d], ""),
            };
            let changed = ui.add(egui::TextEdit::singleline(text).hint_text(hint).desired_width(520.0)).changed();
            if self.focus.button(ui, lang.t("Browse...", "Procurar..."), true) {
                let picked = match disc {
                    None => rfd::FileDialog::new().add_filter("Memory card", &["mcd", "mcr"]).save_file(),
                    Some(_) => rfd::FileDialog::new().add_filter("CD image", &["chd", "cue"]).pick_file(),
                };
                if let Some(p) = picked {
                    match disc {
                        None => self.edit.card = p.display().to_string(),
                        Some(d) => {
                            self.edit.discs[d] = p.display().to_string();
                            self.checks[d] = Some(check_disc(&self.edit.discs[d], d, lang));
                        }
                    }
                }
            }
            if let (true, Some(d)) = (changed, disc) {
                self.checks[d] = Some(check_disc(&self.edit.discs[d], d, lang));
            }
        });
    }

    /// Disc and memory card pickers (Setup and Settings).
    fn files(&mut self, ui: &mut egui::Ui, cards: bool) {
        let lang = self.lang();
        for d in 0..2 {
            ui.horizontal(|ui| {
                ui.label(format!("{} {}", lang.t("Disc", "Disco"), d + 1));
                ui.add_space(12.0);
                match &self.checks[d] {
                    Some(Ok(())) => ui.colored_label(egui::Color32::LIGHT_GREEN, lang.t("Found.", "Encontrado.")),
                    Some(Err(e)) => ui.colored_label(egui::Color32::LIGHT_RED, e.as_str()),
                    None => ui.label(""),
                };
            });
            self.path_row(ui, Some(d));
        }
        if cards {
            self.cards(ui);
        }
    }

    /// The memory card picker (port 1, the only one the game uses).
    fn cards(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        ui.label(lang.t("Memory card", "Memory card"));
        self.path_row(ui, None);
    }

    /// The picture's settings: resolution, look, window.
    fn picture(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        let (f, e) = (&mut self.focus, &mut self.edit);
        let scales: Vec<(i64, String)> = (1..=8).map(|s| (s, format!("{s}x ({}x{})", 320 * s, 240 * s))).collect();
        f.choice(ui, lang.t("Resolution", "Resolução"), &mut e.scale, &scales, true);
        // A size set by hand in pe2.toml stays among the choices.
        let mut sizes = vec![("auto".to_string(), lang.t("Default", "Padrão").to_string())];
        sizes.extend(WINDOW_SIZES.iter().map(|s| (s.to_string(), s.to_string())));
        if !sizes.iter().any(|(s, _)| *s == e.window_size) {
            sizes.push((e.window_size.clone(), e.window_size.clone()));
        }
        f.choice(ui, lang.t("Window size", "Tamanho da janela"), &mut e.window_size, &sizes, true);
        let apis = [
            (
                "auto".to_string(),
                lang.t("Automatic (Vulkan, else OpenGL)", "Automático (Vulkan, senão OpenGL)").to_string(),
            ),
            ("vulkan".to_string(), "Vulkan".to_string()),
            ("opengl".to_string(), "OpenGL".to_string()),
        ];
        f.choice(ui, lang.t("Graphics API", "API gráfica"), &mut e.graphics_api, &apis, true);
        f.checkbox(ui, &mut e.crt, lang.t("CRT look (scanlines)", "Visual de TV de tubo (linhas)"), true);
        f.checkbox(
            ui,
            &mut e.integer_scaling,
            lang.t("Integer scaling (sharp, with borders)", "Escala inteira (nítida, com bordas)"),
            true,
        );
        f.checkbox(
            ui,
            &mut e.fullscreen,
            lang.t("Start in full screen (Alt+Enter toggles)", "Iniciar em tela cheia (Alt+Enter alterna)"),
            true,
        );
        f.checkbox(
            ui,
            &mut e.hide_cursor,
            lang.t("Hide the mouse cursor in full screen", "Esconder o cursor do mouse em tela cheia"),
            true,
        );
    }

    /// What the higher resolutions add: precision, smoothing, motion.
    fn enhancements(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        let (f, e) = (&mut self.focus, &mut self.edit);
        let hi = e.scale > 1;
        f.checkbox(
            ui,
            &mut e.subpixel,
            lang.t("Sub-pixel precision (no wobbling)", "Precisão subpixel (sem tremido)"),
            hi,
        );
        f.checkbox(
            ui,
            &mut e.perspective,
            lang.t("Perspective-correct textures", "Texturas com correção de perspectiva"),
            hi && e.subpixel,
        );
        f.checkbox(ui, &mut e.bilinear, lang.t("Smooth 3D textures (bilinear)", "Suavizar texturas 3D (bilinear)"), hi);
        f.checkbox(
            ui,
            &mut e.hd_fonts,
            lang.t("Text in high-definition fonts", "Textos em fontes de alta definição"),
            hi,
        );
        f.checkbox(
            ui,
            &mut e.smooth_backgrounds,
            lang.t("Smooth backgrounds (bicubic)", "Suavizar cenários (bicúbico)"),
            hi,
        );
        f.checkbox(
            ui,
            &mut e.interpolation,
            lang.t("Smooth motion (60 fps)", "Movimento suave (60 fps)"),
            hi && e.subpixel,
        );
    }

    fn language(&mut self, ui: &mut egui::Ui, label: &str) {
        let options = [(Lang::En, "English".to_string()), (Lang::Pt, "Português".to_string())];
        self.focus.choice(ui, label, &mut self.edit.language, &options, true);
    }

    fn setup_screen(&mut self, ui: &mut egui::Ui, step: u8) {
        let lang = self.lang();
        ui.heading(lang.t("Setup", "Configuração inicial"));
        ui.label(egui::RichText::new(format!("{} {} / 4", lang.t("Step", "Passo"), step + 1)).weak());
        ui.add_space(10.0);
        match step {
            0 => card(ui, lang.t("Welcome", "Boas-vindas"), |ui| {
                self.language(ui, lang.t("Language", "Idioma"));
                ui.label(lang.t(
                    "You need your own dumps of Parasite Eve II (USA), discs 1 and 2, as .chd or .cue images.",
                    "Você precisa das suas próprias cópias do Parasite Eve II (USA), discos 1 e 2, em imagens .chd ou .cue.",
                ));
            }),
            1 => card(ui, lang.t("Discs", "Discos"), |ui| self.files(ui, false)),
            2 => card(ui, lang.t("Memory cards", "Memory cards"), |ui| {
                ui.label(lang.t(
                    "Saves go to a memory card image. Leave empty for the default one in the memcards folder of PE2PC's data folder; DuckStation's .mcd cards work too.",
                    "Os saves ficam numa imagem de memory card. Deixe vazio para usar a padrão na pasta memcards da pasta de dados do PE2PC; os cartões .mcd do DuckStation também funcionam.",
                ));
                self.cards(ui);
            }),
            _ => {
                ui.columns(2, |cols| {
                    card(&mut cols[0], lang.t("Picture", "Imagem"), |ui| self.picture(ui));
                    card(&mut cols[1], lang.t("Enhancements", "Melhorias"), |ui| self.enhancements(ui));
                });
            }
        }
        if step == 1 && !self.checks[0].as_ref().is_some_and(|c| c.is_ok()) {
            ui.add_space(8.0);
            ui.label(lang.t(
                "Disc 1 is needed to go on (disc 2 can be added later).",
                "O disco 1 é necessário para continuar (o disco 2 pode ser adicionado depois).",
            ));
        }
        to_bottom(ui);
        let discs_ok = self.checks[0].as_ref().is_some_and(|c| c.is_ok());
        let ready = self.saved_ready();
        let mut go = None;
        ui.horizontal(|ui| {
            if step > 0 && self.focus.button(ui, lang.t("Back", "Voltar"), true) {
                go = Some(Screen::Setup(step - 1));
            }
            if step < 3 {
                if self.focus.button(ui, lang.t("Next", "Avançar"), step != 1 || discs_ok) {
                    go = Some(Screen::Setup(step + 1));
                }
            } else if self.focus.button(ui, lang.t("Finish", "Concluir"), true) {
                self.save();
                go = Some(Screen::Main);
            }
            if ready && self.focus.button(ui, lang.t("Cancel", "Cancelar"), true) {
                self.edit = self.saved.clone();
                self.recheck();
                go = Some(Screen::Main);
            }
        });
        if let Some(screen) = go {
            self.go(screen);
        }
    }

    /// Shows another screen, its focus on its first item.
    fn go(&mut self, screen: Screen) {
        self.screen = screen;
        self.focus.reset();
    }

    /// Whether the saved settings already have a working disc 1.
    fn saved_ready(&self) -> bool {
        check_disc(&self.saved.discs[0], 0, self.lang()).is_ok()
    }

    fn settings_screen(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        ui.heading(lang.t("Settings", "Configurações"));
        ui.add_space(6.0);
        let tabs = [lang.t("General", "Geral"), lang.t("Graphics", "Gráficos"), lang.t("Discs", "Discos")];
        let mut tab = self.settings_tab;
        self.focus.tabs(ui, &mut tab, &tabs);
        self.settings_tab = tab;
        ui.add_space(10.0);
        // The panel's height, but for the buttons below.
        egui::ScrollArea::vertical().max_height(ui.available_height() - 64.0).show(ui, |ui| match tab {
            0 => {
                card(ui, lang.t("Launcher", "Launcher"), |ui| {
                    self.language(ui, lang.t("Language", "Idioma"));
                    self.focus.checkbox(
                        ui,
                        &mut self.edit.confirm_exit,
                        lang.t("Ask before Esc quits", "Perguntar antes de Esc sair"),
                        true,
                    );
                    self.focus.checkbox(
                        ui,
                        &mut self.edit.discord,
                        lang.t("Show the game on Discord", "Mostrar o jogo no Discord"),
                        true,
                    );
                });
                ui.add_space(12.0);
                card(ui, lang.t("Experimental", "Experimental"), |ui| {
                    self.focus.checkbox(
                        ui,
                        &mut self.edit.experimental,
                        lang.t("Experimental features (mods)", "Recursos experimentais (mods)"),
                        true,
                    );
                });
                ui.add_space(12.0);
                card(ui, lang.t("Data folder", "Pasta de dados"), |ui| {
                    ui.label(lang.t(
                        "Settings, memory cards, screenshots, mods and what the extract saves.",
                        "Configurações, memory cards, capturas, mods e o que a extração salva.",
                    ));
                    ui.horizontal(|ui| {
                        if self.focus.button(ui, lang.t("Open", "Abrir"), true) {
                            let (error, ctx) = (self.launch_error.clone(), ui.ctx().clone());
                            crate::paths::open_user_dir(move |e| {
                                *error.lock().unwrap() = Some(e);
                                ctx.request_repaint();
                            });
                        }
                        ui.label(egui::RichText::new(crate::paths::user_dir().display().to_string()).weak());
                    });
                });
            }
            1 => {
                ui.columns(2, |cols| {
                    card(&mut cols[0], lang.t("Picture", "Imagem"), |ui| self.picture(ui));
                    card(&mut cols[1], lang.t("Enhancements", "Melhorias"), |ui| self.enhancements(ui));
                });
            }
            _ => {
                card(ui, lang.t("Discs", "Discos"), |ui| self.files(ui, false));
                ui.add_space(12.0);
                card(ui, lang.t("Memory cards", "Memory cards"), |ui| self.cards(ui));
            }
        });
        to_bottom(ui);
        ui.horizontal(|ui| {
            if self.focus.button(ui, lang.t("Save", "Salvar"), true) {
                self.save();
                if self.message.is_none() {
                    self.go(Screen::Main);
                }
            }
            if self.focus.button(ui, lang.t("Cancel", "Cancelar"), true) {
                self.edit = self.saved.clone();
                self.recheck();
                self.go(Screen::Main);
            }
        });
    }
}

impl Launcher {
    /// A key or controller button for the button being changed, if one was
    /// pressed this frame; Esc gives up.
    fn catch(&mut self, ctx: &egui::Context) {
        let lang = self.lang();
        // The controller button pressed this frame (raw_input_hook).
        let pressed = self.pad_pressed.take();
        let Some((pad, i)) = self.capture else { return };
        if pad {
            if let Some(name) = pressed.filter(|n| pad_button(n).is_some()) {
                self.edit.pads[i] = name;
                self.capture = None;
            }
            if ctx.input(|inp| inp.key_pressed(egui::Key::Escape)) {
                self.capture = None;
            }
            return;
        }
        let mut caught = None;
        ctx.input_mut(|inp| {
            for ev in &inp.events {
                if let egui::Event::Key { key, pressed: true, .. } = ev {
                    caught = Some(if *key == egui::Key::Escape { None } else { egui_key_name(*key) });
                    break;
                }
            }
            if caught.is_none() && inp.modifiers.shift {
                caught = Some(Some("ShiftLeft".to_string()));
            } else if caught.is_none() && inp.modifiers.ctrl {
                caught = Some(Some("ControlLeft".to_string()));
            }
            if caught.is_some() {
                // The key must not also press the widget in focus.
                inp.events.clear();
            }
        });
        match caught {
            Some(Some(name)) if RESERVED_KEYS.contains(&name.as_str()) => {
                self.message = Some(format!(
                    "{}: {}",
                    key_label(&name),
                    lang.t("reserved (Esc, P, F9-F12, Alt)", "reservada (Esc, P, F9-F12, Alt)")
                ));
            }
            Some(Some(name)) => {
                self.edit.keys[i] = name;
                self.capture = None;
                self.message = None;
            }
            Some(None) => self.capture = None,
            None => {}
        }
    }

    fn controls_screen(&mut self, ui: &mut egui::Ui) {
        self.catch(ui.ctx());
        let lang = self.lang();
        ui.heading(lang.t("Controls", "Controles"));
        ui.add_space(6.0);
        let tab = [lang.t("Keyboard", "Teclado"), lang.t("Controller", "Controle")];
        let mut at = usize::from(self.pad_tab);
        if self.capture.is_none() {
            self.focus.tabs(ui, &mut at, &tab);
        }
        if (at == 1) != self.pad_tab {
            self.pad_tab = at == 1;
            self.capture = None;
        }
        ui.add_space(10.0);
        egui::ScrollArea::vertical().max_height(ui.available_height() - 64.0).show(ui, |ui| {
            ui.columns(2, |cols| {
                card(&mut cols[0], lang.t("Movement", "Movimento"), |ui| {
                    let movement = [
                        (false, lang.t("Tank (original)", "Tanque (original)").to_string()),
                        (true, lang.t("Modern (camera)", "Moderno (câmera)").to_string()),
                    ];
                    self.focus.choice(ui, "", &mut self.edit.modern, &movement, true);
                    let tank = !self.edit.modern;
                    self.focus.checkbox(
                        ui,
                        &mut self.edit.quick_turn,
                        lang.t("Quick turn (back + run)", "Meia-volta rápida (trás + correr)"),
                        tank,
                    );
                });
                if self.pad_tab {
                    card(&mut cols[1], lang.t("Controller", "Controle"), |ui| {
                        let names: Vec<String> = self
                            .gilrs
                            .as_ref()
                            .map(|g| g.gamepads().map(|(_, p)| p.name().to_string()).collect())
                            .unwrap_or_default();
                        let mut options = vec![(String::new(), lang.t("Last one used", "O último usado").to_string())];
                        options.extend(names.iter().map(|n| (n.clone(), n.clone())));
                        if !self.edit.controller.is_empty() && !names.contains(&self.edit.controller) {
                            options.push((self.edit.controller.clone(), self.edit.controller.clone()));
                        }
                        self.focus.choice(ui, "", &mut self.edit.controller, &options, true);
                        if names.is_empty() {
                            ui.label(lang.t("No controller connected.", "Nenhum controle conectado."));
                        }
                        self.focus.checkbox(ui, &mut self.edit.vibration, lang.t("Vibration", "Vibração"), true);
                    });
                } else {
                    card(&mut cols[1], lang.t("Keyboard", "Teclado"), |ui| {
                        ui.label(lang.t(
                            "Esc, P, F9-F12 and Alt keep their own jobs.",
                            "Esc, P, F9-F12 e Alt mantêm suas funções.",
                        ));
                    });
                }
            });
            ui.add_space(12.0);
            card(ui, lang.t("Buttons", "Botões"), |ui| {
                // Four columns of four buttons: the directions, the face
                // buttons, Start, Select and the front ones, the rest.
                let rows = BUTTONS.len().div_ceil(4);
                egui::Grid::new("bindings").num_columns(8).spacing([10.0, 6.0]).show(ui, |ui| {
                    for row in 0..rows {
                        for i in (0..4).map(|c| c * rows + row) {
                            let Some(b) = BUTTONS.get(i) else { continue };
                            ui.label(b.label);
                            let waiting = self.capture == Some((self.pad_tab, i));
                            let text = if waiting {
                                if self.pad_tab {
                                    lang.t("Press a button...", "Aperte um botão...")
                                } else {
                                    lang.t("Press a key...", "Aperte uma tecla...")
                                }
                                .to_string()
                            } else if self.pad_tab {
                                pad_label(&self.edit.pads[i])
                            } else {
                                key_label(&self.edit.keys[i])
                            };
                            let response = ui.add(egui::Button::new(text).min_size([116.0, 28.0].into()));
                            let pressed = self.focus.item(ui, &response, false)
                                && self.capture.is_none()
                                && std::mem::take(&mut self.focus.accept);
                            if response.clicked() || pressed {
                                self.capture = Some((self.pad_tab, i));
                                self.message = None;
                            }
                        }
                        ui.end_row();
                    }
                });
                if self.capture.is_some() {
                    ui.label(egui::RichText::new(lang.t("Esc cancels.", "Esc cancela.")).weak());
                }
            });
        });
        to_bottom(ui);
        ui.horizontal(|ui| {
            if self.focus.button(ui, lang.t("Save", "Salvar"), true) {
                self.capture = None;
                self.save();
                if self.message.is_none() {
                    self.go(Screen::Main);
                }
            }
            if self.focus.button(ui, lang.t("Cancel", "Cancelar"), true) {
                self.capture = None;
                self.edit = self.saved.clone();
                self.message = None;
                self.go(Screen::Main);
            }
            if self.focus.button(ui, lang.t("Restore defaults", "Restaurar padrões"), true) {
                self.capture = None;
                for (i, b) in BUTTONS.iter().enumerate() {
                    if self.pad_tab {
                        self.edit.pads[i] = b.pad.to_string();
                    } else {
                        self.edit.keys[i] = b.key.to_string();
                    }
                }
            }
        });
    }
}

impl Launcher {
    /// The installed mods, in their order, for the Mods screen.
    fn load_mods(&mut self) {
        self.mods = crate::mods::installed()
            .into_iter()
            .enumerate()
            .map(|(k, m)| {
                let mut about = Vec::new();
                if let Some(a) = &m.author {
                    about.push(a.clone());
                }
                if let Some(d) = &m.description {
                    about.push(d.clone());
                }
                ModRow {
                    name: match &m.version {
                        Some(v) => format!("{} {v}", m.name),
                        None => m.name.clone(),
                    },
                    about: about.join(" — "),
                    enabled: m.enabled,
                    was_enabled: m.enabled,
                    was_at: k,
                    dir: m.dir,
                }
            })
            .collect();
    }

    /// Writes each mod's mod.toml where something changed: on or off, and,
    /// when the order changed, every mod's priority (the first in the list
    /// highest). The file's other contents are kept.
    fn save_mods(&mut self) -> Result<(), String> {
        let count = self.mods.len() as i64;
        let reordered = self.mods.iter().enumerate().any(|(k, m)| m.was_at != k);
        for (k, m) in self.mods.iter().enumerate() {
            let priority = count - 1 - k as i64;
            if m.enabled == m.was_enabled && !reordered {
                continue;
            }
            let path = m.dir.join("mod.toml");
            let mut doc = std::fs::read_to_string(&path)
                .ok()
                .and_then(|t| t.parse::<toml_edit::DocumentMut>().ok())
                .unwrap_or_default();
            doc["enabled"] = toml_edit::value(m.enabled);
            if reordered {
                doc["priority"] = toml_edit::value(priority);
            }
            std::fs::write(&path, doc.to_string()).map_err(|e| format!("{}: {e}", path.display()))?;
        }
        Ok(())
    }

    fn mods_screen(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        ui.heading(lang.t("Mods", "Mods"));
        ui.add_space(10.0);
        card(ui, lang.t("Mods folder", "Pasta de mods"), |ui| {
            ui.horizontal(|ui| {
                // A row as tall as its button, the checkbox centred on it.
                ui.spacing_mut().interact_size.y = 30.0;
                self.focus.checkbox(ui, &mut self.edit.mods_enabled, lang.t("Use mods", "Usar mods"), true);
                ui.add_space(24.0);
                if self.focus.button(ui, lang.t("Open the mods folder", "Abrir a pasta de mods"), true) {
                    let dir = crate::mods::mods_dir();
                    let _ = std::fs::create_dir_all(&dir);
                    let (error, ctx) = (self.launch_error.clone(), ui.ctx().clone());
                    crate::paths::open_dir(dir, move |e| {
                        *error.lock().unwrap() = Some(e);
                        ctx.request_repaint();
                    });
                }
            });
            ui.label(egui::RichText::new(crate::mods::mods_dir().display().to_string()).weak());
        });
        ui.add_space(12.0);
        let mut swap = None;
        egui::ScrollArea::vertical().max_height(ui.available_height() - 64.0).show(ui, |ui| {
            card(ui, lang.t("Installed", "Instalados"), |ui| {
                if self.mods.is_empty() {
                    ui.label(lang.t(
                        "No mods installed. A mod is a folder in the mods folder above (see docs/MODDING.md).",
                        "Nenhum mod instalado. Um mod é uma pasta dentro da pasta de mods acima (veja docs/MODDING.md).",
                    ));
                }
                let count = self.mods.len();
                for k in 0..count {
                    if k > 0 {
                        ui.separator();
                    }
                    ui.horizontal(|ui| {
                        ui.spacing_mut().interact_size.y = 30.0;
                        let m = &mut self.mods[k];
                        self.focus.checkbox(ui, &mut m.enabled, &m.name, self.edit.mods_enabled);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Right to left: the lower arrow first.
                            if self.focus.button(ui, "▼", k + 1 < count) {
                                swap = Some(k);
                            }
                            if self.focus.button(ui, "▲", k > 0) {
                                swap = Some(k - 1);
                            }
                        });
                    });
                    if !self.mods[k].about.is_empty() {
                        ui.label(egui::RichText::new(&self.mods[k].about).weak());
                    }
                }
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(lang.t(
                        "Higher in the list wins where two mods change the same thing. Changes apply when the game starts.",
                        "Mais acima na lista vence quando dois mods mudam a mesma coisa. As mudanças valem quando o jogo começa.",
                    ))
                    .weak(),
                );
            });
        });
        if let Some(k) = swap {
            self.mods.swap(k, k + 1);
        }
        to_bottom(ui);
        ui.horizontal(|ui| {
            if self.focus.button(ui, lang.t("Save", "Salvar"), true) {
                match self.save_mods() {
                    Ok(()) => {
                        self.save();
                        if self.message.is_none() {
                            self.go(Screen::Main);
                        }
                    }
                    Err(e) => self.message = Some(e),
                }
            }
            if self.focus.button(ui, lang.t("Cancel", "Cancelar"), true) {
                self.edit = self.saved.clone();
                self.message = None;
                self.go(Screen::Main);
            }
        });
    }
}

impl eframe::App for Launcher {
    /// Reads the controllers before each frame: their moves, and the button
    /// pressed (for the Controls screen to catch); and, on the other
    /// screens, the arrow keys, Enter and Esc as moves.
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.nav.clear();
        self.pad_pressed = None;
        // The keyboard moves the focus too on the other screens, unless a
        // text field is being typed in or a key is being caught.
        let typing = ctx.memory(|m| m.focused().is_some());
        if self.screen != Screen::Main && self.capture.is_none() && !typing {
            raw.events.retain(|ev| {
                let egui::Event::Key { key, pressed: true, modifiers, .. } = ev else { return true };
                let nav = match key {
                    egui::Key::ArrowUp => Nav::Up,
                    egui::Key::ArrowDown => Nav::Down,
                    egui::Key::ArrowLeft => Nav::Left,
                    egui::Key::ArrowRight => Nav::Right,
                    egui::Key::Enter if !modifiers.alt => Nav::Accept,
                    egui::Key::Escape => Nav::Back,
                    _ => return true,
                };
                self.nav.push(nav);
                false
            });
        }
        let Some(g) = self.gilrs.as_mut() else { return };
        while let Some(ev) = g.next_event() {
            match ev.event {
                gilrs::EventType::ButtonPressed(b, _) => {
                    self.pad_pressed = Some(format!("{b:?}"));
                    let nav = match b {
                        gilrs::Button::DPadUp => Some(Nav::Up),
                        gilrs::Button::DPadDown => Some(Nav::Down),
                        gilrs::Button::DPadLeft => Some(Nav::Left),
                        gilrs::Button::DPadRight => Some(Nav::Right),
                        gilrs::Button::South => Some(Nav::Accept),
                        gilrs::Button::East => Some(Nav::Back),
                        _ => None,
                    };
                    self.nav.extend(nav);
                }
                // The stick: a move each time it is pushed past half way.
                gilrs::EventType::AxisChanged(axis @ (gilrs::Axis::LeftStickX | gilrs::Axis::LeftStickY), v, _) => {
                    let k = usize::from(axis == gilrs::Axis::LeftStickY);
                    let dir = if v > 0.6 {
                        1
                    } else if v < -0.6 {
                        -1
                    } else if v.abs() < 0.3 {
                        0
                    } else {
                        self.stick[k]
                    };
                    if dir != self.stick[k] && dir != 0 {
                        self.nav.push(match (k, dir) {
                            (0, 1) => Nav::Right,
                            (0, _) => Nav::Left,
                            (_, 1) => Nav::Up,
                            _ => Nav::Down,
                        });
                    }
                    self.stick[k] = dir;
                }
                _ => {}
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let window = window_handle(frame);
        let full = ui.max_rect();
        // Keep reading the controllers, and the game's start.
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(50));
        if let Some(e) = self.launch_error.lock().unwrap().take() {
            self.message = Some(e);
        }
        // B (Circle): back to the main screen without saving.
        if self.screen != Screen::Main && self.capture.is_none() && self.nav.contains(&Nav::Back) {
            self.edit = self.saved.clone();
            self.recheck();
            self.message = None;
            self.go(if self.saved_ready() { Screen::Main } else { Screen::Setup(0) });
        }
        // While a controller button is being caught, the controller is not
        // moving the focus.
        let nav = if self.capture.is_some() { Vec::new() } else { self.nav.clone() };
        self.focus.frame(&nav);
        match &self.art {
            Some(art) => {
                let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                ui.painter().image(art.background.id(), full, uv, egui::Color32::WHITE);
            }
            None => {
                ui.painter().rect_filled(full, 0.0, egui::Color32::from_rgb(6, 10, 12));
            }
        }
        if self.screen == Screen::Main {
            self.main_screen(ui, window);
        } else {
            // The other screens: a dark glass panel over the dimmed art.
            ui.painter().rect_filled(full, 0.0, egui::Color32::from_black_alpha(130));
            let panel = full.shrink2(egui::vec2(70.0, 28.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(panel), |ui| {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgba_unmultiplied(8, 12, 14, 228))
                    .stroke(egui::Stroke::new(1.0, TEAL_DARK))
                    .corner_radius(8.0)
                    .inner_margin(22.0)
                    .show(ui, |ui| {
                        ui.set_min_size(panel.size() - egui::vec2(44.0, 44.0));
                        match self.screen {
                            Screen::Main => {}
                            Screen::Setup(step) => self.setup_screen(ui, step),
                            Screen::Settings => self.settings_screen(ui),
                            Screen::Controls => self.controls_screen(ui),
                            Screen::Mods => self.mods_screen(ui),
                        }
                        if let Some(m) = &self.message {
                            ui.colored_label(egui::Color32::from_rgb(255, 120, 110), m.as_str());
                        }
                    });
            });
        }
        self.debug_shot(ui.ctx());
    }
}

impl Launcher {
    /// PE2_LAUNCHER_SHOT=<file.png> (with PE2_LAUNCHER_SCREEN=main,
    /// settings, controls or setup, and PE2_LAUNCHER_TAB=<n> for the
    /// settings' tab or the controller's mapping): saves the launcher's
    /// picture once it settles, and quits. For checking its looks.
    fn debug_shot(&mut self, ctx: &egui::Context) {
        let Some(path) = std::env::var_os("PE2_LAUNCHER_SHOT") else { return };
        self.frames += 1;
        if self.frames == 1 {
            let tab = std::env::var("PE2_LAUNCHER_TAB").ok().and_then(|t| t.parse::<usize>().ok()).unwrap_or(0);
            self.settings_tab = tab.min(2);
            self.pad_tab = tab == 1;
            self.screen = match std::env::var("PE2_LAUNCHER_SCREEN").as_deref() {
                Ok("settings") => Screen::Settings,
                Ok("controls") => Screen::Controls,
                Ok("setup") => Screen::Setup(0),
                Ok("mods") => {
                    self.load_mods();
                    Screen::Mods
                }
                _ => self.screen,
            };
        }
        if self.frames == 6 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::ScreenshotCallback::new(move |image| {
                let [w, h] = image.size;
                let rgba: Vec<u8> = image.pixels.iter().flat_map(|c| c.to_array()).collect();
                if let Ok(file) = std::fs::File::create(&path) {
                    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
                    enc.set_color(png::ColorType::Rgba);
                    enc.set_depth(png::BitDepth::Eight);
                    if let Ok(mut writer) = enc.write_header() {
                        let _ = writer.write_image_data(&rgba);
                    }
                }
                std::process::exit(0);
            })));
        }
        ctx.request_repaint();
    }
}

/// The launcher window's HWND.
fn window_handle(frame: &eframe::Frame) -> Option<isize> {
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    match frame.window_handle().ok()?.as_raw() {
        RawWindowHandle::Win32(h) => Some(h.hwnd.get()),
        _ => None,
    }
}

/// Whether process `pid` shows a window yet.
fn has_window(pid: u32) -> bool {
    use windows_sys::Win32::Foundation::{HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible};
    struct Search {
        pid: u32,
        found: bool,
    }
    unsafe extern "system" fn visit(hwnd: HWND, data: LPARAM) -> i32 {
        // SAFETY: `data` is the Search below, alive for the enumeration.
        let search = unsafe { &mut *(data as *mut Search) };
        let mut pid = 0;
        // SAFETY: a window handle Windows passed, and a local to write to.
        unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
        // SAFETY: the same window handle.
        if pid == search.pid && unsafe { IsWindowVisible(hwnd) } != 0 {
            search.found = true;
            return 0;
        }
        1
    }
    let mut search = Search { pid, found: false };
    // SAFETY: `visit` only reads windows and writes `search`.
    unsafe { EnumWindows(Some(visit), &mut search as *mut Search as LPARAM) };
    search.found
}

/// Hides or shows (and brings forward) the launcher window; callable from
/// any thread.
fn show_window(window: Option<isize>, show: bool) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{SW_HIDE, SW_SHOW, SetForegroundWindow, ShowWindow};
    let Some(hwnd) = window else { return };
    let hwnd = hwnd as *mut core::ffi::c_void;
    // SAFETY: the launcher's own window handle, alive while it runs.
    unsafe {
        ShowWindow(hwnd, if show { SW_SHOW } else { SW_HIDE });
        if show {
            SetForegroundWindow(hwnd);
        }
    }
}

/// Windows' fonts, where present: Segoe UI for text; Segoe UI Symbol as a
/// fallback for the PlayStation face-button symbols (the controller
/// labels' ✕ ○ □ △), which egui's fonts lack; and the icon font (Segoe
/// Fluent Icons, or Segoe MDL2 Assets before Windows 11) for the main
/// screen's buttons. Returns whether an icon font was found.
fn add_fonts(ctx: &egui::Context) -> bool {
    let read = |name: &str| std::fs::read(Path::new(r"C:\Windows\Fonts").join(name)).ok();
    let mut fonts = egui::FontDefinitions::default();
    let add = |fonts: &mut egui::FontDefinitions, key: &str, bytes: Vec<u8>| {
        fonts.font_data.insert(key.into(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
    };
    if let Some(bytes) = read("segoeui.ttf") {
        add(&mut fonts, "segoe", bytes);
        fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "segoe".into());
    }
    if let Some(bytes) = read("seguisym.ttf") {
        add(&mut fonts, "symbols", bytes);
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push("symbols".into());
        }
    }
    let icons = read("SegoeIcons.ttf").or_else(|| read("segmdl2.ttf"));
    let found = icons.is_some();
    if let Some(bytes) = icons {
        add(&mut fonts, ICON_FAMILY, bytes);
        fonts.families.insert(egui::FontFamily::Name(ICON_FAMILY.into()), vec![ICON_FAMILY.into()]);
    }
    ctx.set_fonts(fonts);
    found
}

/// The dark look, lit in the art's teal.
fn set_style(ctx: &egui::Context) {
    use egui::{Color32, FontFamily, FontId, Stroke, TextStyle};
    let mut v = egui::Visuals::dark();
    v.selection.bg_fill = TEAL_DARK;
    v.selection.stroke = Stroke::new(1.0, TEAL);
    v.hyperlink_color = TEAL;
    v.widgets.inactive.weak_bg_fill = Color32::from_rgb(22, 28, 30);
    v.widgets.inactive.bg_fill = Color32::from_rgb(22, 28, 30);
    v.widgets.hovered.weak_bg_fill = TEAL_DEEP;
    v.widgets.hovered.bg_fill = TEAL_DEEP;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, TEAL);
    v.widgets.active.weak_bg_fill = TEAL_DARK;
    v.widgets.active.bg_fill = TEAL_DARK;
    v.widgets.active.bg_stroke = Stroke::new(1.0, TEAL);
    v.widgets.open.weak_bg_fill = TEAL_DEEP;
    v.extreme_bg_color = Color32::from_rgb(4, 7, 8);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_gray(212));
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_gray(225));
    v.window_fill = Color32::from_rgb(10, 14, 16);
    v.panel_fill = Color32::from_rgb(6, 10, 12);
    ctx.set_visuals(v);
    ctx.global_style_mut(|style| {
        style.text_styles.insert(TextStyle::Heading, FontId::new(26.0, FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
        style.text_styles.insert(TextStyle::Button, FontId::new(15.0, FontFamily::Proportional));
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
    });
}

/// Runs the launcher until it is closed.
pub fn run() -> ExitCode {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("PE2PC")
        .with_inner_size(WINDOW)
        .with_resizable(false)
        .with_maximize_button(false);
    if std::env::var_os("PE2_LAUNCHER_SHOT").is_some() {
        viewport = viewport.with_active(false);
    }
    if let Some((rgba, side)) = crate::icon::rgba() {
        viewport = viewport.with_icon(egui::IconData { rgba, width: side, height: side });
    }
    let options = eframe::NativeOptions { viewport, renderer: eframe::Renderer::Glow, ..Default::default() };
    let creator = Box::new(|cc: &eframe::CreationContext<'_>| {
        let icons = add_fonts(&cc.egui_ctx);
        set_style(&cc.egui_ctx);
        Ok(Box::new(Launcher::new(Art::load(&cc.egui_ctx), icons)) as Box<dyn eframe::App>)
    });
    match eframe::run_native("Parasite Eve II", options, creator) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("pe2: launcher: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egui_keys_become_mapping_names() {
        assert_eq!(egui_key_name(egui::Key::Z).as_deref(), Some("KeyZ"));
        assert_eq!(egui_key_name(egui::Key::Num7).as_deref(), Some("Digit7"));
        assert_eq!(egui_key_name(egui::Key::ArrowLeft).as_deref(), Some("ArrowLeft"));
        assert_eq!(egui_key_name(egui::Key::OpenBracket).as_deref(), Some("BracketLeft"));
        assert_eq!(egui_key_name(egui::Key::Enter).as_deref(), Some("Enter"));
    }

    /// Settings written into the example file read back the same, through
    /// the game's own settings parser, and the comments stay.
    #[test]
    fn settings_round_trip_through_the_game_parser() {
        let mut doc: toml_edit::DocumentMut = EXAMPLE.parse().unwrap();
        let mut s = Settings::load(&doc);
        s.language = Lang::Pt;
        s.confirm_exit = false;
        s.experimental = true;
        s.discord = false;
        s.hd_fonts = false;
        s.discs = [r"D:\Games\PE2 (Disc 1).chd".into(), String::new()];
        s.scale = 6;
        s.subpixel = false;
        s.bilinear = true;
        s.fullscreen = true;
        s.hide_cursor = false;
        s.graphics_api = "opengl".into();
        s.window_size = "1280x960".into();
        s.card = "cards/one.mcd".into();
        s.controller = "Xbox 360 Controller".into();
        s.vibration = false;
        s.keys[4] = "KeyJ".into(); // cross
        s.pads[8] = "Mode".into(); // start
        s.store(&mut doc);
        let text = doc.to_string();
        assert!(text.contains("# Settings for pe2."), "comments kept");
        let config: crate::config::Config = toml::from_str(&text).expect("the game reads it");
        assert_eq!(config.general.language.as_deref(), Some("pt"));
        assert_eq!(config.general.confirm_exit, Some(false));
        assert_eq!(config.general.experimental, Some(true));
        assert_eq!(config.general.discord, Some(false));
        assert_eq!(config.video.hd_fonts, Some(false));
        assert_eq!(config.discs.disc1.as_deref(), Some(Path::new(r"D:\Games\PE2 (Disc 1).chd")));
        assert_eq!(config.discs.disc2, None);
        assert_eq!(config.video.scale, Some(6));
        assert_eq!(config.video.subpixel, Some(false));
        assert_eq!(config.video.texture_filter.as_deref(), Some("bilinear"));
        assert_eq!(config.video.hide_cursor, Some(false));
        assert_eq!(config.video.graphics_api.as_deref(), Some("opengl"));
        assert_eq!(config.video.window_size.as_deref(), Some("1280x960"));
        assert_eq!(config.memcard.slot1.as_deref(), Some(Path::new("cards/one.mcd")));
        assert_eq!(config.input.controller.as_deref(), Some("Xbox 360 Controller"));
        assert_eq!(config.input.vibration, Some(false));
        assert_eq!(config.input.keyboard.get("cross").map(String::as_str), Some("KeyJ"));
        assert_eq!(config.input.keyboard.get("up").map(String::as_str), Some("ArrowUp"));
        assert_eq!(config.input.gamepad.get("start").map(String::as_str), Some("Mode"));
        let again = Settings::load(&doc);
        assert!(again.keys[4] == "KeyJ" && again.pads[8] == "Mode" && !again.vibration);
        assert!(again.language == Lang::Pt && again.scale == 6 && again.bilinear && !again.subpixel);
        assert!(again.graphics_api == "opengl" && again.window_size == "1280x960");
    }
}
