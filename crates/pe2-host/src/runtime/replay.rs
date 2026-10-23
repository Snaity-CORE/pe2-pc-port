//! Input recordings, to bring the game back to a place for debugging.
//!
//! With `[debug] record_input = true` in pe2.toml (off by default), each
//! session records the controller as the game reads it, into
//! `recordings/<date>_<time>.txt` in the folder of the player's files, with
//! the memory card as it was at power-on beside it (`.mcd`). With
//! `PE2_REPLAY=<recording>.txt`, the game plays it back from power-on with
//! a copy of that card, then pauses (P resumes, with the controls back),
//! saving a screenshot.
//!
//! Input is tied to the game's main-loop updates (the fork's gameflow.c
//! calls `pc_pad_frame` before sampling the buttons), not to time: a load
//! that takes a little longer does not shift what follows. While the main
//! loop is held up (movies, some screens) for STALL vertical blanks or
//! more, input is tied to the blanks counted since its last update.
//! Between those points the game's pad buffers hold what was taken at the
//! last one, both while recording and while replaying, so both see the
//! same.
//!
//! The file: `#` lines (the movement options, the card), then one line per
//! change, `<update> <blanks> <buttons> <analog> <sticks>` (buttons and
//! sticks in hex), and a lone `<update>` every HEARTBEAT updates so a
//! replay knows how long the session ran. To tell where a replay goes out
//! of step, also `p <update> <x> <z> <yaw>` when the player moves (the
//! fork's pad_input.c calls `pc_replay_player` in play), and
//! `t <update> <blanks>` when the blanks an update takes change; a replay
//! reports the first difference in where the player is, and the first few
//! in timing.

use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// What port 1 holds: buttons (active high), DualShock or digital pad, and
/// the sticks' four bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PadInput {
    pub held: u16,
    pub analog: bool,
    pub sticks: u32,
}

/// Blanks without a main-loop update after which input is taken at blanks.
const STALL: u32 = 8;
/// Updates between the heartbeat lines.
const HEARTBEAT: u64 = 30;
/// Recordings kept (the oldest go).
const KEPT: usize = 10;

/// Where the player is: x, z and yaw.
type Place = (i32, i32, i32);

struct Recorder {
    out: std::io::BufWriter<std::fs::File>,
    last: Option<PadInput>,
    place: Option<Place>,
    timing: Option<u32>,
}

struct Player {
    /// (update, blanks, input), in order.
    events: Vec<(u64, u32, PadInput)>,
    next: usize,
    /// The update the recording ran to.
    end: u64,
    /// Where the player was from each update on, and the blanks updates
    /// took from each update on (in order), with the next of each to
    /// compare.
    places: Vec<(u64, Place)>,
    timings: Vec<(u64, u32)>,
    next_place: usize,
    next_timing: usize,
    /// Reported: the player out of place, and timing differences.
    out_of_place: bool,
    timing_reports: u32,
}

enum Mode {
    Off,
    Record(Recorder),
    Replay(Player),
}

struct State {
    mode: Mode,
    /// Main-loop updates since power-on, and blanks since the last one.
    update: u64,
    blanks: u32,
    /// What the game's pad buffers get, while recording or replaying.
    latched: Option<PadInput>,
}

static STATE: Mutex<State> = Mutex::new(State { mode: Mode::Off, update: 0, blanks: 0, latched: None });

/// The movement options of the recording replayed (u32::MAX: none). Not
/// in STATE: the game asks for them without the interrupts held off.
static OPTIONS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);

/// Set when a replay has run to its end (the window pauses the game).
static FINISHED: AtomicBool = AtomicBool::new(false);

/// Starts replaying PE2_REPLAY, or recording this session. Before the game
/// runs.
pub fn start() {
    let mode = match std::env::var_os("PE2_REPLAY") {
        Some(path) => match load(Path::new(&path)) {
            Ok(player) => {
                eprintln!(
                    "pe2: replay: {} ({} changes over {} updates)",
                    Path::new(&path).display(),
                    player.events.len(),
                    player.end
                );
                if player.places.is_empty() {
                    eprintln!("pe2: replay: the recording has no places of the player to check against");
                }
                Mode::Replay(player)
            }
            Err(e) => {
                eprintln!("pe2: replay: {}: {e}", Path::new(&path).display());
                return;
            }
        },
        None if crate::config::get().debug.record_input.unwrap_or(false) => match record() {
            Ok(recorder) => Mode::Record(recorder),
            Err(e) => {
                eprintln!("pe2: recording input: {e}");
                return;
            }
        },
        None => return,
    };
    STATE.lock().unwrap().mode = mode;
}

/// The movement options of the recording being replayed (see
/// controls::pc_input_options).
pub fn input_options() -> Option<u32> {
    let options = OPTIONS.load(Ordering::SeqCst);
    (options != u32::MAX).then_some(options)
}

/// What the pad buffers hold now: None for the live controls.
pub fn current() -> Option<PadInput> {
    STATE.lock().unwrap().latched
}

