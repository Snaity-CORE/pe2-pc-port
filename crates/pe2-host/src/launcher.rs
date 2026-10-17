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
        let texture = |name: &str, bytes: &[u8]| Some(ctx.load_texture(name, decode_webp(bytes)?, egui::TextureOptions::LINEAR));
        Some(Art { background: texture("background", BACKGROUND)?, logo: texture("logo", LOGO)? })
    }
}

/// The main screen's buttons.
#[derive(Clone, Copy)]
enum Icon {
    Play,
    Settings,
    Controls,
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
    discs: [String; 2],
    scale: i64,
    hardware: bool,
    subpixel: bool,
    perspective: bool,
    bilinear: bool,
    smooth_backgrounds: bool,
    interpolation: bool,
    fullscreen: bool,
    /// Empty: the default card in memcards/ next to pe2.exe.
    cards: [String; 2],
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
            discs: [text("discs", "disc1"), text("discs", "disc2")],
            scale: get("video", "scale").and_then(|v| v.as_integer()).unwrap_or(1).clamp(1, 8),
            hardware: get("video", "renderer").and_then(|v| v.as_str()) != Some("software"),
            subpixel: flag("video", "subpixel", true),
            perspective: flag("video", "perspective", true),
            bilinear: get("video", "texture_filter").and_then(|v| v.as_str()) == Some("bilinear"),
            smooth_backgrounds: get("video", "background_filter").and_then(|v| v.as_str()) == Some("bicubic"),
            interpolation: flag("video", "frame_interpolation", true),
            fullscreen: flag("video", "fullscreen", false),
            cards: [text("memcard", "slot1"), text("memcard", "slot2")],
            controller: get("input", "controller").and_then(|v| v.as_str()).filter(|n| *n != "auto").unwrap_or("").to_string(),
            vibration: flag("input", "vibration", true),
            modern: get("input", "movement").and_then(|v| v.as_str()) == Some("modern"),
            quick_turn: flag("input", "quick_turn", true),
            keys: std::array::from_fn(|i| {
                let b = &BUTTONS[i];
                doc.get("input").and_then(|t| t.get("keyboard")).and_then(|t| t.get(b.name)).and_then(|v| v.as_str()).unwrap_or(b.key).to_string()
            }),
            pads: std::array::from_fn(|i| {
                let b = &BUTTONS[i];
                doc.get("input").and_then(|t| t.get("gamepad")).and_then(|t| t.get(b.name)).and_then(|v| v.as_str()).unwrap_or(b.pad).to_string()
            }),
        }
    }

    /// Writes the options into `doc`, keeping everything else in it.
    fn store(&self, doc: &mut toml_edit::DocumentMut) {
        use toml_edit::value;
        doc["general"]["language"] = value(self.language.code());
        doc["general"]["confirm_exit"] = value(self.confirm_exit);
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
        path("memcard", "slot1", &self.cards[0]);
        path("memcard", "slot2", &self.cards[1]);
        doc["video"]["scale"] = value(self.scale);
        doc["video"]["renderer"] = value(if self.hardware { "hardware" } else { "software" });
        doc["video"]["subpixel"] = value(self.subpixel);
        doc["video"]["perspective"] = value(self.perspective);
        doc["video"]["texture_filter"] = value(if self.bilinear { "bilinear" } else { "nearest" });
        doc["video"]["background_filter"] = value(if self.smooth_backgrounds { "bicubic" } else { "nearest" });
        doc["video"]["frame_interpolation"] = value(self.interpolation);
        doc["video"]["fullscreen"] = value(self.fullscreen);
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
    /// Controllers, for their names and to catch their buttons.
    gilrs: Option<gilrs::Gilrs>,
    art: Option<Art>,
    /// Windows' icon font was found.
    icons: bool,
    /// PE2_LAUNCHER_SHOT: frames drawn, to take the picture once settled.
    frames: u32,
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
            capture: None,
            gilrs: gilrs::Gilrs::new().ok(),
            art,
            icons,
            frames: 0,
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
        match std::fs::write(&self.file, self.doc.to_string()) {
            Ok(()) => {
                self.saved = self.edit.clone();
                self.message = None;
            }
            Err(e) => self.message = Some(format!("{}: {e}", self.file.display())),
        }
    }

    /// Runs the game as a child process, hiding the launcher until it quits.
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
            // goes to pe2.log next to pe2.exe instead.
            use std::os::windows::process::CommandExt;
            use std::process::Stdio;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.creation_flags(CREATE_NO_WINDOW).stdin(Stdio::null()).stdout(Stdio::null());
            let log = std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join("pe2.log")));
            match log.and_then(|p| std::fs::File::create(p).ok()) {
                Some(file) => command.stderr(file),
                None => command.stderr(Stdio::null()),
            };
        }
        let mut child = match command.spawn() {
            Ok(c) => c,
            Err(e) => {
                self.message = Some(e.to_string());
                return;
            }
        };
        self.running.store(true, Ordering::SeqCst);
        show_window(window, false);
        let (running, ctx) = (self.running.clone(), ctx.clone());
        std::thread::spawn(move || {
            let _ = child.wait();
            running.store(false, Ordering::SeqCst);
            show_window(window, true);
            ctx.request_repaint();
        });
    }

    /// One of the main screen's buttons at `rect`: a dark glass panel with
    /// an icon and a label, lit in the accent colour when pointed at or
    /// focused; `primary` (Play) is lit always.
    fn menu_button(&self, ui: &mut egui::Ui, rect: egui::Rect, icon: Icon, label: &str, enabled: bool, primary: bool) -> egui::Response {
        use egui::{Color32, FontFamily, FontId, Stroke, StrokeKind, pos2};
        let response = ui.allocate_rect(rect, if enabled { egui::Sense::click() } else { egui::Sense::hover() });
        let lit = enabled && (response.hovered() || response.has_focus());
        let (fill, stroke) = match (primary && enabled, lit) {
            (true, false) => (Color32::from_rgba_unmultiplied(8, 78, 74, 215), Stroke::new(1.5, TEAL)),
            (true, true) => (Color32::from_rgba_unmultiplied(14, 112, 106, 230), Stroke::new(2.0, TEAL)),
            (false, false) => (Color32::from_rgba_unmultiplied(12, 16, 18, 200), Stroke::new(1.0, Color32::from_gray(58))),
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
                    vec![pos2(center.x - w / 2.0, center.y - h), pos2(center.x + w, center.y), pos2(center.x - w / 2.0, center.y + h)],
                    colour,
                    Stroke::NONE,
                ));
            }
            _ if self.icons => {
                painter.text(center, egui::Align2::CENTER_CENTER, icon.glyph(), FontId::new(24.0, FontFamily::Name(ICON_FAMILY.into())), text);
            }
            _ => {}
        }
        painter.text(pos2(rect.left() + 84.0, rect.center().y), egui::Align2::LEFT_CENTER, label, FontId::proportional(21.0), text);
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
                ui.painter().image(art.logo.id(), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), egui::Color32::WHITE);
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
        let buttons = [
            (Icon::Play, lang.t("Play", "Jogar"), ready && !running, true),
            (Icon::Settings, lang.t("Settings", "Configurações"), !running, false),
            (Icon::Controls, lang.t("Controls", "Controles"), !running, false),
            (Icon::Setup, lang.t("Setup", "Configuração inicial"), !running, false),
            (Icon::Exit, lang.t("Exit", "Sair"), true, false),
        ];
        let mut y = full.top() + 282.0;
        for (icon, label, enabled, primary) in buttons {
            let rect = Rect::from_min_size(pos2(full.left() + 73.0, y), vec2(324.0, 50.0));
            if self.menu_button(ui, rect, icon, label, enabled, primary).clicked() {
                match icon {
                    Icon::Play => self.play(ui.ctx(), window),
                    Icon::Settings => {
                        self.edit = self.saved.clone();
                        self.screen = Screen::Settings;
                    }
                    Icon::Controls => {
                        self.edit = self.saved.clone();
                        self.capture = None;
                        self.screen = Screen::Controls;
                    }
                    Icon::Setup => {
                        self.edit = self.saved.clone();
                        self.screen = Screen::Setup(0);
                    }
                    Icon::Exit => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
                }
            }
            y += 58.0;
        }
        let status = if running {
            Some((egui::Color32::from_gray(200), lang.t("The game is running.", "O jogo está rodando.")))
        } else if !ready {
            Some((egui::Color32::from_rgb(255, 120, 110), lang.t("Choose the disc images in Setup.", "Escolha as imagens dos discos na Configuração inicial.")))
        } else {
            None
        };
        if let Some((colour, text)) = status {
            ui.painter().text(pos2(full.left() + 75.0, y + 6.0), egui::Align2::LEFT_TOP, text, egui::FontId::proportional(15.0), colour);
        }
        if let Some(m) = &self.message {
            ui.painter().text(pos2(full.left() + 75.0, y + 28.0), egui::Align2::LEFT_TOP, m, egui::FontId::proportional(14.0), egui::Color32::from_rgb(255, 120, 110));
        }
    }

    /// Disc and memory card pickers (Setup and Settings).
    fn files(&mut self, ui: &mut egui::Ui, cards: bool) {
        let lang = self.lang();
        for d in 0..2 {
            ui.label(format!("{} {}", lang.t("Disc", "Disco"), d + 1));
            ui.horizontal(|ui| {
                let changed = ui.add(egui::TextEdit::singleline(&mut self.edit.discs[d]).desired_width(560.0)).changed();
                if ui.button(lang.t("Browse...", "Procurar...")).clicked() {
                    if let Some(p) = rfd::FileDialog::new().add_filter("CD image", &["chd", "cue"]).pick_file() {
                        self.edit.discs[d] = p.display().to_string();
                        self.checks[d] = Some(check_disc(&self.edit.discs[d], d, lang));
                    }
                }
                if changed {
                    self.checks[d] = Some(check_disc(&self.edit.discs[d], d, lang));
                }
            });
            match &self.checks[d] {
                Some(Ok(())) => ui.colored_label(egui::Color32::LIGHT_GREEN, lang.t("Found.", "Encontrado.")),
                Some(Err(e)) => ui.colored_label(egui::Color32::LIGHT_RED, e.as_str()),
                None => ui.label(""),
            };
            ui.add_space(6.0);
        }
        if cards {
            for s in 0..2 {
                ui.label(format!("{} {}", lang.t("Memory card, slot", "Memory card, slot"), s + 1));
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.edit.cards[s])
                            .hint_text(lang.t("default (memcards folder)", "padrão (pasta memcards)"))
                            .desired_width(560.0),
                    );
                    if ui.button(lang.t("Browse...", "Procurar...")).clicked() {
                        if let Some(p) = rfd::FileDialog::new().add_filter("Memory card", &["mcd", "mcr"]).save_file() {
                            self.edit.cards[s] = p.display().to_string();
                        }
                    }
                });
            }
        }
    }

    fn graphics(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        let e = &mut self.edit;
        ui.horizontal(|ui| {
            ui.label(lang.t("Resolution", "Resolução"));
            egui::ComboBox::from_id_salt("scale")
                .selected_text(format!("{}x ({}x{})", e.scale, 320 * e.scale, 240 * e.scale))
                .show_ui(ui, |ui| {
                    for s in 1..=8 {
                        ui.selectable_value(&mut e.scale, s, format!("{s}x ({}x{})", 320 * s, 240 * s));
                    }
                });
        });
        ui.checkbox(&mut e.hardware, lang.t("Graphics card renderer", "Renderizar pela placa de vídeo"));
        ui.add_enabled_ui(e.scale > 1, |ui| {
            ui.checkbox(&mut e.subpixel, lang.t("Sub-pixel precision (no wobbling)", "Precisão subpixel (sem tremido)"));
            ui.add_enabled_ui(e.subpixel, |ui| {
                ui.checkbox(&mut e.perspective, lang.t("Perspective-correct textures", "Texturas com correção de perspectiva"));
            });
            ui.add_enabled_ui(e.hardware, |ui| {
                ui.checkbox(&mut e.bilinear, lang.t("Smooth 3D textures (bilinear)", "Suavizar texturas 3D (bilinear)"));
                ui.checkbox(&mut e.smooth_backgrounds, lang.t("Smooth backgrounds (bicubic)", "Suavizar cenários (bicúbico)"));
            });
            ui.add_enabled_ui(e.hardware && e.subpixel, |ui| {
                ui.checkbox(&mut e.interpolation, lang.t("Smooth motion (60 fps)", "Movimento suave (60 fps)"));
            });
        });
        ui.checkbox(&mut e.fullscreen, lang.t("Start in full screen (Alt+Enter toggles)", "Iniciar em tela cheia (Alt+Enter alterna)"));
    }

    fn language(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.edit.language, Lang::En, "English");
            ui.selectable_value(&mut self.edit.language, Lang::Pt, "Português");
        });
    }

    fn setup_screen(&mut self, ui: &mut egui::Ui, step: u8) {
        let lang = self.lang();
        ui.heading(lang.t("Setup", "Configuração inicial"));
        ui.label(format!("{} {} / 4", lang.t("Step", "Passo"), step + 1));
        ui.separator();
        match step {
            0 => {
                ui.label(lang.t("Language", "Idioma"));
                self.language(ui);
                ui.add_space(8.0);
                ui.label(lang.t(
                    "You need your own dumps of Parasite Eve II (USA), discs 1 and 2, as .chd or .cue images.",
                    "Você precisa das suas próprias cópias do Parasite Eve II (USA), discos 1 e 2, em imagens .chd ou .cue.",
                ));
            }
            1 => self.files(ui, false),
            2 => {
                ui.label(lang.t(
                    "Saves go to memory card images. Leave empty for the default ones in the memcards folder next to pe2.exe; DuckStation's .mcd cards work too.",
                    "Os saves ficam em imagens de memory card. Deixe vazio para usar as padrão na pasta memcards ao lado do pe2.exe; os cartões .mcd do DuckStation também funcionam.",
                ));
                ui.add_space(6.0);
                for s in 0..2 {
                    ui.label(format!("Slot {}", s + 1));
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.edit.cards[s])
                                .hint_text(lang.t("default (memcards folder)", "padrão (pasta memcards)"))
                                .desired_width(560.0),
                        );
                        if ui.button(lang.t("Browse...", "Procurar...")).clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("Memory card", &["mcd", "mcr"]).save_file() {
                                self.edit.cards[s] = p.display().to_string();
                            }
                        }
                    });
                }
            }
            _ => self.graphics(ui),
        }
        ui.add_space(12.0);
        ui.separator();
        ui.horizontal(|ui| {
            if step > 0 && ui.button(lang.t("Back", "Voltar")).clicked() {
                self.screen = Screen::Setup(step - 1);
            }
            let discs_ok = self.checks[0].as_ref().is_some_and(|c| c.is_ok());
            if step < 3 {
                if ui.add_enabled(step != 1 || discs_ok, egui::Button::new(lang.t("Next", "Avançar"))).clicked() {
                    self.screen = Screen::Setup(step + 1);
                }
            } else if ui.button(lang.t("Finish", "Concluir")).clicked() {
                self.save();
                self.screen = Screen::Main;
            }
            if self.saved_ready() && ui.button(lang.t("Cancel", "Cancelar")).clicked() {
                self.edit = self.saved.clone();
                self.recheck();
                self.screen = Screen::Main;
            }
        });
        if step == 1 && !self.checks[0].as_ref().is_some_and(|c| c.is_ok()) {
            ui.label(lang.t("Disc 1 is needed to go on (disc 2 can be added later).", "O disco 1 é necessário para continuar (o disco 2 pode ser adicionado depois)."));
        }
    }

    /// Whether the saved settings already have a working disc 1.
    fn saved_ready(&self) -> bool {
        check_disc(&self.saved.discs[0], 0, self.lang()).is_ok()
    }

    fn settings_screen(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang();
        ui.heading(lang.t("Settings", "Configurações"));
        ui.separator();
        // The panel's height, but for the buttons below.
        egui::ScrollArea::vertical().max_height(ui.available_height() - 64.0).show(ui, |ui| {
            ui.strong(lang.t("General", "Geral"));
            self.language(ui);
            ui.checkbox(&mut self.edit.confirm_exit, lang.t("Ask before Esc quits", "Perguntar antes de Esc sair"));
            ui.add_space(8.0);
            ui.strong(lang.t("Graphics", "Gráficos"));
            self.graphics(ui);
            ui.add_space(8.0);
            ui.strong(lang.t("Discs and memory cards", "Discos e memory cards"));
            self.files(ui, true);
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button(lang.t("Save", "Salvar")).clicked() {
                self.save();
                if self.message.is_none() {
                    self.screen = Screen::Main;
                }
            }
            if ui.button(lang.t("Cancel", "Cancelar")).clicked() {
                self.edit = self.saved.clone();
                self.recheck();
                self.screen = Screen::Main;
            }
        });
    }
}

