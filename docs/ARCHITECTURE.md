# Parasite Eve II PC port: architecture

## Principles

1. **No game data in the repository.** Players supply a dump of their own USA
   discs (`SLUS-01042` and `SLUS-01055`). Everything derived from them
   (packages, textures, models, audio, FMV, generated includes) is produced
   locally and never committed. `tools/check_no_game_data.py` enforces this in
   the pre-commit hook and in CI.
2. **Accuracy first.** The first goal is to play the whole game from start to
   finish with the original behavior. Enhancements come later, as options that
   can be turned off.
3. **Game C stays close to upstream.** The decomp is still active (renaming and
   documenting code). Port changes go into the fork behind `#ifdef PC_PORT`,
   kept small and easy to rebase.

## Layers

```
┌────────────────────────────────────────────────────────────────┐
│ pe2 (Rust) host: window, frame loop, config, mods              │
├────────────────────────────────────────────────────────────────┤
│ PSX runtime (Rust, extern "C" API)                              │
│  disc/CD (BIN/CUE) · GPU → wgpu · SPU → cpal · pad → gilrs      │
│  memory card → files · MDEC → FMV · timers/VSync                │
├────────────────────────────────────────────────────────────────┤
│ C shim (port/): PC Psy-Q headers, inline software GTE,          │
│ C rewrites of the 9 hasm routines, overlay table                │
├────────────────────────────────────────────────────────────────┤
│ Game code: decomp/src (C, ~740k lines) built with clang         │
│ through build.rs (`cc` crate) as a static library               │
└────────────────────────────────────────────────────────────────┘
```

### Why hybrid C + Rust

- **Game code stays in C.** Transpiling ~740k lines (c2rust) produces
  non-idiomatic `unsafe` Rust and breaks sync with upstream.
- **The platform layer is Rust.** That is where Rust pays off most: safe
  parsing of the disc and game formats, wgpu, cpal, gilrs and a robust mod
  system.
- **The GTE stays in C.** It is the hottest path (hundreds of `gte_*` calls per
  frame), so it is implemented as `static inline` functions in a C header, with
  no FFI call per instruction.

## Technical decisions

| Topic | Decision | Reason |
|---|---|---|
| Target architecture | `i686-pc-windows-msvc` (32-bit) in phase 1 | Structs overlay disc data and assume 4-byte pointers. 64-bit is a phase 4 enhancement. |
| C compiler | clang | Compiles the decomp's C almost unchanged. It is also what the `cc` crate drives. |
| Memory | PSX RAM and scratchpad identity-mapped at their console addresses (see below) | Game code addresses data by fixed address across images, and disc data holds PSX pointers. |
| Overlays | Every image linked in; each unit rewritten per image that links it (`tools/pclink.py`) | All overlays of a family load at the same address, so names and addresses repeat across them. |
| Game data | All globals (main, gameplay, overlays) live in PSX RAM, loaded from the player's disc | Exactly the console's initial values and layout; nothing from the disc has to be in the binary. |
| Generated `assets/*.inc` | Still needed to compile, but the data they define is remapped to PSX RAM and unused | A distributable build must drop it; auditing that is tracked for phase 3. |
| Code generation | `-funsigned-char -fwrapv -fno-strict-aliasing -mno-sse` and friends (`tools/build_game.py`) | Matches the PSX build's semantics; globals are only as aligned as on the PSX. |
| Runtime functions | `pcrt_<name>` for code with no C body: Psy-Q libraries and the hand-written assembly | Unimplemented ones fall back to logging stubs through `/alternatename`. |

## Memory model

The port keeps the PlayStation's memory map. At startup the host maps 2 MiB
at `0x80000000` (RAM, KSEG0) and 1 KiB at `0x1F800000` (scratchpad); this
needs a 32-bit, large-address-aware process with a fixed image base. The
player's `SLUS_010.42` is copied to its load address and BSS is cleared, as
the BIOS and crt0 do, and overlays are later loaded by the game's own code.
Every pointer is then a valid host pointer: hard-coded addresses, pointers in
disc data, and pointers the game computes.

The decompiled C is compiled natively, then `tools/pclink.py` rebinds each
object using the addresses from the decomp's matching PSX build:

- **Data** defined in C becomes a reference to `psx_<ADDR>`, an absolute
  symbol at its PSX address, so the game uses the copy in PSX RAM.
- **Functions** keep their native body (`pcfn_<image>_<ADDR>_<name>`). When an
  image is in memory, the runtime writes a `jmp` to each native body at the
  function's PSX address. Taking a function's address yields that PSX address,
  so function pointers stored in game data work, and calling into another
  overlay reaches whichever one is loaded, as on the console.
- **Calls** within an image or into `main` stay direct native calls.
- **Code with no C body** (Psy-Q libraries, assembly routines) is called as
  `pcrt_<name>` in the port runtime. Library state that lives in PSX RAM stays
  there, for example `rand`'s seed.

RAM is one memory section mapped at every address the PSX mirrors it at that
Windows allows: `0x80000000` (KSEG0), `0xA0000000` (KSEG1) and `0x00010000`
upward (KUSEG). Windows never maps the first 64 KiB, so accesses there (the
game reads through null pointers in places, harmlessly on the console) fault,
and a handler decodes the instruction and performs it against the RAM mirror
(`runtime/lowmem.rs`: loads, stores and ALU instructions with exact flags).

