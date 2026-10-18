//! Movies: a mod's video file shown instead of one of the game's FMVs.
//!
//! The game streams its movies (STR) from the disc frame by frame
//! (runtime/stream.rs), each frame numbered from 1. A movie is known by the
//! hash of its first frame's data; a mod's `movies/<hash>.mp4` (H.264
//! without B-frames, and AAC sound) plays instead: its pictures are shown
//! in place of the game's 24-bit ones, at the time the game's frame number
//! gives, and its sound replaces the movie's XA sound. The game plays its
//! own movie as ever underneath (its timing, skipping and ending stay as
//! they are).
//!
//! With dumping on, each movie's first picture is saved as
//! `dumps/movies/<hash>.png`, and its length in frames and seconds as
//! `dumps/movies/<hash>.txt` when it ends, to tell which is which.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

/// The game's movies play at 15 frames a second, a few at 30: which, is
/// told by how fast frames come.
const GAME_FPS: f64 = 15.0;

/// The frame rate of a movie that reached frame `number` `elapsed` seconds
/// after its first.
fn game_fps(number: u32, elapsed: f64) -> f64 {
    if number > 15 && f64::from(number - 1) / elapsed.max(0.001) > 22.0 { 30.0 } else { GAME_FPS }
}
/// A movie whose frames stop coming for this long has ended.
const ENDED: Duration = Duration::from_millis(600);
/// Decoded pictures kept ready ahead of the one shown.
const AHEAD: usize = 4;
const RATE: u32 = 44_100;

/// A picture of a mod's movie: 0RGB pixels, row by row.
pub struct Picture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
    /// When in the movie it shows, in seconds.
    time: f64,
}

/// A mod's movie playing: where its pictures and sound come from.
struct Playing {
    hash: u64,
    /// The game's frame number and when it was handed out, and when the
    /// first was.
    number: u32,
    at: Instant,
    began: Instant,
    pictures: Receiver<Picture>,
    shown: Option<Arc<Picture>>,
    next: Option<Picture>,
    sound: Arc<Mutex<Sound>>,
    stop: Arc<AtomicBool>,
}

/// A movie's sound as decoded so far, at 44.1 kHz, and what was played.
#[derive(Default)]
struct Sound {
    samples: Vec<(i16, i16)>,
    played: usize,
}

/// A movie of the game seen while dumping: its first frame's hash, how many
/// frames it had and when it began.
struct Seen {
    hash: u64,
    frames: u32,
    began: Instant,
    last: Instant,
    pictured: bool,
}

#[derive(Default)]
struct State {
    playing: Option<Playing>,
    seen: Option<Seen>,
}

fn state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(Default::default)
}

/// Movie hash -> the mod's file for it.
fn index() -> &'static HashMap<u64, PathBuf> {
    static INDEX: OnceLock<HashMap<u64, PathBuf>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut out = HashMap::new();
        if !super::enabled() {
            return out;
        }
        for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
            let Ok(entries) = std::fs::read_dir(m.dir.join("movies")) else { continue };
            for e in entries.flatten() {
                let path = e.path();
                let hash = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .filter(|_| path.extension().is_some_and(|x| x.eq_ignore_ascii_case("mp4")))
                    .and_then(|s| u64::from_str_radix(s, 16).ok());
                if let Some(hash) = hash {
                    out.insert(hash, path);
                }
            }
        }
        if !out.is_empty() {
            eprintln!("pe2: mods: {} movies", out.len());
        }
        out
    })
}

/// The game took frame `number` of the movie streaming, `data` (its
/// sectors' payload) to decode.
pub fn frame(number: u32, data: &[u8]) {
    let wanted = !index().is_empty() || super::dumping();
    if !wanted {
        return;
    }
    let mut s = state().lock().unwrap();
    let now = Instant::now();
    if number == 1 {
        let hash = super::fnv1a(data.iter().copied());
        if let Some(p) = s.playing.take() {
            p.stop.store(true, Ordering::Relaxed);
        }
        finish_seen(&mut s);
        if super::dumping() {
            s.seen = Some(Seen { hash, frames: 1, began: now, last: now, pictured: false });
        }
        if crate::runtime::trace("movies") {
            eprintln!("[movies] movie {hash:016x} begins");
        }
        if let Some(path) = index().get(&hash) {
            s.playing = start(hash, path.clone());
        }
        return;
    }
    if let Some(seen) = s.seen.as_mut() {
        seen.frames = seen.frames.max(number);
        seen.last = now;
    }
    if let Some(p) = s.playing.as_mut() {
        p.number = number;
        p.at = now;
    }
}

/// Writes what was seen of the last movie, if it is over.
fn finish_seen(s: &mut State) {
    let Some(seen) = s.seen.take() else { return };
    let dir = crate::paths::user_file("dumps").join("movies");
    let _ = std::fs::create_dir_all(&dir);
    let seconds = (seen.last - seen.began).as_secs_f64() + 1.0 / GAME_FPS;
    let text = format!("frames = {}\nseconds = {seconds:.2}\n", seen.frames);
    let _ = std::fs::write(dir.join(format!("{:016x}.txt", seen.hash)), text);
}

