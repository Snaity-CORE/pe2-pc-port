//! Loading captions from another release's discs: a mod's `captions.toml`
//! (see docs/MODDING.md), the captions of the loading screens (date and
//! place) as that release shows them.
//!
//! A caption is a `TextStream` record of the game's executable (main/text.h:
//! a script of glyph indices into the caption font's cell table, where to
//! draw it and its plate). Another release built again (a European one)
//! has its records and scripts elsewhere and may show them otherwise (the
//! French release draws its own captions, in a secondary record, at the
//! bottom right): their bytes cannot be patched into the game's places. So
//! before a caption is shown (`pc_caption_translate`), the record whose
//! script is one the mod knows is pointed at the mod's script, cell table
//! and plate instead, kept in the host's memory. The pictures of the cells
//! the other release adds (its accented letters) come with the mod's
//! `files/` (the captions' font).
//!
//! ```toml
//! cells = "00000a0a1000090a..."  # the cell table (u, v, width, height bytes), hex
//!
//! [[caption]]
//! original = "121e292d..."       # the game's script, hex, without its end byte
//! text = "37452c1e..."           # the one shown instead
//! x = -34                        # where (draw-environment pixels)
//! y = 54
//! line_height = 15
//! box_width = 190                # its plate
//! box_height = 32
//! ```

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

use super::{enabled, installed};

/// The file of a mod's captions.
const FILE: &str = "captions.toml";

/// The script's end byte (main/text.h `TEXT_STREAM_END`).
const END: u8 = 0xFF;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CaptionFile {
    cells: String,
    caption: Vec<CaptionEntry>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CaptionEntry {
    original: String,
    text: String,
    x: i16,
    y: i16,
    line_height: i16,
    box_width: i16,
    box_height: i16,
}

/// The game's `TextStream` (main/text.h), as laid out in its memory.
#[repr(C)]
pub struct TextStream {
    x: i16,
    y: i16,
    tpage_x: i16,
    tpage_y: i16,
    clut_x: i16,
    clut_y: i16,
    char_delay: i16,
    cursor: i16,
    chars: *const u8,
    glyphs: *const u8,
    line_height: i16,
    delay_reload: i16,
    box_width: i16,
    box_height: i16,
}

/// A caption as the mod shows it: its script (with its end byte) and cell
/// table, leaked for the whole run (records point at them).
struct Shown {
    chars: &'static [u8],
    cells: &'static [u8],
    entry: CaptionEntry,
}

fn hex(text: &str) -> Option<Vec<u8>> {
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    if !text.len().is_multiple_of(2) {
        return None;
    }
    (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).ok()).collect()
}

/// The game's script -> the caption shown instead, from the enabled mods
/// (the highest priority's wins).
fn captions() -> &'static HashMap<Vec<u8>, Shown> {
    static CAPTIONS: OnceLock<HashMap<Vec<u8>, Shown>> = OnceLock::new();
    CAPTIONS.get_or_init(|| {
        let mut out = HashMap::new();
        if !enabled() {
            return out;
        }
        for m in installed().into_iter().filter(|m| m.enabled).rev() {
            let path = m.dir.join(FILE);
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let file = match toml::from_str::<CaptionFile>(&text) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("pe2: mods: {}: {e}", path.display());
                    continue;
                }
            };
            let Some(cells) = hex(&file.cells).filter(|c| !c.is_empty() && c.len() % 4 == 0) else {
                eprintln!("pe2: mods: {}: cells is not a cell table", path.display());
                continue;
            };
            let cells: &'static [u8] = Box::leak(cells.into_boxed_slice());
            for entry in file.caption {
                let (Some(original), Some(mut script)) = (hex(&entry.original), hex(&entry.text)) else {
                    eprintln!("pe2: mods: {}: a caption is not hex", path.display());
                    continue;
                };
                // every glyph a cell of the table (0xFE: a line break)
                if script.iter().any(|&b| b != 0xFE && usize::from(b & 0x7F) >= cells.len() / 4) {
                    eprintln!("pe2: mods: {}: a caption uses a cell the table lacks", path.display());
                    continue;
                }
                script.push(END);
                out.insert(original, Shown { chars: Box::leak(script.into_boxed_slice()), cells, entry });
            }
        }
        if !out.is_empty() {
            eprintln!("pe2: mods: {} loading captions", out.len());
        }
        out
    })
}

/// Before caption `stream` is shown: when its script is one a mod gives
/// another for, the record is pointed at that one (its script, cell table,
/// place and plate). The script then is the mod's, so a record is changed
/// once.
#[unsafe(no_mangle)]
pub extern "C" fn pc_caption_translate(stream: *mut TextStream) {
    if stream.is_null() || captions().is_empty() {
        return;
    }
    // SAFETY: a caption record of the game's, its script ended by END (read
    // up to 256 bytes).
    let s = unsafe { &mut *stream };
    if s.chars.is_null() {
        return;
    }
    let script: Vec<u8> = (0..256).map(|i| unsafe { *s.chars.add(i) }).take_while(|&b| b != END).collect();
    let Some(shown) = captions().get(&script) else { return };
    s.chars = shown.chars.as_ptr();
    s.glyphs = shown.cells.as_ptr();
    s.x = shown.entry.x;
    s.y = shown.entry.y;
    s.line_height = shown.entry.line_height;
    s.box_width = shown.entry.box_width;
    s.box_height = shown.entry.box_height;
    if crate::runtime::trace("text") {
        eprintln!("[text] loading caption {stream:p}: the mod's");
    }
}
