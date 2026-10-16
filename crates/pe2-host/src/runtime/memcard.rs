//! Memory cards: the libmcrd calls the game makes, over card images on disk.
//!
//! Each slot holds a raw 128 KB card image, the `.mcd` / `.mcr` layout most
//! emulators use, so saves can move between them. Layout (psx-spx): sixteen
//! 8 KB blocks of 128-byte frames; block 0 holds the "MC" header frame and
//! one directory frame per data block (1-15):
//!
//! ```text
//!   0x00 u32  state: 0x51 first block of a file, 0x52 middle, 0x53 last,
//!             0xA0 free (0xA1-0xA3: deleted)
//!   0x04 u32  file size in bytes (first block only)
//!   0x08 u16  next block of the file (0-14), or 0xFFFF
//!   0x0A      file name, NUL-terminated (up to 20 characters)
//!   0x7F u8   XOR of bytes 0x00-0x7E
//! ```
//!
//! Images live in `memcards/` next to the executable (`pe2_card1.mcd`,
//! `pe2_card2.mcd`), or wherever PE2_MEMCARD1 / PE2_MEMCARD2 or pe2.toml's
//! [memcard] slot1 / slot2 point. A slot
//! without an image file holds a freshly formatted card; the file is written
//! on the first save.
//!
//! The asynchronous calls (Exist, Accept, ReadData, WriteData, WriteFile)
//! finish at once; MemCardSync reports the result once, then -1 ("nothing
//! registered") until the next call, which is what the game polls for.

use std::path::PathBuf;
use std::sync::Mutex;

use super::irq;

const CARD_SIZE: usize = 128 * 1024;
const FRAME: usize = 128;
const BLOCK: usize = 8192;
const BLOCKS: usize = 15;
const NAME_LEN: usize = 20;

const MC_FUNC_EXIST: i32 = 1;
const MC_FUNC_ACCEPT: i32 = 2;
const MC_FUNC_WRITE_FILE: i32 = 4;
const MC_FUNC_READ_DATA: i32 = 5;
const MC_FUNC_WRITE_DATA: i32 = 6;

const MC_ERR_NONE: i32 = 0;
const MC_ERR_CARD_NOT_EXIST: i32 = 1;
const MC_ERR_NEW_CARD: i32 = 3;
const MC_ERR_NOT_FORMAT: i32 = 4;
const MC_ERR_FILE_NOT_EXIST: i32 = 5;
const MC_ERR_ALREADY_EXIST: i32 = 6;
const MC_ERR_BLOCK_FULL: i32 = 7;

const STATE_FIRST: u32 = 0x51;
const STATE_MIDDLE: u32 = 0x52;
const STATE_LAST: u32 = 0x53;
const STATE_FREE: u32 = 0xA0;

struct Card {
    path: PathBuf,
    data: Vec<u8>,
    /// Set at power-on until the game accepts the card, as the card's own
    /// "new card" flag is.
    new_card: bool,
}

struct OpenFile {
    port: usize,
    first: usize,
}

struct State {
    cards: [Card; 2],
    open: Option<OpenFile>,
    /// Result of the last asynchronous call, until MemCardSync reports it.
    pending: Option<(i32, i32)>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn image_path(slot: usize) -> PathBuf {
    if let Some(p) = std::env::var_os(format!("PE2_MEMCARD{}", slot + 1)) {
        return PathBuf::from(p);
    }
    let cfg = &crate::config::get().memcard;
    if let Some(p) = [&cfg.slot1, &cfg.slot2][slot] {
        return p.clone();
    }
    let dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_default();
    dir.join("memcards").join(format!("pe2_card{}.mcd", slot + 1))
}

fn load(slot: usize) -> Card {
    let path = image_path(slot);
    let data = match std::fs::read(&path) {
        Ok(d) if d.len() == CARD_SIZE => d,
        Ok(d) => {
            eprintln!("pe2: {}: not a 128 KB card image ({} bytes); using a blank card", path.display(), d.len());
            formatted()
        }
        Err(_) => formatted(),
    };
    Card { path, data, new_card: true }
}

fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut s = STATE.lock().unwrap();
    f(s.get_or_insert_with(|| State { cards: [load(0), load(1)], open: None, pending: None }))
}

