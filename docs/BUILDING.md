# Building

The build reads data from your own Parasite Eve II discs (USA, `SLUS-01042`
and `SLUS-01055`). Everything derived from them stays in ignored directories
(`decomp/rom`, `decomp/assets`, `decomp/build`, `build/`) and must never be
committed.

## Requirements (Windows)

- Rust stable with the `i686-pc-windows-msvc` target
- Visual Studio Build Tools with the x86 libraries
- LLVM 18 or newer (`clang`, `llvm-lib`, `llvm-nm`, `llvm-objdump` on `PATH`)
- Python 3.10 or newer
- `chdman` if your dumps are CHD files

Keep `core.autocrlf` off in this repository and in `decomp/` (and its
`tools/maspsx` submodule): the decomp applies patches that do not apply with
CRLF line endings.

## 1. Decomp setup (once)

```sh
cd decomp
git submodule update --init tools/maspsx
python -m venv venv
venv/Scripts/python -m pip install -r requirements.txt
```

Put your disc images in `decomp/rom/image/` as `SLUS-01042.bin/.cue` and
`SLUS-01055.bin/.cue`. From CHD:

```sh
chdman extractcd -i "Parasite Eve II (USA) (Disc 1).chd" -o rom/image/SLUS-01042.cue -ob rom/image/SLUS-01042.bin
chdman extractcd -i "Parasite Eve II (USA) (Disc 2).chd" -o rom/image/SLUS-01055.cue -ob rom/image/SLUS-01055.bin
```

## 2. Decomp extraction and PSX build

```sh
cd decomp
venv/Scripts/python ninja_config.py --iso_extract   # extract the discs
venv/Scripts/python ninja_config.py                 # split and configure
venv/Scripts/ninja                                  # matching PSX build
```

The PSX build provides every symbol's address and which units each image
links. Its checksum step reports `FAILED` under Git Bash because it calls
Windows `find`; check the outputs against `configs/USA/checksum.sha` instead.

## 3. Port build

```sh
python tools/gen_units.py      # units and their images, from the decomp build
python tools/build_game.py     # game library: build/game/pe2game.lib
cargo build                    # host executable
```

### Without the discs

Steps 1 and 2 are needed only to change the decomp or to refresh the
snapshot. Without the decomp's PSX build, the same three commands build
from `snapshot/` instead (or with `PE2_SNAPSHOT=1`): the units, each
image's symbols and fingerprint, and the asset includes the compile reads
with every value taken out (names, types and sizes only). The game reads
its data from the player's discs at run time, so the values never mattered
to the port's code; built from the snapshot it is the same, unit by unit.

The snapshot must match the decomp commit it was made from
(`snapshot/decomp.txt`). After changing the decomp, refresh it where the
discs are, after a normal build:

```sh
python tools/make_snapshot.py  # saves snapshot/, then checks the code built from it is the same
```

It refuses any include that still holds a value, as does
`tools/check_no_game_data.py` for what is committed.

