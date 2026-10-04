#!/usr/bin/env python3
"""Build the game's native static library, build/game/pe2game.lib.

Steps, each skipped when its outputs are up to date:

1. gen_port_headers.py  - C versions of the decomp's GTE macro headers
2. compile              - every unit in build/units.json with clang (i686)
3. pclink.py            - rebind symbols for the identity-mapped PSX memory
4. runtime              - absolute PSX symbols, thunk tables, runtime stubs and
                          the port's C runtime (port/src)
5. archive              - everything into pe2game.lib for the Rust host

Prerequisites (see docs/BUILDING.md): the decomp configured and its PSX build
run (`python ninja_config.py && ninja` in decomp/), then tools/gen_units.py.

    python tools/build_game.py            # incremental
    python tools/build_game.py --clean    # rebuild everything
"""

import argparse
import concurrent.futures
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"
BUILD = ROOT / "build"
OBJ_DIR = BUILD / "obj"
LINK_DIR = BUILD / "link"
GAME_DIR = BUILD / "game"
TARGET = "i686-pc-windows-msvc"

INCLUDES = [
    BUILD / "port_include",
    ROOT / "port" / "include",
    DECOMP / "include",
    DECOMP / "build",
    DECOMP / "build" / "include",
    DECOMP / "include" / "psyq",
    DECOMP / "include" / "decomp",
]

# Code generation that keeps the PSX build's semantics:
#   -funsigned-char   the decomp builds with unsigned plain char
#   -fwrapv           PSX code relies on wrapping signed arithmetic
#   -fno-strict-aliasing, -fno-delete-null-pointer-checks
#                     the code type-puns freely and may touch address 0
#   -fdata-sections   one section per data object, so pclink can attribute
#                     every section-relative data reference
#   -mno-sse -mno-mmx globals live at their PSX addresses, which are only as
#                     aligned as the PSX build made them; clang would assume
#                     its own (16-byte) alignment and emit aligned SSE moves.
#                     The game is fixed-point, so nothing is lost.
GAME_CFLAGS = [
    f"--target={TARGET}", "-O2", "-g0",
    "-funsigned-char", "-fwrapv", "-fno-strict-aliasing", "-fno-delete-null-pointer-checks",
    "-fdata-sections", "-fno-builtin", "-mno-sse", "-mno-mmx",
    "-D_LANGUAGE_C", "-DPC_PORT",
    "-w",
    # GCC 2.8 accepted these; on i686 cdecl they are harmless.
    "-Wno-error=incompatible-function-pointer-types", "-Wno-error=int-conversion",
    "-Wno-error=return-type", "-Wno-error=implicit-function-declaration",
    "-Wno-error=implicit-int", "-Wno-error=incompatible-pointer-types",
]
RUNTIME_CFLAGS = [f"--target={TARGET}", "-O2", "-Wall", "-Wextra", "-Wno-unused-parameter",
                  f"-I{ROOT / 'port' / 'include'}"]


def run(cmd: list, **kw) -> None:
    proc = subprocess.run([str(c) for c in cmd], capture_output=True, text=True, **kw)
    if proc.returncode:
        sys.stderr.write(proc.stdout + proc.stderr)
        raise SystemExit(f"command failed: {' '.join(map(str, cmd[:6]))} ...")


def outdated(target: Path, deps: list[Path]) -> bool:
    if not target.exists():
        return True
    t = target.stat().st_mtime
    return any(d.exists() and d.stat().st_mtime > t for d in deps)


def depfile_deps(depfile: Path) -> list[Path]:
    if not depfile.exists():
        return []
    text = depfile.read_text(encoding="utf-8", errors="replace").replace("\\\n", " ")
    _, _, deps = text.partition(": ")
    return [Path(d.replace("\\ ", " ")) for d in deps.split() if d != "\\"]


def compile_unit(unit: dict) -> bool:
    out = OBJ_DIR / f"{unit['object']}.o"
    dep = out.with_suffix(".d")
    src = DECOMP / unit["source"]
    if not outdated(out, [src, *depfile_deps(dep)]) and dep.exists():
        return False
    out.parent.mkdir(parents=True, exist_ok=True)
    cmd = ["clang", "-c", *GAME_CFLAGS, *(f"-I{p}" for p in INCLUDES)]
    for name, value in unit["defines"].items():
        cmd.append(f"-D{name}" if value is None else f"-D{name}={value}")
    cmd += ["-MD", "-MF", dep, "-o", out, src]
    run(cmd, cwd=DECOMP)
    return True


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--clean", action="store_true")
    args = parser.parse_args()
    if args.clean:
        for d in (OBJ_DIR, LINK_DIR, GAME_DIR, BUILD / "port_include"):
            shutil.rmtree(d, ignore_errors=True)

    tools = ROOT / "tools"
    print("[1/5] port headers")
    run([sys.executable, tools / "gen_port_headers.py"])

    print("[2/5] compile")
    units = json.loads((BUILD / "units.json").read_text())
    with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
        built = sum(pool.map(compile_unit, units))
    print(f"      {built} of {len(units)} units recompiled")

    print("[3/5] pclink")
    run([sys.executable, tools / "pclink.py"])
    print("      " + (LINK_DIR / "report.txt").read_text().split("\n\n")[0].replace("\n", "\n      "))

    print("[4/5] runtime")
    GAME_DIR.mkdir(parents=True, exist_ok=True)
    extra = []
    run(["clang", f"--target={TARGET}", "-c", LINK_DIR / "psx_abs.s", "-o", GAME_DIR / "psx_abs.obj"])
    extra.append(GAME_DIR / "psx_abs.obj")
    for src in [LINK_DIR / "thunks.c", LINK_DIR / "pcrt_stubs.c", *sorted((ROOT / "port" / "src").glob("*.c"))]:
        obj = GAME_DIR / f"{src.stem}.obj"
        run(["clang", "-c", *RUNTIME_CFLAGS, "-o", obj, src])
        extra.append(obj)

    print("[5/5] archive")
    lib = GAME_DIR / "pe2game.lib"
    lib.unlink(missing_ok=True)
    objects = sorted((LINK_DIR / "obj").rglob("*.obj")) + extra
    rsp = GAME_DIR / "pe2game.rsp"
    rsp.write_text("\n".join(f'"{o}"' for o in objects), encoding="utf-8")
    run(["llvm-lib", f"/out:{lib}", f"@{rsp}"])
    print(f"      {lib} ({len(objects)} objects, {lib.stat().st_size // 1024} KiB)")


if __name__ == "__main__":
    main()
