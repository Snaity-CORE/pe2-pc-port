# PE2PC: Parasite Eve II, native on PC

A native PC port of *Parasite Eve II* (PlayStation, USA release) built on the
[decompilation by GabeRealB](https://github.com/GabeRealB/parasite-eve-2-decomp).
The game's own code, decompiled to C, is compiled for the PC and runs natively;
a small runtime written for this project stands in for the console (graphics,
sound, CD, controllers, memory cards). No emulator and no code from Sony.

> **This repository contains no game data.** To play, you need dumps of your
> own discs (`SLUS-01042` and `SLUS-01055`, as `.chd` or `.cue` images). The
> port reads the game's data from them at run time.

## Status

Playable on Windows: both discs, movies, music and sound, saves, controllers.
It plays as the console does and adds what a PC can, all optional:

- **Higher resolution**, up to 8x the console's 320x240, rendered on the
  graphics card, with exact (sub-pixel) vertex positions so models neither
  wobble nor crack, and perspective-correct textures.
- **Smooth motion at 60 fps**: the game still runs at its 30, and frames in
  between are drawn with the 3D models halfway, with no added delay.
- **Filtering**: smoothed 3D textures (bilinear) and smoothly enlarged
  pre-rendered backgrounds (bicubic).
- **Presentation**: a CRT look (scanlines and aperture grille) and integer
  scaling.
- **Modern movement**: the stick points where to go, relative to the camera,
  in exploration and in battle; or the original tank controls with a quick
  turn (back + run).
- **Mods**: replace the pre-rendered backgrounds and full-screen pictures
  with your own at any resolution, dumped from the game as you play (see
  [docs/MODDING.md](docs/MODDING.md)).
- **Comfort**: a launcher worked by mouse, keyboard or controller (English or
  Portuguese), remappable keys and controller buttons, pause, screenshots,
  borderless full screen.

Memory cards are raw `.mcd` images, so saves move both ways with DuckStation.

## Playing

Build it as [docs/BUILDING.md](docs/BUILDING.md) explains, then start
`pe2.exe`. The launcher's Setup asks for your disc images, memory cards and a
quality level; after that, Play starts the game. Settings and Controls change
everything later.

Default keys (Controls remaps them):

| PlayStation | Keyboard | Controller |
|---|---|---|
| D-pad | Arrow keys | D-pad (or left stick) |
| Cross / Circle / Square / Triangle | Z / X / A / S | A / B / X / Y |
| L1 / R1 / L2 / R2 | Q / W / E / R | LB / RB / LT / RT |
| Start / Select | Enter / Backspace | Start / Back |

| Key | Does |
|---|---|
| Alt+Enter | Borderless full screen on and off |
| P | Pause and resume |
| F9 | Screenshot (in the data folder's `screenshots/`) |
| F10 | Swap discs by hand (the game normally asks and it happens by itself) |
| Esc | Quit (asks first, unless turned off in Settings) |

### Where your files go

Settings (`pe2.toml`), the default memory cards, screenshots and the game's log
live in the data folder: `%APPDATA%\PE2PC` on Windows (Settings has a button
that opens it). For a portable setup, put an empty `portable.txt` next to
`pe2.exe` and everything stays in its folder. `pe2.example.toml` documents every
setting.

## Layout

| Path | Contents |
|---|---|
| `decomp/` | Submodule with the game's C code (fork of the decomp) |
| `port/` | C side: the PC's own Psy-Q headers and library routines, software GTE |
| `crates/pe2-host/` | Rust host: launcher, window, graphics, sound, CD and the rest of the runtime |
| `tools/` | Build, debugging and verification tools |
| `docs/` | [Architecture](docs/ARCHITECTURE.md) and [building](docs/BUILDING.md) |

## Golden rule

Never commit disc images, extracted files, includes generated from the disc,
saves or screenshots. The pre-commit hook (`python tools/install_hooks.py`)
runs `tools/check_no_game_data.py` and blocks them.

## Credits

- **Port**: Adriano Teodoro.
- **Decompilation**: [GabeRealB](https://github.com/GabeRealB/parasite-eve-2-decomp)
  and its contributors. The port stands on their work.
- **Parasite Eve II**: Square, and everyone who made the original game.
- **Hardware documentation**: Martin Korth's
  [psx-spx](https://problemkaputt.de/psx-spx.htm) (Nocash PSX
  Specifications), which the runtime's GPU, SPU, GTE, MDEC and CD-ROM are
  written from.
- **Sub-pixel precision**: the idea of PGXP, by iCatButler.
- **Reference**: [DuckStation](https://github.com/stenzek/duckstation), used
  to compare behaviour (none of its code is used).
- **Libraries**: [wgpu](https://wgpu.rs), [winit](https://github.com/rust-windowing/winit),
  [egui/eframe](https://github.com/emilk/egui), [gilrs](https://gitlab.com/gilrs-project/gilrs),
  [cpal](https://github.com/RustAudio/cpal), [chd-rs](https://github.com/SnowflakePowered/chd-rs),
  [iced-x86](https://github.com/icedland/iced), [rfd](https://github.com/PolyMeilex/rfd),
  [softbuffer](https://github.com/rust-windowing/softbuffer), png, image-webp, toml, toml_edit, serde,
  pollster and windows-sys. Thanks to all their authors.

## Legal

Parasite Eve II is © Square (now Square Enix). This project is not affiliated
with or endorsed by them, and includes none of the game's data: you supply
your own copy. The project's own code is released under CC0-1.0; the
decompilation in `decomp/` keeps its own license.
