//! Just enough ISO 9660 to find files on a PlayStation disc (mode 2 form 1
//! sectors, 2048 data bytes at offset 24).

use super::{Disc, DiscError, RawSector, Result};

const USER_DATA: std::ops::Range<usize> = 24..24 + 2048;
const PVD_LBA: u32 = 16;

/// A file's extent on disc.
#[derive(Debug, Clone, Copy)]
pub struct FileExtent {
    pub lba: u32,
    pub size: u32,
}

fn read_user_data(disc: &mut dyn Disc, lba: u32) -> Result<Vec<u8>> {
    let mut raw: RawSector = [0; 2352];
    disc.read_sector(lba, &mut raw)?;
    Ok(raw[USER_DATA].to_vec())
}

fn u32le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

/// Finds `path` ("SYSTEM.CNF", "DIR/FILE.EXT", case-insensitive, `;1`
/// version suffix optional).
pub fn find_file(disc: &mut dyn Disc, path: &str) -> Result<FileExtent> {
    let pvd = read_user_data(disc, PVD_LBA)?;
    if &pvd[1..6] != b"CD001" {
        return Err(DiscError::Format("not an ISO 9660 disc".into()));
    }
    let root = &pvd[156..156 + 34];
    let mut dir = FileExtent { lba: u32le(&root[2..]), size: u32le(&root[10..]) };

    let wanted: Vec<String> = path
        .trim_start_matches(['\\', '/'])
        .split(['\\', '/'])
        .map(|p| p.split(';').next().unwrap_or(p).to_ascii_uppercase())
        .collect();
    for (depth, name) in wanted.iter().enumerate() {
        let is_last = depth + 1 == wanted.len();
        let entry = find_in_dir(disc, dir, name)?
            .ok_or_else(|| DiscError::Format(format!("{path}: not found on disc")))?;
        if !is_last && !entry.1 {
            return Err(DiscError::Format(format!("{path}: {name} is not a directory")));
        }
        dir = entry.0;
    }
    Ok(dir)
}

/// Returns the entry named `name` in `dir` and whether it is a directory.
fn find_in_dir(disc: &mut dyn Disc, dir: FileExtent, name: &str) -> Result<Option<(FileExtent, bool)>> {
    let sectors = dir.size.div_ceil(2048);
    for i in 0..sectors {
        let data = read_user_data(disc, dir.lba + i)?;
        let mut off = 0;
        while off < data.len() {
            let len = data[off] as usize;
            if len == 0 {
                break; // records do not cross sectors
            }
            let rec = &data[off..off + len];
            let name_len = rec[32] as usize;
            let rec_name = String::from_utf8_lossy(&rec[33..33 + name_len]);
            let rec_name = rec_name.split(';').next().unwrap_or_default();
            if rec_name.eq_ignore_ascii_case(name) {
                let extent = FileExtent { lba: u32le(&rec[2..]), size: u32le(&rec[10..]) };
                return Ok(Some((extent, rec[25] & 0x02 != 0)));
            }
            off += len;
        }
    }
    Ok(None)
}

/// Reads a whole file.
pub fn read_file(disc: &mut dyn Disc, extent: FileExtent) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(extent.size as usize);
    for i in 0..extent.size.div_ceil(2048) {
        out.extend_from_slice(&read_user_data(disc, extent.lba + i)?);
    }
    out.truncate(extent.size as usize);
    Ok(out)
}

/// The boot executable named by SYSTEM.CNF (`BOOT = cdrom:\SLUS_010.42;1`).
pub fn boot_executable(disc: &mut dyn Disc) -> Result<(String, Vec<u8>)> {
    let cnf_extent = find_file(disc, "SYSTEM.CNF")?;
    let cnf = read_file(disc, cnf_extent)?;
    let cnf = String::from_utf8_lossy(&cnf);
    let boot = cnf
        .lines()
        .find_map(|l| {
            let (key, value) = l.split_once('=')?;
            key.trim().eq_ignore_ascii_case("BOOT").then(|| value.trim().to_string())
        })
        .ok_or_else(|| DiscError::Format("SYSTEM.CNF has no BOOT line".into()))?;
    let path = boot.trim_start_matches("cdrom:").trim_start_matches(['\\', '/']);
    let exe_extent = find_file(disc, path)?;
    let exe = read_file(disc, exe_extent)?;
    Ok((path.split(';').next().unwrap_or(path).to_string(), exe))
}
