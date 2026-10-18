//! Disc file replacement, and extracting the disc's files to make mods.
//!
//! The game keeps its data in STAGE0.HED/STAGE0.CDF (a flat file list) and
//! STAGE1..5.CDF (folders of files). A file is a run of chunks, each a
//! 16-byte header (type, last-chunk flag, valid bytes per sector, sector
//! count, load address) and its payload strided over whole sectors; the game
//! reads a file from its first sector on and learns its length from the
//! chunk headers. So a replacement of any size can be served by the CD drive
//! in place of the original, from the file's first sector on, without
//! touching the disc's tables (cdrom.rs does that with `Overrides`).
//!
//! Names follow the decompilation's asset tools (decomp/tools/peassets):
//! `stage0/file<id>` and `stage<n>/<folder>/file<id>` for files, and
//! `<file>/<chunk index>.<ext>` for a chunk's clean payload (header and
//! sector padding removed), the same bytes as their `raw/` store. A mod
//! replaces a chunk with `files/<chunk path>`, or a whole file (its on-disc
//! sectors) with `files/<file>.bin`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::disc::{Disc, iso9660};

const SECTOR: usize = crate::disc::SECTOR_SIZE;
const HEADER: usize = 0x10;
/// STAGE0.HED: three stream descriptors, then this many file entries.
const STAGE0_FILES: usize = 0x360;
const STAGE0_FILE_TABLE: usize = 0x78;
/// STAGEn.CDF: a folder table filling the first sector; each folder starts
/// with a sector holding its file list.
const FOLDERS: usize = 0x100;
const FOLDER_FILES: usize = 0xA2;
/// No chunk on the discs comes near this; a larger count is not a chunk.
const MAX_CHUNK_SECTORS: u32 = 0x1000;

/// A file of the game's file system, and the disc sector it starts at.
#[derive(Debug, Clone)]
pub struct DiscFile {
    pub key: String,
    pub lba: u32,
}

/// One chunk of a file: its header as on disc, and its clean payload.
pub struct Chunk {
    pub header: [u8; HEADER],
    pub payload: Vec<u8>,
}

impl Chunk {
    fn sector_len(&self) -> usize {
        usize::from(u16::from_le_bytes([self.header[2], self.header[3]]))
    }

    /// The extension the asset tools give this chunk's type.
    pub fn extension(&self) -> &'static str {
        match self.header[0] {
            0 => "pe2pkg",
            1 => "pe2img",
            2 => "pe2clut",
            4 => "pe2cap2",
            5 => "bs",
            6 => "spk",
            7 => "txt",
            _ => "bin",
        }
    }

    /// The chunk as on disc: the header (with its sector count for this
    /// payload) and the payload strided over sectors.
    fn pack(&self, last: bool) -> Vec<u8> {
        let sector_len = self.sector_len();
        let first = sector_len - HEADER;
        let sectors = 1 + self.payload.len().saturating_sub(first).div_ceil(sector_len);
        let mut out = vec![0u8; sectors * SECTOR];
        out[..HEADER].copy_from_slice(&self.header);
        out[1] = if last { 0xFF } else { 0x01 };
        out[4..8].copy_from_slice(&(sectors as u32).to_le_bytes());
        let (head, rest) = self.payload.split_at(first.min(self.payload.len()));
        out[HEADER..HEADER + head.len()].copy_from_slice(head);
        for (k, part) in rest.chunks(sector_len).enumerate() {
            let at = (k + 1) * SECTOR;
            out[at..at + part.len()].copy_from_slice(part);
        }
        out
    }
}

fn u32le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn read_sector(disc: &mut dyn Disc, lba: u32) -> Option<Vec<u8>> {
    let mut raw = [0u8; crate::disc::RAW_SECTOR_SIZE];
    disc.read_sector(lba, &mut raw).ok()?;
    Some(raw[24..24 + SECTOR].to_vec())
}

