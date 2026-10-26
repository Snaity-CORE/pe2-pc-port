# Releasing

GitHub builds the port without the game discs, from `snapshot/` (see
[BUILDING.md](BUILDING.md), "Without the discs"):

- `.github/workflows/build.yml`, on every push to `main`, pull request and
  tag: builds the game library and the host (release), runs the tests and
  keeps `pe2.exe` as the run's artifact. A tag `vX.Y.Z` also packages it
  and opens a draft release. A push to `main` also packages it as the
  nightly pre-release (below).
- `.github/workflows/ci.yml`, on every push and pull request: no game data
  in the repository, the Python tools compile, Rust formatting.

If the decomp is a private repository, the build needs a secret
`DECOMP_TOKEN`: a token that can read it (fine-grained, read-only
contents).

## Making a release

1. Set the version in `crates/pe2-host/Cargo.toml` (`version = "X.Y.Z"`)
   and commit. The snapshot must match the decomp commit the port points
   to (`snapshot/decomp.txt`; `tools/make_snapshot.py` refreshes it).
2. Push the port's `main` and the decomp commit it points to.
3. Tag and push the tag:

   ```sh
   git tag -a vX.Y.Z -m "pe2-pc-port vX.Y.Z"
   git push origin vX.Y.Z
   ```

   The Build workflow builds and tests it, then `release.py X.Y.Z --ci`
   checks the version matches, packages
   `pe2-pc-port-vX.Y.Z-windows-x86.zip` and opens a draft release with it,
   its SHA-256 and notes generated from the commits since the last release.
4. Review the draft on GitHub and publish it.

### From this machine instead

With the discs and a normal build, `tools/release.py` does the same here
(the [GitHub CLI](https://cli.github.com/), logged in with `gh auth login`,
is needed):

```sh
python tools/release.py X.Y.Z --dry-run   # build, check, package: no tag, no release
python tools/release.py X.Y.Z             # also tag, push the tag, draft release
```

It first checks that both trees are clean, their commits on GitHub, and
the tag new.

## Nightly

`main`'s last build is always at the same link:

https://github.com/Snaity-CORE/pe2-pc-port/releases/download/nightly/pe2-pc-port-nightly-windows-x86.zip

`release.py --nightly`, after each push's build, moves the tag `nightly`
to the built commit and replaces the zip of the pre-release of that tag
(made the first time). A pre-release, it is never the latest release, and
release notes count from the last real release, not from it.

## The zip

`pe2.exe`, `pe2.example.toml`, `README.md`, `MODDING.md` and the licenses of
the fonts `pe2.exe` embeds (`LICENSE-DejaVu.txt`, `LICENSE-CourierPrime.txt`).
Players bring their own discs; the game reads them at runtime.

## No game data

A release must not carry anything from the discs:

- `tools/check_no_game_data.py`: no tracked file looks like disc data, and
  the snapshot's includes hold no values (also the pre-commit hook and CI).
- Built on GitHub, from the snapshot, the game's data was never there to
  take.
- Built here, `tools/check_exe_game_data.py` checks the exe itself: no
  32-byte run of the game's executables, and no run of 32 values of the
  asset tables the decomp generates from the discs (nor their texts),
  appears in `pe2.exe`. Data the port holds on purpose is listed in its
  `ALLOWED`, with why (libspu's reverb presets, from the public hardware
  documentation). It takes a few minutes.
