#!/usr/bin/env python3
"""Syntax-check every game translation unit for the PC target and summarize errors.

A fast progress gauge for phase 1: runs `clang -fsyntax-only` over each unit in
build/units.json (see gen_units.py) with the port's include paths, then groups
the errors by kind so the next most common blocker is obvious.

    python tools/probe_units.py                 # all units
    python tools/probe_units.py --filter main/  # units whose object starts with main/
    python tools/probe_units.py --show 5        # also print 5 failing units' first errors
"""

import argparse
import collections
import concurrent.futures
import json
import os
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"
TARGET = "i686-pc-windows-msvc"

# Port headers come first so they can override the PSX ones: the generated GTE
# headers (gen_port_headers.py), then the hand-written port headers.
INCLUDES = [
    ROOT / "build" / "port_include",
    ROOT / "port" / "include",
    DECOMP / "include",
    DECOMP / "build",
    DECOMP / "build" / "include",
    DECOMP / "include" / "psyq",
    DECOMP / "include" / "decomp",
]


# The game was written for GCC 2.8, which accepted things modern clang rejects
# by default. On i686 with cdecl these are harmless, so keep them as warnings.
LENIENT_FLAGS = [
    "-Wno-error=incompatible-function-pointer-types",
    "-Wno-error=int-conversion",
    "-Wno-error=return-type",
    "-Wno-error=implicit-function-declaration",
    "-Wno-error=implicit-int",
    "-Wno-error=incompatible-pointer-types",
]


def clang_args(unit: dict) -> list[str]:
    args = ["clang", f"--target={TARGET}", "-fsyntax-only", "-w", "-ferror-limit=0",
            "-D_LANGUAGE_C", "-DPC_PORT", *LENIENT_FLAGS]
    args += [f"-I{p}" for p in INCLUDES]
    for name, value in unit["defines"].items():
        args.append(f"-D{name}" if value is None else f"-D{name}={value}")
    args.append(unit["source"])
    return args


def run(unit: dict) -> tuple[dict, list[str]]:
    proc = subprocess.run(clang_args(unit), cwd=DECOMP, capture_output=True, text=True)
    errors = [line for line in proc.stderr.splitlines() if " error: " in line]
    return unit, errors


def normalize(error: str) -> str:
    msg = error.split(" error: ", 1)[1]
    return re.sub(r"'[^']*'", "X", msg)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--units", type=Path, default=ROOT / "build" / "units.json")
    parser.add_argument("--filter", default="", help="only objects starting with this prefix")
    parser.add_argument("--show", type=int, default=0, help="print first errors of N failing units")
    parser.add_argument("--top", type=int, default=20)
    args = parser.parse_args()

    units = [u for u in json.loads(args.units.read_text()) if u["object"].startswith(args.filter)]
    failed = []
    kinds = collections.Counter()
    first_kind = collections.Counter()
    with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
        for unit, errors in pool.map(run, units):
            if errors:
                failed.append((unit, errors))
                kinds.update(normalize(e) for e in errors)
                first_kind[normalize(errors[0])] += 1

    print(f"{len(units) - len(failed)}/{len(units)} units compile")
    print("\nfirst error per failing unit:")
    for kind, n in first_kind.most_common(args.top):
        print(f"  {n:6}  {kind}")
    print("\nall errors:")
    for kind, n in kinds.most_common(args.top):
        print(f"  {n:6}  {kind}")
    for unit, errors in failed[: args.show]:
        print(f"\n== {unit['object']} ({unit['source']})")
        print("\n".join(errors[:5]))


if __name__ == "__main__":
    main()
