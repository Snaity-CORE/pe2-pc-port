#!/usr/bin/env python3
"""Map native addresses from a crash report to functions, using the linker map.

    python tools/symbolize.py 0x005523BC 0x00537D8D ...
    python tools/symbolize.py --map target/i686-pc-windows-msvc/release/deps/pe2.map 0x...

The map is by default the newest of the release and debug builds': it must
be the one of the pe2.exe that crashed. Game functions appear as
pcfn_<image>_<PSX address>_<name>.
"""

import argparse
import bisect
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MAPS = [ROOT / "target" / "i686-pc-windows-msvc" / build / "deps" / "pe2.map" for build in ("release", "debug")]
DEFAULT_MAP = max((m for m in MAPS if m.exists()), key=lambda m: m.stat().st_mtime, default=MAPS[0])
LINE_RE = re.compile(r"^\s*[0-9a-fA-F]{4}:[0-9a-fA-F]{8}\s+(\S+)\s+([0-9a-fA-F]{8})\s")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("addresses", nargs="+")
    parser.add_argument("--map", type=Path, default=DEFAULT_MAP)
    args = parser.parse_args()

    rows = []
    for line in args.map.read_text(encoding="latin-1").splitlines():
        if m := LINE_RE.match(line):
            rows.append((int(m.group(2), 16), m.group(1)))
    rows.sort()
    keys = [a for a, _ in rows]
    for text in args.addresses:
        addr = int(text, 16)
        i = bisect.bisect_right(keys, addr) - 1
        if i < 0 or addr >= 0x80000000:
            print(f"0x{addr:08X}  (not native code)")
            continue
        base, name = rows[i]
        print(f"0x{addr:08X}  {name} + 0x{addr - base:X}")


if __name__ == "__main__":
    main()
