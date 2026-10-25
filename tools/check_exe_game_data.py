#!/usr/bin/env python3
"""Fail if a built pe2.exe carries data taken from the game discs.

The game's data must come from the player's discs at runtime: pclink.py
sends every data symbol the decomp defines to its PSX address and the
linker drops the native copy. This checks the result against what the
discs hold, before a build is shared:

* the game's executables (SLUS_010.42, SLUS_010.55) from the decomp's
  extraction: no 32-byte run of them may appear in the exe;
* the asset tables the decomp generates from the discs (models,
  collision, animations, text, as build/include/assets/**/*.inc): no run
  of 32 of their values may appear, packed as 8-, 16- or 32-bit, and none
  of their texts of 16 characters or more.

Runs of few distinct values or with a steady step (zeros, 0, 1, 2, ...)
are skipped: they turn up by chance in any binary. ALLOWED lists the
ranges of the executables the port holds on purpose, with why.

Needs the decomp extracted and built (docs/BUILDING.md).

    python tools/check_exe_game_data.py [target/i686-pc-windows-msvc/release/pe2.exe]
"""

import argparse
import re
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"
EXE = ROOT / "target" / "i686-pc-windows-msvc" / "release" / "pe2.exe"
EXECUTABLES = [DECOMP / "rom" / "USA" / "disk1" / "SLUS_010.42", DECOMP / "rom" / "USA" / "disk2" / "SLUS_010.55"]
ASSETS = DECOMP / "build" / "include" / "assets"

WINDOW = 32
# PSX address ranges of the executables the port may hold, with why.
ALLOWED = [
    # libspu's reverb presets (_spu_rev_param): runtime/spu/libspu.rs
    # PRESETS, register values from the public hardware documentation
    # (psx-spx, "Reverb Examples"), which lists this same table.
    (0x800686D4, 0x80068984, "reverb presets"),
]
# The executables load at 0x80010000, after their 2 KiB header.
LOAD_ADDRESS = 0x80010000
NUMBER = re.compile(r"-?\b(?:0x[0-9a-fA-F]+|\d+)\b")
TEXT = re.compile(r'"([^"\\\n]{16,})"')


def allowed(src: int, length: int) -> bool:
    """A run (at `src` in the executable, past its header) lies in ALLOWED."""
    start, end = LOAD_ADDRESS + src, LOAD_ADDRESS + src + length
    return any(lo <= start and end <= hi for lo, hi, _ in ALLOWED)


def trivial(values) -> bool:
    """Too regular to tell disc data from chance."""
    if len(set(values)) < WINDOW // 2:
        return True
    steps = {b - a for a, b in zip(values, values[1:])}
    return len(steps) <= 3


def executable_runs(exe: bytes, path: Path) -> list[tuple[int, int, int]]:
    """Runs of `path` found in `exe`: (exe offset, file offset, length)."""
    data = path.read_bytes()
    if data[:8] == b"PS-X EXE":
        data = data[2048:]
    index = {}
    for off in range(0, len(data) - WINDOW, 4):
        w = data[off:off + WINDOW]
        if not trivial(w):
            index.setdefault(w, off)
    runs = []
    pos = 0
    while pos < len(exe) - WINDOW:
        src = index.get(exe[pos:pos + WINDOW])
        if src is None:
            pos += 1
            continue
        # Extend the match both ways.
        start = 0
        while pos - start > 0 and src - start > 0 and exe[pos - start - 1] == data[src - start - 1]:
            start += 1
        end = WINDOW
        while pos + end < len(exe) and src + end < len(data) and exe[pos + end] == data[src + end]:
            end += 1
        runs.append((pos - start, src - start, start + end))
        pos += end
    return runs


def asset_matches(exe: bytes) -> tuple[int, list[str]]:
    """How many asset tables were checked, and those found in `exe`."""
    checked, found = 0, []
    for path in sorted(ASSETS.rglob("*.inc")):
        text = path.read_text(encoding="utf-8", errors="replace").split("*/", 1)[-1]
        values = [int(v, 0) for v in NUMBER.findall(text)]
        windows = [values[i:i + WINDOW] for i in range(0, len(values) - WINDOW + 1, WINDOW)]
        windows = [w for w in windows if not trivial(w)][:4]
        if not windows:
            continue
        checked += 1
        for w in windows:
            packed = []
            for fmt in ("b", "B", "h", "H", "i", "I"):
                try:
                    packed.append(struct.pack(f"<{WINDOW}{fmt}", *w))
                except struct.error:
                    pass
            if any(p in exe for p in packed):
                found.append(str(path.relative_to(ASSETS)))
                break
    return checked, found


def text_matches(exe: bytes) -> tuple[int, list[str]]:
    """How many texts of the generated includes were checked, and those found in `exe`."""
    texts = set()
    for path in ASSETS.rglob("*"):
        if path.suffix in (".inc", ".h"):
            texts.update(TEXT.findall(path.read_text(encoding="utf-8", errors="replace")))
    return len(texts), sorted(t for t in texts if t.encode("latin-1", "ignore") in exe)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("exe", nargs="?", type=Path, default=EXE)
    args = parser.parse_args()
    exe = args.exe.read_bytes()
    failed = False
    for path in EXECUTABLES:
        if not path.exists():
            print(f"missing {path.relative_to(ROOT)}: extract the discs first (docs/BUILDING.md)")
            return 2
        runs = [r for r in executable_runs(exe, path) if not allowed(r[1], r[2])]
        print(f"{path.name}: {len(runs)} runs found")
        for at, src, length in runs[:10]:
            print(f"  exe offset 0x{at:X}: {length} bytes of {path.name} from 0x{src:X}")
        failed |= bool(runs)
    if not ASSETS.is_dir():
        print(f"missing {ASSETS.relative_to(ROOT)}: build the decomp first (docs/BUILDING.md)")
        return 2
    checked, found = asset_matches(exe)
    print(f"asset tables: {checked} checked, {len(found)} found")
    for name in found[:20]:
        print(f"  {name}")
    failed |= bool(found)
    checked, found = text_matches(exe)
    print(f"asset texts: {checked} checked, {len(found)} found")
    for text in found[:20]:
        print(f"  {text!r}")
    failed |= bool(found)
    print("FAILED: the exe carries game data" if failed else "ok: no game data found")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