/// Whether a replay has just run to its end (once).
pub fn take_finished() -> bool {
    FINISHED.swap(false, Ordering::SeqCst)
}

fn record() -> std::io::Result<Recorder> {
    let dir = crate::paths::user_file("recordings");
    std::fs::create_dir_all(&dir)?;
    prune(&dir);
    // SAFETY: fills a plain struct.
    let t = unsafe {
        let mut t = std::mem::zeroed();
        windows_sys::Win32::System::SystemInformation::GetLocalTime(&mut t);
        t
    };
    let stem = format!("{:04}-{:02}-{:02}_{:02}-{:02}-{:02}", t.wYear, t.wMonth, t.wDay, t.wHour, t.wMinute, t.wSecond);
    let card = super::memcard::card_path();
    let card_name = if card.is_file() {
        std::fs::copy(&card, dir.join(format!("{stem}.mcd")))?;
        format!("{stem}.mcd")
    } else {
        "none".to_string()
    };
    let mut out = std::io::BufWriter::new(std::fs::File::create(dir.join(format!("{stem}.txt")))?);
    writeln!(out, "# pe2 input recording 1")?;
    writeln!(out, "# options {}", crate::controls::pc_input_options())?;
    writeln!(out, "# card {card_name}")?;
    out.flush()?;
    Ok(Recorder { out, last: None, place: None, timing: None })
}

/// Deletes the oldest recordings beyond KEPT (with their cards).
fn prune(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut stems: Vec<String> = entries
        .flatten()
        .filter_map(|e| e.file_name().to_str()?.strip_suffix(".txt").map(str::to_string))
        .filter(|s| s.len() == 19 && s.as_bytes()[10] == b'_')
        .collect();
    stems.sort();
    let excess = (stems.len() + 1).saturating_sub(KEPT);
    for stem in &stems[..excess] {
        let _ = std::fs::remove_file(dir.join(format!("{stem}.txt")));
        let _ = std::fs::remove_file(dir.join(format!("{stem}.mcd")));
    }
}

fn load(path: &Path) -> Result<Player, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (mut events, mut end, mut options) = (Vec::new(), 0, None);
    let (mut places, mut timings) = (Vec::new(), Vec::new());
    let mut card = None;
    for (n, line) in text.lines().enumerate() {
        let bad = || format!("line {}: {line:?}", n + 1);
        if let Some(rest) = line.strip_prefix("# options ") {
            options = Some(rest.trim().parse::<u32>().map_err(|_| bad())?);
            continue;
        }
        if let Some(rest) = line.strip_prefix("# card ") {
            card = Some(rest.trim().to_string());
            continue;
        }
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        let number = |i: usize| f.get(i).and_then(|v| v.parse::<i64>().ok()).ok_or_else(bad);
        match f[0] {
            "p" => {
                places.push((number(1)? as u64, (number(2)? as i32, number(3)? as i32, number(4)? as i32)));
                continue;
            }
            "t" => {
                timings.push((number(1)? as u64, number(2)? as u32));
                continue;
            }
            _ => {}
        }
        let update = f[0].parse::<u64>().map_err(|_| bad())?;
        end = end.max(update);
        if f.len() == 1 {
            continue;
        }
        let [_, blanks, held, analog, sticks] = f[..] else { return Err(bad()) };
        let input = PadInput {
            held: u16::from_str_radix(held, 16).map_err(|_| bad())?,
            analog: analog == "1",
            sticks: u32::from_str_radix(sticks, 16).map_err(|_| bad())?,
        };
        events.push((update, blanks.parse::<u32>().map_err(|_| bad())?, input));
    }
    // The game plays on a copy of the card it was recorded with.
    let copy = crate::paths::user_file("recordings").join("replay.mcd");
    let _ = std::fs::create_dir_all(copy.parent().unwrap());
    let _ = std::fs::remove_file(&copy);
    match card.as_deref() {
        Some("none") | None => {}
        Some(name) => {
            let from = path.parent().unwrap_or(Path::new(".")).join(name);
            std::fs::copy(&from, &copy).map_err(|e| format!("{}: {e}", from.display()))?;
        }
    }
    super::memcard::set_card_path(copy);
    OPTIONS.store(options.ok_or("no options line")?, Ordering::SeqCst);
    Ok(Player {
        events,
        next: 0,
        end,
        places,
        timings,
        next_place: 0,
        next_timing: 0,
        out_of_place: false,
        timing_reports: 0,
    })
}

