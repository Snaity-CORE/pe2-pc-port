#!/usr/bin/env python3
"""Write the tables PE2PC reads the game's pictures of text by
(crates/pe2-host/src/mods/glyphs/): for each picture, a hash of it and the
text it shows. Only hashes and texts: nothing of the discs.

- captions.rs: the loading captions' font (glyphmaps.rs `loading`): the
  game's letters, by their place in `gTextCaptionGlyphCells`, in both its
  copies; and each translation's letters drawn anew, by its labels.
- menus.rs: the menus' font (glyphmaps.rs `ui_font`), per face: the game's
  letters by their codes, and each translation's syllables and letters, by
  its labels.
- speakers.rs: each translation's speakers' name labels, matched to the
  game's in the same lines and named as its labels say.

Each translation is named by its labels, tools/glyph_labels/<name>.toml,
and given by its discs' dumps (`pe2 --extract files,textures,texts`, each
version in a data folder of its own, PE2_HOME):

    python tools/gen_glyph_tables.py <game's dumps> [<name>=<dumps> ...]
    python tools/gen_glyph_tables.py usa/dumps ptbr=ptbr/dumps
"""

import argparse
import hashlib
import os
import sys
import tomllib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from import_strings import CAPTIONS, FACES, png_indices  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, 'crates', 'pe2-host', 'src', 'mods', 'glyphs')
LABELS = os.path.join(ROOT, 'tools', 'glyph_labels')
EXE_BASE = 0x80010000 - 0x800

# The captions' cells (main/caption_draw.c), in order, and the font's
# copies: where the extract finds each and how far down its sheet the
# cells are drawn (glyphmaps.rs `loading`).
CAPTION_TEXT = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890:;',.?!"
CAPTION_COPIES = [('stage0/file10/0#0', 0), ('stage1/1801/file0/1#0', 48)]
UI_SHEET = 'stage0/file1/0#0'
FACE_INDEX = {face: n for n, (face, *_) in enumerate(FACES)}