## Interrupts

Game callbacks run in interrupt context on the console: VSync, CD data ready
and command complete, and later GPU, SPU and timers. The game relies on that,
for example by spinning on a flag that only a callback sets.

The runtime keeps a timeline of events on an IRQ thread (`runtime/irq.rs`).
When an event is due, it suspends the game thread, checks that interrupts are
enabled, runs the handler on its own stack and resumes the game thread,
exactly like an interrupt. Interrupts are held off while the game is in
EnterCriticalSection, and while the game thread is inside a runtime function
so it is never suspended holding a runtime or CRT lock. Runtime functions that
wait for an interrupt (`VSync`, `CdSync`, `CdControlB`) let interrupts in while
waiting.

## Library code

The game calls about 430 library routines with no C body (Psy-Q libraries
and the decomp's hand-written assembly). They are handled in three ways:

1. **Interpreted.** By default a routine runs its original MIPS code, which
   is in PSX RAM after boot, in a small R3000 interpreter
   (`runtime/mips.rs`), with GTE instructions on the software GTE. Pure code
   (GTE math, GPU packet builders, the LZ decompressor, MDEC VLC decoding)
   then behaves exactly as on the console, without a rewrite.
2. **Native.** Routines that drive hardware or the BIOS kernel have Rust
   implementations: CD-ROM, GPU, VSync and timers, interrupts and events,
   controller, memory card, sound. At boot each native routine also gets a
   thunk at its PSX address, so interpreted code and stored function
   pointers reach it too.
3. **Interpreted over emulated registers.** Thin layers over a device
   (libpress over the MDEC) run interpreted, and their register accesses go
   to device models (`runtime/hw.rs`: MDEC, DMA channels 0/1).

Callbacks registered by the game or by libraries are called through
`call_psx`: natively when a thunk is installed at the address, otherwise in
the interpreter.

Every library routine gets a thunk at its PSX address: to its native
implementation, or to the stub that interprets it. The two MIPS words a
thunk overwrites are kept aside for the interpreter. Native calls into
another image go through a trampoline that runs the target natively when it
has a thunk and in the interpreter otherwise (`port/src/dispatch.s`).

## Static functions

clang drops a `static` function nothing in its file references, but other
images still call some of them by address (overlay entry points). Each unit
is compiled through a small wrapper that includes the original source and
references every static function the PSX build emitted for it
(`tools/build_game.py`), so they all exist natively.

## Overlays

The game loads its code packages itself. The build records, for every
image, its load address, size and two hashes of its contents (head and tail,
with the bytes thunks cover read as zero); no image bytes reach the binary.
After every CD callback, vertical blank and VSync the runtime checks each
load address and, when a different image is now there (or the same one was
reloaded over its thunks), installs its thunks (`port/src/images.c`). The
check right after CD callbacks matters: the game decompresses packages
inside them and may call into one as soon as the interrupt returns.

## GPU, MDEC and streaming

The GPU is emulated in software at the command level (`runtime/gpu`):
a 1024x512 15-bit VRAM, GP0 polygons, lines, rectangles, textures with
CLUTs and texture windows, semi-transparency, dithering and mask bits, fed
by the native libgpu calls (ordering tables, image transfers, drawing and
display environments). The display area is shown in a host window
(`video.rs`), with keyboard input mapped to the controller.

The MDEC decodes macroblocks from the run-length data libpress sends it
(`runtime/mdec.rs`). STR movies are streamed with a native CdRead2 and St*
ring (`runtime/stream.rs`) that assembles each frame's sectors contiguously
in the game's ring buffer.

## Disc and CD-ROM

The host reads the player's images directly, as BIN/CUE or CHD
(`disc/`), finds the boot executable through `SYSTEM.CNF` and ISO 9660, and
loads it like the BIOS. The CD-ROM drive is emulated at the libcd level
(`runtime/cdrom.rs`): primitive commands, sync and data-ready interrupts with
their callbacks, and sectors delivered in real time at 75 or 150 sectors per
second. XA-ADPCM and CD-DA audio come with the sound work in phase 3.

## Feasibility (measured 2026-10-04)

`-fsyntax-only` build of all 559 decomp translation units with clang
`--target=i686-pc-windows-msvc`:

| Status | Units |
|---|---:|
| Compile unchanged | 122 |
| Only missing the disc-generated `assets/*.inc` | 332 |
| Only fail on the GTE macros (inline MIPS asm) | 92 |
| Other fixes (lvalue casts, per-package parameters, function pointer types) | 13 |

## Phases

0. **Infrastructure** (done). Repository layout, game-data guard,
   documentation.
1. **Link** (done). All game C built for x86 with a software GTE, the
   identity-mapped memory model and a stubbed runtime. The native `main()`
   boots through sound and graphics init into its first CD commands.
2. **Title screen** (in progress). Disc reader (BIN/CUE, CHD) and the game's file system,
   CD runtime, C rewrites of the hasm routines (LZ decompression, TMD
   dispatch), 2D GPU (VRAM, sprites, polygons), controller, VSync.
3. **Full game.** GTE and 3D models, rooms, combat, audio (SPU and CD-DA/XA),
   FMV (MDEC/STR), memory card and disc swap. Goal: finish the game, compared
   against a reference emulator.
4. **Enhancements and mods.** 64-bit, high resolution, widescreen, 60 fps,
   replaceable textures and a mod API.