The icon (pe2.exe in Explorer and the taskbar, and the game's and
launcher's windows) is `crates/pe2-host/res/icon.png`. To use a picture of
your own instead, put a square PNG at `local/icon.png` (ignored by git) and
build again.

Releases are built the same way, by `tools/release.py`: see
[RELEASING.md](RELEASING.md).

## 4. Run

The host reads your disc images directly (`.cue` or `.chd`):

```sh
target/i686-pc-windows-msvc/debug/pe2.exe "Parasite Eve II (USA) (Disc 1).chd" "Parasite Eve II (USA) (Disc 2).chd"
```

Started without arguments, `pe2` opens a launcher (Setup on first run: disc
images, memory cards, quality; Play; Settings; Exit, in English or Portuguese)
that writes `pe2.toml` (Controls remaps the keyboard and controller and picks
player 1's controller) and runs the game as `pe2 --play`, coming back when it
quits. Runs driven by `PE2_INPUT`, `PE2_LIBTEST` or `PE2_SELFTEST` skip it.

Settings: the launcher writes `pe2.toml` in the data folder (below); to edit
it by hand, copy `pe2.example.toml` there, which documents every setting.
With the disc images set in it, `pe2 --play` needs no arguments. Arguments
and `PE2_*` environment variables override it.

Debug output:

- `PE2_TRACE` takes comma-separated topics, or `all`:
  - CD and files: `cd` (CD commands), `cdsector` (every delivered sector),
    `files` (each game file read, by its mod name), `images` (overlays
    recognized in RAM), `mc` (memory cards), `startup` (how long reading the
    discs for fonts and mods takes, step by step);
  - graphics: `gpu` (command lists and display changes), `gpubig`
    (oversized polygons), `dma`, `vram`, `hw` and `present` (the renderer),
    `textures`, `pgxp` (vertices with and without exact positions),
    `interp`, `interpcheck` and `interpshots` (frames in between);
  - movies and sound: `stream` (movie frames), `mdec`, `movies` (movies
    beginning and ending, by hash), `xa` (XA audio sectors played), `spu`
    and `spukeys`, `music` (songs and replacement gain);
  - timing: `vsyncwait`, `vsyncmode`, `waits`, `scene` (scene pacing),
    `cputime`, `hang`, `lowmem`;
  - the rest: `move` (modern movement), `rooms`, `text` (translated
    dialogue files loaded), `stubs`.
- `PE2_RECORD_AUDIO=<file.wav>:<start>:<seconds>` saves what the sound chip
  plays over that span, to check the mix without listening.
- Performance: `PE2_TRACE=cpu` prints each thread's CPU use every 10 seconds
  (and interrupt delivery costs); `PE2_TRACE=prof` samples the game thread
  every millisecond into `pe2-prof.bin`, summarized by function with
  `python tools/profile_report.py pe2-prof.bin [--skip SECONDS]` (release
  build: uses `target/i686-pc-windows-msvc/release/deps/pe2.map`).
- `PE2_SCALE=1..8` renders at that multiple of the console's resolution, in
  true colour without dithering (24-bit movies stay at their own
  resolution). The game reads back what the graphics card drew, one
  console pixel per block.
- With `[debug] record_input = true` in pe2.toml (off by default), each
  session records the controller (`recordings/<date>_<time>.txt` in the
  data folder, the last ten, each with the memory card it started with).
  `PE2_REPLAY=<recording>.txt` plays one back from power-on on a copy of
  that card, the same way, update for update, then pauses where it
  ended (P resumes with the controls) and saves a screenshot: a way to get
  back to a place to debug it. `PE2_REPLAY_PAUSE=0` lets the game go on
  past the end instead, and `PE2_REPLAY_EXIT=<seconds>` closes it that long
  after the end.
- `PE2_SCREENSHOT=<dir>` saves the displayed picture as a PNG every two
  seconds. Screenshots show your game's content; keep them out of the
  repository.
- `PE2_INPUT=start@20,cross@31.5` presses buttons at given times (seconds
  after start) for unattended runs; `pause@t` toggles pause and
  `screenshot@t` saves one, as P and F9 do.
- `PE2_WATCH=0x801D49C4[,...]` (up to four addresses) logs every write to
  those words with the writing instruction's address, using the CPU's debug
  registers.
- `PE2_PEEK=0x80082818:0x58[,addr:len...]` prints those bytes of PSX RAM
  every five seconds: a game's state while it seems stuck.
- `PE2_GPUDUMP=<seconds>` logs the first command lists drawn after that
  time, a line per command (F12 dumps the current one).
- `PE2_FONTSHEETS=<dir>` saves the text pictures redrawn from the built-in
  fonts as indexed replacements, to look at them.
- A native crash prints the faulting address and stack; map the addresses to
  game functions with `python tools/symbolize.py <addr>...` (uses the linker
  map, `target/.../deps/pe2.map`).
- A native game function that misbehaves can be found against the original
  MIPS code: built with `python tools/build_game.py --route-calls`,
  `PE2_INTERPRET=<addr>,<A-B>...` runs those functions in the interpreter
  instead, `PE2_SHADOW=...` runs both and reports where they differ, and
  `python tools/bisect_native.py` bisects the game's functions for one that
  breaks 3D geometry.

Keyboard: arrows = d-pad, Z/X/A/S = Cross/Circle/Square/Triangle,
Enter = Start, Backspace = Select, Q/W/E/R = L1/R1/L2/R2, Alt+Enter toggles
borderless full screen (`fullscreen = true` in pe2.toml starts in it; the mouse
cursor is hidden there unless `hide_cursor = false`), Esc asks to
quit (`[general] confirm_exit = false` quits at once), P pauses and resumes, F9
saves a screenshot at the internal resolution in `screenshots/` in the data
folder, F10 swaps discs by hand; for debugging, F11 writes a timed mark in
the log and F12 dumps the GPU's current command list.

The data folder holds pe2.toml, the default memory cards (`memcards/`),
screenshots, input recordings (`recordings/`) and pe2.log: `%APPDATA%\PE2PC` on Windows,
`~/Library/Application Support/PE2PC` on macOS, `$XDG_DATA_HOME/pe2pc`
(`~/.local/share/pe2pc`) elsewhere. For a portable setup, put an empty
`portable.txt` next to pe2.exe: everything then stays in pe2.exe's folder
(as it does when a pe2.toml is already there). `PE2_HOME` names another
folder.

## Checks

```sh
python tools/check_no_game_data.py   # nothing derived from the discs is tracked
python tools/check_psx_unchanged.py  # PC_PORT changes leave the PSX build identical
python tools/probe_units.py          # every unit compiles for the PC target
cargo fmt --check                    # Rust code formatted (rustfmt.toml)
cargo clippy --release -p pe2-host --tests   # no lint warnings
```
