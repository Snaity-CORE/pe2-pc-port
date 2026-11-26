//! The window: PE2PC, the USA discs it plays and the other release's
//! discs to import, then the import's progress and what it made.

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};

use eframe::egui;

use crate::i18n::{LANGS, Lang};
use crate::lang::RELEASES;
use crate::pe2pc;
use crate::pipeline::{self, Done, Event, Job, Step};

const WINDOW: [f32; 2] = [640.0, 560.0];
/// The width of the chosen files' paths.
const PATH_WIDTH: f32 = 340.0;
const STEPS: [&str; 4] =
    ["Reading the USA discs", "Reading your discs", "Making the mod", "Converting the movies with text"];

struct App {
    lang: Lang,
    pe2: Option<PathBuf>,
    usa: [Option<PathBuf>; 2],
    release: [Option<PathBuf>; 2],
    /// The import running: its events, the step it is at, PE2PC's last line.
    running: Option<(Receiver<Event>, usize, String)>,
    result: Option<crate::Result<Done>>,
}

impl App {
    fn new(pe2: Option<PathBuf>) -> App {
        let mut app = App {
            lang: Lang::system(),
            pe2: None,
            usa: [None, None],
            release: [None, None],
            running: None,
            result: None,
        };
        app.set_pe2(pe2.or_else(pe2pc::next_to_me));
        app
    }

    /// PE2PC's program, and the USA discs its settings name.
    fn set_pe2(&mut self, pe2: Option<PathBuf>) {
        if let Some(pe2) = &pe2 {
            let discs = pe2pc::settings(&pe2pc::data_dir(pe2)).discs;
            for (slot, disc) in self.usa.iter_mut().zip(discs) {
                slot.get_or_insert(disc);
            }
        }
        self.pe2 = pe2;
    }

    fn ready(&self) -> bool {
        self.pe2.is_some() && self.usa.iter().chain(&self.release).all(Option::is_some)
    }

