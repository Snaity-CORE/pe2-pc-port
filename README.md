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
- **Sharp text**: the dialogue, the menus, the loading captions and the
  disc change's lines drawn from high-definition fonts (the built-in DejaVu
  Sans Bold and Courier Prime) instead of the game's small pictures of text,
  at internal resolutions above the console's.
- **Mods** (experimental, turned on in Settings): replace the pre-rendered
  backgrounds and full-screen pictures and the textures with your own at
  any resolution, the music with
  recordings, the movies with video files, any of the game's files, and
  the dialogue with translations; cheats and memory patches; the game's
  pictures, textures, music, movies, files and dialogue extracted from your
  discs with `pe2 --extract` to make them (see [docs/MODDING.md](docs/MODDING.md)).
- **Cheats** (in `pe2.toml`, `[cheats]`): infinite HP or MP, one-hit kills,
  every hit critical, and EXP and BP multipliers.
- **Comfort**: a launcher worked by mouse, keyboard or controller (English or
  Portuguese), remappable keys and controller buttons, pause, screenshots,
  borderless full screen, and Discord showing where you are in the game,
  with your EXP and BP.

Memory cards are raw `.mcd` images, so saves move both ways with DuckStation.

## How it was made

This port was developed with AI assistance: much of its code, tools and
documentation was written with [Claude Code](https://claude.com/claude-code)
(Anthropic). That is why it came together quickly.

Every step was directed and supervised by a person. Snaity set the goals
and the rules the project keeps (no game data, no Sony code, the game's own
behaviour as the reference), reviewed each change, played and tested it in
the game, compared it with the console through DuckStation, and made every
commit. Bugs were tracked down by measuring the running game, not by
guesswork, and fixes went in only after they held up in play.

## Playing

You need Windows 10 or 11 and a graphics card with Vulkan or OpenGL 3.3.

Take the [latest release](https://github.com/Snaity-CORE/pe2-pc-port/releases/latest),
or the [last build of `main`](https://github.com/Snaity-CORE/pe2-pc-port/releases/download/nightly/pe2-pc-port-nightly-windows-x86.zip)
(the nightly pre-release: untested beyond CI), or build it as
[docs/BUILDING.md](docs/BUILDING.md) explains; then start `pe2.exe`. The
launcher's Setup asks for your disc images, memory cards and a quality
level; after that, Play starts the game. Settings and Controls change
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
| `tools/` | Build, debugging, verification and release tools |
| `snapshot/` | What the build takes from the decomp's PSX build, without the game's data, to build without the discs |
| `docs/` | [Architecture](docs/ARCHITECTURE.md), [building](docs/BUILDING.md), [modding](docs/MODDING.md) and [releasing](docs/RELEASING.md) |

## Golden rule

Never commit disc images, extracted files, includes generated from the disc,
saves or screenshots. The pre-commit hook (`python tools/install_hooks.py`)
runs `tools/check_no_game_data.py` and blocks them.

## Credits

- **Port**: Snaity (Adriano Teodoro), developed with AI assistance (Claude
  Code, by Anthropic) under Snaity's direction and review (see
  [How it was made](#how-it-was-made)).
- **Launcher art** (the background, the PE2PC logo and the icon): made
  with AI image generation under Snaity's direction; CC0 1.0 as the rest
  of the project.
- **Decompilation**: [GabeRealB](https://github.com/GabeRealB/parasite-eve-2-decomp)
  and its contributors. The port stands on their work.
- **Parasite Eve II**: Square, and everyone who made the original game.
- **Hardware documentation**: Martin Korth's
  [psx-spx](https://problemkaputt.de/psx-spx.htm) (Nocash PSX
  Specifications), which the runtime's GPU, SPU, GTE, MDEC and CD-ROM are
  written from.
- **Sub-pixel precision**: the idea of PGXP, by iCatButler.
- **Fonts**: [DejaVu Sans](https://dejavu-fonts.github.io/) Bold, built in for
  the dialogue and the menus (Bitstream Vera license with DejaVu's changes
  in the public domain), and
  [Courier Prime](https://github.com/quoteunquoteapps/CourierPrime) for the
  loading captions and the disc change (SIL Open Font License 1.1); their
  licenses are in `crates/pe2-host/res/fonts/`.
- **Reference**: [DuckStation](https://github.com/stenzek/duckstation), used
  to compare behaviour (none of its code is used).
- **Libraries**: [wgpu](https://wgpu.rs), [winit](https://github.com/rust-windowing/winit),
  [egui/eframe](https://github.com/emilk/egui), [gilrs](https://gitlab.com/gilrs-project/gilrs),
  [cpal](https://github.com/RustAudio/cpal), [chd-rs](https://github.com/SnowflakePowered/chd-rs),
  [iced-x86](https://github.com/icedland/iced), [rfd](https://github.com/PolyMeilex/rfd),
  [ab_glyph](https://github.com/alexheretic/ab-glyph) (the fonts),
  [discord-rich-presence](https://github.com/vionya/discord-rich-presence), png, image-webp, toml, toml_edit, serde,
  pollster, windows-sys, lewton, claxon and hound (mods' music),
  [openh264](https://github.com/ralfbiedert/openh264-rs) with Cisco's
  [OpenH264](https://github.com/cisco/openh264) (BSD-2-Clause, © Cisco Systems),
  mp4 and [Symphonia](https://github.com/pdeljanov/Symphonia) (mods' movies),
  and the crates they build on. Thanks to all their authors.

## Legal

Parasite Eve II is © Square (now Square Enix). This project is not affiliated
with or endorsed by them, and includes none of the game's data: you supply
your own copy.

The project's own code is dedicated to the public domain under
[CC0 1.0](LICENSE), as the decompilation in `decomp/` is. The fonts in
`crates/pe2-host/res/fonts/` keep their own licenses, and so do the
libraries `pe2.exe` is built from: each release carries their notices in
`THIRD-PARTY-NOTICES.txt` (`python tools/third_party.py` writes them).