fn set_checksum(frame: &mut [u8]) {
    frame[FRAME - 1] = frame[..FRAME - 1].iter().fold(0, |a, b| a ^ b);
}

/// An empty, formatted card.
fn formatted() -> Vec<u8> {
    let mut d = vec![0u8; CARD_SIZE];
    d[0] = b'M';
    d[1] = b'C';
    set_checksum(&mut d[..FRAME]);
    for i in 1..=BLOCKS {
        let f = &mut d[i * FRAME..(i + 1) * FRAME];
        f[0..4].copy_from_slice(&STATE_FREE.to_le_bytes());
        f[8..10].copy_from_slice(&0xFFFFu16.to_le_bytes());
        set_checksum(f);
    }
    // Broken-sector list: no entries.
    for i in 16..36 {
        let f = &mut d[i * FRAME..(i + 1) * FRAME];
        f[0..4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        f[8..10].copy_from_slice(&0xFFFFu16.to_le_bytes());
        set_checksum(f);
    }
    // The write-test frame mirrors the header.
    let header = d[..FRAME].to_vec();
    d[63 * FRAME..64 * FRAME].copy_from_slice(&header);
    d
}

/// Directory entry of data block `b` (0-14).
struct Entry<'a>(&'a [u8]);

impl Entry<'_> {
    fn state(&self) -> u32 {
        u32::from_le_bytes(self.0[0..4].try_into().unwrap())
    }
    fn size(&self) -> u32 {
        u32::from_le_bytes(self.0[4..8].try_into().unwrap())
    }
    fn next(&self) -> u16 {
        u16::from_le_bytes(self.0[8..10].try_into().unwrap())
    }
    fn name(&self) -> &[u8] {
        let n = &self.0[10..10 + NAME_LEN + 1];
        &n[..n.iter().position(|&c| c == 0).unwrap_or(NAME_LEN)]
    }
}

impl Card {
    fn is_formatted(&self) -> bool {
        &self.data[0..2] == b"MC"
    }

    fn entry(&self, b: usize) -> Entry<'_> {
        Entry(&self.data[(b + 1) * FRAME..(b + 2) * FRAME])
    }

    fn write_entry(&mut self, b: usize, state: u32, size: u32, next: u16, name: &[u8]) {
        let f = &mut self.data[(b + 1) * FRAME..(b + 2) * FRAME];
        f.fill(0);
        f[0..4].copy_from_slice(&state.to_le_bytes());
        f[4..8].copy_from_slice(&size.to_le_bytes());
        f[8..10].copy_from_slice(&next.to_le_bytes());
        let n = name.len().min(NAME_LEN);
        f[10..10 + n].copy_from_slice(&name[..n]);
        set_checksum(f);
    }

    /// First block of the file called `name`.
    fn find(&self, name: &[u8]) -> Option<usize> {
        (0..BLOCKS).find(|&b| self.entry(b).state() == STATE_FIRST && self.entry(b).name() == name)
    }

    /// The blocks of the file starting at `first`, in order.
    fn chain(&self, first: usize) -> Vec<usize> {
        let mut blocks = vec![first];
        let mut b = first;
        while let Some(next) = Some(self.entry(b).next() as usize).filter(|&n| n < BLOCKS) {
            if blocks.contains(&next) {
                break; // corrupt link
            }
            blocks.push(next);
            b = next;
        }
        blocks
    }

    /// Copies between the file at `first` (from byte `ofs`) and `buf`.
    fn file_io(&mut self, first: usize, ofs: usize, buf: &mut [u8], write: bool) -> bool {
        let chain = self.chain(first);
        if ofs + buf.len() > chain.len() * BLOCK {
            return false;
        }
        for (i, byte) in buf.iter_mut().enumerate() {
            let pos = ofs + i;
            let at = (chain[pos / BLOCK] + 1) * BLOCK + pos % BLOCK;
            if write {
                self.data[at] = *byte;
            } else {
                *byte = self.data[at];
            }
        }
        true
    }

    fn create(&mut self, name: &[u8], blocks: usize) -> i32 {
        if self.find(name).is_some() {
            return MC_ERR_ALREADY_EXIST;
        }
        let free: Vec<usize> = (0..BLOCKS).filter(|&b| self.entry(b).state() & 0xF0 == STATE_FREE).collect();
        if blocks == 0 || free.len() < blocks {
            return MC_ERR_BLOCK_FULL;
        }
        let chosen = &free[..blocks];
        for (i, &b) in chosen.iter().enumerate() {
            let state = match i {
                0 => STATE_FIRST,
                _ if i == blocks - 1 => STATE_LAST,
                _ => STATE_MIDDLE,
            };
            let next = chosen.get(i + 1).map_or(0xFFFF, |&n| n as u16);
            let size = if i == 0 { (blocks * BLOCK) as u32 } else { 0 };
            self.write_entry(b, state, size, next, if i == 0 { name } else { b"" });
        }
        MC_ERR_NONE
    }

    fn save(&self) {
        let result = (|| {
            if let Some(dir) = self.path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            let tmp = self.path.with_extension("tmp");
            std::fs::write(&tmp, &self.data)?;
            std::fs::rename(&tmp, &self.path)
        })();
        if let Err(e) = result {
            eprintln!("pe2: cannot save memory card {}: {e}", self.path.display());
        }
    }
}