/// Starts playing a mod's movie: its pictures and sound decoded on threads
/// of their own.
fn start(hash: u64, path: PathBuf) -> Option<Playing> {
    let (send, pictures) = sync_channel(AHEAD);
    let stop = Arc::new(AtomicBool::new(false));
    let sound = Arc::new(Mutex::new(Sound::default()));
    {
        let (path, stop) = (path.clone(), stop.clone());
        std::thread::spawn(move || {
            if let Err(e) = decode_video(&path, &send, &stop) {
                eprintln!("pe2: mods: {}: {e}", path.display());
            }
        });
    }
    {
        let (path, stop, sound) = (path.clone(), stop.clone(), sound.clone());
        std::thread::spawn(move || {
            if let Err(e) = decode_sound(&path, &sound, &stop) {
                eprintln!("pe2: mods: {}: sound: {e}", path.display());
            }
        });
    }
    eprintln!("pe2: mods: movie {hash:016x} from {}", path.display());
    let now = Instant::now();
    Some(Playing { hash, number: 1, at: now, began: now, pictures, shown: None, next: None, sound, stop })
}

/// The picture to show now, while a mod's movie plays (else None).
pub fn picture() -> Option<Arc<Picture>> {
    let mut s = state().lock().unwrap();
    let p = s.playing.as_mut()?;
    let frame = 1.0 / game_fps(p.number, (p.at - p.began).as_secs_f64());
    let now = f64::from(p.number - 1) * frame + p.at.elapsed().as_secs_f64().min(frame);
    loop {
        if p.next.is_none() {
            p.next = p.pictures.try_recv().ok();
        }
        match &p.next {
            Some(n) if n.time <= now => p.shown = p.next.take().map(Arc::new),
            _ => break,
        }
    }
    p.shown.clone()
}

/// Once a vertical blank: a movie whose frames stopped coming has ended.
pub fn tick() {
    let mut s = state().lock().unwrap();
    if let Some(p) = s.playing.as_ref()
        && p.at.elapsed() > ENDED
    {
        p.stop.store(true, Ordering::Relaxed);
        if crate::runtime::trace("movies") {
            eprintln!("[movies] movie {:016x} ends", p.hash);
        }
        s.playing = None;
    }
    if s.seen.as_ref().is_some_and(|seen| seen.last.elapsed() > ENDED) {
        finish_seen(&mut s);
    }
}

/// Whether the movie's own sound (XA) is to be left out: a mod's plays.
pub fn mutes_xa() -> bool {
    state().lock().unwrap().playing.is_some()
}

/// Adds the playing movie's sound to `out` (the SPU's next `frames`
/// samples of mods' sound).
pub fn mix(out: &mut VecDeque<(i16, i16)>, frames: usize) {
    let s = state().lock().unwrap();
    let Some(p) = s.playing.as_ref() else { return };
    let mut sound = p.sound.lock().unwrap();
    while out.len() < frames {
        out.push_back((0, 0));
    }
    let start = sound.played;
    for (k, o) in out.iter_mut().take(frames).enumerate() {
        let (l, r) = sound.samples.get(start + k).copied().unwrap_or((0, 0));
        *o = (o.0.saturating_add(l), o.1.saturating_add(r));
    }
    sound.played = (start + frames).min(sound.samples.len().max(start));
}

/// With dumping on, the game's own movie picture (24-bit, `w` x `h`, rows
/// of `row_words` halfwords): one of each movie is saved, the first that is
/// not dark (or the one three seconds in), to tell which movie it is.
pub fn observe_picture(w: usize, h: usize, rows: &[u16], row_words: usize) {
    if !super::dumping() || rows.len() < h * row_words {
        return;
    }
    let mut s = state().lock().unwrap();
    let Some(seen) = s.seen.as_mut().filter(|s| !s.pictured) else { return };
    let mut rgb = Vec::with_capacity(w * h * 3);
    for y in 0..h {
        let row = &rows[y * row_words..(y + 1) * row_words];
        for i in 0..w * 3 {
            let word = row[i / 2];
            rgb.push(if i.is_multiple_of(2) { word as u8 } else { (word >> 8) as u8 });
        }
    }
    let mean = rgb.iter().map(|&b| u64::from(b)).sum::<u64>() / rgb.len().max(1) as u64;
    if mean < 24 && seen.frames < 3 * GAME_FPS as u32 {
        return;
    }
    seen.pictured = true;
    let hash = seen.hash;
    std::thread::spawn(move || {
        let dir = crate::paths::user_file("dumps").join("movies");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("{hash:016x}.png"));
        let written = std::fs::File::create(&path).map_err(|e| e.to_string()).and_then(|file| {
            let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
            enc.set_color(png::ColorType::Rgb);
            enc.set_depth(png::BitDepth::Eight);
            let mut writer = enc.write_header().map_err(|e| e.to_string())?;
            writer.write_image_data(&rgb).map_err(|e| e.to_string())
        });
        match written {
            Ok(()) => eprintln!("pe2: dumped {}", path.display()),
            Err(e) => eprintln!("pe2: dump {}: {e}", path.display()),
        }
    });
}

