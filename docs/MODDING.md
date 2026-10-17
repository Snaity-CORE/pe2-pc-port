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

`[mods] enabled = false` in `pe2.toml` (or `PE2_MODS=0`) turns all mods off.
The game's log lists the mods it uses at startup.

## Backgrounds and full-screen pictures

Parasite Eve II's rooms are pre-rendered pictures of 320x240 pixels, and its
title and menu screens are full-screen pictures too. A mod can replace any of
them with a picture of its own, at any resolution: 1280x960 (4x) suits most
setups, and the picture is scaled to the internal resolution.

1. Turn on dumping: `[mods] dump = true` in `pe2.toml` (or `PE2_DUMP=1`).
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

## The plan

Mod support grows in this order. Each kind follows the same rules: found by a
hash or a name of what it replaces, never containing the game's files, never
changing the game's own data in memory unless the kind is meant to.

1. **Backgrounds and full-screen pictures** (done): dump and replace, as above.
2. **A Mods screen in the launcher**: list, turn on and off, order.
3. **Memory patches**: an address, a value and when to apply it (GameShark
   style), for cheats and fixes.
4. **Music and voices**: the XA audio tracks replaced by OGG or FLAC files,
   mixed by the sound chip's emulation.
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
and music to WAV) comes with the kinds that need it (4 and 5).
