#!/usr/bin/env python3
"""Extract the game's C translation units and the overlays that link them.

The decomp's configure step (`python ninja_config.py`, run inside `decomp/`)
knows exactly which sources make up the game, including the variant units that
compile one shared source several times with different `-D` parameters (weapon
slots, relocated actor copies). Its PSX build (`ninja`) then links each unit
into one or more overlay images: a shared actor body is linked into every RAM
slot that runs it, each at its own address.

Rather than re-deriving any of that, this tool reads the generated build.ninja
(units and their defines) and the PSX link maps (unit -> images) and writes
`build/units.json`:

    [{"object": "actors/actor_00100/actor_00100",
      "source": "src/actors/actor_00100/actor_00100.c",
      "defines": {"VER_USA": null, ...},
      "overlays": ["actor_400100", "actor_407500"]}, ...]

Overlay names are the PSX image names (`build/USA/out/<name>.elf`), with the
main executable `SLUS_010.42` called `main`.
"""

import argparse
import json
import re
import shlex
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"
OUT_DIR = DECOMP / "build" / "USA" / "out"

BUILD_RE = re.compile(r"^build (\S+): cpp (\S+)")
VAR_RE = re.compile(r"^\s+(\w+) = (.*)$")
MAP_OBJ_RE = re.compile(r"build/USA/src/(\S+)\.c\.o\b")
MAIN_IMAGE = "SLUS_010.42"


def parse_defines(flags: str) -> dict[str, str | None]:
    defines: dict[str, str | None] = {}
    for tok in shlex.split(flags):
        if tok.startswith("-D"):
            # -DNAME (defined as 1) is None; -DNAME= (defined empty) is "".
            name, eq, value = tok[2:].partition("=")
            defines[name] = value if eq else None
    return defines


def parse_units(build_ninja: Path) -> list[dict]:
    units = []
    current = None
    for line in build_ninja.read_text(encoding="utf-8").splitlines():
        if m := BUILD_RE.match(line):
            out, src = m.groups()
            obj = re.sub(r"^build/[^/]+/src/", "", out.replace("\\", "/"))
            current = {
                "object": re.sub(r"\.i$", "", obj),
                "source": src.replace("\\", "/"),
                "defines": {},
                "overlays": [],
            }
            units.append(current)
        elif current is not None and (m := VAR_RE.match(line)):
            name, value = m.groups()
            # PACKAGE_DEFINES: a shared source's per-package parameters
            # (WEAPON_INDEX, ...); PKGID and NONMATCHINGFLAG: older builds.
            if name in ("VERSION", "PACKAGE_DEFINES", "PKGID", "NONMATCHINGFLAG", "SKIPASMFLAG"):
                current["defines"].update(parse_defines(value))
        else:
            current = None
    return units


def parse_maps(out_dir: Path) -> dict[str, set[str]]:
    """object -> set of overlay images that link it."""
    members: dict[str, set[str]] = {}
    maps = sorted(out_dir.glob("*.elf.map"))
    if not maps:
        raise SystemExit(f"no link maps in {out_dir}: run the PSX build (`ninja` in decomp/) first")
    for path in maps:
        image = path.name[: -len(".elf.map")]
        overlay = "main" if image == MAIN_IMAGE else image
        text = path.read_text(encoding="utf-8", errors="replace").replace("\\", "/")
        for obj in set(MAP_OBJ_RE.findall(text)):
            members.setdefault(obj, set()).add(overlay)
    return members


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--build-ninja", type=Path, default=DECOMP / "build.ninja")
    parser.add_argument("--out", type=Path, default=ROOT / "build" / "units.json")
    args = parser.parse_args()

    if not args.build_ninja.exists():
        raise SystemExit(f"{args.build_ninja} not found: run `python ninja_config.py` in decomp/ first")
    units = parse_units(args.build_ninja)
    members = parse_maps(OUT_DIR)
    unlinked = []
    for unit in units:
        unit["overlays"] = sorted(members.get(unit["object"], ()))
        if not unit["overlays"]:
            unlinked.append(unit["object"])
    if unlinked:
        raise SystemExit(f"{len(unlinked)} units are not linked into any image, e.g. {unlinked[:5]}")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(units, indent=1), encoding="utf-8")
    overlays = {o for u in units for o in u["overlays"]}
    shared = sum(1 for u in units if len(u["overlays"]) > 1)
    print(f"wrote {args.out}: {len(units)} units in {len(overlays)} overlays "
          f"({shared} units linked into more than one)")


if __name__ == "__main__":
    main()
