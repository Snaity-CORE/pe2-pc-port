//! The folders the extract files the pictures it saves in, by the
//! decompilation's names for the disc's files and their chunks
//! (res/asset_names.tsv, written by tools/gen_asset_names.py):
//! characters/aya/10200, items/recovery1, rooms/akropolis/square...

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

static NAMES: &str = include_str!("../../res/asset_names.tsv");

/// (file key, chunk) -> folder.
fn table() -> &'static HashMap<(&'static str, usize), &'static str> {
    static TABLE: OnceLock<HashMap<(&str, usize), &str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        NAMES
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let mut parts = l.split('\t');
                Some(((parts.next()?, parts.next()?.parse().ok()?), parts.next()?))
            })
            .collect()
    })
}

/// The folder for what chunk `chunk` of the disc file `key` holds (a
/// chunk past the named ones: its file's); `other` for a file the
/// decompilation does not name.
pub fn folder(key: &str, chunk: usize) -> PathBuf {
    let names = table();
    let found = names.get(&(key, chunk)).or_else(|| names.get(&(key, 0)));
    PathBuf::from(found.copied().unwrap_or("other"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_are_filed_by_the_decompilation_names() {
        assert_eq!(folder("stage0/file10200", 1), PathBuf::from("characters/aya/10200"));
        assert_eq!(folder("stage0/file1", 3), PathBuf::from("ui/sheet"));
        assert_eq!(folder("stage1/1001/file2", 0), PathBuf::from("rooms/akropolis/observatory"));
        // A dialogue atlas, named without a folder: its room's; and a room
        // the decompilation has not named yet.
        assert_eq!(folder("stage1/1001/file0", 1), PathBuf::from("rooms/akropolis/observatory"));
        assert_eq!(folder("stage4/4701/file16", 0), PathBuf::from("rooms/shelter/r47"));
        assert_eq!(folder("stage9/file1", 0), PathBuf::from("other"));
    }
}
