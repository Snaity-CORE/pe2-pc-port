//! Installing a mod from a .zip (the launcher's Mods screen): its files are
//! extracted once into a folder of the mods folder, where the game reads
//! them as any other mod's.
//!
//! A zip may hold the mod's files at its root (mod.toml, backgrounds/...)
//! or in one folder; the mod's folder takes that folder's name, else the
//! zip's. A mod already installed in that folder is replaced, keeping
//! whether it is on and its place in the list (its mod.toml's `enabled`
//! and `priority`).

use std::io::Read;
use std::path::{Path, PathBuf};

/// Extracts the mod in the zip at `zip` into the mods folder; returns the
/// folder it went into.
pub fn install_zip(zip: &Path) -> Result<PathBuf, String> {
    install_zip_into(zip, &super::mods_dir())
}

fn install_zip_into(zip: &Path, mods: &Path) -> Result<PathBuf, String> {
    let fail = |e: &dyn std::fmt::Display| format!("{}: {e}", zip.display());
    let file = std::fs::File::open(zip).map_err(|e| fail(&e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| fail(&e))?;
    // Each file's path inside the zip; one leading folder they all share
    // is left out.
    let mut names = Vec::new();
    for k in 0..archive.len() {
        let entry = archive.by_index(k).map_err(|e| fail(&e))?;
        let Some(path) = entry.enclosed_name() else {
            let name = entry.name().map(|n| n.into_owned()).unwrap_or_default();
            return Err(fail(&format!("{name}: a path outside the zip")));
        };
        if !entry.is_dir() {
            names.push((k, path));
        }
    }
    if names.is_empty() {
        return Err(fail(&"no files"));
    }
    let root = shared_folder(names.iter().map(|(_, p)| p.as_path()));
    let name = match &root {
        Some(folder) => folder.to_string_lossy().into_owned(),
        None => zip.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default(),
    };
    let name = folder_name(&name).ok_or_else(|| fail(&"no name for the mod's folder"))?;

    let target = mods.join(&name);
    // Extracted beside it first, so a zip that fails leaves the mod
    // installed as it was.
    let staging = mods.join(format!(".installing-{name}"));
    let _ = std::fs::remove_dir_all(&staging);
    let extracted = (|| -> Result<(), String> {
        for (k, path) in &names {
            let relative = match &root {
                Some(folder) => path.strip_prefix(folder).unwrap_or(path),
                None => path,
            };
            let out = staging.join(relative);
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent).map_err(|e| fail(&e))?;
            }
            let mut entry = archive.by_index(*k).map_err(|e| fail(&e))?;
            let mut data = Vec::with_capacity(entry.size() as usize);
            entry.read_to_end(&mut data).map_err(|e| fail(&format!("{}: {e}", path.display())))?;
            std::fs::write(&out, data).map_err(|e| fail(&format!("{}: {e}", out.display())))?;
        }
        keep_settings(&target.join("mod.toml"), &staging.join("mod.toml"))
    })();
    if let Err(e) = extracted {
        let _ = std::fs::remove_dir_all(&staging);
        return Err(e);
    }
    if target.exists() {
        std::fs::remove_dir_all(&target).map_err(|e| format!("{}: {e}", target.display()))?;
    }
    std::fs::rename(&staging, &target).map_err(|e| format!("{}: {e}", target.display()))?;
    Ok(target)
}

/// The one folder every path starts with, when they are all inside it.
fn shared_folder<'a>(mut paths: impl Iterator<Item = &'a Path>) -> Option<PathBuf> {
    let first = paths.next()?;
    let mut parts = first.components();
    let folder = PathBuf::from(parts.next()?.as_os_str());
    // A file at the root is no folder.
    parts.next()?;
    paths.all(|p| p.starts_with(&folder) && p != folder).then_some(folder)
}

/// `name` as a folder name: without characters Windows does not take in
/// one, nor leading dots (the mods folder's own, `.installing-`).
fn folder_name(name: &str) -> Option<String> {
    let clean: String =
        name.chars().map(|c| if c.is_control() || "<>:\"/\\|?*".contains(c) { '_' } else { c }).collect();
    let clean = clean.trim().trim_start_matches('.').trim_end_matches(['.', ' ']).to_string();
    (!clean.is_empty()).then_some(clean)
}