impl Launcher {
    /// A key or controller button for the button being changed, if one was
    /// pressed this frame; Esc gives up.
    fn catch(&mut self, ctx: &egui::Context) {
        let lang = self.lang();
        // Controller events: drained always, used while catching one.
        let mut pressed = None;
        if let Some(g) = self.gilrs.as_mut() {
            while let Some(ev) = g.next_event() {
                if let gilrs::EventType::ButtonPressed(b, _) = ev.event {
                    pressed = Some(format!("{b:?}"));
                }
            }
        }
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
                self.message = Some(format!("{}: {}", key_label(&name), lang.t("reserved (Esc, P, F9-F12, Alt)", "reservada (Esc, P, F9-F12, Alt)")));
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
        // Keep polling the controllers while this screen is open.
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(30));
        let lang = self.lang();
        ui.heading(lang.t("Controls", "Controles"));
        ui.horizontal(|ui| {
            ui.label(lang.t("Movement", "Movimento"));
            ui.selectable_value(&mut self.edit.modern, false, lang.t("Tank (original)", "Tanque (original)"));
            ui.selectable_value(&mut self.edit.modern, true, lang.t("Modern (relative to the camera)", "Moderno (relativo à câmera)"));
        });
        ui.add_enabled_ui(!self.edit.modern, |ui| {
            ui.checkbox(&mut self.edit.quick_turn, lang.t("Quick turn (back + run)", "Meia-volta rápida (trás + correr)"));
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.pad_tab, false, lang.t("Keyboard", "Teclado"));
            ui.selectable_value(&mut self.pad_tab, true, lang.t("Controller", "Controle"));
        });
        ui.separator();
        if self.pad_tab {
            let names: Vec<String> =
                self.gilrs.as_ref().map(|g| g.gamepads().map(|(_, p)| p.name().to_string()).collect()).unwrap_or_default();
            ui.horizontal(|ui| {
                ui.label(lang.t("Player 1", "Jogador 1"));
                let auto = lang.t("Last one used", "O último usado");
                egui::ComboBox::from_id_salt("controller")
                    .selected_text(if self.edit.controller.is_empty() { auto } else { self.edit.controller.as_str() })
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.edit.controller, String::new(), auto);
                        let mut all = names.clone();
                        if !self.edit.controller.is_empty() && !all.contains(&self.edit.controller) {
                            all.push(self.edit.controller.clone());
                        }
                        for n in all {
                            ui.selectable_value(&mut self.edit.controller, n.clone(), n);
                        }
                    });
            });
            if names.is_empty() {
                ui.label(lang.t("No controller connected.", "Nenhum controle conectado."));
            }
            ui.checkbox(&mut self.edit.vibration, lang.t("Vibration", "Vibração"));
        } else {
            ui.label(lang.t("Esc, P, F9-F12 and Alt keep their own jobs.", "Esc, P, F9-F12 e Alt mantêm suas funções."));
        }
        ui.add_space(4.0);
        egui::ScrollArea::vertical().max_height(ui.available_height() - 64.0).show(ui, |ui| {
            egui::Grid::new("bindings").num_columns(2).spacing([24.0, 4.0]).show(ui, |ui| {
                for (i, b) in BUTTONS.iter().enumerate() {
                    ui.label(b.label);
                    let waiting = self.capture == Some((self.pad_tab, i));
                    let text = if waiting {
                        lang.t("Press a button... (Esc cancels)", "Aperte um botão... (Esc cancela)").to_string()
                    } else if self.pad_tab {
                        pad_label(&self.edit.pads[i])
                    } else {
                        key_label(&self.edit.keys[i])
                    };
                    if ui.add(egui::Button::new(text).min_size([230.0, 0.0].into())).clicked() {
                        self.capture = Some((self.pad_tab, i));
                        self.message = None;
                    }
                    ui.end_row();
                }
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button(lang.t("Save", "Salvar")).clicked() {
                self.capture = None;
                self.save();
                if self.message.is_none() {
                    self.screen = Screen::Main;
                }
            }
            if ui.button(lang.t("Cancel", "Cancelar")).clicked() {
                self.capture = None;
                self.edit = self.saved.clone();
                self.message = None;
                self.screen = Screen::Main;
            }
            if ui.button(lang.t("Restore defaults", "Restaurar padrões")).clicked() {
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

impl eframe::App for Launcher {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let window = window_handle(frame);
        let full = ui.max_rect();
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
    /// settings, controls or setup): saves the launcher's picture once it
    /// settles, and quits. For checking its looks.
    fn debug_shot(&mut self, ctx: &egui::Context) {
        let Some(path) = std::env::var_os("PE2_LAUNCHER_SHOT") else { return };
        self.frames += 1;
        if self.frames == 1 {
            self.screen = match std::env::var("PE2_LAUNCHER_SCREEN").as_deref() {
                Ok("settings") => Screen::Settings,
                Ok("controls") => Screen::Controls,
                Ok("setup") => Screen::Setup(0),
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
    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
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
        s.discs = [r"D:\Games\PE2 (Disc 1).chd".into(), String::new()];
        s.scale = 6;
        s.hardware = true;
        s.subpixel = false;
        s.bilinear = true;
        s.fullscreen = true;
        s.cards = [String::new(), "cards/two.mcd".into()];
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
        assert_eq!(config.discs.disc1.as_deref(), Some(Path::new(r"D:\Games\PE2 (Disc 1).chd")));
        assert_eq!(config.discs.disc2, None);
        assert_eq!(config.video.scale, Some(6));
        assert_eq!(config.video.subpixel, Some(false));
        assert_eq!(config.video.texture_filter.as_deref(), Some("bilinear"));
        assert_eq!(config.memcard.slot2.as_deref(), Some(Path::new("cards/two.mcd")));
        assert_eq!(config.input.controller.as_deref(), Some("Xbox 360 Controller"));
        assert_eq!(config.input.vibration, Some(false));
        assert_eq!(config.input.keyboard.get("cross").map(String::as_str), Some("KeyJ"));
        assert_eq!(config.input.keyboard.get("up").map(String::as_str), Some("ArrowUp"));
        assert_eq!(config.input.gamepad.get("start").map(String::as_str), Some("Mode"));
        let again = Settings::load(&doc);
        assert!(again.keys[4] == "KeyJ" && again.pads[8] == "Mode" && !again.vibration);
        assert!(again.language == Lang::Pt && again.scale == 6 && again.bilinear && !again.subpixel);
    }
}
