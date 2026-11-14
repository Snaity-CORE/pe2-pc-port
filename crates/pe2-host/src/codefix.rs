//! What a translation's discs change in the game's code, done in its data.
//!
//! The game's code runs natively, built from the USA release; only the
//! data of the disc's executable is used (main.rs). A translation that
//! repoints a text in the code itself, rather than rewriting the text in
//! place, is therefore not followed. Each fix here recognizes such a change
//! by the instruction the disc's executable holds, and makes the native
//! code show what the translation meant by changing the data instead.

/// A change to the code, and its counterpart in the data.
struct Fix {
    /// Why, for the log.
    what: &'static str,
    /// The instruction's address, and what it is on the translation's
    /// discs.
    code: u32,
    reads: u32,
    /// What to write where, in its place.
    address: u32,
    bytes: &'static [u8],
}

const FIXES: &[Fix] = &[
    // The Brazilian Portuguese fan translation's discs leave the memory
    // card menu's "OK" row blank: _mcMenuOkRow (main/mcmenu.c) draws
    // gMcTextOk ("OK", its only user) 2 bytes further on, at its end.
    Fix {
        what: "the memory card menu's OK row left blank",
        code: 0x8003_6EAC,
        reads: 0x24E7_0A66, // addiu $a3, $a3, 0x0A66
        address: 0x8006_0A64,
        bytes: b"\0",
    },
];

/// The word at `address` of PS-X EXE `exe` (its header's load address
/// tells where the file's code goes).
fn exe_word(exe: &[u8], address: u32) -> Option<u32> {
    let load = u32::from_le_bytes(exe.get(0x18..0x1C)?.try_into().ok()?);
    let at = 0x800 + address.checked_sub(load)? as usize;
    Some(u32::from_le_bytes(exe.get(at..at + 4)?.try_into().ok()?))
}

/// The fixes boot executable `exe` needs.
fn wanted(exe: &[u8]) -> impl Iterator<Item = &'static Fix> + '_ {
    FIXES.iter().filter(move |f| exe_word(exe, f.code) == Some(f.reads))
}

/// Applies the fixes `exe` needs to the PlayStation memory it was loaded
/// into (main.rs, after pc_boot_load).
pub fn apply(exe: &[u8]) {
    for f in wanted(exe) {
        for (k, &b) in f.bytes.iter().enumerate() {
            let physical = (f.address + k as u32) & 0x1F_FFFF;
            // SAFETY: the PlayStation RAM is mapped at 0x80000000
            // (pc_boot_load), and `physical` is inside its 2 MB.
            unsafe { ((0x8000_0000u32 | physical) as usize as *mut u8).write_volatile(b) };
        }
        eprintln!("pe2: game data: {}, as the disc's code does", f.what);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fix applies only where the disc's code reads as the translation's.
    #[test]
    fn fixes_follow_the_code() {
        let mut exe = vec![0u8; 0x800 + 0x2_8000];
        exe[0x18..0x1C].copy_from_slice(&0x8001_0000u32.to_le_bytes());
        let at = 0x800 + (0x8003_6EACu32 - 0x8001_0000) as usize;
        exe[at..at + 4].copy_from_slice(&0x24E7_0A64u32.to_le_bytes());
        assert_eq!(wanted(&exe).count(), 0);
        exe[at..at + 4].copy_from_slice(&0x24E7_0A66u32.to_le_bytes());
        assert_eq!(wanted(&exe).count(), 1);
        assert_eq!(exe_word(&exe[..0x100], 0x8003_6EAC), None);
    }
}
