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
    python tools/build_game.py --sanitize # report undefined behaviour (ubsan.c)
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
    ROOT / "port" / "include" / "psyq",  # the port's own SDK headers
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
# --sanitize: undefined behaviour whose result differs from the R3000's,
# reported at run time by port/src/ubsan.c. Left shifts of negative values
# are left out: with -fwrapv they behave as on the console.
SANITIZE_CFLAGS = ["-fsanitize=shift-exponent,array-bounds"]
RUNTIME_CFLAGS = [f"--target={TARGET}", "-O2", "-Wall", "-Wextra", "-Wno-unused-parameter",
                  f"-I{ROOT / 'port' / 'include'}"]


def run(cmd: list, **kw) -> None:
    proc = subprocess.run([str(c) for c in cmd], capture_output=True, text=True, **kw)
    if proc.returncode:
        sys.stderr.write(proc.stdout + proc.stderr)
        raise SystemExit(f"command failed: {' '.join(map(str, cmd[:6]))} ...")


# The port's own SDK headers. The decomp includes them as <psyq/NAME.h>,
# which port/include resolves before decomp/include.
SDK_HEADERS = ROOT / "port" / "include" / "psyq"


def check_no_sdk_headers() -> None:
    """The native build uses the port's own SDK headers only. A header the
    port does not have yet would silently come from the decomp's SDK copy
    (it is on the include path as decomp/include/psyq/...): refuse that."""
    sdk = (DECOMP / "include" / "psyq").resolve()
    offenders = set()
    for dep in OBJ_DIR.rglob("*.d"):
        for path in depfile_deps(dep):
            try:
                if path.resolve().is_relative_to(sdk):
                    offenders.add(path.resolve().relative_to(sdk).as_posix())
            except OSError:
                pass
    if offenders:
        raise SystemExit("SDK headers without a port version: " + ", ".join(sorted(offenders)) +
                         " (add them to port/include/psyq)")


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


WRAP_DIR = BUILD / "wrap"


def static_functions(units: list[dict]) -> dict[str, list[str]]:
    """Static functions the PSX build emitted for each unit (from its image's
    ELF symbols, attributed by the FILE symbol). clang drops static functions
    nothing in their file references, but other images still call some of
    them by address, so every one the console binary has must exist."""
    sys.path.insert(0, str(ROOT / "tools"))
    import pclink

    by_image: dict[str, list] = {}
    out: dict[str, list[str]] = {}
    for unit in units:
        image = unit["overlays"][0]
        if image not in by_image:
            by_image[image] = pclink.read_elf_symbols(pclink.image_file(image))
        file = f"build/USA/src/{unit['object']}.i"
        names = sorted({s.name for s in by_image[image]
                        if s.file == file and s.is_local and s.typed_func
                        and s.name.isidentifier() and not s.name.startswith("__")})
        out[unit["object"]] = names
    return out


def wrapper_source(unit: dict, keep: list[str]) -> Path:
    """A translation unit that includes the original source, then references
    its static functions so the compiler keeps them."""
    src = (DECOMP / unit["source"]).resolve().as_posix()
    text = f'#include "{src}"\n'
    if keep:
        refs = ", ".join(f"(void*){n}" for n in keep)
        text += f"static void* const pc_keep_statics[] __attribute__((used)) = {{{refs}}};\n"
    path = WRAP_DIR / f"{unit['object']}.c"
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.exists() or path.read_text(encoding="utf-8") != text:
        path.write_text(text, encoding="utf-8")
    return path


def compile_unit(unit: dict, keep: list[str], cflags: list[str], stamp: Path) -> bool:
    out = OBJ_DIR / f"{unit['object']}.o"
    dep = out.with_suffix(".d")
    src = wrapper_source(unit, keep)
    if not outdated(out, [src, stamp, *depfile_deps(dep)]) and dep.exists():
        return False
    out.parent.mkdir(parents=True, exist_ok=True)
    cmd = ["clang", "-c", *cflags, *(f"-I{p}" for p in INCLUDES)]
    for name, value in unit["defines"].items():
        cmd.append(f"-D{name}" if value is None else f"-D{name}={value}")
    cmd += ["-MD", "-MF", dep, "-o", out, src]
    run(cmd, cwd=DECOMP)
    return True


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--clean", action="store_true")
    parser.add_argument("--route-calls", action="store_true",
                        help="debug build: calls between game functions go through the dispatcher, "
                             "so PE2_INTERPRET can run any function as original MIPS code")
    parser.add_argument("--sanitize", action="store_true",
                        help="debug build: report shifts by 32 or more and out-of-bounds array "
                             "indexing at run time ([ubsan] lines on stderr)")
    args = parser.parse_args()
    if args.clean:
        for d in (OBJ_DIR, LINK_DIR, GAME_DIR, BUILD / "port_include"):
            shutil.rmtree(d, ignore_errors=True)

    tools = ROOT / "tools"
    print("[1/5] port headers")
    run([sys.executable, tools / "gen_port_headers.py"])

    print("[2/5] compile")
    units = json.loads((BUILD / "units.json").read_text())
    keep = static_functions(units)
    cflags = GAME_CFLAGS + (SANITIZE_CFLAGS if args.sanitize else [])
    # Every unit is recompiled when the flags or include paths change (e.g.
    # into or out of a sanitized build).
    stamp = OBJ_DIR / "cflags.txt"
    OBJ_DIR.mkdir(parents=True, exist_ok=True)
    # A new port SDK header shadows the decomp's without touching any
    # dependency file, so their names are part of the signature too.
    sdk_headers = sorted(h.relative_to(SDK_HEADERS).as_posix() for h in SDK_HEADERS.rglob("*.h"))
    signature = " ".join([*cflags, *(str(i) for i in INCLUDES), *sdk_headers])
    if not stamp.exists() or stamp.read_text(encoding="utf-8") != signature:
        stamp.write_text(signature, encoding="utf-8")
    with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
        built = sum(pool.map(lambda u: compile_unit(u, keep[u["object"]], cflags, stamp), units))
    print(f"      {built} of {len(units)} units recompiled")
    check_no_sdk_headers()

    print("[3/5] pclink")
    run([sys.executable, tools / "pclink.py", *(["--route-calls"] if args.route_calls else [])])
    print("      " + (LINK_DIR / "report.txt").read_text().split("\n\n")[0].replace("\n", "\n      "))

    print("[4/5] runtime")
    GAME_DIR.mkdir(parents=True, exist_ok=True)
    extra = []
    run(["clang", f"--target={TARGET}", "-c", LINK_DIR / "psx_abs.s", "-o", GAME_DIR / "psx_abs.obj"])
    extra.append(GAME_DIR / "psx_abs.obj")
    for src in sorted((ROOT / "port" / "src").glob("*.s")):
        obj = GAME_DIR / f"{src.stem}.obj"
        run(["clang", f"--target={TARGET}", "-c", src, "-o", obj])
        extra.append(obj)
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
