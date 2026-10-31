# Modding

PE2PC can show mods: folders of your own files that change what the game
shows, without touching the game's data or its logic. This page explains how
to make one, and the design the mod support follows as it grows.

Mods are an **experimental feature**: they load only with experimental
features on (the launcher's Settings, General, Experimental; or
`[general] experimental = true` in `pe2.toml`), and the launcher shows its
Mods screen only then. `PE2_MODS=1` turns mods on for one run regardless.

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
and opens the mods folder. Changes apply when the game starts; its log lists
the mods it uses.

## Extracting the game's files

Everything a mod replaces is extracted from your discs with a command, run
in a terminal from the folder `pe2.exe` is in:

```text
pe2 --extract
```

It reads the discs `pe2.toml` names (or the images given after
`--extract`) and saves what it finds in `dumps/` in the data folder: the
files, pictures, textures, songs, movies and texts (see each kind below).
It takes a minute or two. To save some of them only, name them, separated
by commas: `files` (the discs' files, and the executable), `backgrounds`,
`textures`, `music`, `movies` and `texts` (the dialogue and the menus'
texts); `all` is the default:

```text
pe2 --extract textures,backgrounds [disc1.chd disc2.chd]
```

## Backgrounds and full-screen pictures

Parasite Eve II's rooms are pre-rendered pictures of 320x240 pixels, and its
title and menu screens are full-screen pictures too. A mod can replace any of
them with a picture of its own, at any resolution: 1280x960 (4x) suits most
setups, and the picture is scaled to the internal resolution.

1. Get the pictures: the extract (`pe2 --extract`, see above) decodes all of
   them from the discs, about 1,760 (rooms, title and menu screens, close-ups
   shown when examining things), as the game does, into
   `dumps/backgrounds/<hash>.png` in the data folder.
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
- **Any resolution.** Replacements show at any internal resolution.
- **Loading.** A picture is read the first time its room shows (the original
  shows for a frame or two meanwhile) and kept while it is in use.

## Textures

Models (Aya, enemies, weapons), the interface (fonts, the HUD, menus) and the
pieces of rooms drawn over backgrounds are drawn from textures, and a mod can
replace them at a higher resolution. Replaced textures (and a mod's dialogue
font) are drawn at internal resolutions above 1 only: at the console's own,
they would be shrunk to its texels, worse than the game's.

The game's textures are stored in its `.pe2img` pictures as sheets: columns
of texels (256 wide at 4 bits per texel, 128 at 8 bits, up to 256 tall),
each one asset. A sheet is known by a hash of its contents, the same in
every room and whatever else is loaded with it.

Most sheets are 4- or 8-bit: their texels are not colours but indices into
a palette (a CLUT), and the game picks the palette each time it draws, so
one sheet is often drawn in several: the UI font's fill and its outlines,
the dialogue text and the speakers' labels, an enemy's colour variants. A
mod can replace a sheet in two ways:

- **Indexed** (`<sheet>-<4|8>bit.idx.png`, below): the replacement keeps
  indices too, and the game's palettes colour them as they colour the
  original, whichever palette each primitive is drawn with. This is the
  way for 4- and 8-bit sheets.
- **In colours** (`<sheet>.png`, `<sheet>-<palette>.png`, further below): a
  picture with its colours fixed, for one palette or any. The way for
  15-bit (direct colour) sheets, and what earlier mods use.

An indexed replacement wins where both exist for a sheet.

### Indexed replacements

1. Extract (`pe2 --extract` or `pe2 --extract textures`, see above): every
   4- and 8-bit sheet, about 2,090, is saved in `dumps/textures-indexed/`
   at its own size, as indices, at both depths:
   `<sheet>-4bit.idx.png` and `<sheet>-8bit.idx.png`, each with its header
   (`.idx.toml`). The game may draw one sheet at either depth (the title's
   8-bit picture is also the menus' 4-bit frames); the header's
   `depth_source` says which depth the game is known to draw it at
   ("drawn so: ...") and which is the other one.
2. Make it larger, in one of two ways:
   - **With an upscaler (AI or other), in bulk.** An upscaler cannot work
     on indices themselves (an index between two others is another palette
     entry, not a colour between them), so it enlarges a picture of the
     sheet, and PE2PC turns the result back into indices. Two pictures of
     each sheet are saved for that:
     - `dumps/textures-ramp/<sheet>-<4|8>bit.png`, the sheet as a ramp:
       each index its own grey (17 apart at 4 bits). Best for fonts, the
       interface and anything drawn with several palettes, whose colours
       may look alike in each palette (the UI font's fill and outline do).
     - `dumps/textures/<sheet>.png` (or `-<palette>.png`), the sheet in
       colours. Best for pictures (rooms' pieces, characters) whose
       palette tells its indices apart, where an upscaler does better on
       natural colours.

     Enlarge the ones you want by a whole factor (4x: a 256 x 256 sheet to
     1024 x 1024), keep their names, and put them in one folder. Then:

     ```text
     pe2 --indexed-from-upscale my-upscaled-folder mods/my-mod/textures
     ```

     Each enlarged pixel becomes, of the indices of its own texel and the
     eight around it in the original, the one whose colour (or grey) is
     nearest its own; a tie keeps its own texel's. The upscaler decides
     where edges go; the indices stay the sheet's, even where a palette
     repeats a colour, and no index appears that was not near.
   - **By hand.** Enlarge by a whole factor, each texel repeated, so its
     values stay the same indices, and draw your detail into it with the
     indices the sheet uses (a pencil on a grey or indexed PNG, no
     antialiasing):

     ```text
     pe2 --indexed-scale dumps/textures-indexed/7f1ed755e0e1401a-4bit.idx.png 4 mods/my-mod/textures
     ```
3. Check it (and any you made yourself):

   ```text
   pe2 --indexed-check mods/my-mod/textures/7f1ed755e0e1401a-4bit.idx.png
   ```

The game draws it wherever it draws the sheet at that depth: each
replacement texel's index is looked up in the CLUT the game set for the
primitive, in the shader, as the console looks up the original's. Index
colour 0 is transparent, the colour's bit 15 still decides
semi-transparency, and the primitive's blending, shading (its vertex
colours) and texture window apply as before.

The format, version 1. `<sheet>-<4|8>bit.idx.png` holds one value per
texel: a grey or indexed PNG of 8 bits per pixel or fewer, without alpha;
its stored values are the indices (no palette, gamma or colour conversion
is applied when it is read). Rows go top to bottom, texels left to right:
the sheet's texel (u, v) is the block at (u, v) times the scale. Its
header, `<sheet>-<4|8>bit.idx.toml`:

| Field | Value |
|---|---|
| `format` | `"pe2pc-indexed-texture"` |
| `version` | `1` |
| `sheet` | the sheet's hash, 16 hexadecimal digits, as in the file name |
| `depth` | `4` or `8`, as in the file name |
| `native_width`, `native_height` | the sheet in texels: 256 (4-bit) or 128 (8-bit) wide, 1 to 512 high |
| `width`, `height` | the PNG's size: the native size times one whole factor (the same on both sides), at most 2048 |
| `layout` | `"sheet-rows"` |
| `depth_source` | optional, written by the extract, not read |

A replacement is refused (logged, the original drawn) when any of these is
off, when a 4-bit file holds a value above 15, or when the game draws the
sheet at the other depth or another size than the header says.

What to know:

- **Nearest only.** Indices are not filtered: the bilinear option smooths
  the game's own textures, not indexed replacements. The detail is what
  you draw at the higher resolution.
- **Checked against the game.** The UI font's ramp, enlarged 4x (with a
  smooth resampler standing in for an upscaler) and turned back into
  indices, draws clean letters with the game's fill and outline colours;
  its colour preview, made larger the same way, did not (in each of the
  font's palettes its fill and outline indices look alike, and the
  result was speckled): use ramps for such sheets. Unchanged and enlarged
  copies of the UI font, the menus' 4-bit frames and a dialogue page draw
  exactly as the originals do (screen captures compared pixel by pixel, at 4x internal
  resolution, in the Load Game screen and the first captions of a new
  game), and changed indices show in the game's own colours, in each of
  the palettes the sheet is drawn with. Semi-transparent passes (the
  captions' shadow) drew identically, but no test isolated a
  semi-transparent primitive by itself.
- **Hardware renderer.** As with every replacement, only the graphics card
  renderer draws them.

#### Text redrawn from fonts

The game's pictures of text are better redrawn than enlarged. The extract
writes a map of each in `dumps/textures-glyphs/<sheet>-<4|8>bit.toml`: its
`kind`, its `sheets` (side by side) and each glyph's `cell` (`[u, v,
width, height]` in texels) and `text` (`{name}` for an icon or logo, empty
when not known); the UI font's also give each glyph's `face` and
`baseline` (the cell row its letters stand on). The kinds:

| `kind` | What |
|---|---|
| `ui-font` | the menus' font: three faces (medium, large, small), from the executable's metrics |
| `loading` | the loading screen's captions (date and place), and the M.I.S.T. briefing's copy of their font: two maps |
| `title` | the title screen: PRESS START BUTTON, the copyright and logo, New Game, Continue, Option; its `palette` file under the dumps |
| `dialogue` | each dialogue glyph page (the `l` glyph also draws capital I); a mod's font does this better, see Dialogue |
| `disc-prompt` | the disc change's lines: Change to DISC 1, Change to DISC 2, Wrong DISC |

PE2PC already redraws the menus' font (`ui-font`), the loading captions
and the disc change's lines from its built-in fonts by default (`[video] hd_fonts`, at scales above 1); a mod's indexed
replacement of the same sheet wins. To use other fonts,
`tools/hd_fonts.py` redraws the first three from fonts of your choice
(needs Python with numpy, pillow and scipy):

```text
python tools/hd_fonts.py ui      <dumps> DejaVuSans-Bold.ttf mods/my-mod/textures
python tools/hd_fonts.py loading <dumps> DejaVuSerif.ttf mods/my-mod/textures
python tools/hd_fonts.py title   <dumps> CourierPrime-Regular.ttf CourierPrime-Bold.ttf mods/my-mod/textures --serif DejaVuSerif.ttf --logo logo.svg
```

Each glyph is drawn 8 times finer (`--factor`) in its cell, in the indices
the game's glyphs use, so the game's palettes still colour it; `--weight`
makes strokes thicker or thinner (in the game's pixels). The UI font's
letters share one scale per face and stand on their records' baselines,
as the game lays them out (pairs the game kerns together, as Wr or MM,
overlap as in the original). Without `--serif` or `--logo` the title keeps
those as they were, only smoother. A logo is drawn from an SVG's shapes
left of x = 320 (in its units: the Squaresoft wordmark's SQUARE); a
company's logo is its trademark, so keep such mods to yourself.

### Replacements in colours

1. Extract the game's files (`pe2 --extract`, see above): every sheet, about
   2,100, is saved whole in colours in `dumps/textures/`, with `index.txt`
   listing each one, where it comes from and where its colours do. These
   pictures are also the way to look at the sheets: the indexed ones are
   numbers.

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
   - **Dialogue fonts**: every page of glyphs a room's dialogue uses,
     wherever it loads, with the palettes the game's code draws it with
     (the speakers' name labels with theirs).
   - **Rooms' own screens** (computer terminals, maps): with the palettes
     the room's code draws them with.
   - **The credits**: as the bonus screens after the ending draw them.
   - The rest (7 sheets: pieces of pictures the game's code is not found to
     draw) comes out in grey, each texel by its number.
2. Make your version, at any multiple of the sheet's size (4x: 1024 wide
   for a 256-texel sheet), and save it as
   `mods/<your mod>/textures/<same name>.png`.

What to know:

- **Palettes.** A sheet's parts may be drawn with different palettes (an
  enemy in two colours): `<sheet>.png` is drawn whichever the palette.
  `<sheet>-<palette>.png` (with a palette's hash) replaces it for that
  palette only. The UI font (`stage0/file1/0#0`, 4-bit) is saved once per
  palette it is drawn with: the four fill palettes and the three outline
  ones the game loads from its executable (`textUploadPalettes`).
- **Shape.** A picture whose proportions do not match the sheet at the
  depth the game draws it is refused (logged with the expected size): the
  original shows.
- **Depth.** Whether a sheet's texels are 4 or 8 bits comes from the
  palette or the sprites that colour it; for the grey ones the extract
  guesses (a wrong guess shows as noise, at the other width). With
  `PE2_TRACE=textures` the game's log names each sheet as it is drawn, with
  its depth.
- **Transparency.** Your picture's transparency is used: pixels less than
  half opaque are not drawn. Whether the rest blends (semi-transparency)
  still follows the game's own texture. A picture in colours fixes one
  palette: where the game draws the sheet with another (a font's outline
  pass, a colour variant), use `<sheet>-<palette>.png` per palette, or an
  indexed replacement.
- **Hardware renderer.** As with backgrounds, replacements show with the
  graphics card renderer.

## Music

Parasite Eve II's music is sequenced: the game plays MIDI-like songs on the
sound chip's voices, one song at a time, each known by a sequence id. A mod can
replace a song with a recording of its own (OGG Vorbis, FLAC or WAV, any sample
rate, mono or stereo):

1. Extract the game's files (`pe2 --extract`, see above): every song, 69 in
   all, is played by the game's own sequencer and saved as
   `dumps/music/<id>.wav` in the data folder, up to where its loop repeats and
   a few seconds more. Next to it, `<id>.txt` holds the song's loop as
   `music.toml` (below) takes it, in seconds of that `.wav` (precise to a
   frame, 1/60 s, as the game places notes); songs that end have `looping =
   false`.
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

1. Extract the game's files (`pe2 --extract`, see above): every movie the game
   lists, 28 in all, is read from the discs into `dumps/movies/`. A movie is
   known by a hash of its first frame: `<hash>.str` is the movie itself, as
   on the disc (video and sound), `<hash>.png` shows a picture of it,
   `<hash>.wav` is its sound (when it has its own), and `<hash>.txt` says its
   length in frames and seconds, its frame rate, its size and where it is
   listed.
2. Save your video as `mods/<your mod>/movies/<same hash>.mp4`.

The video must not use B-frames (the decoder does not support them). With
ffmpeg:

```
ffmpeg -i my-movie.mkv -c:v libx264 -bf 0 -pix_fmt yuv420p -c:a aac -b:a 192k <hash>.mp4
```

The game still plays its own movie underneath: the video is shown in its
place, at the time the game's frame gives (so it keeps pace with it, and
stops when the game's movie ends or is skipped), and its sound replaces the
movie's. Make it as long as the original (the `.txt` says how long).

ffmpeg reads the `.str` files, so the original movies can be converted as
they are, to start a remaster from (ffmpeg's "I/O error" at their end is
only it reaching the end):

```
ffmpeg -i <hash>.str -c:v libx264 -bf 0 -pix_fmt yuv420p -c:a aac <hash>.mp4
```

## Game files

Everything else the game reads from the disc (models, textures, sound banks,
room packages, menu texts, dialogue) lives in its own file system: the files
of `STAGE0.CDF` and the folders of `STAGE1.CDF` to `STAGE5.CDF`. A mod can
replace any of them.

1. Extract the files: `pe2 --extract` (see above). They go to `dumps/files/`
   in the data folder (about 130 MB; both discs share them), every room
   background as a picture to `dumps/backgrounds/`, every texture sheet to
   `dumps/textures/`, every song to `dumps/music/`, every movie to
   `dumps/movies/` (see above), and the dialogue as text to `dumps/text/` (see
   below).
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

### A translation from another version's discs

The extract reads the dialogue of translated discs too (the Brazilian
Portuguese fan translation, for one): their glyph pictures are the game's
own letters framed otherwise in their cells, plus the letters they add,
and their cells may hold whole syllables, with the space between words in
the cell; glyphs are recognized by their shape (the game's letters, and
the translation's accented ones, labelled by eye), and a cell of several
letters is read letter by letter. Cells that point at no glyph (left by
editing the discs by hand) stay `{gN}`.

`tools/import_dialogue.py` makes a translation mod of it: extract the
game's discs and the translated ones, each with its own data folder, and

```text
python tools/import_dialogue.py <game's dumps/text> <translated dumps/text> mods/my-translation --name "My translation"
```

puts each translated line into the `text` of the game's line in the same
place (same file, same order: translated files' records are moved), where
both have the same controls. Translators may have dropped a line (an empty
one holding only a camera change, say) or added one, so the lines are
paired by their shape: controls, the distance to the next record, line
breaks, closing punctuation, numbers. `report.txt` lists what was left out
(lines with no counterpart, lines whose controls differ) and lines with
glyphs not read (shown as `?`). Drawn in a mod's font (below), any accent
shows.

### Dialogue in your own font

PE2PC draws the dialogue from a high-definition font by default, its own
DejaVu Sans Bold (`[video] hd_fonts`, on unless turned off). A mod's
`fonts/captions.ttf` (or `.otf`) replaces that font for all the dialogue,
its translations included (and then any character the font has, accents
and all, with no marks drawn by PE2PC). The highest-priority enabled mod
that has one is used. An optional `fonts/captions.toml` tunes it:

```toml
cap_height = 9.0   # a capital's height in the game's pixels (the game's: 9)
tracking = 0       # pixels added between letters (may be negative)
weight = 0.0       # pixels the strokes grow (negative: thinner)
```

Each dialogue file the game loads gets a copy whose lines use glyphs drawn
from the font, added to its glyph picture below the original ones, in the
same palette entries as the game's glyphs (a dark outline a pixel wide,
the fill, the shades between at the edges). So the game itself still lays
the lines out (with the font's widths: lines are centred on them, and
their box fits them), colours them, draws their shadow and reveals them
letter by letter. The glyphs are drawn twice: at the game's resolution,
into the glyph picture, for any renderer and scale; and 8 times finer,
which the hardware renderer draws instead, so the text is sharp at any
scale.

Which character each of the game's glyphs is comes from the discs,
collected in the seconds after the game starts (a dialogue loaded before
that shows the original until loaded again). Glyphs not recognized and
speakers' names stay the game's pictures. Lines take the font's widths, so
a wide font may need `tracking` or a smaller `cap_height` to keep long
lines inside the screen.

For mods that redraw the glyph pictures instead (indexed replacements of
the pages), the extract also maps each dialogue glyph page: see "Text
redrawn from fonts" under Textures.

## Texts of the menus

The menus, items, Parasite Energy, places, options, shops and memory card
messages are texts in the game's code (the executable and its code
packages), in the menus' font's encoding: ASCII, the accented letters of
Latin-1, and `\` codes (`\B9`, `\Sm`: columns, faces, offsets; `}` `{`
`~` `|` draw the pad's buttons). The extract lists them in
`dumps/strings/<file>/<load address>.toml`, each text once (where it is
first found):

```toml
[[string]]
at = 0x800FC16C            # where it is, for you
original = '''
Customize game settings.'''
text = ''
```

Finding them is a guess (runs of letters ended by a zero), so a few entries
are bytes of code that happen to read as letters: leave those empty.

Items' details (Triangle on an item: specifications and the long
description) are text files on the disc, one for each item; the extract
lists each as `dumps/strings/<file>/txt-<chunk>.toml`, the whole file in
one entry. The game reads a translated one in place of the disc's, so it
may have any number of lines (its first five are the specifications'
rows, and `\Z` ends it).

### Translating them

Copy the files into `mods/<your mod>/strings/` (any folders inside) and
write each translation in its `text`; empty ones keep the original. The
game reads only `original` and `text`, so a file of your own with entries
from anywhere works as well. A translation applies wherever the game uses
that exact text.

- Keep the `\` codes and the buttons where they belong.
- Keep the number of lines. An item's text is fields, one per line (its
  name, its description's lines, then the same unidentified), and the game
  draws a field from the middle of the text; a translation with another
  number of lines is used for its first line only.
- A line holds up to 63 characters (the game copies lines into buffers of
  64 bytes); longer ones are cut. Menus and boxes keep their sizes, so
  translations longer than the original may spill out of them.
- The font has the accented letters of Latin-1 (á à â ä é è ê ë í ì î ï ó
  ò ô ö ú ù û ü ç ñ ¿ ¡ and capitals), and PE2PC adds ã õ Ã Õ: made from
  the font's own â ô Â Ô with a tilde, and drawn from the high-definition
  font above the console's resolution. The menus' small face (panels'
  titles, Lv., some labels) has no accents at all: there, accented letters
  are drawn plain (SELEÇÃO as SELECAO). Typographic quotes, dashes and the
  ellipsis become the plain ones; any other character shows as `?`.
- Texts the game puts together (`Obtained` + an item's name + `.`) are
  translated piece by piece, in the original's order.

The added letters' cells are in the menus' font's glyph map (see "Text
redrawn from fonts"), with `like`, the cell of the letter each is made
from: a mod redrawing that sheet should draw them there.

### From another version's discs

`tools/import_strings.py` makes a translation mod of the menus' texts of
translated discs: extract the game's discs and the translated ones, each
with its own data folder (`pe2 --extract files,texts` is enough: the
importer reads the code from `dumps/files` and the words of the dialogue
from `dumps/text`), and

```text
python tools/import_strings.py <game's dumps> <translated dumps> mods/my-menus --name "My menus"
```

Translations of discs rewrite the texts in place and repoint what uses
them, so each text is found where the translated version's data and code
point instead of the original (texts nothing points at, between their
neighbours). Their bytes are read through the translated version's own
font: such translations draw the letters they need, and whole syllables,
over codes the menus did not use, a different set in each of the font's
three faces. A code reads as the game's letter of the same picture in the
same face, or as a picture labelled in the tool's `LABELS` (the Brazilian
Portuguese fan translation's are there); each text is read in the face
where the most codes are known and the most words are the translation's
own (from its dialogue, `dumps/text`). `report.txt` lists what was left
out, and pictures not known are saved in `report-glyphs/` to be labelled.

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
6. **Dialogue** (done): extracted as text and translated, as above, and
   the texts of the menus. Still to come: speakers' names.
7. **Textures** (done): extracted from the disc and dumped in colour, and
   replaced at any resolution, by sheet, as above.
8. **Movies** (done): FMVs replaced by MP4 files, as above.

The extract saves files as the game stores them. Converting them to common
formats (pictures to PNG, sound banks to WAV) and back is left to the
decompilation's tools for now.
