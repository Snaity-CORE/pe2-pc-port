# Modding

PE2PC can show mods: folders of your own files that change what the game
shows, without touching the game's data or its logic. This page explains how
to make one, and the design the mod support follows as it grows.

> Mods contain only their authors' work. The game's own files never go into a
> mod (assets are found by a hash of what they replace, so a mod does not need
> them), and nothing taken from the discs ever goes into this repository.

## Where mods go

Mods live in `mods/` in the data folder (`%APPDATA%\PE2PC\mods` on Windows; the
launcher's Settings opens the data folder). Each mod is a folder:

```
mods/
  better-backgrounds/
    mod.toml
    backgrounds/
      20ddfa255291fdb6.png
      ...
```

`mod.toml` is optional; every field in it is too:

```toml
name = "Better backgrounds"
author = "Your name"
version = "1.0"
description = "Hand-painted backgrounds for Dryfield"
priority = 0      # higher wins where two mods replace the same thing
enabled = true    # false keeps the mod installed but unused
```

The launcher's **Mods** screen lists the installed mods, turns each on or
off and sets their order (higher in the list wins), writing `enabled` and
`priority` into each `mod.toml` and keeping the rest of the file. It also
turns all mods off (`[mods] enabled = false` in `pe2.toml`, or `PE2_MODS=0`),
turns dumping on, and opens the mods folder. Changes apply when the game
starts; its log lists the mods it uses.

## Backgrounds and full-screen pictures

Parasite Eve II's rooms are pre-rendered pictures of 320x240 pixels, and its
title and menu screens are full-screen pictures too. A mod can replace any of
them with a picture of its own, at any resolution: 1280x960 (4x) suits most
setups, and the picture is scaled to the internal resolution.

1. Turn on dumping: in the launcher's Mods screen, or `[mods] dump = true` in
   `pe2.toml` (or `PE2_DUMP=1`).
2. Play through the places you want to change. Each picture the game shows is
   saved once as `dumps/backgrounds/<hash>.png` in the data folder.
3. Make your version of a picture (paint over it, upscale it, ...) and save it
   as `mods/<your mod>/backgrounds/<same hash>.png`.

The file name is what ties a replacement to the picture it replaces: a 64-bit
FNV-1a hash of the picture's size and pixels exactly as the game sends them to
the graphics chip. Keep the name; the content can be anything.

What to know:

- **Foreground pieces.** Parts of a room that pass in front of the characters
  (pillars, railings, furniture edges) are drawn separately, from texture
  memory, on top of the background. They keep the original look until texture
  replacement exists (see the plan below): match your background to them.
- **Only the picture changes.** The game still has its own picture in video
  memory; replacing it never changes how the game plays.
- **Hardware renderer.** Replacements show with the graphics card renderer at
  any internal resolution; with the software renderer the original shows.
- **Loading.** A picture is read the first time its room shows (the original
  shows for a frame or two meanwhile) and kept while it is in use.

## Music

Parasite Eve II's music is sequenced: the game plays MIDI-like songs on the
sound chip's voices, one song at a time, each known by a sequence id. A mod can
replace a song with a recording of its own (OGG Vorbis, FLAC or WAV, any sample
rate, mono or stereo):

1. Turn on dumping, as for backgrounds, and play to where the song plays. The
   first two minutes of each song are saved as `dumps/music/<id>.wav` in the
   data folder (written when the song changes, or after two minutes).
2. Save your recording as `mods/<your mod>/music/<same id>.ogg` (or `.flac`,
   `.wav`). The id is two hexadecimal digits, as the dump's name has it: `26`
   is the Akropolis Tower's fire escape.

While a replacement plays, the song's voices are left out of the mix and the
recording plays instead, following the game's fades, mutes and volume
settings and stopping when the song stops. Sound effects and the movies' audio
are untouched. The recording loops from its end back to its start; OGG files
with `LOOPSTART` and `LOOPLENGTH` tags (in samples, as many looped tracks
carry) loop over that span instead. `music.toml` in the mod sets these per
song:

```toml
[track.26]
volume = 0.8        # loudness against the original song (default 1)
loop_start = 12.5   # seconds (default: the tag, or the start)
loop_end = 95.0     # seconds (default: the tag, or the end)
looping = true      # false plays the recording once
```

A recording is decoded the first time its song plays (the original plays for
a moment meanwhile) and kept from then on.

## Cheats and memory patches

A mod's `patches.toml` changes the game's memory each frame, as a cheat
cartridge does: GameShark (Action Replay) codes as they circulate for the
PlayStation, or plain writes.

```toml
[[cheat]]
name = "Always 50 HP"
codes = ["80073BA0 0032"]

[[patch]]
name = "Always 7 MP"
address = 0x80073BA4   # a PlayStation RAM address
size = 2               # 1, 2 or 4 bytes (default 4)
value = 7
```

Both apply at each vertical blank, before the game's own handler, while the
game is stopped. Each entry takes `enabled = false` to keep it but not use
it. Where two mods write the same place, the one higher in the order writes
last and wins.

GameShark code types supported: `80` (16-bit write), `30` (8-bit write), `10`,
`11`, `20`, `21` (16- and 8-bit increment and decrement), `D0` to `D3` and `E0`
to `E3` (16- and 8-bit equal, different, less and greater: the next code runs
only if the test passes) and `50` (serial repeater). A cheat with another
type (button activators such as `D4`, timers such as `C0`) is skipped, and the
game's log says why.

The decompilation's symbol maps give the addresses of the game's variables
(`gPlayerStatus` holds HP, MP, experience and more).

A patch that writes every frame also writes while a save loads: one that
changes `gPlayerStatus` makes the game report the save as corrupted. Turn such
mods off to load, or guard the code with a condition.

## The plan

Mod support grows in this order. Each kind follows the same rules: found by a
hash or a name of what it replaces, never containing the game's files, never
changing the game's own data in memory unless the kind is meant to.

1. **Backgrounds and full-screen pictures** (done): dump and replace, as above.
2. **A Mods screen in the launcher** (done): list, turn on and off, order.
3. **Memory patches** (done): GameShark codes and plain writes, each frame.
4. **Music** (done): sequenced songs dumped, and replaced by recordings, as
   above. (Recorded audio, the movies' and cutscenes' XA tracks, comes with
   movies.)
5. **Disc file replacement**: a file, or a part of a stage's CDF bundle, served
   from the mod instead of the disc, in the game's own formats; with text
   extraction, this is what translations need. Files larger than the original
   need the game's sector tables adjusted.
6. **Textures**: model, interface and foreground textures replaced at a higher
   resolution. On the PlayStation a texture is a region of video memory read
   through a palette, so a texture's identity is that region and its palette,
   hashed as the game uploads them.
7. **Movies**: FMVs replaced by video files, which needs a video decoder.

An extractor for the disc's own files (`pe2 --extract`: images to PNG, sounds
to WAV) comes with the kinds that need it (5 onwards).
