//! Links the game library built by `python tools/build_game.py`.

use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let game_dir = root.join("build/game");
    let lib = game_dir.join("pe2game.lib");
    if !lib.exists() {
        panic!(
            "{} not found: run `python tools/build_game.py` first",
            lib.display()
        );
    }
    println!("cargo:rerun-if-changed={}", lib.display());
    println!("cargo:rustc-link-search=native={}", game_dir.display());
    // Whole-archive: the thunk tables reference every native body, and the
    // game's functions are reached only through them.
    println!("cargo:rustc-link-lib=static:+whole-archive=pe2game");

    // PSX memory is identity-mapped at 0x80000000 and 0x1F800000, so the image
    // must be 32-bit, large-address-aware and loaded at a fixed base that
    // keeps out of those ranges. The absolute-symbol object has no SafeSEH
    // table.
    for arg in [
        "/LARGEADDRESSAWARE",
        "/FIXED",
        "/DYNAMICBASE:NO",
        "/BASE:0x00400000",
        "/SAFESEH:NO",
        "/MAP",
    ] {
        println!("cargo:rustc-link-arg-bins={arg}");
    }
}
