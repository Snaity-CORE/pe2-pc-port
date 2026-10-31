#!/usr/bin/env python3
"""Make a translation mod from another version's discs (docs/MODDING.md,
"Translating").

Extract both versions (`pe2 --extract`, each with its own data folder):
this reads each dialogue file's lines from the other version's dumps/text
and puts them, as translations, into the lines in the same place of the
game's own (same file, same order, a line dropped or added there
allowed for), where both have the same controls (camera changes,
choices, icons). Writes `text/**/*.toml`, `mod.toml` and
`report.txt` (what was left out, and lines whose glyphs were not read,
shown as ?) into the mod folder.

    python tools/import_dialogue.py <game's dumps/text> <other's dumps/text> <mod folder>
        [--name NAME] [--description TEXT]
"""

import argparse
import glob
import os
import re
import sys
import tomllib

CONTROL = re.compile(r'\{([0-9A-F]{4})\}')
NOT_READ = re.compile(r'\{g\d+\}')


def literal(text):
    """A TOML multi-line literal string."""
    if "'''" in text:
        sys.exit(f"a line holds ''': {text!r}")
    return "'''\n" + text + "'''"


END = re.compile(r'(\.\.\.|[.?!"\]]|)\s*$')
DIGITS = re.compile(r'\d+')
GAP = -2


def features(lines):
    """What a line and its translation share: its controls, the distance to
    the next record (the script's layout, kept by translators who only
    retype the text), its number of lines, its closing punctuation, its
    numbers, its bracketed labels and colour changes."""
    out = []
    for i, line in enumerate(lines):
        text = line['original']
        step = lines[i + 1]['record'] - line['record'] if i + 1 < len(lines) else None
        out.append((CONTROL.findall(text), step, text.count('\n'), END.search(text).group(1),
                    DIGITS.findall(text), text.count('['), text.count('{color')))
    return out


def similarity(a, b):
    controls, step, breaks, end, digits, brackets, colours = a
    score = 3 if controls == b[0] else -3
    score += 2 if step is not None and step == b[1] else 0
    score += 1 if breaks == b[2] else 0
    score += 1 if end == b[3] else 0
    score += 2 if digits and digits == b[4] else 0
    score += 1 if brackets and brackets == b[5] else 0
    score += 1 if colours and colours == b[6] else 0
    return score


def align(mine, theirs):
    """Pairs each of the game's lines with the other version's line in the
    same place: in order, but translators may have dropped or added a line
    (an empty one with only a camera change, say), so this is the best
    alignment of the two lists (Needleman-Wunsch on `similarity`). Yields
    (game line, other line), either None where the other list has none."""
    a, b = features(mine), features(theirs)
    n, m = len(a), len(b)
    best = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(1, n + 1):
        best[i][0] = i * GAP
    for j in range(1, m + 1):
        best[0][j] = j * GAP
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            best[i][j] = max(best[i - 1][j - 1] + similarity(a[i - 1], b[j - 1]),
                             best[i - 1][j] + GAP, best[i][j - 1] + GAP)
    pairs, i, j = [], n, m
    while i or j:
        if i and j and best[i][j] == best[i - 1][j - 1] + similarity(a[i - 1], b[j - 1]):
            pairs.append((mine[i - 1], theirs[j - 1]))
            i, j = i - 1, j - 1
        elif i and best[i][j] == best[i - 1][j] + GAP:
            pairs.append((mine[i - 1], None))
            i -= 1
        else:
            pairs.append((None, theirs[j - 1]))
            j -= 1
    return reversed(pairs)


def main():
    p = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    p.add_argument('game')
    p.add_argument('other')
    p.add_argument('out')
    p.add_argument('--name', default='Translation')
    p.add_argument('--description', default="Dialogue from another version's discs.")
    args = p.parse_args()

    report, files, lines, not_read = [], 0, 0, 0
    for game_path in sorted(glob.glob(os.path.join(args.game, '**', '*.toml'), recursive=True)):
        rel = os.path.relpath(game_path, args.game)
        other_path = os.path.join(args.other, rel)
        if not os.path.exists(other_path):
            report.append(f'{rel}: not in the other version')
            continue
        with open(game_path, 'rb') as f:
            game = tomllib.load(f)
        with open(other_path, 'rb') as f:
            other = tomllib.load(f)
        mine, theirs = game.get('line', []), other.get('line', [])
        out = [f'# {args.name}: {rel.replace(os.sep, "/")}', f'cap = "{game["cap"]}"', '']
        for m, t in align(mine, theirs):
            if m is None:
                report.append(f'{rel}: a line only there, left out\n    there: {t["original"]!r}')
                continue
            if t is None:
                report.append(f'{rel} record 0x{m["record"]:04X}: no line there, left out\n'
                              f'    here:  {m["original"]!r}')
                continue
            text = t['original']
            if CONTROL.findall(text) != CONTROL.findall(m['original']):
                report.append(f'{rel} record 0x{m["record"]:04X}: controls differ, left out\n'
                              f'    here:  {m["original"]!r}\n    there: {text!r}')
                continue
            if NOT_READ.search(text):
                not_read += 1
                report.append(f'{rel} record 0x{m["record"]:04X}: glyphs not read, shown as ?\n    there: {text!r}')
                text = NOT_READ.sub('?', text)
            out += ['[[line]]', f'record = 0x{m["record"]:04X}']
            if 'speaker' in m:
                out.append(f'speaker = "{m["speaker"]}"')
            out += [f'original = {literal(m["original"])}', f'text = {literal(text)}', '']
            lines += 1
        dest = os.path.join(args.out, 'text', rel)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        with open(dest, 'w', encoding='utf-8', newline='\n') as f:
            f.write('\n'.join(out))
        files += 1

    os.makedirs(args.out, exist_ok=True)
    with open(os.path.join(args.out, 'mod.toml'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(f'name = "{args.name}"\ndescription = "{args.description}"\nenabled = true\n')
    with open(os.path.join(args.out, 'report.txt'), 'w', encoding='utf-8', newline='\n') as f:
        f.write('\n'.join(report) + '\n')
    print(f'{files} files, {lines} lines; {not_read} with glyphs not read; {len(report)} notes in report.txt')


if __name__ == '__main__':
    main()
