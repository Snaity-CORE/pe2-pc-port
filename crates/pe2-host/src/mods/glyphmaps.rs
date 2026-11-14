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

/// The hash `uiglyphs::UI_GLYPHS` knows a glyph picture of face `face`
/// (its index in UI_FACES) by: the face, the cell's width and height, and
/// each texel's index (4-bit sheet), rows first.
fn ui_picture(rows: &[[u16; 64]], face: u8, [u, v, w, h]: [u16; 4]) -> Option<u64> {
    let mut texels = vec![face, w as u8, h as u8];
    for y in v..v + h {
        let row = rows.get(usize::from(y))?;
        for x in u..u + w {
            texels.push((row.get(usize::from(x / 4))? >> ((x % 4) * 4) & 15) as u8);
        }
    }
    Some(super::fnv1a(texels))
}

/// The UI font's sheet (stage0/file1, at (960, 256)) and its three faces'
/// glyphs, each cell once (the first face to use it), each read by its
/// picture (uiglyphs.rs): a translation's discs draw syllables and letters
/// over codes, which the code alone would read wrong. Pictures not known
/// are not redrawn.
pub fn ui_font(catalog: &Catalog, exe: &[u8]) -> Option<Map> {
    let sheet = catalog.sheets.iter().find(|s| s.origin == "stage0/file1/0#0" && s.place == (960, 256))?;
    let known: std::collections::HashMap<u64, &str> = super::uiglyphs::UI_GLYPHS.iter().copied().collect();
    let mut glyphs: Vec<Glyph> = Vec::new();
    for (n, (face, address, count, bias)) in UI_FACES.into_iter().enumerate() {
        for r in super::exe_at(exe, address, count * 12)?.as_chunks::<12>().0 {
            let (u, v, w, h, y_offset) =
                (r[0], r[1].wrapping_add(bias), u16::from(r[2]) + 1, u16::from(r[3]) + 1, r[5] as i8);
            let cell = [u16::from(u), u16::from(v), w, h];
            if w <= 1 || glyphs.iter().any(|g| g.cell == cell) {
                continue;
            }
            let Some(text) = ui_picture(&sheet.rows, n as u8, cell).and_then(|p| known.get(&p)).map(|t| t.to_string())
            else {
                continue;
            };
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
    Some(Map { kind: "ui-font", sheets: vec![sheet.hash], depth: 4, palette: None, glyphs })
}

/// The loading captions' glyph pictures (their width, height and texels'
/// indices, `caption_picture`) -> the text each shows: the game's font,
/// and the letters and syllables the Brazilian Portuguese fan
/// translation's discs draw into it (tools/import_strings.py
/// CAPTION_LABELS). Made from the discs; it holds no pictures.
const CAPTION_GLYPHS: &[(u64, &str)] = &[
    (0x23C7CB7FF5E2E536, "A"),
    (0x531A62394E49CC6C, "B"),
    (0x1E02D4A4E86297DA, "C"),
    (0x0BBFC6960DCDDEF6, "D"),
    (0x040B28242F7FD7CA, "E"),
    (0x585DA3DF2222843B, "F"),
    (0xDBD59FA5C60D32E0, "G"),
    (0x2C1D0B8AD615BE7F, "H"),
    (0x064173DB432C0BCD, "I"),
    (0x156DEE5571E1DAB5, "J"),
    (0x48B887FE98E8004D, "K"),
    (0x264A635F55E6ABA8, "L"),
    (0x809D2D6F61C53807, "M"),
    (0x89722784A7871D1C, "N"),
    (0x221C87ACAACF6FDC, "O"),
    (0x7FDFEBD4F78B53A2, "P"),
    (0x90B2F22E0ECEA49E, "Q"),
    (0xE82624D230297136, "R"),
    (0x7218F8FB4913E580, "S"),
    (0xDF97936DDFF70A52, "T"),
    (0x0E0A674CEE6787EA, "U"),
    (0x27259B3C112AE3C7, "V"),
    (0x8B27214A5E477A0F, "W"),
    (0x9CAEB6F95F02F9EA, "X"),
    (0x4E4FA6F6871FBC24, "Y"),
    (0x49C9A551D29B26E9, "Z"),
    (0x613A6C111D4443AB, "a"),
    (0x7D23BFA969968FDA, "b"),
    (0x9490F600306F009F, "c"),
    (0xEB1F06F65B5A6B13, "d"),
    (0x2E5E6EAC36654959, "e"),
    (0xA6A70A26710633AF, "f"),
    (0x7924A46D10543C4B, "g"),
    (0xCC373D9D1B4634B3, "h"),
    (0xA896B1C368A7FCA7, "i"),
    (0x5C5976E5F91BC934, "j"),
    (0x72081D161E574816, "k"),
    (0x2994A9CFAB7DF516, "l"),
    (0xCFE4795F1222E86B, "m"),
    (0xBAC0AC512CBA8542, "n"),
    (0xA9938B0DC067E1AE, "o"),
    (0xB5A6852A904783E7, "p"),
    (0xF7F45A9B696F3155, "q"),
    (0x9BACFB76F205619A, "r"),
    (0xA6162CA6100C65B4, "s"),
    (0x51CDC20CBE71E6B9, "t"),
    (0xE72A8F1BED5D0915, "u"),
    (0x2BBE6186111B6548, "v"),
    (0x7169DDCFE57E2630, "w"),
    (0xBFC4C6A95003468E, "x"),
    (0x23E6745B31A40CEB, "y"),
    (0x123670DE27702884, "z"),
    (0x8F4CDCE7999E8F34, "1"),
    (0xF63023D6CBFD7E82, "2"),
    (0x1776664EA624197A, "3"),
    (0x3E02A2CC82F5A9A2, "4"),
    (0x1B7B6A493FD39AE8, "5"),
    (0x4907AA69FC39CD0F, "6"),
    (0x3DFD875F8E5EC42D, "7"),
    (0xE8225D266F4A6699, "8"),
    (0x397019019A08B4CE, "9"),
    (0xC6018FDDCDA01152, "0"),
    (0x006422D5E27FB203, ":"),
    (0x51631B3AC11F4D0E, ";"),
    (0xA1CFD73BE7F54531, "'"),
    (0xC4098911AB5857D6, ","),
    (0xD70B1045D22FC55D, "."),
    (0xD5C7F02A02F67EFD, "?"),
    (0xD6469CE808948453, "!"),
    // The Brazilian Portuguese translation's.
    (0xFCFC45EB27B9187B, "de"),
    (0x10608AAAB97E96DB, "t"),
    (0x4037C9EB8BB1D032, "m"),
    (0xA318B782B4FF25A5, "b"),
    (0xBEDD3ABD3392BC3D, "P"),
    (0xC809562019897891, "5"),
    (0x6785E6A92D44083C, "ó"),
    (0x17C5C764581DB9E3, ","),
    (0x67A3C45BA6B80E4E, "ã"),
    (0xAC660F8BE88D1552, "7"),
    (0xAE50844985CCE340, "c"),
    (0x1C7FCF631A41963A, "y"),
    (0xDF035048A39DE927, "í"),
    (0x92DD3121434CEBA3, "x"),
];

/// The hash `CAPTION_GLYPHS` knows a glyph picture by: its width, height
/// and each texel's index (4-bit sheet), rows first.
fn caption_picture(rows: &[[u16; 64]], [u, v, w, h]: [u16; 4]) -> Option<u64> {
    let texel = |x: u16, y: u16| {
        let word = rows.get(usize::from(y))?.get(usize::from(x / 4))?;
        Some((word >> ((x % 4) * 4) & 15) as u8)
    };
    let mut texels = vec![w as u8, h as u8];
    for y in v..v + h {
        for x in u..u + w {
            texels.push(texel(x, y)?);
        }
    }
    Some(super::fnv1a(texels))
}

/// The loading screen's captions: their sheet (stage0/file10, at (960, 0))
/// and `gTextCaptionGlyphCells` (main/caption_draw.c), each cell read by
/// its picture (`CAPTION_GLYPHS`): on the game's discs capitals, small
/// letters, the digits 1 to 9 then 0, and : ; ' , . ? !; a translation's
/// discs point some at letters and syllables of their own. The M.I.S.T.
/// briefing (mist_r18.c) has its own copy of the font, 48 rows down its
/// sheet (stage1/1801/file0, at (704, 0)), drawn through the same cells
/// (the translation's copy is a row off: its letters are found there, and
/// are `like` the cell's). Cells not known are not redrawn.
pub fn loading(catalog: &Catalog, exe: &[u8]) -> Vec<Map> {
    const CELLS: usize = 69;
    const COPIES: [(&str, (u16, u16), u16); 2] =
        [("stage0/file10/0#0", (960, 0), 0), ("stage1/1801/file0/1#0", (704, 0), 48)];
    let Some(cells) = super::exe_at(exe, 0x8006_27E0, CELLS * 4) else { return Vec::new() };
    let known: std::collections::HashMap<u64, &str> = CAPTION_GLYPHS.iter().copied().collect();
    COPIES
        .iter()
        .filter_map(|&(origin, place, down)| {
            let sheet = catalog.sheets.iter().find(|s| s.origin == origin && s.place == place)?;
            let mut glyphs: Vec<Glyph> = Vec::new();
            for c in cells.as_chunks::<4>().0 {
                let [u, v, w, h] = c.map(u16::from);
                let cell = [u, v + down, w, h];
                if w == 0 || h == 0 || glyphs.iter().any(|g| g.cell == cell) {
                    continue;
                }
                let found = [0i16, 1, -1].into_iter().find_map(|off| {
                    let at = [u, v.checked_add_signed(off)? + down, w, h];
                    Some((at, *known.get(&caption_picture(&sheet.rows, at)?)?))
                });
                let Some((at, text)) = found else { continue };
                glyphs.push(Glyph { like: (at != cell).then_some(at), ..Glyph::new(cell, text) });
            }
            (!glyphs.is_empty()).then(|| Map {
                kind: "loading",
                sheets: vec![sheet.hash],
                depth: 4,
                palette: None,
                glyphs,
            })
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

/// A map as `write` saves it, which a mod may give.
#[derive(serde::Deserialize)]
struct MapFile {
    kind: String,
    sheets: Vec<String>,
    depth: u32,
    #[serde(default)]
    glyph: Vec<GlyphFile>,
}

#[derive(serde::Deserialize)]
struct GlyphFile {
    cell: [u16; 4],
    text: String,
    /// Where the letter's picture is when not in its cell (a translation's
    /// copy of a font drawn a row off): its ink tells where the letter goes.
    #[serde(default)]
    like: Option<[u16; 4]>,
}

/// The maps mods give, as `glyphs/<sheet>-<4|8>bit.toml` (in the extract's
/// form), of pictures of text their files replace (a translation's own
/// font), so those are redrawn as the game's are: of the sheets `catalog`
/// holds, `loading` ones (the captions' font). A higher mod's map of a
/// sheet wins.
pub fn from_mods(catalog: &Catalog) -> Vec<Map> {
    let mut maps: std::collections::HashMap<u64, Map> = std::collections::HashMap::new();
    for (path, _) in super::mod_files("glyphs", ".toml") {
        let parsed = std::fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|t| toml::from_str::<MapFile>(&t).map_err(|e| e.to_string()));
        let file = match parsed {
            Ok(f) => f,
            Err(e) => {
                eprintln!("pe2: mods: {}: {e}", path.display());
                continue;
            }
        };
        let sheets: Vec<u64> = file.sheets.iter().filter_map(|s| u64::from_str_radix(s, 16).ok()).collect();
        let [sheet] = sheets[..] else { continue };
        if file.kind != "loading" || !catalog.sheets.iter().any(|s| s.hash == sheet) {
            continue;
        }
        let glyphs = file.glyph.into_iter().map(|g| Glyph { like: g.like, ..Glyph::new(g.cell, g.text) }).collect();
        maps.insert(sheet, Map { kind: "loading", sheets, depth: file.depth, palette: None, glyphs });
    }
    maps.into_values().collect()
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

    /// Each picture is known as one text.
    #[test]
    fn picture_tables_name_each_picture_once() {
        for table in [super::super::uiglyphs::UI_GLYPHS, CAPTION_GLYPHS] {
            let keys: std::collections::HashSet<u64> = table.iter().map(|g| g.0).collect();
            assert_eq!(keys.len(), table.len());
        }
    }

    /// A picture's texels are read low nibble first, rows first, after
    /// the face (menus) or straight after the size (captions).
    #[test]
    fn pictures_hash_their_texels() {
        let mut rows = vec![[0u16; 64]; 2];
        rows[0][0] = 0x4321;
        rows[1][0] = 0x8765;
        assert_eq!(ui_picture(&rows, 1, [1, 0, 2, 2]), Some(super::super::fnv1a([1, 2, 2, 2, 3, 6, 7])));
        assert_eq!(caption_picture(&rows, [0, 1, 4, 1]), Some(super::super::fnv1a([4, 1, 5, 6, 7, 8])));
        assert_eq!(caption_picture(&rows, [0, 1, 4, 2]), None);
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