/// Decodes the video track, sending each picture with its time.
fn decode_video(path: &PathBuf, send: &SyncSender<Picture>, stop: &AtomicBool) -> Result<(), String> {
    use openh264::formats::YUVSource;
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let size = file.metadata().map_err(|e| e.to_string())?.len();
    let mut mp4 = mp4::Mp4Reader::read_header(std::io::BufReader::new(file), size).map_err(|e| e.to_string())?;
    let track = mp4
        .tracks()
        .values()
        .find(|t| t.track_type().is_ok_and(|k| k == mp4::TrackType::Video))
        .ok_or("no video track")?;
    let (id, scale) = (track.track_id(), f64::from(track.timescale().max(1)));
    let sps = track.sequence_parameter_set().map_err(|e| e.to_string())?.to_vec();
    let pps = track.picture_parameter_set().map_err(|e| e.to_string())?.to_vec();
    let count = mp4.sample_count(id).map_err(|e| e.to_string())?;
    let mut decoder = openh264::decoder::Decoder::new().map_err(|e| e.to_string())?;
    let mut nal = Vec::new();
    for p in [&sps, &pps] {
        nal.extend_from_slice(&[0, 0, 0, 1]);
        nal.extend_from_slice(p);
    }
    for i in 1..=count {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let Some(sample) = mp4.read_sample(id, i).map_err(|e| e.to_string())? else { continue };
        // Length-prefixed units to start codes.
        let b = &sample.bytes[..];
        let mut at = 0;
        while at + 4 <= b.len() {
            let n = u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize;
            let unit = b.get(at + 4..at + 4 + n).ok_or("a broken sample")?;
            nal.extend_from_slice(&[0, 0, 0, 1]);
            nal.extend_from_slice(unit);
            at += 4 + n;
        }
        let decoded = decoder.decode(&nal);
        nal.clear();
        let yuv = match decoded {
            Ok(Some(yuv)) => yuv,
            Ok(None) => continue,
            Err(e) => return Err(format!("frame {i}: {e} (B-frames are not supported: encode with -bf 0)")),
        };
        let (width, height) = yuv.dimensions();
        let mut rgb = vec![0u8; width * height * 3];
        yuv.write_rgb8(&mut rgb);
        let pixels = rgb
            .as_chunks::<3>()
            .0
            .iter()
            .map(|p| (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2]))
            .collect();
        let time = (sample.start_time as f64 + f64::from(sample.rendering_offset)) / scale;
        if send.send(Picture { width, height, pixels, time }).is_err() {
            break;
        }
    }
    Ok(())
}

/// Decodes the sound track into `sound`, at 44.1 kHz stereo, as it goes.
fn decode_sound(path: &PathBuf, sound: &Mutex<Sound>, stop: &AtomicBool) -> Result<(), String> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let stream = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp4");
    let probed = symphonia::default::get_probe()
        .format(&hint, stream, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| e.to_string())?;
    let mut format = probed.format;
    let Some(track) = format.tracks().iter().find(|t| t.codec_params.codec != CODEC_TYPE_NULL) else {
        return Ok(());
    };
    let (id, rate) = (track.id, track.codec_params.sample_rate.unwrap_or(RATE));
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| e.to_string())?;
    // Linear resampling to 44.1 kHz, carried across packets.
    let step = f64::from(rate) / f64::from(RATE);
    let (mut phase, mut last) = (0.0f64, (0i16, 0i16));
    while let Ok(packet) = format.next_packet() {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        if packet.track_id() != id {
            continue;
        }
        let Ok(decoded) = decoder.decode(&packet) else { continue };
        let channels = decoded.spec().channels.count().max(1);
        let mut buf = SampleBuffer::<i16>::new(decoded.capacity() as u64, *decoded.spec());
        buf.copy_interleaved_ref(decoded);
        let frames: Vec<(i16, i16)> = buf
            .samples()
            .chunks_exact(channels)
            .map(|f| if channels == 1 { (f[0], f[0]) } else { (f[0], f[1]) })
            .collect();
        let mut out = Vec::with_capacity((frames.len() as f64 / step) as usize + 1);
        while phase < frames.len() as f64 {
            let i = phase as usize;
            let t = (phase - i as f64) as f32;
            let a = if i == 0 { last } else { frames[i - 1] };
            let b = frames[i];
            let mixed = |x: i16, y: i16| (f32::from(x) * (1.0 - t) + f32::from(y) * t) as i16;
            out.push((mixed(a.0, b.0), mixed(a.1, b.1)));
            phase += step;
        }
        phase -= frames.len() as f64;
        if let Some(&l) = frames.last() {
            last = l;
        }
        sound.lock().unwrap().samples.extend(out);
    }
    Ok(())
}