/// The files of the game's file system on `disc`, in disc order.
pub fn layout(disc: &mut dyn Disc) -> Vec<DiscFile> {
    let mut files = Vec::new();
    let stage0 = iso9660::find_file(disc, "STAGE0.HED").ok().zip(iso9660::find_file(disc, "STAGE0.CDF").ok());
    if let Some((hed, cdf)) = stage0
        && let Ok(table) = iso9660::read_file(disc, hed)
    {
        for k in 0..STAGE0_FILES {
            let at = STAGE0_FILE_TABLE + k * 8;
            let Some(entry) = table.get(at..at + 8) else { break };
            let (id, offset) = (u32le(entry), u32le(&entry[4..]));
            if id == u32::MAX {
                break;
            }
            files.push(DiscFile { key: format!("stage0/file{id}"), lba: cdf.lba + offset });
        }
    }
    for stage in 1..=5 {
        let Ok(cdf) = iso9660::find_file(disc, &format!("STAGE{stage}.CDF")) else { continue };
        let Some(table) = read_sector(disc, cdf.lba) else { continue };
        let mut folder_lba = cdf.lba + 1;
        for k in 0..FOLDERS {
            let (folder, sectors) = (u32le(&table[k * 8..]), u32le(&table[k * 8 + 4..]));
            // The game's own rule (Fs_InitFolderTable): a zero size ends it.
            if sectors == 0 {
                break;
            }
            if let Some(list) = read_sector(disc, folder_lba) {
                for f in 0..FOLDER_FILES {
                    let (id, offset) = (u32le(&list[f * 8..]), u32le(&list[f * 8 + 4..]));
                    // As Fs_BuildFolderTables reads it: a zero offset ends it.
                    if offset == 0 {
                        break;
                    }
                    files.push(DiscFile { key: format!("stage{stage}/{folder}/file{id}"), lba: folder_lba + offset });
                }
            }
            folder_lba += sectors;
        }
    }
    files
}

/// The chunks of the file starting at `lba`.
pub fn read_chunks(disc: &mut dyn Disc, lba: u32) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut at = lba;
    while let Some(first) = read_sector(disc, at) {
        let (kind, last, sector_len, count) =
            (first[0], first[1], usize::from(u16::from_le_bytes([first[2], first[3]])), u32le(&first[4..]));
        let valid = matches!(kind, 0 | 1 | 2 | 4 | 5 | 6 | 7)
            && matches!(last, 0x01 | 0xFF)
            && (HEADER + 1..=SECTOR).contains(&sector_len)
            && (1..=MAX_CHUNK_SECTORS).contains(&count)
            && u32le(&first[12..]) == 0;
        if !valid {
            break;
        }
        let mut header = [0u8; HEADER];
        header.copy_from_slice(&first[..HEADER]);
        let mut payload = first[HEADER..sector_len].to_vec();
        for k in 1..count {
            let Some(s) = read_sector(disc, at + k) else { break };
            payload.extend_from_slice(&s[..sector_len]);
        }
        chunks.push(Chunk { header, payload });
        at += count;
        if last == 0xFF {
            break;
        }
    }
    chunks
}

/// What the mods replace in one file.
#[derive(Default)]
struct Replacement {
    whole: Option<PathBuf>,
    chunks: BTreeMap<usize, PathBuf>,
}

/// The file replacements of the mods in use, by file key (higher-priority
/// mods last, so they win).
fn replacements() -> HashMap<String, Replacement> {
    let mut out: HashMap<String, Replacement> = HashMap::new();
    if !super::enabled() {
        return out;
    }
    for m in super::installed().into_iter().filter(|m| m.enabled).rev() {
        let root = m.dir.join("files");
        let mut stack = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            for e in entries.flatten() {
                let path = e.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                let Ok(rel) = path.strip_prefix(&root) else { continue };
                let rel = rel.to_string_lossy().replace('\\', "/");
                let Some((stem, ext)) = rel.rsplit_once('.') else { continue };
                if ext.eq_ignore_ascii_case("bin") && stem.rsplit('/').next().is_some_and(|n| n.starts_with("file")) {
                    out.entry(stem.to_string()).or_default().whole = Some(path);
                } else if let Some((file, index)) = stem.rsplit_once('/')
                    && let Ok(index) = index.parse::<usize>()
                {
                    out.entry(file.to_string()).or_default().chunks.insert(index, path);
                }
            }
        }
    }
    out
}

