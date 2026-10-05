//! BIN/CUE images: one or more raw (2352-byte sector) files described by a
//! cue sheet.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use super::{DiscError, RAW_SECTOR_SIZE, RawSector, Result, Track, TrackKind};

struct TrackFile {
    track: Track,
    file: usize,
    /// Sector in `file` that holds `track.start_lba`.
    file_sector: u32,
}

pub struct CueDisc {
    files: Vec<File>,
    tracks: Vec<Track>,
    layout: Vec<TrackFile>,
}

impl CueDisc {
    pub fn open(cue_path: &Path) -> Result<Self> {
        let sheet = std::fs::read_to_string(cue_path)?;
        let dir = cue_path.parent().unwrap_or(Path::new("."));

        struct Pending {
            number: u8,
            kind: TrackKind,
            file: usize,
            index1: Option<u32>,
        }
        let mut file_paths: Vec<PathBuf> = Vec::new();
        let mut pending: Vec<Pending> = Vec::new();
        for line in sheet.lines() {
            let line = line.trim();
            let mut words = line.split_whitespace();
            match words.next().map(str::to_ascii_uppercase).as_deref() {
                Some("FILE") => {
                    let name = quoted_or_word(&line[4..])
                        .ok_or_else(|| DiscError::Format(format!("bad FILE line: {line}")))?;
                    file_paths.push(dir.join(name));
                }
                Some("TRACK") => {
                    let number = words.next().and_then(|n| n.parse().ok());
                    let mode = words.next().unwrap_or_default().to_ascii_uppercase();
                    let (Some(number), false) = (number, file_paths.is_empty()) else {
                        return Err(DiscError::Format(format!("bad TRACK line: {line}")));
                    };
                    let kind = match mode.as_str() {
                        "AUDIO" => TrackKind::Audio,
                        "MODE2/2352" | "MODE1/2352" => TrackKind::Data,
                        other => {
                            return Err(DiscError::Format(format!(
                                "unsupported track mode {other} (need raw 2352-byte sectors)"
                            )));
                        }
                    };
                    pending.push(Pending { number, kind, file: file_paths.len() - 1, index1: None });
                }
                Some("INDEX") => {
                    let idx: Option<u32> = words.next().and_then(|n| n.parse().ok());
                    let msf = words.next().and_then(parse_msf);
                    if let (Some(1), Some(sector), Some(track)) = (idx, msf, pending.last_mut()) {
                        track.index1 = Some(sector);
                    }
                }
                _ => {}
            }
        }
        if pending.is_empty() {
            return Err(DiscError::Format(format!("{}: no tracks", cue_path.display())));
        }

        let mut files = Vec::new();
        let mut file_sectors = Vec::new();
        for path in &file_paths {
            let f = File::open(path).map_err(|e| {
                DiscError::Format(format!("{}: {e} (check the FILE line in the cue sheet)", path.display()))
            })?;
            file_sectors.push((f.metadata()?.len() / RAW_SECTOR_SIZE as u64) as u32);
            files.push(f);
        }

        // Lay tracks out on the disc: each file follows the previous one.
        let mut layout: Vec<TrackFile> = Vec::new();
        let mut file_base = 0u32; // disc LBA of the current file's first sector
        for (i, t) in pending.iter().enumerate() {
            if i > 0 && pending[i - 1].file != t.file {
                file_base += file_sectors[pending[i - 1].file];
            }
            let index1 = t.index1.ok_or_else(|| DiscError::Format(format!("track {} has no INDEX 01", t.number)))?;
            let next_in_file = pending.get(i + 1).filter(|n| n.file == t.file).and_then(|n| n.index1);
            let end = next_in_file.unwrap_or(file_sectors[t.file]);
            layout.push(TrackFile {
                track: Track {
                    number: t.number,
                    kind: t.kind,
                    start_lba: file_base + index1,
                    sectors: end.saturating_sub(index1),
                },
                file: t.file,
                file_sector: index1,
            });
        }
        let tracks = layout.iter().map(|l| l.track.clone()).collect();
        Ok(CueDisc { files, tracks, layout })
    }
}

impl super::Disc for CueDisc {
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
        let sector = t.file_sector + (lba - t.track.start_lba);
        let f = &mut self.files[t.file];
        f.seek(SeekFrom::Start(u64::from(sector) * RAW_SECTOR_SIZE as u64))?;
        f.read_exact(out)?;
        Ok(())
    }
}

fn quoted_or_word(s: &str) -> Option<String> {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix('"') {
        rest.find('"').map(|end| rest[..end].to_string())
    } else {
        s.split_whitespace().next().map(str::to_string)
    }
}

/// Parses a cue "mm:ss:ff" (decimal) into a sector count.
fn parse_msf(s: &str) -> Option<u32> {
    let mut parts = s.split(':').map(|p| p.parse::<u32>().ok());
    let (m, sec, f) = (parts.next()??, parts.next()??, parts.next()??);
    Some((m * 60 + sec) * 75 + f)
}
