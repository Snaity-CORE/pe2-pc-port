#!/usr/bin/env python3
"""Make a translation mod of the texts of the menus from another version's
discs (docs/MODDING.md, "Texts of the menus").

Extract both versions (`pe2 --extract`, each with its own data folder):
for each text the game's `dumps/strings` lists, this finds the other
version's text in its place, where the other version's code and data
point instead (translations rewrite the texts and repoint their users),
or, for a text nothing points at, between its neighbours. The other
version's bytes are read through its own menus' font: each code is the
character whose picture the game's font has in the same shape, or a
picture labelled in LABELS (translations draw new letters and whole
syllables into the font). Writes `strings/**/*.toml`, `mod.toml` and
`report.txt` (texts left out, codes not known, with their pictures in
`report-glyphs/`) into the mod folder.

    python tools/import_strings.py <game's dumps> <other's dumps> <mod folder>
        [--name NAME] [--description TEXT]
"""

import argparse
import glob
import hashlib
import os
import re
import struct
import sys
import tomllib
import zlib

# The menus' font's faces: their glyph tables (main/textdraw.c,
# _gFontGlyphs*, records of 12 bytes from ' '), how many, and the V bias
# each adds. A text is drawn in one of them, and a translation may give a
# code another picture in each.
WORD = re.compile(r"[^\W\d_]+")
# A mark inside a word: a code read in the wrong face (SELE#"O).
ODD = re.compile(r"(?<=[^\W\d_])[^\w\s'.,!?&-]+(?=[^\W\d_])")
# A capital after a small letter: likely the same (IfNS), though names
# have some (FateKeeper).
CAMEL = re.compile(r"(?<=[^\W\d_A-ZÀ-Þ])[A-ZÀ-Þ]")
# A small letter after two capitals (JOg, ARk): the same, in the small
# face, whose small letters' codes are its own syllables.
SHOUTED = re.compile(r"(?<=[A-ZÀ-Þ]{2})[a-zß-ÿ]")
# The font's codes (\B9, \Sm, \Co, \Z): not words.
CODES = re.compile(r"\\[A-Za-z][A-Za-z0-9]?")

FACES = [('medium', 0x8005EFB0, 224, 38), ('large', 0x8005FA30, 224, 128), ('small', 0x800604B0, 91, 0)]

# Pictures of the other version's font that are not the game's letters,
# by the hash of their cell (`report-glyphs/<face>-<hash>.png`): what they
# read.
LABELS = {
    # The Brazilian Portuguese fan translation's discs: syllables and the
    # letters the font lacked, drawn over codes the menus did not use.
    'large-0158da9273d7': 'io',
    'large-1b050a518fbe': '—',
    'large-1baa457baa82': 'de',
    'large-406ec412273b': 'do',
    'large-4177e2eca2e8': 'ar',
    'large-5021140827b2': 'õ',
    'large-58cf6d2b598b': 'rer',
    'large-84f85202d0c3': 'Cor',
    'large-577ad7cf6e43': 'sa',
    'large-5b272cfcf7fe': 'se',
    'large-7de77b179639': 'co',
    'large-c9ca3031fa01': 'ã',
    'large-fcf46ec0b7f4': 'Ã',
    'medium-06090fce5b7d': 'ce',
    'medium-073462714d8d': 'as',
    'medium-0a45cc0f6186': 'õ',
    'medium-2e5267f6cb82': 'das',
    'medium-3360c1ba4dcb': 'tar',
    'medium-80c2b2ca3cc4': 'Ven',
    'medium-8595b6bd9b04': 'antia Máxima de',
    'medium-af41d3c4e548': 'ipal',
    'medium-294854820591': 'ta',
    'medium-523c043291e2': 'CIL',
    'medium-84207a359a11': 'Ã',
    'medium-b3fa43608c1a': 'Corre',
    'medium-bf30c4e5b981': 'ar',
    'medium-d01d717681d1': 'ci',
    'medium-f8d409fd8805': 'ã',
    'small-0d1d0d164209': 'MO',
    'small-13c3cd60c0a2': 'Õ',
    'small-2e38efd26773': 'CI',
    'small-321effbaf0d0': 'É',
    'small-395a8c4a9124': 'Ã',
    'small-40c6eaab8e5a': 'PE',
    'small-448f3375663e': 'Ç',
    'small-47c21d4f0948': 'Á',
    'small-4aa637aa4009': 'Ô',
    'small-5443738535a4': 'DE',
    'small-56531a57d369': 'DO',
    'small-678f16d138a9': 'BÔNUS',
    'small-73f347cc0fde': 'IRIDO',
    'small-7afb9f4aa994': 'LE',
    'small-81381c2a2407': 'GO',
    'small-9ab632be65c8': 'TA',
    'small-c213aa7d1a24': 'Ú',
    'small-a8cb4caec790': 'Ó',
    'small-b55af57150c5': 'EL',
    'small-cef2f54878a4': 'SO',
    'small-d9de80068f53': 'Í',
    'small-df79fc8c269a': 'MA',
    'small-ea98c3156252': 'SA',
    'small-f476a92e612c': 'TE',
}


