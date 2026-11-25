//! The window's languages: its English texts, and res/ui/<code>.toml's
//! translations of them (English text -> its translation; a text not there
//! stays in English).

use std::collections::HashMap;
use std::sync::OnceLock;

/// Code, name, translations.
pub const LANGS: [(&str, &str, &str); 5] = [
    ("en", "English", ""),
    ("fr", "Français", include_str!("../res/ui/fr.toml")),
    ("de", "Deutsch", include_str!("../res/ui/de.toml")),
    ("es", "Español", include_str!("../res/ui/es.toml")),
    ("it", "Italiano", include_str!("../res/ui/it.toml")),
];

fn translations() -> &'static [HashMap<String, String>] {
    static ALL: OnceLock<Vec<HashMap<String, String>>> = OnceLock::new();
    ALL.get_or_init(|| LANGS.iter().map(|l| toml::from_str(l.2).unwrap_or_default()).collect())
}

/// One of LANGS, by its index.
#[derive(Clone, Copy, PartialEq)]
pub struct Lang(pub usize);

impl Lang {
    /// The system's language, if it is one of LANGS; else English.
    pub fn system() -> Lang {
        let locale = sys_locale::get_locale().unwrap_or_default().to_lowercase();
        Lang(LANGS.iter().position(|l| locale.starts_with(l.0)).unwrap_or(0))
    }

    pub fn t(self, en: &'static str) -> &'static str {
        translations()[self.0].get(en).map_or(en, String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each language translates every text of the window, and only those.
    #[test]
    fn translations_are_the_windows_texts() {
        let source = include_str!("gui.rs");
        for (code, _, text) in &LANGS[1..] {
            let table: HashMap<String, String> = toml::from_str(text).expect(code);
            for en in table.keys() {
                assert!(source.contains(&format!("{en:?}")), "{code}: {en:?} is not one of the window's texts");
            }
            let fr: HashMap<String, String> = toml::from_str(LANGS[1].2).expect("fr");
            assert_eq!(table.len(), fr.len(), "{code}: not the same texts as fr");
            assert!(fr.keys().all(|k| table.contains_key(k)), "{code}: not the same texts as fr");
        }
    }
}
