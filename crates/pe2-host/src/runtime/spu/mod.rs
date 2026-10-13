//! Sound: the SPU model (`core`), the libspu calls the game makes
//! (`libspu`), CD-XA audio decoding (`xa`), and the host audio output.
//!
//! The audio device pulls samples: its callback runs the SPU at 44100 Hz
//! (resampling to the device rate), so sound is paced by the sound card.
//! Interrupts the SPU raises are delivered to the game through `irq`.

pub mod core;
pub mod libspu;
pub mod xa;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use self::core::{SAMPLE_RATE, Spu};
use super::{call_psx, irq};

pub static SPU: Mutex<Option<Spu>> = Mutex::new(None);
/// The game's SPU IRQ callback (SpuSetIRQCallback).
pub static IRQ_CALLBACK: AtomicU32 = AtomicU32::new(0);
/// CD audio queued beyond one sector's worth is backlog: the console plays
/// XA as the sectors arrive, so a burst of late sectors (after a host stall)
/// must not leave the sound running behind the picture for good.
const CD_MIN_SLACK: usize = 2048;

/// Seconds since the first sound trace line, the time base of `[spu t]`
/// trace lines.
pub fn stamp() -> f64 {
    static T0: OnceLock<Instant> = OnceLock::new();
    T0.get_or_init(Instant::now).elapsed().as_secs_f64()
}

pub fn with_spu<R>(f: impl FnOnce(&mut Spu) -> R) -> R {
    let mut s = SPU.lock().unwrap();
    f(s.get_or_insert_with(Spu::default))
}

/// Queues one sector of decoded CD audio (44.1 kHz stereo) for the SPU's CD
/// input. Whatever is still queued from earlier sectors beyond one sector's
/// length is dropped, which keeps the latency at most two sectors.
pub fn push_cd_audio(samples: Vec<(i16, i16)>) {
    with_spu(|s| {
        let slack = samples.len().max(CD_MIN_SLACK);
        let excess = s.cd_input.len().saturating_sub(slack);
        if excess > 0 {
            if super::trace("spu") {
                eprint!("[spu {:8.3}] ", stamp());
                eprintln!("CD audio {} samples behind, dropping {excess}", s.cd_input.len());
            }
            s.cd_input.drain(..excess);
        }
        s.cd_input.extend(samples);
    });
}

/// Drops queued CD audio: the drive stopped or moved, so what was buffered
/// would play out of place.
pub fn clear_cd_audio() {
    with_spu(|s| s.cd_input.clear());
}

/// Runs the SPU for `frames` samples into `out` (interleaved stereo) and
/// raises the IRQ if a voice reached the IRQ address.
fn render(out: &mut Vec<(i16, i16)>, frames: usize) {
    let irq = with_spu(|s| {
        out.clear();
        out.extend((0..frames).map(|_| s.tick()));
        std::mem::take(&mut s.irq_flag)
    });
    if irq {
        raise_irq();
    }
}

/// SPU interrupt (IRQ9): runs the game's callback in interrupt context. The
/// IRQ stays disabled until the game enables it again, as the hardware's
/// acknowledge-by-clearing-the-enable-bit protocol leaves it.
pub fn raise_irq() {
    with_spu(|s| s.control &= !self::core::CONTROL_IRQ_ENABLE);
    let cb = IRQ_CALLBACK.load(Ordering::SeqCst);
    if super::trace("spu") {
        eprintln!("[spu {:8.3}] irq (callback 0x{cb:08X})", stamp());
    }
    if cb != 0 {
        irq::schedule(Instant::now(), Box::new(move || {
            call_psx(cb, &[]);
        }));
    }
}

struct Output {
    _stream: cpal::Stream,
}

// SAFETY: the stream handle is only kept alive, never used from two threads.
unsafe impl Send for Output {}
unsafe impl Sync for Output {}

static OUTPUT: OnceLock<Option<Output>> = OnceLock::new();

/// Opens the default audio device and starts pulling samples. Without a
/// device the game still runs; the SPU then advances with nothing heard.
pub fn start() {
    OUTPUT.get_or_init(|| match open_output() {
        Ok(o) => Some(o),
        Err(e) => {
            eprintln!("pe2: no audio output ({e}); continuing without sound");
            None
        }
    });
}

fn open_output() -> Result<Output, String> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("no default output device")?;
    let supported = device.default_output_config().map_err(|e| e.to_string())?;
    let format = supported.sample_format();
    let config: cpal::StreamConfig = supported.into();
    let channels = usize::from(config.channels);
    let device_rate = f64::from(config.sample_rate.0);

    // Resampler state: SPU samples are produced at 44.1 kHz and read at the
    // device rate with linear interpolation.
    let mut spu_buf: Vec<(i16, i16)> = Vec::new();
    let mut pos = 0.0f64;
    let mut last = (0i16, 0i16);
    let step = f64::from(SAMPLE_RATE) / device_rate;
    let mut last_callback: Option<Instant> = None;
    let mut watch_gaps = move || {
        let now = Instant::now();
        if let Some(prev) = last_callback.replace(now) {
            let gap = now - prev;
            if gap.as_millis() > 100 && super::trace("spu") {
                eprintln!("[spu {:8.3}] audio output stalled {} ms", stamp(), gap.as_millis());
            }
        }
    };
    let mut next_frame = move || -> (f32, f32) {
        pos += step;
        while pos >= 1.0 {
            pos -= 1.0;
            if spu_buf.is_empty() {
                render(&mut spu_buf, 256);
                spu_buf.reverse();
            }
            last = spu_buf.pop().unwrap_or((0, 0));
        }
        (f32::from(last.0) / 32768.0, f32::from(last.1) / 32768.0)
    };

    let err = |e| eprintln!("pe2: audio stream error: {e}");
    let stream = match format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                watch_gaps();
                for frame in data.chunks_mut(channels) {
                    let (l, r) = next_frame();
                    frame[0] = l;
                    if channels > 1 {
                        frame[1] = r;
                    }
                    for x in frame.iter_mut().skip(2) {
                        *x = 0.0;
                    }
                }
            },
            err,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data: &mut [i16], _| {
                watch_gaps();
                for frame in data.chunks_mut(channels) {
                    let (l, r) = next_frame();
                    frame[0] = (l * 32767.0) as i16;
                    if channels > 1 {
                        frame[1] = (r * 32767.0) as i16;
                    }
                    for x in frame.iter_mut().skip(2) {
                        *x = 0;
                    }
                }
            },
            err,
            None,
        ),
        other => return Err(format!("unsupported sample format {other:?}")),
    }
    .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(Output { _stream: stream })
}