/// A disc's replaced files: first sector -> the file's on-disc bytes.
pub type Overrides = HashMap<u32, Arc<Vec<u8>>>;

/// Every file's first sector (where a read of a file begins) and name.
pub type Starts = HashMap<u32, String>;

/// The files the mods replace on `disc`, built as the drive serves them,
/// and where every file starts (also wanted by PE2_TRACE=files).
pub fn overrides(disc: &mut dyn Disc, number: usize) -> (Overrides, Starts) {
    let wanted = replacements();
    if wanted.is_empty() && !crate::runtime::trace("files") {
        return Default::default();
    }
    let files = layout(disc);
    let starts = files.iter().map(|f| (f.lba, f.key.clone())).collect();
    let mut out = Overrides::new();
    let mut found = HashSet::new();
    for f in &files {
        let Some(r) = wanted.get(&f.key) else { continue };
        found.insert(f.key.as_str());
        let bytes = if let Some(path) = &r.whole {
            match std::fs::read(path) {
                Ok(mut b) => {
                    b.resize(b.len().div_ceil(SECTOR) * SECTOR, 0);
                    b
                }
                Err(e) => {
                    eprintln!("pe2: mods: {}: {e}", path.display());
                    continue;
                }
            }
        } else {
            let mut chunks = read_chunks(disc, f.lba);
            for (&index, path) in &r.chunks {
                let Some(chunk) = chunks.get_mut(index) else {
                    eprintln!("pe2: mods: {}: {} has no chunk {index}", path.display(), f.key);
                    continue;
                };
                match std::fs::read(path) {
                    Ok(b) => chunk.payload = b,
                    Err(e) => eprintln!("pe2: mods: {}: {e}", path.display()),
                }
            }
            let count = chunks.len();
            chunks.iter().enumerate().flat_map(|(k, c)| c.pack(k + 1 == count)).collect()
        };
        out.insert(f.lba, Arc::new(bytes));
    }
    if number == 0 {
        for key in wanted.keys().filter(|k| !found.contains(k.as_str())) {
            eprintln!("pe2: mods: files/{key}: no such file on the disc");
        }
    }
    if !out.is_empty() {
        eprintln!("pe2: mods: disc {}: {} files replaced", number + 1, out.len());
    }
    (out, starts)
}

unsafe extern "C" {
    fn pcrt_DecDCTvlcSize2(size: i32) -> i32;
    fn pcrt_DecDCTvlc2(bs: *mut u32, buf: *mut u32, table: *mut u16) -> i32;
}

