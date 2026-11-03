//! Cheats and memory patches (docs/MODDING.md): a mod's `patches.toml`,
//! GameShark codes and plain writes, applied to the game's memory at each
//! vertical blank.

use std::sync::OnceLock;

use serde::Deserialize;

use super::{enabled, installed};

/// A mod's `patches.toml`: GameShark codes, and plain writes.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct PatchFile {
    cheat: Vec<CheatEntry>,
    patch: Vec<PatchEntry>,
}

/// `[[cheat]]`: GameShark (Action Replay) codes, "AAAAAAAA VVVV" each.
#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct CheatEntry {
    name: Option<String>,
    enabled: Option<bool>,
    codes: Vec<String>,
}

/// `[[patch]]`: a value written to an address each frame.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct PatchEntry {
    name: Option<String>,
    enabled: Option<bool>,
    address: u32,
    /// 1, 2 or 4 bytes (default 4).
    size: Option<u32>,
    value: u32,
}

/// A comparison of a GameShark condition code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Test {
    Equal,
    Different,
    Less,
    Greater,
}

/// One code, as it runs each frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Code {
    /// Write `size` bytes.
    Write { address: u32, size: u32, value: u32 },
    /// Add (or, negative, subtract) to `size` bytes.
    Add { address: u32, size: u32, delta: i32 },
    /// Run the next code only if `size` bytes at `address` pass `test`.
    If { address: u32, size: u32, test: Test, value: u32 },
    /// The next code (a write) `count` times, its address advanced by
    /// `step` bytes and its value by `increment` each time.
    Repeat { count: u32, step: u32, increment: u32 },
}

/// Parses one GameShark line ("80012345 0063"), or says why it cannot.
fn parse_code(line: &str) -> Result<Code, String> {
    let shape = || format!("{line:?}: not \"AAAAAAAA VVVV\"");
    let mut parts = line.split_whitespace();
    let (Some(a), Some(v), None) = (parts.next(), parts.next(), parts.next()) else { return Err(shape()) };
    if a.len() != 8 || v.len() != 4 {
        return Err(shape());
    }
    let word = u32::from_str_radix(a, 16).map_err(|_| shape())?;
    let value = u32::from_str_radix(v, 16).map_err(|_| shape())?;
    let address = 0x8000_0000 | (word & 0x00FF_FFFF);
    let condition = |size, test| Code::If { address, size, test, value: if size == 1 { value & 0xFF } else { value } };
    Ok(match word >> 24 {
        0x80 => Code::Write { address, size: 2, value },
        0x30 => Code::Write { address, size: 1, value: value & 0xFF },
        0x10 => Code::Add { address, size: 2, delta: value as i32 },
        0x11 => Code::Add { address, size: 2, delta: -(value as i32) },
        0x20 => Code::Add { address, size: 1, delta: (value & 0xFF) as i32 },
        0x21 => Code::Add { address, size: 1, delta: -((value & 0xFF) as i32) },
        0xD0 => condition(2, Test::Equal),
        0xD1 => condition(2, Test::Different),
        0xD2 => condition(2, Test::Less),
        0xD3 => condition(2, Test::Greater),
        0xE0 => condition(1, Test::Equal),
        0xE1 => condition(1, Test::Different),
        0xE2 => condition(1, Test::Less),
        0xE3 => condition(1, Test::Greater),
        0x50 => Code::Repeat { count: (word >> 8) & 0xFF, step: word & 0xFF, increment: value },
        kind => return Err(format!("{line:?}: code type {kind:02X} is not supported")),
    })
}

/// A cheat or patch ready to run: its codes in order.
struct Patch {
    codes: Vec<Code>,
}

/// The patches of the mods in use, lowest priority first (so the highest
/// writes last and wins).
fn patches() -> &'static [Patch] {
    static PATCHES: OnceLock<Vec<Patch>> = OnceLock::new();
    PATCHES.get_or_init(|| {
        let mut out = Vec::new();
        if !enabled() {
            return out;
        }
        for m in installed().into_iter().filter(|m| m.enabled).rev() {
            let path = m.dir.join("patches.toml");
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let file: PatchFile = match toml::from_str(&text) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("pe2: mods: {}: {err}", path.display());
                    continue;
                }
            };
            for c in file.cheat.iter().filter(|c| c.enabled.unwrap_or(true)) {
                let name = c.name.as_deref().unwrap_or("cheat");
                let codes: Result<Vec<Code>, String> = c.codes.iter().map(|l| parse_code(l)).collect();
                match codes {
                    Ok(codes) => out.push(Patch { codes }),
                    Err(err) => eprintln!("pe2: mods: {}: {name}: {err}; skipped", path.display()),
                }
            }
            for p in file.patch.iter().filter(|p| p.enabled.unwrap_or(true)) {
                let size = p.size.unwrap_or(4);
                if ![1, 2, 4].contains(&size) {
                    let name = p.name.as_deref().unwrap_or("patch");
                    eprintln!("pe2: mods: {}: {name}: size must be 1, 2 or 4; skipped", path.display());
                    continue;
                }
                out.push(Patch { codes: vec![Code::Write { address: p.address, size, value: p.value }] });
            }
        }
        if !out.is_empty() {
            eprintln!("pe2: mods: {} cheats and patches", out.len());
        }
        out
    })
}

