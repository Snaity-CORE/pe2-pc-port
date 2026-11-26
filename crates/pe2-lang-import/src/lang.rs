//! The releases this makes mods of, and what each mod has that is not on
//! the discs: for the European ones, the speakers' names (pictures there,
//! read by eye), the loading captions' places the release leaves in
//! English (translated for this program) and PE2PC's texts it has no word
//! for (its "Lv."); for the Japanese one, what each
//! dialogue glyph is (japanese.rs).

/// The movies whose pictures have text in the European releases (the title
/// menu's, the prologue's), by where the discs list them, and the name
/// PE2PC knows the game's by (the hash of its first frame, and its length
/// when others begin with the same frame).
const PAL_MOVIES: &[(&str, &str)] =
    &[("STAGE0.HED entry 0", "0d17397b6feab122"), ("stage1/2001 entry 0", "39d9715252254830-1397")];

pub enum Kind {
    /// A European release: everything (dialogue, menus, items, loading
    /// captions, pictures), with its speakers' names, the places of its
    /// loading captions and PE2PC's own texts in its language (its
    /// strings/pc/, None when English serves).
    Pal { speakers: &'static str, places: &'static str, own: Option<&'static str> },
    /// The Japanese release: its dialogue (japanese.rs).
    Japanese,
}

pub struct Release {
    /// Its executable's name on disc 1.
    pub exe: &'static str,
    /// The mod's folder in PE2PC's mods/.
    pub folder: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub kind: Kind,
    /// Its movies with text: [`PAL_MOVIES`]' form.
    pub movies: &'static [(&'static str, &'static str)],
}

pub const RELEASES: [Release; 5] = [
    Release {
        exe: "SLES_025.59",
        folder: "francais",
        name: "Français (version PAL)",
        description: "Dialogues, noms, menus, objets et images de la version française (SLES-02559), importés des disques PAL.",
        kind: Kind::Pal {
            speakers: include_str!("../res/fr/speakers.toml"),
            places: include_str!("../res/fr/places.toml"),
            own: Some(include_str!("../res/fr/pc.toml")),
        },
        movies: PAL_MOVIES,
    },
    Release {
        exe: "SLES_025.60",
        folder: "deutsch",
        name: "Deutsch (PAL-Version)",
        description: "Dialoge, Namen, Menüs, Gegenstände und Bilder der deutschen Version (SLES-02560), von den PAL-Discs importiert.",
        kind: Kind::Pal {
            speakers: include_str!("../res/de/speakers.toml"),
            places: include_str!("../res/de/places.toml"),
            own: Some(include_str!("../res/de/pc.toml")),
        },
        movies: PAL_MOVIES,
    },
    Release {
        exe: "SLES_025.61",
        folder: "espanol",
        name: "Español (versión PAL)",
        description: "Diálogos, nombres, menús, objetos e imágenes de la versión española (SLES-02561), importados de los discos PAL.",
        kind: Kind::Pal {
            speakers: include_str!("../res/es/speakers.toml"),
            places: include_str!("../res/es/places.toml"),
            own: Some(include_str!("../res/es/pc.toml")),
        },
        movies: PAL_MOVIES,
    },
    Release {
        exe: "SLES_025.62",
        folder: "italiano",
        name: "Italiano (versione PAL)",
        description: "Dialoghi, nomi, menu, oggetti e immagini della versione italiana (SLES-02562), importati dai dischi PAL.",
        kind: Kind::Pal {
            speakers: include_str!("../res/it/speakers.toml"),
            places: include_str!("../res/it/places.toml"),
            // "Lv.", as the Italian release's menus have it
            own: None,
        },
        movies: PAL_MOVIES,
    },
    Release {
        exe: "SLPS_024.80",
        folder: "japonais",
        name: "日本語（日本版）",
        description: "日本版（SLPS-02480/81）のディスクから取り込んだ会話。メニューは英語のままです。",
        kind: Kind::Japanese,
        // and Dryfield's arrival, its name in katakana
        movies: &[
            ("STAGE0.HED entry 0", "0d17397b6feab122"),
            ("stage1/2001 entry 0", "39d9715252254830-1397"),
            ("stage2/101 entry 0", "39d9715252254830-255"),
        ],
    },
];

/// The release whose disc 1 has executable `exe`.
pub fn release(exe: &str) -> Option<&'static Release> {
    RELEASES.iter().find(|r| r.exe.eq_ignore_ascii_case(exe))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_reads() {
        for r in &RELEASES {
            if let Kind::Pal { speakers, places, own } = r.kind {
                let speakers: toml::Table = speakers.parse().expect("speakers");
                let places: toml::Table = places.parse().expect("places");
                assert!(!speakers.is_empty() && !places.is_empty(), "{}", r.folder);
                if let Some(own) = own {
                    own.parse::<toml::Table>().expect("own");
                }
            }
        }
    }
}
