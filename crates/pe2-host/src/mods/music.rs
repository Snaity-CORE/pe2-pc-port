//! Music replacement: a mod's recording plays instead of a MIDI song.
//!
//! The game's music is a MIDI sequencer driving SPU voices 0..15 (sound
//! effects use 16..23), one song at a time, known by its sequence id. Each
//! vertical blank the song's state is read from the game (pc_midi_state in
//! the fork's sndevt.c): while a mod has `music/<id>.ogg` (or .flac, .wav)
//! for it, those voices are left out of the mix and the recording plays,
//! following the song's fades, mute and volume, and stopping with it. The
//! sequencer itself runs on untouched.
//!
//! With dumping on, the first two minutes of each song's voices are saved
//! as `dumps/music/<id>.wav`, to tell which id is which song.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use serde::Deserialize;

use crate::runtime::spu;

/// The SPU voices the MIDI sequencer uses.
const MIDI_VOICES: u32 = 0xFFFF;
const RATE: u32 = 44_100;
/// Dumps keep this much of each song.
const DUMP_SECONDS: usize = 120;

/// What pc_midi_state reports.
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct MidiState {
    sequence_id: u8,
    status: u8,
    game_muted: u8,
    unused: u8,
    ramp: u32,
    volume_scale: i32,
    base_scale: i32,
    master: i32,
}

unsafe extern "C" {
    fn pc_midi_state(out: *mut MidiState);
}

const NO_SEQUENCE: u8 = 0xFF;
/// Statuses with the song sounding (playing, muted, unmuting, fading).
const SOUNDING: u8 = 0x02 | 0x08 | 0x10 | 0x40 | 0x80;

/// A mod's `music.toml`: per sequence id (two hex digits), optional
/// settings for its recording.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct MusicFile {
    track: HashMap<String, TrackSettings>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct TrackSettings {
    /// Loudness against the original (default 1).
    volume: Option<f32>,
    /// Loop from `loop_end` (default the end) back to `loop_start` (default
    /// the start), in seconds; `looping = false` plays once.
    loop_start: Option<f64>,
    loop_end: Option<f64>,
    looping: Option<bool>,
}

/// A recording, decoded at 44.1 kHz.
struct Track {
    samples: Vec<(i16, i16)>,
    loop_start: usize,
    loop_end: usize,
    looping: bool,
    volume: f32,
}

/// The recordings mods supply: sequence id -> file and settings.
fn index() -> &'static HashMap<u8, (PathBuf, TrackSettings)> {
    static INDEX: OnceLock<HashMap<u8, (PathBuf, TrackSettings)>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut out = HashMap::new();
        if !super::enabled() {
            return out;
        }
        // Lowest priority first, so the highest overwrites it.
        for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
            let settings: MusicFile = std::fs::read_to_string(m.dir.join("music.toml"))
                .ok()
                .and_then(|t| {
                    toml::from_str(&t)
                        .map_err(|e| eprintln!("pe2: mods: {}: {e}", m.dir.join("music.toml").display()))
                        .ok()
                })
                .unwrap_or_default();
            let Ok(entries) = std::fs::read_dir(m.dir.join("music")) else { continue };
            for e in entries.flatten() {
                let path = e.path();
                let ext = path.extension().and_then(|x| x.to_str()).map(str::to_ascii_lowercase);
                if !matches!(ext.as_deref(), Some("ogg" | "flac" | "wav")) {
                    continue;
                }
                let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else { continue };
                let Ok(id) = u8::from_str_radix(stem, 16) else { continue };
                let key = format!("{id:02x}");
                let track = settings.track.get(&key).or_else(|| settings.track.get(stem)).cloned().unwrap_or_default();
                out.insert(id, (path, track));
            }
        }
        if !out.is_empty() {
            eprintln!("pe2: mods: {} music tracks", out.len());
        }
        out
    })
}

/// A decoded recording: 44.1 kHz stereo frames, and the loop points its
/// tags give (start, end), if any.
type Decoded = (Vec<(i16, i16)>, Option<(usize, usize)>);