/// Where PlayStation address `address` (KUSEG, KSEG0 or KSEG1) is in the
/// RAM mapping, if it is in RAM.
fn ram(address: u32) -> Option<*mut u8> {
    let physical = address & 0x1FFF_FFFF;
    (physical < 0x0020_0000).then_some((0x8000_0000u32 | physical) as usize as *mut u8)
}

fn read(address: u32, size: u32) -> Option<u32> {
    let p = ram(address)?;
    // SAFETY: inside the PlayStation RAM mapping (see `ram`).
    Some(unsafe {
        match size {
            1 => u32::from(p.read_volatile()),
            2 => u32::from(p.cast::<u16>().read_unaligned()),
            _ => p.cast::<u32>().read_unaligned(),
        }
    })
}

fn write(address: u32, size: u32, value: u32) {
    let Some(p) = ram(address) else { return };
    // SAFETY: inside the PlayStation RAM mapping (see `ram`).
    unsafe {
        match size {
            1 => p.write_volatile(value as u8),
            2 => p.cast::<u16>().write_unaligned(value as u16),
            _ => p.cast::<u32>().write_unaligned(value),
        }
    }
}

/// Runs `codes` once, against `read` and `write`.
fn run_codes(codes: &[Code], read: impl Fn(u32, u32) -> Option<u32>, mut write: impl FnMut(u32, u32, u32)) {
    let mut i = 0;
    while i < codes.len() {
        match codes[i] {
            Code::Write { address, size, value } => write(address, size, value),
            Code::Add { address, size, delta } => {
                if let Some(v) = read(address, size) {
                    write(address, size, v.wrapping_add(delta as u32));
                }
            }
            Code::If { address, size, test, value } => {
                let pass = read(address, size).is_some_and(|v| match test {
                    Test::Equal => v == value,
                    Test::Different => v != value,
                    Test::Less => v < value,
                    Test::Greater => v > value,
                });
                if !pass {
                    i += 1;
                }
            }
            Code::Repeat { count, step, increment } => {
                if let Some(Code::Write { address, size, value }) = codes.get(i + 1).copied() {
                    for k in 0..count {
                        write(address + k * step, size, value.wrapping_add(k * increment));
                    }
                    i += 1;
                }
            }
        }
        i += 1;
    }
}

/// Applies the mods' cheats and patches: each vertical blank, before the
/// game's own handler, with the game stopped, as a cheat cartridge does.
pub fn apply_patches() {
    for p in patches() {
        run_codes(&p.codes, read, write);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GameShark lines parse to the codes they stand for.
    #[test]
    fn gameshark_codes() {
        assert_eq!(parse_code("80073BA0 0032"), Ok(Code::Write { address: 0x80073BA0, size: 2, value: 0x32 }));
        assert_eq!(parse_code("30012345 00FF"), Ok(Code::Write { address: 0x80012345, size: 1, value: 0xFF }));
        assert_eq!(parse_code("11012344 0002"), Ok(Code::Add { address: 0x80012344, size: 2, delta: -2 }));
        assert_eq!(
            parse_code("D0012344 1234"),
            Ok(Code::If { address: 0x80012344, size: 2, test: Test::Equal, value: 0x1234 })
        );
        assert_eq!(parse_code("50000402 0001"), Ok(Code::Repeat { count: 4, step: 2, increment: 1 }));
        assert!(parse_code("C0012344 0001").is_err());
        assert!(parse_code("80012344").is_err());
    }

    /// Conditions guard the next code; repeaters repeat it.
    #[test]
    fn codes_run() {
        let read = |a: u32, _| Some(if a == 0x10 { 5 } else { 0 });
        let codes = [
            Code::If { address: 0x10, size: 2, test: Test::Equal, value: 5 },
            Code::Write { address: 0x20, size: 2, value: 1 },
            Code::If { address: 0x10, size: 2, test: Test::Equal, value: 6 },
            Code::Write { address: 0x30, size: 2, value: 1 },
            Code::Repeat { count: 3, step: 2, increment: 10 },
            Code::Write { address: 0x40, size: 2, value: 1 },
        ];
        let mut writes = Vec::new();
        run_codes(&codes, read, |a, _, v| writes.push((a, v)));
        assert_eq!(writes, [(0x20, 1), (0x40, 1), (0x42, 11), (0x44, 21)]);
    }
}
