# Parasite Eve II: native PC port

A native PC port of *Parasite Eve II* (PlayStation, USA release) built on the
[decompilation by GabeRealB](https://github.com/GabeRealB/parasite-eve-2-decomp).

> **This repository contains no game data.** To play, you need a dump of your
> own discs (`SLUS-01042` and `SLUS-01055`). The port reads their data at
> runtime.

Status: **phase 0** (infrastructure). Architecture and roadmap are in
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Layout

| Path | Contents |
|---|---|
| `decomp/` | Submodule with the game's C code (fork of the decomp) |
| `port/` | C shim: PC Psy-Q headers, software GTE, C versions of the hasm routines |
| `crates/pe2-host/` | Rust host executable and PSX runtime |
| `tools/` | Build and verification tools |
| `docs/` | Documentation |

## Requirements (Windows)

- Rust stable with the `i686-pc-windows-msvc` target (`rustup target add i686-pc-windows-msvc`)
- LLVM/clang 18 or newer
- Visual Studio Build Tools (MSVC x86)
- Python 3.10 or newer

## Getting started

```sh
git clone --recurse-submodules <repo-url>
python tools/install_hooks.py   # installs the game-data guard
cargo build
```

## Golden rule

Never commit disc images, extracted files, includes generated from the disc, or
saves. The pre-commit hook and CI run `tools/check_no_game_data.py` and block
them.