/// A room background (a `.bs` chunk) as the game shows it: decoded as the
/// game does (libpress's run-length decoding with its default MDEC tables,
/// 15-bit output without the mask bit), 16-pixel columns of macroblocks
/// laid side by side; 320x240 pixels, row by row.
pub fn background_pixels(bs: &[u8]) -> Option<Vec<u16>> {
    const W: usize = 320;
    const H: usize = 240;
    let words = usize::from(u16::from_le_bytes([*bs.first()?, *bs.get(1)?]));
    if bs.len() < 8 || u16::from_le_bytes([bs[2], bs[3]]) != 0x3800 || words == 0 {
        return None;
    }
    // The bitstream, word-aligned, with room for the decoder to read past
    // its end.
    let mut input = vec![0u32; bs.len().div_ceil(4) + 1024];
    for (w, c) in input.iter_mut().zip(bs.chunks(4)) {
        let mut b = [0u8; 4];
        b[..c.len()].copy_from_slice(c);
        *w = u32::from_le_bytes(b);
    }
    let mut rle = vec![0u32; words + 1];
    // SAFETY: `rle` holds the header word and the `words` words the frame
    // decodes to; `input` is padded past the bitstream.
    unsafe {
        pcrt_DecDCTvlcSize2(0);
        pcrt_DecDCTvlc2(input.as_mut_ptr(), rle.as_mut_ptr(), std::ptr::null_mut());
    }
    let mut mdec = crate::runtime::mdec::Mdec::default();
    let quant = crate::runtime::libpress::default_quant();
    mdec.write_word(0x4000_0001);
    for c in quant.chunks(4).chain(quant.chunks(4)) {
        mdec.write_word(u32::from_le_bytes([c[0], c[1], c[2], c[3]]));
    }
    mdec.write_word(0x6000_0000);
    for c in crate::runtime::libpress::default_scale().chunks(2) {
        mdec.write_word(u32::from(c[0] as u16) | (u32::from(c[1] as u16) << 16));
    }
    // The decode command, 15-bit (DecDCTin's mode 0), then the data.
    mdec.write_word((rle[0] & !(1 << 25 | 1 << 27)) | 1 << 27);
    for &w in &rle[1..] {
        mdec.write_word(w);
    }
    let out: Vec<u16> = mdec.output.iter().flat_map(|&w| [w as u16, (w >> 16) as u16]).collect();
    if out.len() < W * H {
        return None;
    }
    // Macroblocks come column by column, 16x16 pixels each: a column is a
    // 16x240 strip.
    let mut pixels = vec![0u16; W * H];
    for (strip, column) in out.as_chunks::<{ 16 * H }>().0.iter().take(W / 16).enumerate() {
        for (y, row) in column.as_chunks::<16>().0.iter().enumerate() {
            pixels[y * W + strip * 16..y * W + strip * 16 + 16].copy_from_slice(row);
        }
    }
    Some(pixels)
}

/// The dialogue bundles of `disc` and the pictures that may hold their
/// glyphs (see dialogue.rs).
pub fn dialogue_sources(disc: &mut dyn Disc) -> super::dialogue::Sources {
    let mut out = super::dialogue::Sources::default();
    for f in layout(disc) {
        for (k, c) in read_chunks(disc, f.lba).into_iter().enumerate() {
            match c.header[0] {
                0 => out.packages.entry(f.key.clone()).or_default().push(c.payload),
                1 => out.pictures.entry(f.key.clone()).or_default().push(c.payload),
                2 => out.palettes.entry(f.key.clone()).or_default().push(c.payload),
                5 => out.backgrounds.entry(f.key.clone()).or_default().push(c.payload),
                4 => out.bundles.push((f.key.clone(), k, c.payload)),
                _ => {}
            }
        }
    }
    out
}

