#!/usr/bin/env python3
"""Make a translation mod from another version's discs (docs/MODDING.md,
"Translating").

Extract both versions (`pe2 --extract`, each with its own data folder):
this reads each dialogue file's lines from the other version's dumps/text
and puts them, as translations, into the lines in the same place of the
game's own (same file, same order), where both have the same controls
(camera changes, choices, icons). Writes `text/**/*.toml`, `mod.toml` and
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
        if len(mine) != len(theirs):
            report.append(f'{rel}: {len(mine)} lines here, {len(theirs)} there: left out')
            continue
        out = [f'# {args.name}: {rel.replace(os.sep, "/")}', f'cap = "{game["cap"]}"', '']
        for m, t in zip(mine, theirs):
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
