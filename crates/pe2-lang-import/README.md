# pe2-lang-import

Play [PE2PC](https://github.com/Snaity-CORE/pe2-pc-port), the PC port of Parasite Eve II, in French, German, Spanish, Italian or Japanese.

PE2PC plays the USA discs. This program reads your own discs of another release and turns them into a PE2PC mod with that release's:

- dialogue and speakers' names;
- menus, items and their descriptions;
- pictures of text (maps, signs, the loading captions' font);
- loading captions;
- the two movies with text (the title menu's and the prologue's).

From the Japanese release, it takes the dialogue and the three movies with text (also Dryfield's arrival). The menus, items and loading captions stay in English.

Nothing from the game comes with this program. Everything is read from your discs.

## What you need

- PE2PC, set up with your Parasite Eve II (USA) discs. pe2-lang-import comes with it: `pe2-lang-import.exe` is next to `pe2.exe` in PE2PC's zip.
- Your own dumps of another release, discs 1 and 2, as `.chd` or `.cue` images:

  | Language | Release     |
  |----------|-------------|
  | French   | SLES-02559  |
  | German   | SLES-02560  |
  | Spanish  | SLES-02561  |
  | Italian  | SLES-02562  |
  | Japanese | SLPS-02480/81 |

- About 2 GB of free space for the time of the import (it is removed afterwards).

Like PE2PC, it is a Windows program. On Linux and the Steam Deck, it runs through Proton next to PE2PC (see PE2PC's [Steam Deck and Linux](https://github.com/Snaity-CORE/pe2-pc-port/blob/main/docs/STEAM_DECK.md)).

## How to use it

1. In PE2PC's launcher, open **Mods** and click **Import a language from European discs...**. You can also run `pe2-lang-import.exe` directly.
2. Choose your discs of the other release. The USA discs come from PE2PC's settings.
3. Click **Make the mod**. The import takes a few minutes.

The mod goes to PE2PC's `mods` folder and is turned on. Mods need **Experimental features (mods)** turned on in PE2PC's **Settings > General**. To get PE2PC's own screens (the launcher, the pause menu, Options) in your language too, choose it in **Settings > General > Language**.

Only keep one language mod turned on at a time.

## From a terminal

```
pe2-lang-import run [--pe2 <pe2.exe>] --usa <disc 1> <disc 2> --release <disc 1> <disc 2>
pe2-lang-import import <USA dumps> <release's dumps> <mod folder> [--name NAME]
```

- `run` does what the window does.
- `import` makes the mod from two extracts you already have. Each extract is `pe2 --extract files,textures,texts` run in its own data folder (`PE2_HOME`).

## How it works

The European releases were rebuilt from the same sources, so their code and data sit at other addresses. Their fonts give letters other codes.

For each part of the mod, the import does this:

- **Dialogue:** lines are paired file by file, in order (an alignment on their controls, layout and punctuation).
- **Menus:** each text is found where the European code points. Its letters are read through that release's own font, and matched against its own texts.
- **Items:** paired in order, by their names' models and numbers.
- **Loading captions:** each record is shown with its European translation. The captions shown when loading a save are left in English on the European discs, so they are made from a translated date and the place names in `res/<language>/places.toml`.

The speakers' names (`res/<language>/speakers.toml`) are pictures on the discs, written out by hand.

The Japanese release draws each room's dialogue with a glyph picture of its own, holding only the kana and kanji that room needs. PE2PC's extract cannot read these.

- `res/ja/pages.toml` gives each dialogue file's glyph picture.
- `res/ja/glyphs.toml` gives each glyph's character, keyed by the hash of its picture. The 1775 glyphs were read once, by OCR and by eye.
- The lines are put into the game's lines in the same place, file by file. A file with a different number of lines, or a line with other choices, stays in English.
- PE2PC draws them with the mod's font: [Noto Sans JP](https://github.com/notofonts/noto-cjk) Bold, under the SIL Open Font License (`res/ja/OFL.txt`).

## Building

From the root of PE2PC's repository, as PE2PC (see [docs/BUILDING.md](../../docs/BUILDING.md)):

```
cargo build --release --target i686-pc-windows-msvc -p pe2-lang-import
```

Its tests: `cargo test -p pe2-lang-import`.

## License

CC0 1.0, like PE2PC, except Noto Sans JP (`res/ja/NotoSansJP-Bold.otf`): SIL Open Font License 1.1, see `res/ja/OFL.txt`.