impl State {
    /// Applies the replay's input up to now, or takes the live input while
    /// recording.
    fn take(&mut self) {
        let (update, blanks) = (self.update, self.blanks);
        match &mut self.mode {
            Mode::Off => {}
            Mode::Record(r) => {
                let input = super::stubs::live_pad();
                self.latched = Some(input);
                if r.last != Some(input) {
                    r.last = Some(input);
                    let _ = writeln!(
                        r.out,
                        "{update} {blanks} {:04X} {} {:08X}",
                        input.held,
                        u8::from(input.analog),
                        input.sticks
                    );
                    let _ = r.out.flush();
                }
            }
            Mode::Replay(p) => {
                while let Some(&(u, b, input)) = p.events.get(p.next) {
                    if (u, b) > (update, blanks) {
                        break;
                    }
                    self.latched = Some(input);
                    p.next += 1;
                }
                if p.next == p.events.len() && update >= p.end {
                    eprintln!("pe2: replay: done at update {update}; paused (P resumes, with the controls)");
                    self.mode = Mode::Off;
                    self.latched = None;
                    FINISHED.store(true, Ordering::SeqCst);
                }
            }
        }
    }
}

/// The game's main loop is about to sample the controller.
#[unsafe(no_mangle)]
pub extern "C" fn pc_pad_frame() {
    let _g = super::irq::guard();
    let mut s = STATE.lock().unwrap();
    if matches!(s.mode, Mode::Off) {
        return;
    }
    let (update, blanks) = (s.update + 1, s.blanks);
    match &mut s.mode {
        Mode::Record(r) if r.timing != Some(blanks) => {
            r.timing = Some(blanks);
            let _ = writeln!(r.out, "t {update} {blanks}");
        }
        Mode::Replay(p) => {
            let mut want = None;
            while let Some(&(u, b)) = p.timings.get(p.next_timing).filter(|t| t.0 <= update) {
                want = Some((u, b));
                p.next_timing += 1;
            }
            let want = want.map(|w| w.1).or_else(|| p.timings.get(p.next_timing.wrapping_sub(1)).map(|t| t.1));
            if let Some(want) = want
                && want != blanks
                && p.timing_reports < 10
            {
                p.timing_reports += 1;
                eprintln!("pe2: replay: update {update} came after {blanks} blanks, {want} when recorded");
            }
        }
        _ => {}
    }
    s.update = update;
    s.blanks = 0;
    s.take();
    let update = s.update;
    if let Mode::Record(r) = &mut s.mode
        && update.is_multiple_of(HEARTBEAT)
    {
        let _ = writeln!(r.out, "{update}");
        let _ = r.out.flush();
    }
    drop(s);
    super::stubs::refresh_pads();
}

/// Where the player is, at each update in play.
#[unsafe(no_mangle)]
pub extern "C" fn pc_replay_player(x: i32, z: i32, yaw: i32) {
    let _g = super::irq::guard();
    let mut s = STATE.lock().unwrap();
    let update = s.update;
    let place = (x, z, yaw & 0xFFF);
    match &mut s.mode {
        Mode::Record(r) if r.place != Some(place) => {
            r.place = Some(place);
            let _ = writeln!(r.out, "p {update} {} {} {}", place.0, place.1, place.2);
        }
        Mode::Replay(p) if !p.out_of_place => {
            while p.places.get(p.next_place).is_some_and(|q| q.0 <= update) {
                p.next_place += 1;
            }
            if let Some(&(at, want)) = p.places.get(p.next_place.wrapping_sub(1))
                && want != place
            {
                p.out_of_place = true;
                eprintln!(
                    "pe2: replay: out of step at update {update}: the player is at {place:?} (x, z, yaw), \
                     recorded at {want:?} (from update {at})"
                );
            }
        }
        _ => {}
    }
}

/// A vertical blank (on the interrupt thread, before the pads refresh).
pub fn vblank() {
    let mut s = STATE.lock().unwrap();
    if matches!(s.mode, Mode::Off) {
        return;
    }
    s.blanks += 1;
    if s.blanks >= STALL {
        s.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A replay gives the game the recorded input at the same updates and
    /// blanks, and hands the controls back at the end.
    #[test]
    fn replay_follows_the_updates() {
        let a = PadInput { held: 0x1000, analog: false, sticks: 0x8080_8080 };
        let b = PadInput { held: 0, analog: false, sticks: 0x8080_8080 };
        let player = Player {
            events: vec![(3, 0, a), (5, 9, b)],
            next: 0,
            end: 7,
            places: Vec::new(),
            timings: Vec::new(),
            next_place: 0,
            next_timing: 0,
            out_of_place: false,
            timing_reports: 0,
        };
        let mut s = State { mode: Mode::Replay(player), update: 0, blanks: 0, latched: None };
        let mut seen = Vec::new();
        for _ in 0..8 {
            s.update += 1;
            s.blanks = 0;
            s.take();
            seen.push(s.latched);
            for _ in 0..10 {
                s.blanks += 1;
                if s.blanks >= STALL {
                    s.take();
                }
            }
            seen.push(s.latched);
        }
        // Update 3 presses; update 5's ninth blank releases; done at 7.
        assert_eq!(seen[3], None);
        assert_eq!(seen[4], Some(a));
        assert_eq!(seen[8], Some(a));
        assert_eq!(seen[9], Some(b));
        assert_eq!(seen[12], None);
        assert!(take_finished());
    }
}