/// Saves every file of the discs' file systems, chunk by chunk, under
/// `dumps/files` (`<file>/<index>.<ext>`; a file differing on a later disc
/// goes under `disc<n>/`), each room background as the picture the game
/// shows, as `dumps/backgrounds/<hash>.png` (see mods::background), the
/// texture sheets under `dumps/textures` (see sheets.rs), and the dialogue
/// as text under `dumps/text` (see dialogue.rs).
pub fn extract(discs: &mut [Box<dyn Disc>], dumps: &Path) -> std::io::Result<()> {
    let out = dumps.join("files");
    let pictures = dumps.join("backgrounds");
    std::fs::create_dir_all(&pictures)?;
    let mut seen: HashMap<String, u64> = HashMap::new();
    let mut backgrounds = HashSet::new();
    let mut dialogue = super::dialogue::Sources::default();
    let (mut files, mut chunks, mut bytes) = (0usize, 0usize, 0usize);
    for (n, disc) in discs.iter_mut().enumerate() {
        let layout = layout(disc.as_mut());
        eprintln!("pe2: extract: disc {}: {} files", n + 1, layout.len());
        for f in &layout {
            let parts = read_chunks(disc.as_mut(), f.lba);
            let hash = super::fnv1a(parts.iter().flat_map(|c| c.header.iter().chain(&c.payload)).copied());
            let dir = match seen.get(&f.key) {
                Some(&h) if h == hash => continue,
                Some(_) => out.join(format!("disc{}", n + 1)).join(&f.key),
                None => out.join(&f.key),
            };
            seen.insert(f.key.clone(), hash);
            std::fs::create_dir_all(&dir)?;
            for (k, c) in parts.iter().enumerate() {
                // Text without the zeros padding it, for editing.
                let data = match c.extension() {
                    "txt" => &c.payload[..c.payload.iter().rposition(|&b| b != 0).map_or(0, |e| e + 1)],
                    _ => &c.payload[..],
                };
                std::fs::write(dir.join(format!("{k}.{}", c.extension())), data)?;
                if c.header[0] == 5
                    && let Some(pixels) = background_pixels(&c.payload)
                {
                    let hash = super::screen_hash(320, 240, &pixels);
                    let path = pictures.join(format!("{hash:016x}.png"));
                    if backgrounds.insert(hash) && !path.exists() {
                        super::save_png(&path, 320, 240, &pixels).map_err(std::io::Error::other)?;
                    }
                }
                match c.header[0] {
                    0 => dialogue.packages.entry(f.key.clone()).or_default().push(c.payload.clone()),
                    1 => dialogue.pictures.entry(f.key.clone()).or_default().push(c.payload.clone()),
                    2 => dialogue.palettes.entry(f.key.clone()).or_default().push(c.payload.clone()),
                    5 => dialogue.backgrounds.entry(f.key.clone()).or_default().push(c.payload.clone()),
                    4 => dialogue.bundles.push((f.key.clone(), k, c.payload.clone())),
                    _ => {}
                }
                chunks += 1;
                bytes += c.payload.len();
            }
            files += 1;
        }
    }
    eprintln!("pe2: extract: {files} files, {chunks} chunks, {} MB in {}", bytes >> 20, out.display());
    eprintln!("pe2: extract: {} backgrounds in {}", backgrounds.len(), pictures.display());
    let textures = dumps.join("textures");
    let catalog = super::sheets::Catalog::new(&dialogue);
    let (painted, checked, mismatched) = super::rooms::paint(&dialogue, &catalog);
    let coloured = super::sheets::extract(&catalog, &painted, &textures)?;
    eprintln!("pe2: extract: {} texture sheets ({coloured} in colour) in {}", catalog.sheets.len(), textures.display());
    eprintln!(
        "pe2: extract: {} sheets coloured from rooms' sprites; {checked} sprite tables checked against their backgrounds, {mismatched} differ",
        painted.len()
    );
    let text = dumps.join("text");
    let (caps, lines) = super::dialogue::extract(&dialogue, &text)?;
    eprintln!("pe2: extract: {lines} lines of dialogue in {caps} files in {}", text.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Packing strides the payload over sectors at the chunk's sector_len
    /// and counts the sectors in the header.
    #[test]
    fn pack_strides_payload() {
        let mut header = [0u8; HEADER];
        header[0] = 7;
        header[1] = 0x01;
        header[2..4].copy_from_slice(&0x500u16.to_le_bytes());
        header[4..8].copy_from_slice(&9u32.to_le_bytes());
        let payload: Vec<u8> = (0..0x4F0 + 0x500 + 3).map(|i| (i % 251) as u8 + 1).collect();
        let chunk = Chunk { header, payload: payload.clone() };
        let disc = chunk.pack(true);
        assert_eq!(disc.len(), 3 * SECTOR);
        assert_eq!(disc[1], 0xFF);
        assert_eq!(u32le(&disc[4..]), 3);
        let mut back = disc[HEADER..0x500].to_vec();
        back.extend_from_slice(&disc[SECTOR..SECTOR + 0x500]);
        back.extend_from_slice(&disc[2 * SECTOR..2 * SECTOR + 3]);
        assert_eq!(back, payload);
        assert!(disc[0x500..SECTOR].iter().all(|&b| b == 0));
    }
}
