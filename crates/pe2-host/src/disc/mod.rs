//! Reading the player's discs: raw 2352-byte sectors from BIN/CUE or CHD
//! images, and files through ISO 9660.

mod chd;
mod cue;
pub mod iso9660;

use std::fmt;
use std::io;
use std::path::Path;

/// Bytes in a raw CD sector (sync, header, subheader, data, EDC/ECC).
pub const RAW_SECTOR_SIZE: usize = 2352;
/// Bytes of data in a mode 2 form 1 sector, as files are read.
pub const SECTOR_SIZE: usize = 2048;
/// Sectors before LBA 0 (the two-second lead-in): MSF 00:02:00 is LBA 0.
pub const LBA_MSF_OFFSET: u32 = 150;

pub type RawSector = [u8; RAW_SECTOR_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackKind {
    Data,
    Audio,
}

/// A track as the drive reports it (CdlGetTN / CdlGetTD).
#[derive(Debug, Clone)]
pub struct Track {
    pub number: u8,
    pub kind: TrackKind,
    /// LBA of INDEX 01.
    pub start_lba: u32,
    pub sectors: u32,
}

#[derive(Debug)]
pub enum DiscError {
    Io(io::Error),
    Format(String),
}

impl fmt::Display for DiscError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscError::Io(e) => write!(f, "{e}"),
            DiscError::Format(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<io::Error> for DiscError {
    fn from(e: io::Error) -> Self {
        DiscError::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, DiscError>;

/// A disc image the drive reads raw sectors from.
pub trait Disc: Send {
    fn tracks(&self) -> &[Track];

    /// Reads the raw sector at `lba`. Sectors outside every track read as
    /// zeroes, like the gaps of a real disc.
    fn read_sector(&mut self, lba: u32, out: &mut RawSector) -> Result<()>;

    fn total_sectors(&self) -> u32 {
        self.tracks().last().map_or(0, |t| t.start_lba + t.sectors)
    }
}

/// Opens a `.cue` or `.chd` image.
pub fn open(path: &Path) -> Result<Box<dyn Disc>> {
    let ext = path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase);
    match ext.as_deref() {
        Some("cue") => Ok(Box::new(cue::CueDisc::open(path)?)),
        Some("chd") => Ok(Box::new(chd::ChdDisc::open(path)?)),
        _ => Err(DiscError::Format(format!("{}: unsupported image (use a .cue or .chd)", path.display()))),
    }
}

/// Converts an LBA to the drive's BCD minute/second/sector.
pub fn lba_to_bcd_msf(lba: u32) -> [u8; 3] {
    let pos = lba + LBA_MSF_OFFSET;
    let bcd = |v: u32| (((v / 10) << 4) | (v % 10)) as u8;
    [bcd(pos / 75 / 60), bcd((pos / 75) % 60), bcd(pos % 75)]
}

/// Converts a BCD minute/second/sector to an LBA.
pub fn bcd_msf_to_lba(msf: [u8; 3]) -> u32 {
    let dec = |b: u8| u32::from(b >> 4) * 10 + u32::from(b & 0xF);
    let pos = (dec(msf[0]) * 60 + dec(msf[1])) * 75 + dec(msf[2]);
    pos.saturating_sub(LBA_MSF_OFFSET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msf_round_trip() {
        assert_eq!(lba_to_bcd_msf(0), [0x00, 0x02, 0x00]);
        assert_eq!(lba_to_bcd_msf(16), [0x00, 0x02, 0x16]);
        assert_eq!(bcd_msf_to_lba([0x00, 0x02, 0x16]), 16);
        for lba in [0, 1, 74, 75, 4499, 4500, 333_000] {
            assert_eq!(bcd_msf_to_lba(lba_to_bcd_msf(lba)), lba);
        }
    }
}
