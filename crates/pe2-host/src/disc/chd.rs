//! CHD (MAME compressed hunks) CD images.
//!
//! CD hunks hold whole frames of 2352 data bytes plus 96 subcode bytes, and
//! each track's frames are padded to a multiple of four in the file. The
//! track layout comes from the CHT2/CHTR metadata.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chd::Chd;
use chd::metadata::Metadata;

use super::{DiscError, RAW_SECTOR_SIZE, RawSector, Result, Track, TrackKind};

const FRAME_SIZE: usize = RAW_SECTOR_SIZE + 96;
const TRACK_PADDING: u32 = 4;
const TAG_CHT2: u32 = u32::from_be_bytes(*b"CHT2");
const TAG_CHTR: u32 = u32::from_be_bytes(*b"CHTR");

struct TrackFrames {
    track: Track,
    /// CHD frame holding `track.start_lba`.
    frame: u32,
}

pub struct ChdDisc {
    chd: Chd<BufReader<File>>,
    tracks: Vec<Track>,
    layout: Vec<TrackFrames>,
    frames_per_hunk: u32,
    hunk_buf: Vec<u8>,
    cmp_buf: Vec<u8>,
    cached_hunk: Option<u32>,
}

fn chd_err(e: chd::Error) -> DiscError {
    DiscError::Format(format!("CHD: {e}"))
}

impl ChdDisc {
    pub fn open(path: &Path) -> Result<Self> {
        let file = BufReader::new(File::open(path)?);
        let mut chd = Chd::open(file, None).map_err(chd_err)?;
        let hunk_size = chd.header().hunk_size() as usize;
        if !hunk_size.is_multiple_of(FRAME_SIZE) {
            return Err(DiscError::Format(format!("{}: not a CD image", path.display())));
        }
        let metas: Vec<Metadata> = Vec::try_from(chd.metadata_refs()).map_err(chd_err)?;

        let mut layout = Vec::new();
        let mut frame = 0u32;
        let mut lba = 0u32;
        for meta in metas.iter().filter(|m| m.metatag == TAG_CHT2 || m.metatag == TAG_CHTR) {
            let text = String::from_utf8_lossy(&meta.value);
            let text = text.trim_end_matches('\0');
            let field = |key: &str| -> Option<&str> {
                text.split_whitespace()
                    .find_map(|kv| kv.strip_prefix(key).and_then(|v| v.strip_prefix(':')))
            };
            let number: u8 = field("TRACK").and_then(|v| v.parse().ok()).unwrap_or(0);
            let frames: u32 = field("FRAMES").and_then(|v| v.parse().ok()).unwrap_or(0);
            let pregap: u32 = field("PREGAP").and_then(|v| v.parse().ok()).unwrap_or(0);
            // A pregap whose type starts with 'V' is stored in the file.
            let pregap_in_file = field("PGTYPE").is_some_and(|t| t.starts_with('V'));
            let kind = if field("TYPE") == Some("AUDIO") { TrackKind::Audio } else { TrackKind::Data };

            let (start_frame, sectors) = if pregap_in_file {
                (frame + pregap, frames - pregap.min(frames))
            } else {
                (frame, frames)
            };
            let start_lba = lba + pregap;
            layout.push(TrackFrames {
                track: Track { number, kind, start_lba, sectors },
                frame: start_frame,
            });
            frame += frames.div_ceil(TRACK_PADDING) * TRACK_PADDING;
            lba = start_lba + sectors;
        }
        if layout.is_empty() {
            return Err(DiscError::Format(format!("{}: no CD track metadata", path.display())));
        }
        let tracks = layout.iter().map(|l| l.track.clone()).collect();
        let hunk_buf = chd.get_hunksized_buffer();
        Ok(ChdDisc {
            chd,
            tracks,
            layout,
            frames_per_hunk: (hunk_size / FRAME_SIZE) as u32,
            hunk_buf,
            cmp_buf: Vec::new(),
            cached_hunk: None,
        })
    }
}

impl super::Disc for ChdDisc {
    fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    fn read_sector(&mut self, lba: u32, out: &mut RawSector) -> Result<()> {
        let Some(t) = self
            .layout
            .iter()
            .find(|l| lba >= l.track.start_lba && lba < l.track.start_lba + l.track.sectors)
        else {
            out.fill(0);
            return Ok(());
        };
        let frame = t.frame + (lba - t.track.start_lba);
        let hunk = frame / self.frames_per_hunk;
        if self.cached_hunk != Some(hunk) {
            self.cached_hunk = None;
            let mut h = self.chd.hunk(hunk).map_err(chd_err)?;
            h.read_hunk_in(&mut self.cmp_buf, &mut self.hunk_buf).map_err(chd_err)?;
            self.cached_hunk = Some(hunk);
        }
        let off = (frame % self.frames_per_hunk) as usize * FRAME_SIZE;
        out.copy_from_slice(&self.hunk_buf[off..off + RAW_SECTOR_SIZE]);
        // CHD stores audio samples big-endian; the drive delivers little-endian.
        if t.track.kind == TrackKind::Audio {
            for pair in out.as_chunks_mut::<2>().0 {
                pair.swap(0, 1);
            }
        }
        Ok(())
    }
}
