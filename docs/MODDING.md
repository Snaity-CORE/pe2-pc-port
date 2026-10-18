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

1. Get the pictures: **Extract the game's files** in the launcher's Mods
   screen (or `pe2 --extract`) decodes all of them from the discs, about
   1,760 (rooms, title and menu screens, close-ups shown when examining
   things), as the game does, into `dumps/backgrounds/<hash>.png` in the
   data folder.
2. Make your version of a picture (paint over it, upscale it, ...) and save it
   as `mods/<your mod>/backgrounds/<same hash>.png`.

The file name is what ties a replacement to the picture it replaces: a 64-bit
FNV-1a hash of the picture's size and pixels exactly as the game sends them to
the graphics chip. Keep the name; the content can be anything.

What to know:

- **Foreground pieces.** Parts of a room that pass in front of the characters
  (pillars, railings, furniture edges) are drawn separately, from texture
  memory, on top of the background: they are textures (see below), to be
  replaced to match your background.
- **Changed pictures.** Some rooms change their picture a little before
  showing it again (a light turning on, the room's own frame kept behind a
  menu). Such a picture keeps your replacement, with what changed (in 16x16
  tiles) drawn over it from the game's picture, when no more than a quarter
  of it changed.
- **Only the picture changes.** The game still has its own picture in video
  memory; replacing it never changes how the game plays.
- **Hardware renderer.** Replacements show with the graphics card renderer at
  any internal resolution; with the software renderer the original shows.
- **Loading.** A picture is read the first time its room shows (the original
  shows for a frame or two meanwhile) and kept while it is in use.

## Textures

Models (Aya, enemies, weapons), the interface (fonts, the HUD, menus) and the
pieces of rooms drawn over backgrounds are drawn from textures, and a mod can
replace them at a higher resolution.

The game's textures are stored in its `.pe2img` pictures as sheets: columns
of texels (256 wide at 4 bits per texel, 128 at 8 bits, up to 256 tall),
each one asset. A sheet is known by a hash of its contents: its file name is
`<sheet>.png`, the same in every room and whatever else is loaded with it.

1. Extract the game's files (**Extract the game's files** in the launcher's
   Mods screen, or `pe2 --extract`): every sheet, about 2,100, is saved
   whole in `dumps/textures/`, with `index.txt` listing each one, where it
   comes from and where its colours do.

   A sheet's texels are numbers into a palette, and which palette colours
   them is chosen as the game draws. The extract finds it on the disc:
   - **Rooms' pieces** (what passes in front of the characters, cut from
     the backgrounds): for each camera, the game finds the sprites drawing
     them through its stage's map and the room's package, each sprite with
     its palette, and the extract finds them the same way. Every texel
     they draw is coloured as the game draws it, and each camera's sprites
     are checked against its background (the extract's log says how many
     differ). What no sprite draws is never shown, and is left transparent.
   - **Characters, enemies, weapons, items, maps**: their files carry their
     palettes, one per column of their pictures.
   - **Rooms' dialogue fonts**: with the palette the game's code draws them
     with.
   - The rest (about 55 sheets: cameras' pieces that no sprite table draws,
     drawn by rooms' own code; the credits) comes out in grey, each texel by
     its number.

   With dumping on, playing also colours sheets as the game draws them,
   which can fill in those.
2. Make your version, at any multiple of the sheet's size (4x: 1024 wide
   for a 256-texel sheet), and save it as
   `mods/<your mod>/textures/<same name>.png`.

What to know:

- **Palettes.** A sheet's parts may be drawn with different palettes (an
  enemy in two colours): `<sheet>.png` is drawn whichever the palette.
  `<sheet>-<palette>.png` (with a palette's hash) replaces it for that
  palette only.
- **Depth.** Whether a sheet's texels are 4 or 8 bits comes from its
  palette when its file has one; otherwise the extract guesses (right about
  94% of the time; a wrong guess shows as noise, at the other width).
  Playing corrects it: the dump is remade at the depth the game draws it
  with.
- **Transparency.** Your picture's transparency is used: pixels less than
  half opaque are not drawn. Whether the rest blends (semi-transparency)
  still follows the game's own texture.
- **Hardware renderer.** As with backgrounds, replacements show with the
  graphics card renderer; dumping needs it too.

## Music

Parasite Eve II's music is sequenced: the game plays MIDI-like songs on the
sound chip's voices, one song at a time, each known by a sequence id. A mod can
replace a song with a recording of its own (OGG Vorbis, FLAC or WAV, any sample
rate, mono or stereo):

1. Extract the game's files (**Extract the game's files** in the launcher's
   Mods screen, or `pe2 --extract`): every song, 69 in all, is played by the
   game's own sequencer and saved as `dumps/music/<id>.wav` in the data
   folder, up to where its loop repeats and a few seconds more. Next to it,
   `<id>.txt` holds the song's loop as `music.toml` (below) takes it, in
   seconds of that `.wav` (precise to a frame, 1/60 s, as the game places
   notes); songs that end have `looping = false`.
2. Save your recording as `mods/<your mod>/music/<same id>.ogg` (or `.flac`,
   `.wav`). The id is two hexadecimal digits, as the file's name has it: `26`
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

## Movies

The game's movies (FMVs) stream from the disc frame by frame. A mod can
show a video file instead: an MP4 with H.264 video and AAC sound, at any
resolution and frame rate.

1. Extract the game's files (**Extract the game's files** in the launcher's
   Mods screen, or `pe2 --extract`): every movie the game lists, 28 in all,
   is read from the discs into `dumps/movies/`. A movie is known by a hash
   of its first frame: `<hash>.png` shows a picture of it, `<hash>.wav` is
   its sound (when it has its own), and `<hash>.txt` says its length in
   frames and seconds, its frame rate, its size and where it is listed.
2. Save your video as `mods/<your mod>/movies/<same hash>.mp4`.

The video must not use B-frames (the decoder does not support them). With
ffmpeg:

```
ffmpeg -i my-movie.mkv -c:v libx264 -bf 0 -pix_fmt yuv420p -c:a aac -b:a 192k <hash>.mp4
```

The game still plays its own movie underneath: the video is shown in its
place, at the time the game's frame gives (so it keeps pace with it, and
stops when the game's movie ends or is skipped), and its sound replaces the
movie's. Make it as long as the original (the `.txt` says how long). The
decompilation's asset tools extract the original movies, for remastering
them.

## Game files

Everything else the game reads from the disc (models, textures, sound banks,
room packages, menu texts, dialogue) lives in its own file system: the files
of `STAGE0.CDF` and the folders of `STAGE1.CDF` to `STAGE5.CDF`. A mod can
replace any of them.

1. Extract the files: **Extract the game's files** in the launcher's Mods
   screen, or `pe2 --extract`. They go to `dumps/files/` in the data folder
   (about 130 MB; both discs share them), every room background as a
   picture to `dumps/backgrounds/`, every texture sheet to `dumps/textures/`,
   every song to `dumps/music/`, every movie to `dumps/movies/` (see above),
   and the dialogue as text to `dumps/text/` (see below).
2. Find the one you want. With `PE2_TRACE=files` the game's log names each
   file as it is read, so playing to a place shows which files it uses.
3. Put your version in `mods/<your mod>/files/` under the same path.