/// Slot for a libmcrd channel: 0x00 is port 1, 0x10 port 2. Multitap slots
/// (low nibble) hold no card.
fn port(chan: i32) -> Option<usize> {
    match chan {
        0x00 => Some(0),
        0x10 => Some(1),
        _ => None,
    }
}

/// Reads a NUL-terminated file name from PSX memory.
fn c_name(p: *const u8) -> Vec<u8> {
    let mut name = Vec::new();
    if p.is_null() {
        return name;
    }
    // SAFETY: the game passes a NUL-terminated string in PSX RAM.
    unsafe {
        while name.len() < NAME_LEN && *p.add(name.len()) != 0 {
            name.push(*p.add(name.len()));
        }
    }
    name
}

/// BIOS file-name matching: `?` matches any character, `*` the rest.
fn matches(pattern: &[u8], name: &[u8]) -> bool {
    let mut i = 0;
    for &p in pattern {
        match p {
            b'*' => return true,
            b'?' if i < name.len() => {}
            c if i < name.len() && name[i] == c => {}
            _ => return false,
        }
        i += 1;
    }
    i == name.len()
}

fn trace(what: std::fmt::Arguments) {
    if super::trace("mc") {
        eprintln!("[mc] {what}");
    }
}

fn finish(s: &mut State, func: i32, result: i32) -> i32 {
    trace(format_args!("func {func} -> {result}"));
    s.pending = Some((func, result));
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardInit(_val: i32) {}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardStart() {}

#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardClose() {
    let _g = irq::guard();
    with_state(|s| s.open = None);
}

/// MemCardExist(chan): is a card connected (and has it been changed)?
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardExist(chan: i32) -> i32 {
    let _g = irq::guard();
    with_state(|s| {
        let result = match port(chan) {
            None => MC_ERR_CARD_NOT_EXIST,
            Some(p) if s.cards[p].new_card => MC_ERR_NEW_CARD,
            Some(_) => MC_ERR_NONE,
        };
        finish(s, MC_FUNC_EXIST, result)
    })
}

/// MemCardAccept(chan): checks the card in full and clears its new-card
/// state.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardAccept(chan: i32) -> i32 {
    let _g = irq::guard();
    with_state(|s| {
        let result = match port(chan) {
            None => MC_ERR_CARD_NOT_EXIST,
            Some(p) => {
                let card = &mut s.cards[p];
                let was_new = std::mem::take(&mut card.new_card);
                if !card.is_formatted() {
                    MC_ERR_NOT_FORMAT
                } else if was_new {
                    MC_ERR_NEW_CARD
                } else {
                    MC_ERR_NONE
                }
            }
        };
        finish(s, MC_FUNC_ACCEPT, result)
    })
}

