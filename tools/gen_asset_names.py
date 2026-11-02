#!/usr/bin/env python3
"""Write crates/pe2-host/res/asset_names.tsv from the decompilation's asset
tree (decomp/tools/peassets/asset_data.py TREE): for each chunk of each disc
file, the folder the extract files what it holds in. Names only: nothing of
the game's data.

- STAGE0's files (characters, enemies, items, the interface...): the
  folder of the chunk's name (characters/aya/10200/image:
  characters/aya/10200), or of its file's other chunks' for a name
  without one; for a file named without folders, maps/ for the maps
  (mappic_*), characters/<who>/<id> for a character's (aya_10400), else
  other/<its name>.
- STAGE1..5's folders (one a room): rooms/<area>/<room>, from the
  folder's name (shelter_b1_storeroom: rooms/shelter/b1_storeroom). Their
  chunks' names are no guide: a chunk the same as one seen before has the
  first one's name (Dryfield's sounds named rooms/akropolis).

Run again when the decompilation renames its assets:

    python tools/gen_asset_names.py
"""

import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, 'decomp', 'tools', 'peassets'))
from asset_data import TREE  # noqa: E402

OUT = os.path.join(ROOT, 'crates', 'pe2-host', 'res', 'asset_names.tsv')
# The areas rooms' folder names start with, longest first.
AREAS = ['dryfield_night', 'neo_ark', 'akropolis', 'dryfield', 'shelter', 'mine', 'mist']


def room_folder(name):
    for area in AREAS:
        if name.startswith(area + '_'):
            return f'rooms/{area}/{name[len(area) + 1:]}'
    return f'rooms/{name}'


def unnamed_folder(file_name, chunk_names, characters):
    """Where a STAGE0 file whose chunks' names have no folders goes."""
    if any(n.startswith('mappic') for n in chunk_names + [file_name]):
        return 'maps'
    who, _, id_ = file_name.rpartition('_')
    if who in characters and id_.isdigit():
        return f'characters/{who}/{id_}'
    return f'other/{file_name}'


def main():
    lines = []
    files0 = TREE[0]['files']
    characters = {n.split('/')[1] for f in files0.values() for n in f['chunks'].values()
                  if n.startswith('characters/') and n.count('/') >= 2}
    for stage, entry in sorted(TREE.items()):
        if stage == 0:
            for file_name, file in entry['files'].items():
                key = f'stage0/file{file["id"]}'
                names = list(file['chunks'].values())
                dirs = [n.rsplit('/', 1)[0] for n in names if '/' in n]
                fallback = dirs[0] if dirs else unnamed_folder(file_name, names, characters)
                for index, name in enumerate(names):
                    folder = name.rsplit('/', 1)[0] if '/' in name else fallback
                    lines.append((key, index, folder))
        else:
            for name, folder in entry['folders'].items():
                for file in folder['files'].values():
                    key = f'stage{stage}/{folder["id"]}/file{file["id"]}'
                    for index in range(len(file['chunks'])):
                        lines.append((key, index, room_folder(name)))
    lines.sort()
    with open(OUT, 'w', encoding='utf-8', newline='\n') as f:
        f.write("# disc file\tchunk\tthe folder the extract files it in (tools/gen_asset_names.py)\n")
        f.writelines(f'{k}\t{i}\t{d}\n' for k, i, d in lines)
    print(f'{len(lines)} chunks in {OUT}')


if __name__ == '__main__':
    main()