/// Into the new mod.toml at `new`, `enabled` and `priority` from the mod
/// installed before (`old`), when there was one.
fn keep_settings(old: &Path, new: &Path) -> Result<(), String> {
    let Some(before) = std::fs::read_to_string(old).ok().and_then(|t| t.parse::<toml_edit::DocumentMut>().ok()) else {
        return Ok(());
    };
    let mut doc =
        std::fs::read_to_string(new).ok().and_then(|t| t.parse::<toml_edit::DocumentMut>().ok()).unwrap_or_default();
    for key in ["enabled", "priority"] {
        if let Some(v) = before.get(key) {
            doc[key] = v.clone();
        }
    }
    std::fs::write(new, doc.to_string()).map_err(|e| format!("{}: {e}", new.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shared_folder_is_left_out() {
        let paths = |list: &[&str]| list.iter().map(PathBuf::from).collect::<Vec<_>>();
        let shared = |list: &[&str]| shared_folder(paths(list).iter().map(PathBuf::as_path));
        assert_eq!(shared(&["ptbr/mod.toml", "ptbr/backgrounds/a.png"]), Some(PathBuf::from("ptbr")));
        assert_eq!(shared(&["mod.toml", "backgrounds/a.png"]), None);
        assert_eq!(shared(&["a/mod.toml", "b/x.png"]), None);
        assert_eq!(shared(&["mod.toml"]), None);
    }

    /// A zip's mod goes into its folder; installed again, it replaces it,
    /// keeping whether it is on and its place.
    #[test]
    fn zips_install_and_update() {
        use std::io::Write;
        let dir = std::env::temp_dir().join(format!("pe2-install-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let write_zip = |files: &[(&str, &str)]| {
            let path = dir.join("My Mod.zip");
            let mut zip = zip::ZipWriter::new(std::fs::File::create(&path).unwrap());
            for (name, text) in files {
                zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
                zip.write_all(text.as_bytes()).unwrap();
            }
            zip.finish().unwrap();
            path
        };
        let mods = dir.join("mods");
        let zip = write_zip(&[
            (
                "ptbr/mod.toml", "name = \"PT\"
",
            ),
            ("ptbr/backgrounds/rooms/a.png", "1"),
        ]);
        let target = install_zip_into(&zip, &mods).unwrap();
        assert_eq!(target, mods.join("ptbr"));
        assert_eq!(std::fs::read_to_string(target.join("backgrounds/rooms/a.png")).unwrap(), "1");
        std::fs::write(
            target.join("mod.toml"),
            "name = \"PT\"
enabled = false
priority = 3
",
        )
        .unwrap();
        let zip = write_zip(&[
            (
                "ptbr/mod.toml",
                "name = \"PT\"
version = \"2\"
",
            ),
            ("ptbr/b.png", "2"),
        ]);
        install_zip_into(&zip, &mods).unwrap();
        let toml = std::fs::read_to_string(target.join("mod.toml")).unwrap();
        let files = super::super::files_under(&target);
        let entries: Vec<_> = std::fs::read_dir(&mods).unwrap().flatten().map(|e| e.file_name()).collect();
        // Files at the root: the zip's name.
        let zip = write_zip(&[("mod.toml", ""), ("backgrounds/c.png", "3")]);
        let root = install_zip_into(&zip, &mods).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(toml.contains("version = \"2\"") && toml.contains("enabled = false") && toml.contains("priority = 3"));
        assert_eq!(files, [target.join("b.png"), target.join("mod.toml")]);
        assert_eq!(entries, ["ptbr"]);
        assert_eq!(root, mods.join("My Mod"));
    }

    #[test]
    fn folder_names_are_cleaned() {
        assert_eq!(folder_name("PT-BR: v1.0").as_deref(), Some("PT-BR_ v1.0"));
        assert_eq!(folder_name("..hidden.").as_deref(), Some("hidden"));
        assert_eq!(folder_name(" . "), None);
    }
}
