//! The whole import: both releases extracted by PE2PC (`pe2 --extract`,
//! each into a work folder of its own), the mod made from them into PE2PC's
//! mods folder, with the release's movies with text (PE2PC's extract writes
//! each movie as a mod's, `<name>.mp4`, and where the discs list it,
//! `<name>.txt`).

use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::Result;
use crate::import::{self, Made};

pub struct Job {
    /// PE2PC's program.
    pub pe2: PathBuf,
    /// PE2PC's data folder: the mod goes to its mods/.
    pub data: PathBuf,
    pub usa: Vec<PathBuf>,
    pub release: Vec<PathBuf>,
    /// Where the extracts go (removed after).
    pub work: PathBuf,
}

pub enum Step {
    ExtractUsa,
    ExtractRelease,
    Import,
    Movies,
}

pub enum Event {
    Step(Step),
    /// A line of PE2PC's extract.
    Line(String),
    Done(Result<Done>),
}

pub struct Done {
    pub made: Made,
    pub folder: PathBuf,
    /// The movies with text put in the mod, or why not.
    pub movies: std::result::Result<usize, String>,
}

/// Runs `job`, telling `tell` how it goes.
pub fn run(job: &Job, tell: &dyn Fn(Event)) {
    let result = run_steps(job, tell);
    let _ = std::fs::remove_dir_all(&job.work);
    tell(Event::Done(result));
}

fn run_steps(job: &Job, tell: &dyn Fn(Event)) -> Result<Done> {
    let (usa, other) = (job.work.join("usa"), job.work.join("release"));
    tell(Event::Step(Step::ExtractUsa));
    extract(job, &usa, &job.usa, "files,textures,texts", tell)?;
    tell(Event::Step(Step::ExtractRelease));
    extract(job, &other, &job.release, "files,textures,texts,movies", tell)?;
    tell(Event::Step(Step::Import));
    let (usa, other) = (usa.join("dumps"), other.join("dumps"));
    let release = import::release_of(&other)?;
    let folder = job.data.join("mods").join(release.folder);
    let made = import::make(&usa, &other, &folder, None)?;
    tell(Event::Step(Step::Movies));
    let movies = movies(release.movies, &other.join("movies"), &folder.join("movies"));
    Ok(Done { made, folder, movies })
}

/// `pe2 --extract <parts> <discs>` into `home`'s dumps/.
fn extract(job: &Job, home: &Path, discs: &[PathBuf], parts: &str, tell: &dyn Fn(Event)) -> Result<()> {
    std::fs::create_dir_all(home)?;
    let mut command = Command::new(&job.pe2);
    command.env("PE2_HOME", home).arg("--extract").arg(parts).args(discs);
    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped());
    no_window(&mut command);
    let mut child = command.spawn().map_err(|e| format!("{}: {e}", job.pe2.display()))?;
    let mut last = Vec::new();
    if let Some(err) = child.stderr.take() {
        for line in std::io::BufReader::new(err).lines().map_while(std::io::Result::ok) {
            // without the log's time stamp
            let line =
                line.split_once("] ").filter(|_| line.starts_with('[')).map_or(line.as_str(), |l| l.1).to_string();
            tell(Event::Line(line.clone()));
            last.push(line);
            if last.len() > 5 {
                last.remove(0);
            }
        }
    }
    if !child.wait()?.success() {
        return Err(format!("PE2PC's extract failed:\n{}", last.join("\n")).into());
    }
    Ok(())
}

/// The movies with text (`movies`: lang.rs's), from the release's extract
/// to the mod. Answers how many, or why none.
fn movies(movies: &[(&str, &str)], from: &Path, to: &Path) -> std::result::Result<usize, String> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    // where the discs list each movie -> its name in the release's extract
    let mut names = std::collections::HashMap::new();
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))?.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|x| x != "txt") {
            continue;
        }
        let listed = std::fs::read_to_string(&path)
            .ok()
            .and_then(|t| t.parse::<toml::Table>().ok())
            .and_then(|t| t.get("listed")?.as_str().map(str::to_string));
        if let (Some(listed), Some(name)) = (listed, path.file_stem().and_then(|s| s.to_str())) {
            names.insert(listed, name.to_string());
        }
    }
    let mut copied = 0;
    for &(listed, game) in movies {
        let theirs = names.get(listed).map(|name| from.join(format!("{name}.mp4"))).filter(|p| p.is_file());
        let Some(theirs) = theirs else {
            return Err("this PE2PC's extract writes no .mp4 of the movies: a newer PE2PC does".into());
        };
        let mine = to.join(format!("{game}.mp4"));
        std::fs::copy(&theirs, &mine).map_err(|e| format!("{}: {e}", mine.display()))?;
        copied += 1;
    }
    Ok(copied)
}

/// No console window for a program started from the window.
fn no_window(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    let _ = command;
}
