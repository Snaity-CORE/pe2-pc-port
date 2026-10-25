#!/usr/bin/env python3
"""Fail if any tracked (or staged) file looks like data taken from the game discs.

The port must never ship copyrighted game data: the player supplies their own
disc dump and the game reads it at runtime. This check runs as a pre-commit
hook and in CI. It inspects this repository only; the `decomp` submodule is
checked by its own project.

Usage:
    python tools/check_no_game_data.py            # all tracked files
    python tools/check_no_game_data.py --staged   # only the staged changes
"""

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

FORBIDDEN_DIRS = ("rom/", "assets/", "disc/", "build/", "target/")
FORBIDDEN_SUFFIXES = {
    ".bin", ".cue", ".iso", ".img", ".chd",  # disc images
    ".pe2pkg", ".pe2img", ".pe2clut",       # extracted game packages
    ".str", ".xa", ".vab", ".vh", ".vb",    # video / audio streams and banks
    ".tim", ".tmd",                         # textures / models
    ".mcd", ".mcr",                         # memory card images
}
# Binary files larger than this need an explicit allowlist entry.
MAX_BINARY_BYTES = 64 * 1024
ALLOWED_BINARIES: set[str] = {
    # The built-in fonts (their licenses beside them): the dialogue's and
    # the menus' (DejaVu), the loading captions' (Courier Prime).
    "crates/pe2-host/res/fonts/DejaVuSans-Bold.ttf",
    "crates/pe2-host/res/fonts/CourierPrime-Regular.ttf",
}

# Byte signatures that only appear in disc-derived data.
SIGNATURES = {
    b"PS-X EXE": "PlayStation executable header",
    b"\x00\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x00": "raw CD sector sync",
    b"CD001": "ISO 9660 volume descriptor",
}


def git_files(staged: bool) -> list[str]:
    if staged:
        cmd = ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"]
    else:
        cmd = ["git", "ls-files"]
    out = subprocess.run(cmd, cwd=ROOT, check=True, capture_output=True, text=True)
    return [line for line in out.stdout.splitlines() if line and line != "decomp"]


def is_binary(data: bytes) -> bool:
    return b"\x00" in data[:8192]


def check(path: str) -> list[str]:
    problems = []
    lower = path.lower()
    if lower.startswith(FORBIDDEN_DIRS):
        problems.append("lives in a directory reserved for game data / build output")
    if Path(lower).suffix in FORBIDDEN_SUFFIXES:
        problems.append(f"has a game-data extension ({Path(lower).suffix})")

    full = ROOT / path
    if not full.is_file():
        return problems
    data = full.read_bytes()
    # Signatures only count in binary files: source code may name them (the
    # boot code checks for the "PS-X EXE" magic, for example).
    if is_binary(data):
        for sig, what in SIGNATURES.items():
            if sig in data:
                problems.append(f"contains a {what} signature")
    if is_binary(data) and len(data) > MAX_BINARY_BYTES and path not in ALLOWED_BINARIES:
        problems.append(f"is a {len(data)} byte binary file (limit {MAX_BINARY_BYTES})")
    return problems


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--staged", action="store_true", help="check only staged files")
    args = parser.parse_args()

    failures = {p: probs for p in git_files(args.staged) if (probs := check(p))}
    for path, probs in failures.items():
        for prob in probs:
            print(f"{path}: {prob}", file=sys.stderr)
    if failures:
        print(f"\n{len(failures)} file(s) look like game data. Game data must never be "
              "committed; the port reads the player's own disc at runtime.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