    fn start(&mut self, ctx: &egui::Context) {
        let Some(pe2) = self.pe2.clone() else { return };
        let job = Job {
            data: pe2pc::data_dir(&pe2),
            pe2,
            usa: self.usa.iter().flatten().cloned().collect(),
            release: self.release.iter().flatten().cloned().collect(),
            work: std::env::temp_dir().join("pe2-lang-import"),
        };
        let (send, receive) = channel();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            pipeline::run(&job, &|event| {
                let _ = send.send(event);
                ctx.request_repaint();
            })
        });
        self.running = Some((receive, 0, String::new()));
        self.result = None;
    }

    /// The import's events since the last frame.
    fn poll(&mut self) {
        let Some((events, step, line)) = &mut self.running else { return };
        while let Ok(event) = events.try_recv() {
            match event {
                Event::Step(s) => {
                    *step = match s {
                        Step::ExtractUsa => 0,
                        Step::ExtractRelease => 1,
                        Step::Import => 2,
                        Step::Movies => 3,
                    };
                    line.clear();
                }
                Event::Line(l) => *line = l,
                Event::Done(result) => {
                    self.result = Some(result);
                    self.running = None;
                    return;
                }
            }
        }
    }

    fn file_row(
        ui: &mut egui::Ui,
        lang: Lang,
        label: &str,
        path: &mut Option<PathBuf>,
        filter: (&str, &[&str]),
    ) -> bool {
        ui.label(label);
        let shown = path.as_ref().map_or_else(|| lang.t("Not chosen").to_string(), |p| p.display().to_string());
        ui.allocate_ui_with_layout(
            egui::vec2(PATH_WIDTH, 20.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| ui.add(egui::Label::new(shown).truncate()),
        );
        let mut changed = false;
        if ui.button(lang.t("Browse...")).clicked() {
            let mut dialog = rfd::FileDialog::new();
            if !filter.1.is_empty() {
                dialog = dialog.add_filter(filter.0, filter.1);
            }
            if let Some(file) = dialog.pick_file() {
                *path = Some(file);
                changed = true;
            }
        }
        ui.end_row();
        changed
    }

    fn form(&mut self, ui: &mut egui::Ui) {
        let lang = self.lang;
        let disc = (lang.t("CD image"), &["chd", "cue"][..]);
        egui::Grid::new("files").num_columns(3).spacing([12.0, 8.0]).min_col_width(130.0).show(ui, |ui| {
            let mut pe2 = self.pe2.clone();
            // a program has no extension but on Windows
            let program = ("PE2PC", if cfg!(windows) { &["exe"][..] } else { &[][..] });
            if App::file_row(ui, lang, "PE2PC", &mut pe2, program) {
                self.set_pe2(pe2);
            }
            App::file_row(ui, lang, lang.t("USA disc 1"), &mut self.usa[0], disc);
            App::file_row(ui, lang, lang.t("USA disc 2"), &mut self.usa[1], disc);
            App::file_row(ui, lang, lang.t("Your disc 1"), &mut self.release[0], disc);
            App::file_row(ui, lang, lang.t("Your disc 2"), &mut self.release[1], disc);
        });
        if self.pe2.is_none() {
            ui.label(lang.t("Choose PE2PC's program (pe2.exe): the import reads the discs with it."));
        }
        ui.add_space(12.0);
        if ui.add_enabled(self.ready(), egui::Button::new(lang.t("Make the mod"))).clicked() {
            self.start(ui.ctx());
        }
    }

    fn progress(ui: &mut egui::Ui, lang: Lang, step: usize, line: &str) {
        for (k, text) in STEPS.iter().enumerate() {
            ui.horizontal(|ui| {
                if k == step {
                    ui.add(egui::Spinner::new().size(16.0));
                } else {
                    ui.add_enabled(false, egui::Checkbox::without_text(&mut (k < step)));
                }
                ui.label(lang.t(text));
            });
        }
        ui.add_space(8.0);
        ui.add(egui::Label::new(egui::RichText::new(line).small().weak()).truncate());
        ui.add_space(8.0);
        ui.label(lang.t(
            "This takes a few minutes. The discs are read into a temporary folder (about 2 GB), removed at the end.",
        ));
    }

    fn outcome(&self, ui: &mut egui::Ui) {
        let lang = self.lang;
        match &self.result {
            Some(Ok(done)) => {
                ui.heading(lang.t("The mod is made."));
                ui.label(format!("{}: {}", done.made.release.name, done.folder.display()));
                ui.label(format!(
                    "{} {}, {} {}, {} {}",
                    done.made.lines,
                    lang.t("lines of dialogue"),
                    done.made.texts + done.made.details,
                    lang.t("texts of the menus"),
                    done.made.captions,
                    lang.t("loading captions")
                ));
                if let Err(why) = &done.movies {
                    ui.label(format!("{} ({why})", lang.t("The movies with text stay in English.")));
                }
                ui.add_space(8.0);
                let data = self.pe2.as_deref().map(pe2pc::data_dir).unwrap_or_default();
                if !pe2pc::settings(&data).mods_on {
                    ui.label(lang.t("Turn on Experimental features (mods) in PE2PC's Settings > General."));
                }
                ui.label(lang.t("In PE2PC's Mods tab, the mod is on. For PE2PC's own screens in your language too: Settings > General > Language."));
                let others: Vec<&str> = RELEASES
                    .iter()
                    .filter(|r| r.folder != done.made.release.folder)
                    .filter(|r| {
                        let path = data.join("mods").join(r.folder).join("mod.toml");
                        let table = std::fs::read_to_string(path).ok().and_then(|t| t.parse::<toml::Table>().ok());
                        table.and_then(|t| t.get("enabled")?.as_bool()).unwrap_or(false)
                    })
                    .map(|r| r.name)
                    .collect();
                if !others.is_empty() {
                    ui.label(format!("{} {}", lang.t("Also on, turn it off in the Mods tab:"), others.join(", ")));
                }
            }
            Some(Err(e)) => {
                ui.heading(lang.t("The import failed."));
                ui.label(e.to_string());
            }
            None => {}
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll();
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.lang.t("Languages for PE2PC"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_id_salt("language").selected_text(LANGS[self.lang.0].1).show_ui(ui, |ui| {
                        for (k, l) in LANGS.iter().enumerate() {
                            ui.selectable_value(&mut self.lang, Lang(k), l.1);
                        }
                    });
                });
            });
            ui.add_space(6.0);
            ui.label(self.lang.t(
                "Makes a PE2PC mod of the dialogue, menus, items, pictures of text, loading captions and movies of your own French, German, Spanish or Italian discs of Parasite Eve II, or of the dialogue and movies of your Japanese discs. Nothing of the game comes with this program: all is read from your discs.",
            ));
            ui.add_space(12.0);
            match &self.running {
                Some((_, step, line)) => App::progress(ui, self.lang, *step, line),
                None => {
                    self.form(ui);
                    ui.add_space(12.0);
                    self.outcome(ui);
                }
            }
        });
    }
}

/// Runs the window until it is closed; `pe2` is PE2PC's program, if given.
pub fn run(pe2: Option<PathBuf>) -> crate::Result<()> {
    let viewport =
        egui::ViewportBuilder::default().with_title("PE2PC").with_inner_size(WINDOW).with_min_inner_size(WINDOW);
    let options = eframe::NativeOptions { viewport, renderer: eframe::Renderer::Glow, ..Default::default() };
    eframe::run_native(
        "pe2-lang-import",
        options,
        Box::new(move |cc| {
            // dark, as PE2PC's launcher
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(App::new(pe2)) as Box<dyn eframe::App>)
        }),
    )
    .map_err(|e| e.to_string().into())
}
