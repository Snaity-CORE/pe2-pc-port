//! Maps of the game's pictures of text, for font makers (docs/MODDING.md):
//! for each, its sheets (as in `dumps/textures-indexed`) and each glyph's
//! cell and the text it shows, saved by the extract as
//! `dumps/textures-glyphs/<sheet>-<4|8>bit.toml`. Tools redraw the text of
//! those sheets from them (tools/hd_fonts.py); nothing of the pictures is in
//! a map.
//!
//! Five kinds: `dialogue` (each CAP file's glyph page, dialogue.rs),
//! `ui-font` (the menus' font, its three faces' metrics in the executable),
//! `loading` (the loading screen's captions, and the M.I.S.T. briefing's
//! copy of their font), `title` (the title screen's 8-bit page: its rows of
//! text, two sheets side by side) and `disc-prompt` (the disc change's
//! three lines).

use std::path::Path;

use super::sheets::Catalog;

/// A picture of text: its kind, sheets, texel depth (4 or 8), where its
/// palette is (under the dumps, when a tool needs it) and its glyphs.
pub struct Map {
    pub kind: &'static str,
    pub sheets: Vec<u64>,
    pub depth: u32,
    pub palette: Option<&'static str>,
    pub glyphs: Vec<Glyph>,
}

/// A glyph: its cell (u, v, width, height, in texels of the sheets side by
/// side), the text it shows (empty: not known; `{name}` an icon or logo),
/// for the UI font its face and the cell row its letters stand on, and for
/// a letter the sheet leaves blank (menuglyphs.rs) the cell of the letter
/// it is made from, whose ink tells where its own goes.
pub struct Glyph {
    pub cell: [u16; 4],
    pub text: String,
    pub face: Option<&'static str>,
    pub baseline: Option<u16>,
    pub like: Option<[u16; 4]>,
}

impl Glyph {
    fn new(cell: [u16; 4], text: impl Into<String>) -> Glyph {
        Glyph { cell, text: text.into(), face: None, baseline: None, like: None }
    }
}

/// The dialogue's glyph pages (dialogue.rs `glyph_maps`).
pub fn dialogue(sources: &super::dialogue::Sources) -> Vec<Map> {
    super::dialogue::glyph_maps(sources)
        .into_iter()
        .map(|(sheet, cells)| Map {
            kind: "dialogue",
            sheets: vec![sheet],
            depth: 4,
            palette: None,
            glyphs: cells.into_iter().map(|(c, t)| Glyph::new(c.map(u16::from), t)).collect(),
        })
        .collect()
}

/// The sheet of `catalog` loaded from `origin` to `place`.
fn sheet_at(catalog: &Catalog, origin: &str, place: (u16, u16)) -> Option<u64> {
    catalog.sheets.iter().find(|s| s.origin == origin && s.place == place).map(|s| s.hash)
}

/// The UI font's faces: their `_FontGlyph` tables (main/textdraw.c), how
/// many records (from ' ') and the V bias each adds.
const UI_FACES: [(&str, u32, usize, u8); 3] =
    [("medium", 0x8005_EFB0, 224, 38), ("large", 0x8005_FA30, 224, 128), ("small", 0x8006_04B0, 91, 0)];

/// What the UI font's glyph for `code` shows in `face` (its record's V
/// given): Latin-1 and the pad's buttons; the small face's records past
/// row 34 point into the medium face's cells.
fn ui_text(face: &str, code: u8, v: u8) -> Option<String> {
    let icon = |name: &str| Some(format!("{{{name}}}"));
    if face == "small" {
        if v >= 35 {
            return None;
        }
        return match code {
            0x24 => icon("circle"),
            0x25 => icon("square"),
            0x26 => icon("triangle"),
            0x2A => icon("cross"),
            c if c.is_ascii_uppercase() || c.is_ascii_digit() || b".,()!?".contains(&c) => {
                Some(char::from(c).to_string())
            }
            _ => None,
        };
    }
    match code {
        0x7B => icon("circle"),
        0x7C => icon("triangle"),
        0x7D => icon("cross"),
        0x7E => icon("square"),
        0x8C => Some("Œ".into()),
        0x9C => Some("œ".into()),
        0x97 => Some("—".into()),
        0xA1 | 0xAB | 0xB0 | 0xBB | 0xBF | 0xC0..=0xFF => Some(char::from(code).to_string()),
        c if c.is_ascii_alphanumeric() || b"!\"%&'()+,-./:?#".contains(&c) => Some(char::from(c).to_string()),
        _ => None,
    }
}