/// MemCardOpen(chan, file, flag): opens a file for ReadData / WriteData.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardOpen(chan: i32, file: *const u8, _flag: i32) -> i32 {
    let _g = irq::guard();
    let name = c_name(file);
    with_state(|s| {
        let Some(p) = port(chan) else { return MC_ERR_CARD_NOT_EXIST };
        let card = &s.cards[p];
        if !card.is_formatted() {
            return MC_ERR_NOT_FORMAT;
        }
        let result = match card.find(&name) {
            Some(first) => {
                s.open = Some(OpenFile { port: p, first });
                MC_ERR_NONE
            }
            None => MC_ERR_FILE_NOT_EXIST,
        };
        trace(format_args!("open {} -> {result}", String::from_utf8_lossy(&name)));
        result
    })
}

fn data_io(func: i32, adrs: *mut u8, ofs: i32, bytes: i32, write: bool) -> i32 {
    with_state(|s| {
        let Some(OpenFile { port, first }) = s.open.as_ref().map(|o| OpenFile { port: o.port, first: o.first }) else {
            return 0;
        };
        if ofs < 0 || bytes < 0 {
            return 0;
        }
        // SAFETY: the game passes `bytes` bytes of PSX RAM at `adrs`.
        let buf = unsafe { std::slice::from_raw_parts_mut(adrs, bytes as usize) };
        let card = &mut s.cards[port];
        if !card.file_io(first, ofs as usize, buf, write) {
            return 0;
        }
        if write {
            card.save();
        }
        trace(format_args!("{} {bytes} bytes at {ofs}", if write { "write" } else { "read" }));
        finish(s, func, MC_ERR_NONE)
    })
}

/// MemCardReadData(adrs, ofs, bytes): reads from the open file.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardReadData(adrs: *mut u8, ofs: i32, bytes: i32) -> i32 {
    let _g = irq::guard();
    data_io(MC_FUNC_READ_DATA, adrs, ofs, bytes, false)
}

/// MemCardWriteData(adrs, ofs, bytes): writes to the open file.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardWriteData(adrs: *mut u8, ofs: i32, bytes: i32) -> i32 {
    let _g = irq::guard();
    data_io(MC_FUNC_WRITE_DATA, adrs, ofs, bytes, true)
}

/// MemCardWriteFile(chan, file, adrs, ofs, bytes): open, write and close.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardWriteFile(chan: i32, file: *const u8, adrs: *mut u8, ofs: i32, bytes: i32) -> i32 {
    let _g = irq::guard();
    let name = c_name(file);
    with_state(|s| {
        let Some(p) = port(chan) else { return finish(s, MC_FUNC_WRITE_FILE, MC_ERR_CARD_NOT_EXIST) };
        let card = &mut s.cards[p];
        let Some(first) = card.find(&name) else { return finish(s, MC_FUNC_WRITE_FILE, MC_ERR_FILE_NOT_EXIST) };
        if ofs < 0 || bytes < 0 {
            return 0;
        }
        // SAFETY: the game passes `bytes` bytes of PSX RAM at `adrs`.
        let buf = unsafe { std::slice::from_raw_parts_mut(adrs, bytes as usize) };
        if !card.file_io(first, ofs as usize, buf, true) {
            return 0;
        }
        card.save();
        finish(s, MC_FUNC_WRITE_FILE, MC_ERR_NONE)
    })
}

/// MemCardCreateFile(chan, file, blocks).
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardCreateFile(chan: i32, file: *const u8, blocks: i32) -> i32 {
    let _g = irq::guard();
    let name = c_name(file);
    with_state(|s| {
        let Some(p) = port(chan) else { return MC_ERR_CARD_NOT_EXIST };
        let card = &mut s.cards[p];
        if !card.is_formatted() {
            return MC_ERR_NOT_FORMAT;
        }
        let result = card.create(&name, blocks.max(0) as usize);
        if result == MC_ERR_NONE {
            card.save();
        }
        trace(format_args!("create {} ({blocks} blocks) -> {result}", String::from_utf8_lossy(&name)));
        result
    })
}

/// MemCardFormat(chan): erases the card.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardFormat(chan: i32) -> i32 {
    let _g = irq::guard();
    with_state(|s| {
        let Some(p) = port(chan) else { return MC_ERR_CARD_NOT_EXIST };
        s.open = None;
        let card = &mut s.cards[p];
        card.data = formatted();
        card.save();
        trace(format_args!("format port {}", p + 1));
        MC_ERR_NONE
    })
}