def fnv1a(values):
    h = 0xCBF29CE484222325
    for b in values:
        h = ((h ^ b) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


class Dumps:
    def __init__(self, path):
        self.path = path
        self.exe = open(os.path.join(path, 'files', 'SLUS_010.42'), 'rb').read()
        self.sheets = {}
        for line in open(os.path.join(path, 'textures', 'index.txt'), encoding='utf-8'):
            parts = line.split()
            if len(parts) > 1:
                # <folder>/<sheet>[-<palette>]
                self.sheets.setdefault(parts[1], parts[0].split('/')[-1].split('-')[0])

    def rows(self, origin):
        sheet = self.sheets[origin]
        return png_indices(os.path.join(self.path, 'textures-indexed', f'{sheet}-4bit.idx.png'))

    def words(self, address, n, size):
        at = address - EXE_BASE
        return [self.exe[at + k * size:at + (k + 1) * size] for k in range(n)]

    def labels(self):
        """Each name label's place -> (its picture's hash, the name read)."""
        out = {}
        for line in open(os.path.join(self.path, 'text', 'labels.tsv'), encoding='utf-8'):
            if line.startswith('#'):
                continue
            key, chunk, cap, ln, picture, name = line.rstrip('\n').split('\t')
            out[(key, chunk, cap, ln)] = (int(picture, 16), name)
        return out


def crop(rows, u, v, w, h):
    return tuple(tuple(rows[v + y][u:u + w]) for y in range(h) if v + y < len(rows))


def caption_hash(picture, w, h):
    return fnv1a([w, h] + [t for row in picture for t in row])


def captions(game, translations):
    table = {}
    for origin, down in CAPTION_COPIES:
        rows = game.rows(origin)
        for (u, v, w, h), text in zip(game.words(CAPTIONS[0], len(CAPTION_TEXT), 4), CAPTION_TEXT):
            table.setdefault(caption_hash(crop(rows, u, v + down, w, h), w, h), text)
    game_pictures = set(table)
    for name, dumps, labels in translations:
        for origin, down in CAPTION_COPIES:
            rows = dumps.rows(origin)
            for u, v, w, h in dumps.words(CAPTIONS[0], len(CAPTION_TEXT), 4):
                if w == 0 or h == 0:
                    continue
                for off in (down, down + 1, down - 1):
                    picture = crop(rows, u, v + off, w, h)
                    key = hashlib.sha1(repr(picture).encode()).hexdigest()[:12]
                    picture_hash = caption_hash(picture, w, h)
                    if picture_hash in game_pictures or key not in labels.get('captions', {}):
                        continue
                    table.setdefault(picture_hash, labels['captions'][key])
    return table


def ui_text(face, code, v):
    """What the game's menus' font shows for `code` in `face`: Latin-1 and
    the pad's buttons; the small face's records past row 34 point into the
    medium face's cells."""
    icons = {0x24: 'circle', 0x25: 'square', 0x26: 'triangle', 0x2A: 'cross'} if face == 'small' else \
        {0x7B: 'circle', 0x7C: 'triangle', 0x7D: 'cross', 0x7E: 'square'}
    if code in icons:
        return '{' + icons[code] + '}'
    c = chr(code)
    if face == 'small':
        ok = v < 35 and ((c.isascii() and (c.isupper() or c.isdigit())) or c in '.,()!?')
        return c if ok else None
    special = {0x8C: 'Œ', 0x9C: 'œ', 0x97: '—'}
    if code in special:
        return special[code]
    if code in (0xA1, 0xAB, 0xB0, 0xBB, 0xBF) or 0xC0 <= code <= 0xFF:
        return c
    return c if (c.isascii() and c.isalnum()) or c in '!"%&\'()+,-./:?#' else None


def ui_glyphs(dumps):
    """Each face's records: (face, code, v, the picture's hash, its key)."""
    rows = dumps.rows(UI_SHEET)
    for face, table, count, bias in FACES:
        for k, record in enumerate(dumps.words(table, count, 12)):
            u, v, w, h = record[:4]
            v = (v + bias) & 255
            if w == 0:
                continue
            picture = tuple(rows[v + y][u:u + w + 1] for y in range(h + 1) if v + y < len(rows))
            key = f'{face}-' + hashlib.sha1(repr(picture).encode()).hexdigest()[:12]
            texels = [FACE_INDEX[face], w + 1, h + 1] + [t for row in picture for t in row]
            yield face, 0x20 + k, v, fnv1a(texels), key


def menus(game, translations):
    table = {}
    for face, code, v, picture_hash, _ in ui_glyphs(game):
        text = ui_text(face, code, v)
        if text is not None:
            table.setdefault(picture_hash, text)
    for name, dumps, labels in translations:
        for face, code, v, picture_hash, key in ui_glyphs(dumps):
            text = labels.get('menus', {}).get(key)
            if text is not None and picture_hash not in table:
                table[picture_hash] = text
    return table


def speakers(game, translations):
    """A translation's label pictures the game's discs do not have, named
    by the game's label in the same places (a name: `[NAME]`, or one its
    labels translate, No.9), the name most of those places give."""
    table = {}
    ours = game.labels()
    game_pictures = {picture for picture, _ in ours.values()}
    for name, dumps, labels in translations:
        names = labels.get('speakers', {})
        votes = {}
        for place, (picture, _) in dumps.labels().items():
            if place not in ours or picture in game_pictures:
                continue
            read = ours[place][1]
            spoken = read.strip('[]').strip()
            if not (read.startswith('[') or spoken in names) or not spoken:
                continue
            text = f'[{names.get(spoken, spoken)}]'
            votes.setdefault(picture, {}).setdefault(text, 0)
            votes[picture][text] += 1
        for picture, counts in votes.items():
            table.setdefault(picture, max(counts, key=counts.get))
    return dict(sorted(table.items(), key=lambda kv: (kv[1], kv[0])))


def write(name, doc, table):
    def quoted(t):
        return '"' + t.replace('\\', '\\\\').replace('"', '\\"') + '"'
    lines = ['// GENERATED by tools/gen_glyph_tables.py: do not edit.', '//']
    lines += [f'// {line}' if line else '//' for line in doc.split('\n')]
    lines += ['', 'pub const TABLE: &[(u64, &str)] = &[']
    lines += [f'    (0x{h:016X}, {quoted(t)}),' for h, t in table.items()]
    lines += ['];', '']
    with open(os.path.join(OUT, f'{name}.rs'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(lines))
    print(f'{name}.rs: {len(table)} pictures')


def main():
    p = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    p.add_argument('game')
    p.add_argument('translations', nargs='*', metavar='name=dumps')
    args = p.parse_args()
    game = Dumps(args.game)
    translations = []
    for t in args.translations:
        name, _, path = t.partition('=')
        labels = tomllib.load(open(os.path.join(LABELS, f'{name}.toml'), 'rb'))
        translations.append((name, Dumps(path), labels))
    names = ', '.join(n for n, _, _ in translations) or 'none'
    write('captions', "The loading captions' font's pictures: the game's letters, and the\n"
          f"translations' ({names}).", captions(game, translations))
    write('menus', "The menus' font's pictures, per face: the game's letters, and the\n"
          f"translations' syllables and letters ({names}).", menus(game, translations))
    write('speakers', "The speakers' name labels the translations draw, named as they show\n"
          f"them ({names}); the game's own are dialogue.rs GLYPHS.", speakers(game, translations))


if __name__ == '__main__':
    main()