/// The UI font's sheet (stage0/file1, at (960, 256)) and its three faces'
/// glyphs, each cell once (the first face to use it).
pub fn ui_font(catalog: &Catalog, exe: &[u8]) -> Option<Map> {
    let sheet = sheet_at(catalog, "stage0/file1/0#0", (960, 256))?;
    let mut glyphs: Vec<Glyph> = Vec::new();
    for (face, address, count, bias) in UI_FACES {
        for (i, r) in super::exe_at(exe, address, count * 12)?.as_chunks::<12>().0.iter().enumerate() {
            let (u, v, w, h, y_offset) =
                (r[0], r[1].wrapping_add(bias), u16::from(r[2]) + 1, u16::from(r[3]) + 1, r[5] as i8);
            let cell = [u16::from(u), u16::from(v), w, h];
            let Some(text) = ui_text(face, 0x20 + i as u8, v) else { continue };
            if w <= 1 || glyphs.iter().any(|g| g.cell == cell) {
                continue;
            }
            // The last row is drawn y_offset below the pen's baseline.
            let baseline = (h as i16 - i16::from(y_offset)).clamp(0, h as i16) as u16;
            glyphs.push(Glyph { cell, text, face: Some(face), baseline: Some(baseline), like: None });
        }
    }
    // The letters translations add (menuglyphs.rs), where they go.
    for a in super::menuglyphs::cells(exe) {
        let text = char::from(a.code).to_string();
        let face = Some(UI_FACES[a.face].0);
        glyphs.push(Glyph { cell: a.cell, text, face, baseline: Some(a.baseline), like: Some(a.like) });
    }
    Some(Map { kind: "ui-font", sheets: vec![sheet], depth: 4, palette: None, glyphs })
}

/// The loading screen's captions: their sheet (stage0/file10, at (960, 0))
/// and `gTextCaptionGlyphCells` (main/caption_draw.c): capitals, small
/// letters, the digits 1 to 9 then 0, and : ; ' , . ? ! The M.I.S.T.
/// briefing (mist_r18.c) has its own copy of the font, 48 rows down its
/// sheet (stage1/1801/file0, at (704, 0)), drawn through the same cells.
pub fn loading(catalog: &Catalog, exe: &[u8]) -> Vec<Map> {
    const TEXT: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890:;',.?!";
    const COPIES: [(&str, (u16, u16), u16); 2] =
        [("stage0/file10/0#0", (960, 0), 0), ("stage1/1801/file0/1#0", (704, 0), 48)];
    let Some(cells) = super::exe_at(exe, 0x8006_27E0, TEXT.len() * 4) else { return Vec::new() };
    COPIES
        .iter()
        .filter_map(|&(origin, place, down)| {
            let sheet = sheet_at(catalog, origin, place)?;
            let glyphs = cells
                .as_chunks::<4>()
                .0
                .iter()
                .zip(TEXT.chars())
                .map(|(c, t)| {
                    let [u, v, w, h] = c.map(u16::from);
                    Glyph::new([u, v + down, w, h], t)
                })
                .collect();
            Some(Map { kind: "loading", sheets: vec![sheet], depth: 4, palette: None, glyphs })
        })
        .collect()
}

/// The disc change's lines (`_loadUiDrawDiskSwapMessage`, main/loadui.c):
/// its sheet (stage0/file20160, the same on both discs, at (960, 0)), a
/// line each 16 rows.
pub fn disc_prompt(catalog: &Catalog) -> Option<Map> {
    const LINES: [(&str, u16); 3] = [("Change to DISC 1", 159), ("Change to DISC 2", 159), ("Wrong DISC", 104)];
    let sheet = sheet_at(catalog, "stage0/file20160/0#0", (960, 0))?;
    let glyphs = LINES.iter().zip(0..).map(|(&(text, w), row)| Glyph::new([0, row * 16, w, 16], text)).collect();
    Some(Map { kind: "disc-prompt", sheets: vec![sheet], depth: 4, palette: None, glyphs })
}