/// Decodes a recording to 44.1 kHz stereo.
fn decode(path: &Path) -> Result<Decoded, String> {
    let ext = path.extension().and_then(|x| x.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
    let (interleaved, channels, rate, tags) = match ext.as_str() {
        "ogg" => {
            let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
            let mut ogg =
                lewton::inside_ogg::OggStreamReader::new(std::io::BufReader::new(file)).map_err(|e| e.to_string())?;
            let (channels, rate) = (ogg.ident_hdr.audio_channels as usize, ogg.ident_hdr.audio_sample_rate);
            // RPG Maker-style loop tags, which many looped tracks carry.
            let tag = |name: &str| {
                ogg.comment_hdr
                    .comment_list
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(name))
                    .and_then(|(_, v)| v.trim().parse::<usize>().ok())
            };
            let tags = tag("LOOPSTART").map(|s| (s, tag("LOOPLENGTH").map_or(usize::MAX, |l| s + l)));
            let mut all = Vec::new();
            while let Some(packet) = ogg.read_dec_packet_itl().map_err(|e| e.to_string())? {
                all.extend(packet.into_iter().map(i32::from));
            }
            (all, channels, rate, tags)
        }
        "flac" => {
            let mut flac = claxon::FlacReader::open(path).map_err(|e| e.to_string())?;
            let info = flac.streaminfo();
            let shift = info.bits_per_sample as i32 - 16;
            let all: Result<Vec<i32>, _> =
                flac.samples().map(|s| s.map(|v| if shift >= 0 { v >> shift } else { v << -shift })).collect();
            (all.map_err(|e| e.to_string())?, info.channels as usize, info.sample_rate, None)
        }
        _ => {
            let mut wav = hound::WavReader::open(path).map_err(|e| e.to_string())?;
            let spec = wav.spec();
            let all: Vec<i32> = match spec.sample_format {
                hound::SampleFormat::Float => {
                    wav.samples::<f32>().map(|s| (s.unwrap_or(0.0) * 32767.0) as i32).collect()
                }
                hound::SampleFormat::Int => {
                    let shift = spec.bits_per_sample as i32 - 16;
                    wav.samples::<i32>()
                        .map(|s| {
                            let v = s.unwrap_or(0);
                            if shift >= 0 { v >> shift } else { v << -shift }
                        })
                        .collect()
                }
            };
            (all, spec.channels as usize, spec.sample_rate, None)
        }
    };
    if channels == 0 || rate == 0 {
        return Err("no audio".into());
    }
    let clamp = |v: i32| v.clamp(-32768, 32767) as i16;
    let frames: Vec<(i16, i16)> = interleaved
        .chunks_exact(channels)
        .map(|f| if channels == 1 { (clamp(f[0]), clamp(f[0])) } else { (clamp(f[0]), clamp(f[1])) })
        .collect();
    if rate == RATE {
        return Ok((frames, tags));
    }
    // Linear resampling to 44.1 kHz; loop points move with it.
    let ratio = f64::from(rate) / f64::from(RATE);
    let n = (frames.len() as f64 / ratio) as usize;
    let out = (0..n)
        .map(|i| {
            let p = i as f64 * ratio;
            let (a, t) = (p as usize, (p - p.floor()) as f32);
            let (x, y) = (frames[a.min(frames.len() - 1)], frames[(a + 1).min(frames.len() - 1)]);
            let mix = |u: i16, v: i16| (f32::from(u) * (1.0 - t) + f32::from(v) * t) as i16;
            (mix(x.0, y.0), mix(x.1, y.1))
        })
        .collect();
    let scale = |s: usize| if s == usize::MAX { s } else { (s as f64 / ratio) as usize };
    Ok((out, tags.map(|(s, e)| (scale(s), scale(e)))))
}

#[derive(Default)]
struct Player {
    /// The sequence the game is sounding, and whether it started anew since
    /// the last poll.
    sequence: Option<u8>,
    /// The recording for it, when loaded; where it is; its gain (target and
    /// current, smoothed per sample).
    track: Option<Arc<Track>>,
    position: usize,
    target: f32,
    gain: f32,
    /// Recordings being decoded, and ready ones by sequence id.
    loading: HashSet<u8>,
    ready: HashMap<u8, Arc<Track>>,
    /// Dumping: the song being recorded, and those already dumped.
    recording: Option<(u8, Vec<(i16, i16)>)>,
    dumped: HashSet<u8>,
}

static PLAYER: Mutex<Option<Player>> = Mutex::new(None);

fn with_player<R>(f: impl FnOnce(&mut Player) -> R) -> R {
    f(PLAYER.lock().unwrap().get_or_insert_with(Player::default))
}