def lzss(stream):
    """One LZSS stream (dialogue.rs `lzss`)."""
    ring, nxt, out = bytearray(256), 0, bytearray()
    pos, byte, mask = 0, 0, 0

    def bits(n):
        nonlocal pos, byte, mask
        v = 0
        for _ in range(n):
            if mask == 0:
                if pos >= len(stream):
                    raise EOFError
                byte, pos, mask = stream[pos], pos + 1, 0x80
            v = (v << 1) | (1 if byte & mask else 0)
            mask >>= 1
        return v

    try:
        while True:
            if bits(1):
                b = bits(8)
                ring[nxt], nxt = b, (nxt + 1) & 255
                out.append(b)
            else:
                offset = bits(8)
                if offset == 0:
                    break
                length, at = bits(4), (offset - 1) & 255
                for _ in range(length + 2):
                    b = ring[at]
                    ring[nxt], nxt = b, (nxt + 1) & 255
                    out.append(b)
                    at = (at + 1) & 255
    except EOFError:
        pass
    return bytes(out)


def png_indices(path):
    """An 8-bit greyscale PNG's rows (an indexed sheet's palette indices)."""
    data, at, idat = open(path, 'rb').read(), 8, b''
    while at < len(data):
        n, kind = struct.unpack('>I4s', data[at:at + 8])
        chunk = data[at + 8:at + 8 + n]
        if kind == b'IHDR':
            w, h = struct.unpack('>II', chunk[:8])
        elif kind == b'IDAT':
            idat += chunk
        at += 12 + n
    raw, rows, prev = zlib.decompress(idat), [], bytearray(w)
    for y in range(h):
        f, line = raw[y * (w + 1)], bytearray(raw[y * (w + 1) + 1:(y + 1) * (w + 1)])
        for x in range(w):
            a, b, c = line[x - 1] if x else 0, prev[x], prev[x - 1] if x else 0
            if f == 1:
                line[x] = (line[x] + a) & 255
            elif f == 2:
                line[x] = (line[x] + b) & 255
            elif f == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif f == 4:
                pa, pb, pc = abs(b - c), abs(a - c), abs(a + b - 2 * c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line
    return rows


def write_png(path, rows):
    """Rows of 4-bit indices as a greyscale PNG, 8 times larger, to look at."""
    big = [bytes(min(255, v * 17) for v in row for _ in range(8)) for row in rows for _ in range(8)]
    raw = b''.join(b'\0' + r for r in big)
    def chunk(kind, body):
        return struct.pack('>I', len(body)) + kind + body + struct.pack('>I', zlib.crc32(kind + body))
    ihdr = struct.pack('>IIBBBBB', len(big[0]), len(big), 8, 0, 0, 0, 0)
    with open(path, 'wb') as f:
        f.write(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', ihdr) + chunk(b'IDAT', zlib.compress(raw)) + chunk(b'IEND', b''))


class Version:
    """One version's extract: its executable, code packages and the
    menus' font."""

    def __init__(self, dumps):
        self.dumps = dumps
        exes = [p for p in glob.glob(os.path.join(dumps, 'files', '*')) if os.path.isfile(p)]
        if not exes:
            sys.exit(f'{dumps}: no executable in files/ (extract again with this PE2PC)')
        exe = open(exes[0], 'rb').read()
        self.exe_base = struct.unpack_from('<I', exe, 0x18)[0]
        self.exe = exe[0x800:]
        self.packages = {}

    def packages_of(self, key):
        """The code packages of file `key`: (chunk file name, contents
        decompressed)."""
        if key not in self.packages:
            self.packages[key] = [(os.path.basename(path), lzss(open(path, 'rb').read()))
                                  for path in sorted(glob.glob(os.path.join(self.dumps, 'files', key, '*.pe2pkg')))]
        return self.packages[key]

    def fonts(self):
        """For each face, code -> the cell's picture (rows of indices)."""
        maps = glob.glob(os.path.join(self.dumps, 'textures-glyphs', '*.toml'))
        sheet = next(tomllib.load(open(m, 'rb'))['sheets'][0] for m in maps
                     if tomllib.load(open(m, 'rb')).get('kind') == 'ui-font')
        rows = png_indices(os.path.join(self.dumps, 'textures-indexed', f'{sheet}-4bit.idx.png'))
        out = {}
        for face, table, count, bias in FACES:
            at, out[face] = table - self.exe_base, {}
            for code in range(0x20, 0x20 + count):
                u, v, w, h = self.exe[at + (code - 0x20) * 12:at + (code - 0x20) * 12 + 4]
                if w == 0 and h == 0:
                    continue
                v = (v + bias) & 255
                out[face][code] = tuple(rows[v + y][u:u + w + 1] for y in range(h + 1) if v + y < len(rows))
        return out


# Bytes a text may hold besides ASCII (strings.rs `in_font`): the menus'
# font's accented letters and marks.
FONT = (set(range(0x20, 0x7F)) | {0x0A, 0xA1, 0xBF} | set(range(0xC0, 0xD0)) | set(range(0xD1, 0xD7))
        | set(range(0xD9, 0xDD)) | {0xDF} | set(range(0xE0, 0xF0)) | set(range(0xF1, 0xF7)) | set(range(0xF9, 0xFD)))


def in_font(b):
    return b in FONT


def references(data):
    """Offset of each word of `data` -> its value, and of each `lui` that a
    later `addiu`/`ori` completes -> (that one's offset, the address): the
    first such, before another `lui` sets the register again."""
    words = {i: struct.unpack_from('<I', data, i)[0] for i in range(0, len(data) - 3, 4)}
    pairs = {}
    for i, lui in words.items():
        if lui >> 26 != 0x0F:
            continue
        reg = (lui >> 16) & 31
        for j in range(i + 4, min(i + 68, len(data) - 3), 4):
            op = words[j] >> 26
            if op in (0x09, 0x0D) and (words[j] >> 21) & 31 == reg:
                pairs[i] = (j, address_of(lui, words[j]))
                break
            if op == 0x0F and (words[j] >> 16) & 31 == reg:
                break
    return words, pairs


def address_of(lui, low):
    """The address a `lui` and the `addiu` (sign-extended) or `ori` after
    it put together."""
    imm = low & 0xFFFF
    if low >> 26 == 0x09 and imm & 0x8000:
        imm -= 0x10000
    return ((lui & 0xFFFF) << 16) + imm & 0xFFFFFFFF


def where_there(at, mine, theirs, base):
    """Where the other version keeps the text the game keeps at `at`: what
    the same words and instructions that point at it point at there (the
    most common answer)."""
    words, pairs = mine
    votes = {}
    for offset, value in words.items():
        if value == at and offset + 4 <= len(theirs):
            there = struct.unpack_from('<I', theirs, offset)[0]
            votes[there] = votes.get(there, 0) + 1
    for i, (j, address) in pairs.items():
        if address == at and j + 4 <= len(theirs):
            lui, low = struct.unpack_from('<I', theirs, i)[0], struct.unpack_from('<I', theirs, j)[0]
            if lui >> 26 != 0x0F or low >> 26 not in (0x09, 0x0D):
                continue
            there = address_of(lui, low)
            votes[there] = votes.get(there, 0) + 1
    good = {a: n for a, n in votes.items() if base <= a < base + len(theirs)}
    return max(good, key=good.get) if good else None


def read_text(data, offset):
    end = data.find(b'\0', offset)
    return data[offset:end if end >= 0 else len(data)]


def literal(text):
    return "'''\n" + text + "'''" if "'''" not in text else repr(text)


def main():
    p = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    p.add_argument('game')
    p.add_argument('other')
    p.add_argument('out')
    p.add_argument('--name', default='Translation of the menus')
    p.add_argument('--description', default="The menus' texts from another version's discs.")
    args = p.parse_args()

    game, other = Version(args.game), Version(args.other)
    # The other version's codes, read in each face: the game's letter of
    # the same picture in that face, a label, or not known.
    mine_fonts, their_fonts = game.fonts(), other.fonts()
    by_picture = {face: {} for face in mine_fonts}
    for face, font in mine_fonts.items():
        for code, picture in font.items():
            if code >= 0x21 and picture:
                by_picture[face].setdefault(picture, chr(code))
    unknown = {}

    def decode_in(raw, face):
        """`raw` read in `face`: the text, the pictures not known, and how
        many codes read as labelled pictures."""
        out, missing, labelled = [], [], 0
        for b in raw:
            picture = their_fonts[face].get(b)
            if b < 0x21 or b == 0x5C or chr(b) in '{|}~' or picture is None:
                out.append(chr(b))
                continue
            name = f'{face}-' + hashlib.sha1(repr(picture).encode()).hexdigest()[:12]
            text = by_picture[face].get(picture)
            if text is None:
                text = LABELS.get(name)
                labelled += text is not None
            if text is None:
                missing.append((name, b, picture))
                text = '{?' + name + '}'
            out.append(text)
        return ''.join(out), missing, labelled

    # The other version's words, from its dialogue: what tells which face
    # a text is read in when all know its codes.
    vocabulary = set()
    for path in glob.glob(os.path.join(args.other, 'text', '**', '*.toml'), recursive=True):
        for line in tomllib.load(open(path, 'rb')).get('line', []):
            vocabulary.update(WORD.findall(line['original'].lower()))
    # Their first four letters too: a word the dialogue has in another form
    # (correr: corre, corremos).
    stems = {w[:4] for w in vocabulary if len(w) >= 4}
    alphabet = {c for w in vocabulary for c in w}

    def decode_line(raw):
        """The reading in the face that knows all its codes, with no mark
        inside a word and no letter the translation never writes, and the
        best share of words the translation knows (a word 1, only its stem
        1/4), less 1/4 for each labelled picture (a face's own letter is
        likelier than a drawing over it) and 1/2 for each capital after a
        small letter, small letter after two capitals or one-letter word
        not a vowel; the first face on a tie."""
        readings = [decode_in(raw, face) for face, *_ in FACES]
        for _, missing, _ in readings:
            for name, code, picture in missing:
                unknown.setdefault(name, (code, picture))
        def score(reading):
            text, missing, labelled = reading
            text = CODES.sub(' ', text)
            words = WORD.findall(text.lower())
            foreign = sum(c not in alphabet for w in words for c in w)
            known = sum(1 if w in vocabulary else 0.25 if w[:4] in stems and len(w) >= 4 else 0 for w in words)
            # A word of one letter that is not a vowel (d, q: words of one
            # letter are a, e, o, é, à; a dialogue's q is short for que).
            lone = sum(len(w) == 1 and w not in 'aeiouáàâãéêíóôõú' for w in words)
            odd_case = len(CAMEL.findall(text)) + len(SHOUTED.findall(text)) + lone
            doubt = labelled * 0.25 + odd_case * 0.5 - known / max(1, len(words))
            return len(missing), len(ODD.findall(text)), foreign, doubt
        return min(readings, key=score)[0]

    def decode(raw):
        """Line by line: a text may change faces from one to the next."""
        return '\n'.join(decode_line(line) for line in raw.split(b'\n'))

    report, files, texts = [], 0, 0
    originals = set()
    for path in glob.glob(os.path.join(args.game, 'strings', '**', '*.toml'), recursive=True):
        originals.update(e['original'] for e in tomllib.load(open(path, 'rb')).get('string', []))

    def translation(rel, original, at, raw):
        """The other version's text `raw` for `original` (at `at`), read, or
        None (noted in the report when it is not a translation)."""
        # The libraries' messages to a debugger are not the game's.
        if '%' in original or '->' in original:
            return None
        # The same bytes: not translated (read in another face, it might
        # seem to be).
        if raw == original.encode('latin-1'):
            return None
        text = decode(raw)
        if text == original or not text.strip():
            return None
        # Pointers there that lead elsewhere: no words, or another text.
        letters = sum(c.isalpha() for c in text)
        wrong = letters < min(2, sum(c.isalpha() for c in original)) or text in originals
        if wrong or any(ord(c) < 0x20 and c != '\n' for c in text) or '%' in text:
            report.append(f'{rel} 0x{at:08X}: not its text there, left out\n'
                          f'    here:  {original!r}\n    there: {text!r}')
            return None
        if text.count('\n') != original.count('\n'):
            report.append(f'{rel} 0x{at:08X}: another number of lines\n'
                          f'    here:  {original!r}\n    there: {text!r}')
        return text

    outputs = {}    # file -> its entries' lines
    places = []     # the code packages read: (file, base, data, theirs, references, listed addresses)
    seen = []       # each place translated: (original, file, address, their bytes, its translation)
    lists = sorted(glob.glob(os.path.join(args.game, 'strings', '**', '*.toml'), recursive=True))
    newest = max(os.path.getmtime(f) for f in lists)
    for path in lists:
        if os.path.getmtime(path) < newest - 600:
            continue  # left from an older extract
        rel = os.path.relpath(path, os.path.join(args.game, 'strings'))
        key, name = os.path.dirname(rel).replace(os.sep, '/'), os.path.basename(rel)[:-5]
        listed = tomllib.load(open(path, 'rb')).get('string', [])
        if not listed:
            continue
        if name.startswith('txt-'):
            # A text file of the disc (an item's details): the other
            # version's file of the same place.
            there = os.path.join(args.other, 'files', key, name[4:] + '.txt')
            if not os.path.exists(there):
                report.append(f'{rel}: not found in the other version')
                continue
            raw = open(there, 'rb').read().split(b'\0')[0].replace(b'\r', b'')
            original = listed[0]['original']
            if raw == original.encode('latin-1'):
                continue
            text = decode(raw)
            if text != original:
                outputs[rel] = ['[[string]]', f'original = {literal(original)}', f'text = {literal(text)}', '']
            continue
        base = int(name, 16)
        first = listed[0]['original'].encode('latin-1')
        if key == 'exe':
            pairs = [('exe', game.exe, other.exe)]
        else:
            mine = dict(game.packages_of(key))
            theirs = dict(other.packages_of(key))
            pairs = [(n, d, theirs.get(n)) for n, d in mine.items()
                     if d[listed[0]['at'] - base:listed[0]['at'] - base + len(first)] == first]
        if not pairs or pairs[0][2] is None:
            report.append(f'{rel}: not found in the other version')
            continue
        _, data, theirs = pairs[0]
        refs = references(data)
        places.append((rel, base, data, theirs, refs, {s['at'] for s in listed}))
        found = {}
        for s in listed:
            there = where_there(s['at'], refs, theirs, base)
            if there is not None:
                found[s['at']] = there
        # Texts nothing points at: in order between found neighbours, when
        # the other version has as many texts there.
        ats = [s['at'] for s in listed]
        i = 0
        while i < len(ats):
            if ats[i] in found:
                i += 1
                continue
            j = i
            while j < len(ats) and ats[j] not in found:
                j += 1
            if 0 < i and j < len(ats):
                start = found[ats[i - 1]] - base
                start += len(read_text(theirs, start)) + 1
                stop = found[ats[j]] - base
                between, at = [], start
                while at < stop:
                    t = read_text(theirs, at)
                    if t.strip(b'\0'):
                        between.append(at)
                    at += len(t) + 1
                if len(between) == j - i:
                    for k, a in zip(range(i, j), between):
                        found[ats[k]] = base + a
            i = j
        # Then where it was, if a text starts there (tables of names in
        # records of a fixed size, translated in place: the map's rooms).
        for at in ats:
            offset = at - base
            if at not in found and offset < len(theirs) and (offset == 0 or theirs[offset - 1] == 0):
                if read_text(theirs, offset).strip():
                    found[at] = at
        for s in listed:
            if s['at'] not in found:
                continue
            raw = read_text(theirs, found[s['at']] - base)
            text = translation(rel, s['original'], s['at'], raw)
            if text is not None:
                seen.append((s['original'], rel, s['at'], raw, text))

    # The same texts elsewhere (the extract lists each once), found as the
    # other version's code and data point at them.
    by_bytes = {o.encode('latin-1'): o for o in originals if '%' not in o and len(o) >= 2}
    for rel, base, data, theirs, refs, listed in places:
        for offset in range(0, len(data), 4):
            if offset >= 4 and all(b != 0 and in_font(b) for b in data[offset - 4:offset]):
                continue
            original = by_bytes.get(read_text(data, offset))
            at = base + offset
            if original is None or at in listed:
                continue
            there = where_there(at, refs, theirs, base)
            if there is None:
                continue
            raw = read_text(theirs, there - base)
            text = translation(rel, original, at, raw)
            if text is not None:
                seen.append((original, rel, at, raw, text))

    # Each text's translation is the one most of its places have (the
    # first listed on a tie); a place translated otherwise gets its own,
    # for there only (Help: the key configuration's, a panel's title),
    # unless its bytes read as the chosen one in some face.
    places_of = {}
    for record in seen:
        places_of.setdefault(record[0], []).append(record)
    for original, records in places_of.items():
        # Told apart by more than capitals: in the small face, which has
        # capitals only, small letters show as capitals anyway.
        counts = {}
        for record in records:
            counts[record[4].lower()] = counts.get(record[4].lower(), 0) + 1
        wanted = max(counts, key=lambda t: (counts[t], t == records[0][4].lower()))
        variants = [record[4] for record in records if record[4].lower() == wanted]
        main = next((t for t in variants if not t.isupper()), variants[0])
        shown = False
        for _, rel, at, raw, text in records:
            same = text == main or any(decode_in(raw, face)[0] == main for face, *_ in FACES)
            if same and shown:
                continue
            out = outputs.setdefault(rel, [])
            out += ['[[string]]', f'at = 0x{at:08X}'] + ([] if same else ['here_only = true'])
            out += [f'original = {literal(original)}', f'text = {literal(main if same else text)}', '']
            shown = shown or same

    for rel, out in outputs.items():
        if not out:
            continue
        dest = os.path.join(args.out, 'strings', rel)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        with open(dest, 'w', encoding='utf-8', newline='\n') as f:
            f.write('\n'.join([f'# {args.name}: {rel.replace(os.sep, "/")}', ''] + out))
        files += 1
        texts += out.count('[[string]]')

    os.makedirs(args.out, exist_ok=True)
    if unknown:
        pictures = os.path.join(args.out, 'report-glyphs')
        os.makedirs(pictures, exist_ok=True)
        for name, (code, picture) in sorted(unknown.items()):
            write_png(os.path.join(pictures, f'{name}.png'), picture)
            report.append(f'code 0x{code:02X}: picture {name} not known (report-glyphs/{name}.png)')
    with open(os.path.join(args.out, 'mod.toml'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(f'name = "{args.name}"\ndescription = "{args.description}"\nenabled = true\n')
    with open(os.path.join(args.out, 'report.txt'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(report) + '\n')
    print(f'{files} files, {texts} texts; {len(unknown)} pictures not known; {len(report)} notes in report.txt')


if __name__ == '__main__':
    main()