/// The title screen's page (stage0/file2, 8-bit at (768, 256), two sheets
/// side by side) and its rows of text (`Title_DrawSpriteRow`, 256 x 16
/// each); its palette is that file's.
pub fn title(catalog: &Catalog) -> Option<Map> {
    let left = sheet_at(catalog, "stage0/file2/0#0", (768, 256))?;
    let right = sheet_at(catalog, "stage0/file2/0#1", (832, 256))?;
    let glyphs = vec![
        Glyph::new([0, 0, 256, 16], "PRESS START BUTTON"),
        Glyph::new([0, 16, 126, 16], "© 1999 , 2000"),
        Glyph::new([126, 16, 130, 16], "{SQUARE}"),
        Glyph::new([0, 48, 256, 16], "New Game"),
        Glyph::new([0, 64, 256, 16], "Continue"),
        Glyph::new([0, 80, 256, 16], "Option"),
    ];
    Some(Map {
        kind: "title",
        sheets: vec![left, right],
        depth: 8,
        palette: Some("files/stage0/file2/1.pe2clut"),
        glyphs,
    })
}

/// Saves `maps` in `dir`, each as its first sheet's `<sheet>-<4|8>bit.toml`.
/// Answers how many.
pub fn write(maps: &[Map], dir: &Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let quoted = |s: &str| format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""));
    for m in maps {
        let sheets: Vec<String> = m.sheets.iter().map(|s| format!("\"{s:016x}\"")).collect();
        let mut toml = format!(
            "# Glyphs of a picture of text (see docs/MODDING.md): cell = [u, v, width,\n\
             # height] in texels of its sheets side by side; empty text: not known;\n\
             # {{name}}: an icon or logo.\n\
             kind = \"{}\"\nsheets = [{}]\ndepth = {}\n",
            m.kind,
            sheets.join(", "),
            m.depth
        );
        if let Some(p) = m.palette {
            toml.push_str(&format!("palette = {}\n", quoted(p)));
        }
        for g in &m.glyphs {
            let [u, v, w, h] = g.cell;
            toml.push_str(&format!("\n[[glyph]]\ncell = [{u}, {v}, {w}, {h}]\ntext = {}\n", quoted(&g.text)));
            if let Some(f) = g.face {
                toml.push_str(&format!("face = \"{f}\"\n"));
            }
            if let Some(b) = g.baseline {
                toml.push_str(&format!("baseline = {b}\n"));
            }
            if let Some([u, v, w, h]) = g.like {
                toml.push_str(&format!("like = [{u}, {v}, {w}, {h}]\n"));
            }
        }
        std::fs::write(dir.join(format!("{:016x}-{}bit.toml", m.sheets[0], m.depth)), toml)?;
    }
    Ok(maps.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The UI font's codes: Latin-1, the buttons, and the small face's
    /// few characters where its own cells are.
    #[test]
    fn ui_codes_show_their_characters() {
        assert_eq!(ui_text("medium", b'A', 40).as_deref(), Some("A"));
        assert_eq!(ui_text("large", 0xE7, 200).as_deref(), Some("ç"));
        assert_eq!(ui_text("medium", 0x7C, 76).as_deref(), Some("{triangle}"));
        assert_eq!(ui_text("small", 0x26, 23).as_deref(), Some("{triangle}"));
        assert_eq!(ui_text("small", b'+', 40), None);
        assert_eq!(ui_text("medium", b'$', 134), None);
    }

    /// A map as TOML: its kind, sheets, and each glyph's cell and text.
    #[test]
    fn maps_are_written_as_toml() {
        let dir = std::env::temp_dir().join(format!("pe2-glyphmaps-{}", std::process::id()));
        let mut g = Glyph::new([1, 2, 3, 4], "\"");
        g.face = Some("small");
        g.baseline = Some(3);
        let map = Map { kind: "ui-font", sheets: vec![0xAB], depth: 4, palette: None, glyphs: vec![g] };
        assert_eq!(write(&[map], &dir).unwrap(), 1);
        let text = std::fs::read_to_string(dir.join("00000000000000ab-4bit.toml")).unwrap();
        let parsed: toml::Value = toml::from_str(&text).unwrap();
        assert_eq!(parsed["kind"].as_str(), Some("ui-font"));
        assert_eq!(parsed["glyph"][0]["text"].as_str(), Some("\""));
        assert_eq!(parsed["glyph"][0]["baseline"].as_integer(), Some(3));
        let _ = std::fs::remove_dir_all(dir);
    }
}