/// Each vertical blank, with the game stopped: follows the song.
pub fn poll() {
    let replacing = !index().is_empty();
    let dumping = super::dumping();
    if !replacing && !dumping {
        return;
    }
    let mut state = MidiState::default();
    // SAFETY: fills a plain struct from the game's sequencer state.
    unsafe { pc_midi_state(&mut state) };
    let sounding = state.sequence_id != NO_SEQUENCE && state.status & SOUNDING != 0;
    let id = sounding.then_some(state.sequence_id);

    let (mute, capture) = with_player(|p| {
        // A song starting (or another one) starts its recording over.
        if p.sequence != id {
            if crate::runtime::trace("music") {
                let replaced = id.is_some_and(|i| index().contains_key(&i));
                let name = id.map_or("none".to_string(), |i| format!("{i:02x}"));
                eprintln!("[music] sequence {name} (status 0x{:02X}), replaced: {replaced}", state.status);
            }
            p.sequence = id;
            p.track = None;
            p.position = 0;
            p.gain = 0.0;
        }
        if let Some(id) = id
            && let Some((path, settings)) = index().get(&id)
        {
            if let Some(track) = p.ready.get(&id) {
                p.track.get_or_insert_with(|| track.clone());
            } else if p.loading.insert(id) {
                let (path, settings) = (path.clone(), settings.clone());
                std::thread::spawn(move || load(id, &path, &settings));
            }
        }
        // The song's loudness relative to how it starts, at the master
        // volume the game starts with (0x40).
        p.target = match (&p.track, state.base_scale) {
            (Some(t), base) if base > 0 && state.game_muted == 0 => {
                let ramp = state.ramp as f32 / 65535.0;
                let scale = state.volume_scale as f32 / base as f32;
                let master = state.master as f32 / 64.0;
                (ramp * scale * master * t.volume).clamp(0.0, 2.0)
            }
            _ => 0.0,
        };
        // Dumping: the song's own voices, while no recording replaces it.
        let capture = dumping && p.track.is_none() && id.is_some_and(|i| !p.dumped.contains(&i));
        let same = matches!((&p.recording, id), (Some((r, _)), Some(i)) if *r == i);
        if !same {
            finish_recording(p);
        }
        if capture && p.recording.is_none() {
            p.recording = id.map(|i| (i, Vec::new()));
        }
        if crate::runtime::trace("music") && p.track.is_some() {
            static LAST: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);
            let shown = (p.target * 100.0) as u32;
            if LAST.swap(shown, std::sync::atomic::Ordering::Relaxed) != shown {
                eprintln!("[music] recording gain {:.2} at {:.1} s", p.target, p.position as f32 / RATE as f32);
            }
        }
        (p.track.is_some(), capture)
    });
    spu::with_spu(|s| {
        s.muted_voices = if mute { MIDI_VOICES } else { 0 };
        s.captured_voices = if capture { MIDI_VOICES } else { 0 };
        let taken = std::mem::take(&mut s.capture);
        if capture {
            with_player(|p| {
                if let Some((id, buf)) = &mut p.recording {
                    buf.extend(taken);
                    if buf.len() >= DUMP_SECONDS * RATE as usize {
                        let id = *id;
                        finish_recording(p);
                        p.dumped.insert(id);
                    }
                }
            });
        }
    });
}

/// Saves the song being recorded, if long enough to be worth it.
fn finish_recording(p: &mut Player) {
    let Some((id, samples)) = p.recording.take() else { return };
    if samples.len() < RATE as usize * 3 || p.dumped.contains(&id) {
        return;
    }
    p.dumped.insert(id);
    std::thread::spawn(move || {
        let dir = crate::paths::user_file("dumps").join("music");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("{id:02x}.wav"));
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: RATE,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let written = hound::WavWriter::create(&path, spec).map_err(|e| e.to_string()).and_then(|mut w| {
            for (l, r) in samples {
                w.write_sample(l).and_then(|()| w.write_sample(r)).map_err(|e| e.to_string())?;
            }
            w.finalize().map_err(|e| e.to_string())
        });
        match written {
            Ok(()) => eprintln!("pe2: dumped {}", path.display()),
            Err(e) => eprintln!("pe2: dump {}: {e}", path.display()),
        }
    });
}

/// Decodes a recording on a thread of its own.
fn load(id: u8, path: &Path, settings: &TrackSettings) {
    match decode(path) {
        Ok((samples, tags)) => {
            let end = samples.len();
            let frame = |seconds: f64| (seconds.max(0.0) * f64::from(RATE)) as usize;
            let loop_start = settings.loop_start.map(frame).or(tags.map(|t| t.0)).unwrap_or(0).min(end);
            let loop_end = settings.loop_end.map(frame).or(tags.map(|t| t.1)).unwrap_or(end).clamp(loop_start, end);
            let track = Track {
                samples,
                loop_start,
                loop_end,
                looping: settings.looping.unwrap_or(true) && loop_end > loop_start,
                volume: settings.volume.unwrap_or(1.0),
            };
            with_player(|p| {
                p.ready.insert(id, Arc::new(track));
            });
        }
        Err(e) => eprintln!("pe2: mods: {}: {e}", path.display()),
    }
}

/// The recording's next `frames` samples for the SPU's mix (silence when
/// none plays).
pub fn take(out: &mut VecDeque<(i16, i16)>, frames: usize) {
    let mut guard = PLAYER.lock().unwrap();
    let Some(p) = guard.as_mut().filter(|p| p.track.is_some()) else { return };
    let track = p.track.clone().unwrap();
    // About 10 ms to reach a new gain: no zipper noise.
    let step = 1.0 / 441.0;
    for _ in 0..frames {
        if p.position >= track.loop_end && track.looping {
            p.position = track.loop_start;
        }
        let (l, r) = track.samples.get(p.position).copied().unwrap_or((0, 0));
        if p.position < track.samples.len() {
            p.position += 1;
        }
        p.gain += (p.target - p.gain).clamp(-step, step);
        let g = p.gain;
        out.push_back(((f32::from(l) * g) as i16, (f32::from(r) * g) as i16));
    }
}