/// DIRENTRY as the game sees it (PSX layout, 40 bytes).
const DIRENTRY_SIZE: usize = 40;

/// MemCardGetDirentry(chan, name, dir, files, ofs, max): lists the files
/// matching `name`, skipping the first `ofs`, at most `max`.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardGetDirentry(
    chan: i32,
    name: *const u8,
    dir: *mut u8,
    files: *mut i32,
    ofs: i32,
    max: i32,
) -> i32 {
    let _g = irq::guard();
    let pattern = c_name(name);
    with_state(|s| {
        let set_files = |n: i32| {
            if !files.is_null() {
                // SAFETY: the game passes a valid out pointer.
                unsafe { *files = n };
            }
        };
        let Some(p) = port(chan) else {
            set_files(0);
            return MC_ERR_CARD_NOT_EXIST;
        };
        let card = &s.cards[p];
        if !card.is_formatted() {
            set_files(0);
            return MC_ERR_NOT_FORMAT;
        }
        let found = (0..BLOCKS)
            .filter(|&b| card.entry(b).state() == STATE_FIRST && matches(&pattern, card.entry(b).name()))
            .skip(ofs.max(0) as usize)
            .take(max.max(0) as usize);
        let mut n = 0;
        for b in found {
            let e = card.entry(b);
            let mut rec = [0u8; DIRENTRY_SIZE];
            rec[..e.name().len()].copy_from_slice(e.name());
            rec[20..24].copy_from_slice(&e.state().to_le_bytes());
            rec[24..28].copy_from_slice(&e.size().to_le_bytes());
            // `head`: the file's first sector (block 0 is the directory).
            rec[32..36].copy_from_slice(&(((b + 1) * (BLOCK / FRAME)) as u32).to_le_bytes());
            // SAFETY: the game passes room for `max` entries.
            unsafe { std::ptr::copy_nonoverlapping(rec.as_ptr(), dir.add(n * DIRENTRY_SIZE), DIRENTRY_SIZE) };
            n += 1;
        }
        set_files(n as i32);
        trace(format_args!("dir {} -> {n} files", String::from_utf8_lossy(&pattern)));
        MC_ERR_NONE
    })
}

/// MemCardSync(mode, cmds, rslt): 1 and the last call's function and result
/// once it has finished (always, here), -1 when nothing is registered.
#[unsafe(no_mangle)]
pub extern "C" fn pcrt_MemCardSync(_mode: i32, cmds: *mut i32, rslt: *mut i32) -> i32 {
    let _g = irq::guard();
    let Some((cmd, result)) = with_state(|s| s.pending.take()) else { return -1 };
    // SAFETY: the game passes valid out pointers (or null).
    unsafe {
        if !cmds.is_null() {
            *cmds = cmd;
        }
        if !rslt.is_null() {
            *rslt = result;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_write_read_list() {
        let mut card = Card { path: PathBuf::new(), data: formatted(), new_card: false };
        assert_eq!(card.create(b"BASLUS-01042AAAA", 2), MC_ERR_NONE);
        assert_eq!(card.create(b"BASLUS-01042AAAA", 1), MC_ERR_ALREADY_EXIST);
        let first = card.find(b"BASLUS-01042AAAA").unwrap();
        assert_eq!(card.chain(first).len(), 2);
        let mut data: Vec<u8> = (0..256).map(|i| i as u8).collect();
        assert!(card.file_io(first, BLOCK - 128, &mut data, true));
        let mut back = vec![0u8; 256];
        assert!(card.file_io(first, BLOCK - 128, &mut back, false));
        assert_eq!(back, data);
        assert!(matches(b"BASLUS-01042*", b"BASLUS-01042AAAA"));
        assert!(!matches(b"BASLUS-01043*", b"BASLUS-01042AAAA"));
        assert_eq!(card.create(b"X", 14), MC_ERR_BLOCK_FULL);
        for b in 0..BLOCKS {
            let f = &card.data[(b + 1) * FRAME..(b + 2) * FRAME];
            assert_eq!(f[..FRAME - 1].iter().fold(0, |a, x| a ^ x), f[FRAME - 1]);
        }
    }
}
