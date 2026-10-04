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
| Overlays | All 448 built together, namespaced per overlay, with a dispatch table | On the PSX they all load at the same address. There are ~1875 function names duplicated across rooms and actors. |
| Overlay data | Loaded from the `.pe2pkg` files on the player's disc | Models, animations and skeletons follow each package's `.text`. |
| Generated `assets/*.inc` | Phase 1: generated at build time from the local disc into `build/`. Later: loaded at runtime. | Build-time generation embeds game data in the binary, so it cannot be distributed. Loading at runtime lets us ship only the executable. |
| hasm routines | Rewritten in C (LZ decompression, TMD dispatch) | Only 9 routines, all documented in `decomp/src/main/hasm/README.md`. |

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

0. **Infrastructure.** Repository layout, game-data guard, documentation.
1. **Link.** All game C built for x86 against a stubbed runtime: software GTE,
   PC Psy-Q headers, overlay namespacing, C rewrites of the hasm routines.
2. **Title screen.** BIN/CUE reader and the game's file system, LZ
   decompression, 2D GPU (VRAM, sprites, polygons), controller, VSync.
3. **Full game.** GTE and 3D models, rooms, combat, audio (SPU and CD-DA/XA),
   FMV (MDEC/STR), memory card and disc swap. Goal: finish the game, compared
   against a reference emulator.
4. **Enhancements and mods.** 64-bit, high resolution, widescreen, 60 fps,
   replaceable textures and a mod API.