A file is a run of chunks, and the extract saves each chunk's contents as
`<file>/<chunk>.<type>`, for example `stage1/1501/file6/0.bs` (a room's
background) or `stage0/file20101/2.txt` (a weapon's description). A mod
replaces a chunk with a file of the same path; the rest of the file stays as
on the disc. `<file>.bin` instead replaces a whole file with its on-disc
bytes (chunk headers included), for tools that build files themselves.

| Type | What it is | Editing |
|---|---|---|
| `.txt` | Menu and item texts, with markup (`\Ss`, `\B9`, `\Z` ends) | Any text editor |
| `.bs` | A room background as the movie decoder reads it (MDEC) | A BS encoder |
| `.pe2img`, `.pe2clut` | Pictures and palettes for video memory, compressed | The decompilation's tools |
| `.pe2pkg` | Code and data packages (rooms, menus, actors), compressed | The decompilation's tools |
| `.pe2cap2` | Dialogue bundles: pictures, glyph tables and scripts | The decompilation's tools |
| `.spk` | Sound banks | The decompilation's tools |

The paths and the chunk contents are the same as those of the
decompilation's asset tools (`tools/peassets` in the decomp repository): the
extract matches their `raw/` store byte for byte, so they can decompress and
rebuild what it saves.

What to know:

- **Sizes.** A replacement can be larger than the original: the drive serves
  it in place of the original as the game reads it, so nothing on the disc
  moves. But the game loads each chunk at a fixed place in memory, and what
  follows that place may be in use: a text or package much larger than the
  original can overwrite other data. Keep sizes close to the originals unless
  you know the space is free.
- **Code.** This port runs the game's code natively: the code in a package
  is not run, and changing it keeps the package from being recognized (the
  game then stops). Packages' data can change.
- **Dialogue.** Spoken lines are not stored as text: see below.

## Dialogue

The game's lines are not stored as text but as glyph pictures: each
dialogue file (a CAP file, in a `.pe2cap2` bundle) has a table of glyphs
drawn from a picture of the letters its lines use, and its lines are
indices into that table. The extract turns them into text, by recognizing
each glyph picture, and saves each dialogue file's lines in
`dumps/text/<file>/<chunk>-<n>.toml`:

```toml
cap = "1f9d66fcd224f2dd"   # the dialogue file this is for

[[line]]
record = 0x117C            # the line, in that file
speaker = "BALDWIN"
original = '''
Agent Brea, what are you doing?
You're needed downtown now!'''
text = ''
```

In lines, `{_}` is a narrow space, `{color N}` changes the colour,
`{81NN}` to `{83NN}` start the choices of a question and other `{XXXX}`
are controls (camera changes, icons). A glyph that was not recognized shows
as `{gN}`.

All 8,400 lines come out, but for two files whose glyph picture was not
found (stage 1 folder 2101, stage 2 folder 401: 67 lines). Capital I and
small l are the same picture in the game's font; the extract tells them
apart by their place in words, which can be wrong now and then.

### Translating

1. Copy the files you translate from `dumps/text/` into
   `mods/<your mod>/text/` (any folders inside; the `cap` line tells which
   dialogue file each is for).
2. Write each line's translation in its `text`, keeping the `{...}` codes
   where they belong: a question's choices need their `{81NN}` codes, and
   line breaks are yours to place (a line holds about as much as the
   original's longest).

```toml
[[line]]
record = 0x117C
speaker = "BALDWIN"
original = '''
Agent Brea, what are you doing?
You're needed downtown now!'''
text = '''
Agente Brea, o que está fazendo?
Precisam de você no centro agora!'''
```

Lines left empty keep the original, and `original` and `speaker` are only
for you: the game reads `cap`, `record` and `text`.

When the game loads a dialogue file with translated lines, it gets a copy
with those lines' text, and the glyphs they need are added to its glyph
picture, below the original ones. They come from the game's own dialogue
font (collected from the disc in the few seconds after the game starts; a
dialogue loaded before that shows the original until loaded again), and
accented letters (á à â ã ä é è ê ë í ì î ï ó ò ô õ ö ú ù û ü ç ñ, and
capitals) are made from it with marks drawn by PE2PC. A character the font
does not have shows as `?`. Speakers' names stay as the game shows them:
they are pictures of whole names. `PE2_TRACE=text` logs each translated file
as it loads.

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

1. **Backgrounds and full-screen pictures** (done): extracted and replaced, as
   above.
2. **A Mods screen in the launcher** (done): list, turn on and off, order.
3. **Memory patches** (done): GameShark codes and plain writes, each frame.
4. **Music** (done): sequenced songs rendered by the extract, and replaced by
   recordings, as above. (Recorded audio, the movies' and cutscenes' XA tracks, comes with
   movies.)
5. **Game files** (done): extracted, and replaced chunk by chunk or whole,
   at any size, as above.
6. **Dialogue** (done): extracted as text and translated, as above. Still to
   come: speakers' names, and the texts held in the executable (menus).
7. **Textures** (done): extracted from the disc and dumped in colour, and
   replaced at any resolution, by sheet, as above.
8. **Movies** (done): FMVs replaced by MP4 files, as above.

The extract saves files as the game stores them. Converting them to common
formats (pictures to PNG, sound banks to WAV) and back is left to the
decompilation's tools for now.
