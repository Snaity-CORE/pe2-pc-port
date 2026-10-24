#!/usr/bin/env python3
"""Whole-session profile from PE2_TRACE=prof samples (pe2-prof.bin).

    python tools/profile_report.py [pe2-prof.bin] [--map MAP] [--top N] [--skip SECONDS] [--span SECONDS]

The game thread is sampled about every millisecond. "self" is where it
was; samples outside the executable (Windows waits, the C runtime) are
charged to the nearest caller inside it, marked with "~". "total" counts a
function once per sample when it is the current function or on the stack.
"""

import argparse
import bisect
import re
import struct
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_MAP = ROOT / "target" / "i686-pc-windows-msvc" / "release" / "deps" / "pe2.map"
# Code symbols (section 0001); "name$suffix" ones (exception handlers,
# unwind funclets) would shadow the functions they belong to.
LINE_RE = re.compile(r"^\s*0001:[0-9a-fA-F]{8}\s+([^\s$]+)\s+([0-9a-fA-F]{8})\s")
RECORD = struct.Struct("<8I")


def load_map(path: Path):
    rows = []
    for line in path.read_text(encoding="latin-1").splitlines():
        if m := LINE_RE.match(line):
            rows.append((int(m.group(2), 16), m.group(1)))
    rows.sort()
    return [a for a, _ in rows], [readable(n) for _, n in rows]


def readable(name: str) -> str:
    """Approximates a Rust v0 mangled name (_R...) by its identifiers, e.g.
    pe2::runtime::mdec::Mdec::decode_block; other names unchanged."""
    name = name.lstrip("_")
    if not name.startswith("R"):
        return name
    rest = re.sub(r"C[sh][0-9A-Za-z]+_", "", name[1:])  # crate disambiguators
    parts, i, boundary = [], 0, True
    while i < len(rest):
        m = re.match(r"(\d+)_?", rest[i:])
        if m and int(m.group(1)) > 0 and (boundary or not rest[i - 1].isdigit()):
            n = int(m.group(1))
            ident = rest[i + m.end():i + m.end() + n]
            if len(ident) == n and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", ident):
                parts.append(ident)
                i += m.end() + n
                boundary = True
                continue
        boundary = False
        i += 1
    return "::".join(parts[:6]) or name


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("samples", nargs="?", type=Path, default=ROOT / "pe2-prof.bin")
    parser.add_argument("--map", type=Path, default=DEFAULT_MAP)
    parser.add_argument("--top", type=int, default=30)
    parser.add_argument("--skip", type=float, default=0, help="seconds of samples to skip (boot)")
    parser.add_argument("--span", type=float, default=0, help="seconds of samples to keep after --skip (0: all)")
    args = parser.parse_args()

    keys, names = load_map(args.map)

    def name_of(addr: int):
        i = bisect.bisect_right(keys, addr) - 1
        if addr == 0 or i < 0 or addr >= 0x80000000:
            return None
        return names[i]

    data = args.samples.read_bytes()
    records = [RECORD.unpack_from(data, o) for o in range(0, len(data) - RECORD.size + 1, RECORD.size)]
    records = records[int(args.skip * 1000):]
    if args.span > 0:
        records = records[: int(args.span * 1000)]
    total = len(records)
    if not total:
        raise SystemExit("no samples")

    self_counts, total_counts = Counter(), Counter()
    in_irq = 0
    for flags, eip, *callers in records:
        in_irq += flags & 1
        here = None if flags & 2 else name_of(eip)
        stack = [name_of(c) for c in callers if c]
        stack = [n for n in stack if n]
        if here is None:
            here = "~" + stack[0] if stack else "~outside"
        self_counts[here] += 1
        for n in {here.lstrip("~"), *stack}:
            total_counts[n] += 1

    print(f"{total} samples (~{total / 1000:.1f} s), {100 * in_irq / total:.1f}% in interrupt handlers\n")
    print("self:")
    for n, c in self_counts.most_common(args.top):
        print(f"  {100 * c / total:5.1f}%  {n}")
    print("\ntotal:")
    for n, c in total_counts.most_common(args.top):
        print(f"  {100 * c / total:5.1f}%  {n}")


if __name__ == "__main__":
    main()
