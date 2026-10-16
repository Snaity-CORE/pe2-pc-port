//! The launcher: what pe2.exe shows when started without arguments.
//!
//! Setup (on first run, or while a disc is missing) picks the disc images,
//! checking them, the memory cards and a quality; Settings edits every
//! option of pe2.toml (keeping the file's comments); Play runs the game as a
//! child process (`pe2 --play`) and the launcher comes back when it quits.
//! English or Portuguese.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use eframe::egui;

/// The disc images' boot executables: Parasite Eve II (USA).
const DISC_SERIALS: [&str; 2] = ["SLUS_010.42", "SLUS_010.55"];

/// The template of a new pe2.toml: the documented example.
const EXAMPLE: &str = include_str!("../../../pe2.example.toml");

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
    fullscreen: bool,
    /// Empty: the default card in memcards/ next to pe2.exe.
    cards: [String; 2],
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
            fullscreen: flag("video", "fullscreen", false),
            cards: [text("memcard", "slot1"), text("memcard", "slot2")],
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
        doc["video"]["fullscreen"] = value(self.fullscreen);
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
}

impl Launcher {
    fn new() -> Launcher {
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
        let mut child = match std::process::Command::new(exe).arg("--play").spawn() {
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

    fn main_screen(&mut self, ui: &mut egui::Ui, window: Option<isize>) {
        let lang = self.lang();
        let ready = self.checks[0].as_ref().is_some_and(|c| c.is_ok());
        ui.vertical_centered(|ui| {
            ui.add_space(24.0);
            ui.heading(egui::RichText::new("Parasite Eve II").size(32.0));
            ui.label(lang.t("PC port", "Port para PC"));
            ui.add_space(28.0);
            let button = |ui: &mut egui::Ui, text: &str, enabled: bool| {
                ui.add_enabled(enabled, egui::Button::new(egui::RichText::new(text).size(18.0)).min_size([220.0, 40.0].into()))
                    .clicked()
            };
            let running = self.running.load(Ordering::SeqCst);
            if button(ui, lang.t("Play", "Jogar"), ready && !running) {
                self.play(ui.ctx(), window);
            }
            ui.add_space(6.0);
            if button(ui, lang.t("Settings", "Configurações"), !running) {
                self.edit = self.saved.clone();
                self.screen = Screen::Settings;
            }
            ui.add_space(6.0);
            if button(ui, lang.t("Setup", "Configuração inicial"), !running) {
                self.edit = self.saved.clone();
                self.screen = Screen::Setup(0);
            }
            ui.add_space(6.0);
            if button(ui, lang.t("Exit", "Sair"), true) {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ui.add_space(18.0);
            if running {
                ui.label(lang.t("The game is running.", "O jogo está rodando."));
            } else if !ready {
                ui.colored_label(egui::Color32::LIGHT_RED, lang.t("Choose the disc images in Setup.", "Escolha as imagens dos discos na Configuração inicial."));
            }
        });
    }

    /// Disc and memory card pickers (Setup and Settings).
    fn files(&mut self, ui: &mut egui::Ui, cards: bool) {
        let lang = self.lang();
        for d in 0..2 {
            ui.label(format!("{} {}", lang.t("Disc", "Disco"), d + 1));
            ui.horizontal(|ui| {
                let changed = ui.add(egui::TextEdit::singleline(&mut self.edit.discs[d]).desired_width(330.0)).changed();
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
                            .desired_width(330.0),
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
                                .desired_width(330.0),
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
        egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
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

impl eframe::App for Launcher {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let window = window_handle(frame);
        egui::Frame::new().inner_margin(16.0).show(ui, |ui| {
            match self.screen {
                Screen::Main => self.main_screen(ui, window),
                Screen::Setup(step) => self.setup_screen(ui, step),
                Screen::Settings => self.settings_screen(ui),
            }
            if let Some(m) = &self.message {
                ui.colored_label(egui::Color32::LIGHT_RED, m.as_str());
            }
        });
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

/// Runs the launcher until it is closed.
pub fn run() -> ExitCode {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Parasite Eve II")
            .with_inner_size([560.0, 470.0])
            .with_resizable(false),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    match eframe::run_native("Parasite Eve II", options, Box::new(|_| Ok(Box::new(Launcher::new())))) {
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
        let again = Settings::load(&doc);
        assert!(again.language == Lang::Pt && again.scale == 6 && again.bilinear && !again.subpixel);
    }
}
