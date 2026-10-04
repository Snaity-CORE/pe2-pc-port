#!/usr/bin/env python3
"""Verify that port changes in the decomp fork leave the PSX build untouched.

Every PC_PORT change in `decomp/` must preprocess to exactly the same text as
upstream when PC_PORT is not defined, so the matching PSX build cannot be
affected. This preprocesses each unit in build/units.json twice, once from a
reference commit (default: the upstream merge base) and once from the working
tree, and reports any unit whose output differs.

    python tools/check_psx_unchanged.py               # vs merge-base with upstream/main
    python tools/check_psx_unchanged.py --ref HEAD    # vs the last commit
"""

import argparse
import concurrent.futures
import json
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"


def git(*args: str) -> str:
    return subprocess.run(["git", *args], cwd=DECOMP, check=True, capture_output=True,
                          text=True).stdout.strip()


def preprocess(tree: Path, unit: dict) -> str:
    # Generated asset includes live only in the real tree's build/ directory.
    includes = [tree / "include", DECOMP / "build", DECOMP / "build" / "include",
                tree / "include" / "psyq", tree / "include" / "decomp"]
    args = ["clang", "-E", "-P", "-m32", "-D_LANGUAGE_C", "-DUSE_INCLUDE_ASM"]
    args += [f"-I{p}" for p in includes]
    for name, value in unit["defines"].items():
        args.append(f"-D{name}" if value is None else f"-D{name}={value}")
    args.append(str(tree / unit["source"]))
    proc = subprocess.run(args, cwd=tree, capture_output=True, text=True)
    # Blank lines move when directives are added; nothing else may change.
    return re.sub(r"\n\s*\n+", "\n", proc.stdout + proc.stderr)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--ref", help="reference commit (default: merge-base with upstream/main)")
    parser.add_argument("--units", type=Path, default=ROOT / "build" / "units.json")
    args = parser.parse_args()

    ref = args.ref or git("merge-base", "HEAD", "upstream/main")
    units = json.loads(args.units.read_text())
    tmp = Path(tempfile.mkdtemp(prefix="psx-ref-"))
    ref_tree = tmp / "ref"
    git("worktree", "add", "--detach", str(ref_tree), ref)
    # Sources the configure step generates (e.g. packageid.c) are gitignored,
    # so the reference checkout lacks them; they are the same on both sides.
    generated = git("ls-files", "--others", "--ignored", "--exclude-standard", "src").splitlines()
    for rel in generated:
        dst = ref_tree / rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(DECOMP / rel, dst)
    try:
        def compare(unit):
            return unit, preprocess(ref_tree, unit) == preprocess(DECOMP, unit)

        with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
            diffs = [u for u, same in pool.map(compare, units) if not same]
    finally:
        git("worktree", "remove", "--force", str(ref_tree))
        shutil.rmtree(tmp, ignore_errors=True)

    for unit in diffs:
        print(f"DIFFERS: {unit['object']} ({unit['source']})")
    print(f"{len(units) - len(diffs)}/{len(units)} units preprocess identically to {ref[:10]} without PC_PORT")
    raise SystemExit(1 if diffs else 0)


if __name__ == "__main__":
    main()
