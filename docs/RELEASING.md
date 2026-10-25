# Releasing

A build needs the game discs (see [BUILDING.md](BUILDING.md)), so releases
are made on a machine that has them, with `tools/release.py`, not on
GitHub's runners. GitHub Actions (`.github/workflows/ci.yml`) runs the
checks that need no discs on every push and pull request: no game data in
the repository, the Python tools compile, Rust formatting.

## Once

- The [GitHub CLI](https://cli.github.com/): `winget install GitHub.cli`,
  then `gh auth login`.
- A working build (BUILDING.md, steps 1 to 3).

## Making a release

1. Set the version in `crates/pe2-host/Cargo.toml` (`version = "X.Y.Z"`)
   and commit.
2. Push the port's `main` and the decomp commit it points to.
3. Try it without publishing anything:

   ```sh
   python tools/release.py X.Y.Z --dry-run
   ```

   It builds (units, game library, host in release, tests), checks for game
   data and writes `dist/pe2-pc-port-vX.Y.Z-windows-x86.zip`.
4. Release:

   ```sh
   python tools/release.py X.Y.Z
   ```

   Before building it also checks that both trees are clean and their
   commits are on GitHub, and that the tag is new. After packaging it tags
   the commit `vX.Y.Z`, pushes the tag and opens a draft release with the
   zip, its SHA-256 and notes generated from the commits since the last
   release.
5. Review the draft on GitHub and publish it.

## The zip

`pe2.exe`, `pe2.example.toml`, `README.md` and `MODDING.md`. Players bring
their own discs; the game reads them at runtime.

## No game data

A release must not carry anything from the discs. Two checks run before
anything is packaged:

- `tools/check_no_game_data.py`: no tracked file looks like disc data
  (also the pre-commit hook and CI).
- `tools/check_exe_game_data.py`: no 32-byte run of the game's executables,
  and no run of 32 values of the asset tables the decomp generates from the
  discs, appears in `pe2.exe`. Data the port holds on purpose is listed in
  its `ALLOWED`, with why (libspu's reverb presets, from the public hardware
  documentation). It takes a few minutes.
