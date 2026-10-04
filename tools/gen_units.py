#!/usr/bin/env python3
"""Extract the game's C translation units from the decomp's build.ninja.

The decomp's configure step (`python ninja_config.py`, run inside `decomp/`)
knows exactly which sources make up each overlay, including the variant units
that compile one shared source several times with different `-D` parameters
(weapon slots, relocated actor copies). Rather than re-deriving that, this tool
reads the generated build.ninja and writes `build/units.json`:

    [{"overlay": "weapons/mm1", "object": "weapons/mm1/grenade_pistol",
      "source": "src/weapons/grenade_pistol/grenade_pistol.c",
      "defines": {"VER_USA": null, "PKG_ID": "19", ...}}, ...]

`overlay` is `main`, `gameplay`, `title`, or `<family>/<package>`.
"""

import argparse
import json
import re
import shlex
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"

BUILD_RE = re.compile(r"^build (\S+): cpp (\S+)")
VAR_RE = re.compile(r"^\s+(\w+) = (.*)$")
# Overlays nest one level under their family (src/weapons/mm1/...); the three
# hand-configured images (main, gameplay, title) sit directly under src/.
TOP_LEVEL = {"main", "gameplay", "title"}


def parse_defines(flags: str) -> dict[str, str | None]:
    defines: dict[str, str | None] = {}
    for tok in shlex.split(flags):
        if tok.startswith("-D"):
            name, _, value = tok[2:].partition("=")
            defines[name] = value or None
    return defines


def overlay_of(obj: str) -> str:
    parts = obj.split("/")
    if parts[0] in TOP_LEVEL:
        return parts[0]
    return "/".join(parts[:2])


def parse(build_ninja: Path) -> list[dict]:
    units = []
    current = None
    for line in build_ninja.read_text(encoding="utf-8").splitlines():
        if m := BUILD_RE.match(line):
            out, src = m.groups()
            obj = re.sub(r"^build/[^/]+/src/", "", out.replace("\\", "/"))
            obj = re.sub(r"\.i$", "", obj)
            current = {
                "overlay": overlay_of(obj),
                "object": obj,
                "source": src.replace("\\", "/"),
                "defines": {},
            }
            units.append(current)
        elif current is not None and (m := VAR_RE.match(line)):
            name, value = m.groups()
            if name in ("VERSION", "PKGID", "NONMATCHINGFLAG", "SKIPASMFLAG"):
                current["defines"].update(parse_defines(value))
        else:
            current = None
    return units


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--build-ninja", type=Path, default=DECOMP / "build.ninja")
    parser.add_argument("--out", type=Path, default=ROOT / "build" / "units.json")
    args = parser.parse_args()

    if not args.build_ninja.exists():
        raise SystemExit(f"{args.build_ninja} not found: run `python ninja_config.py` in decomp/ first")
    units = parse(args.build_ninja)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(units, indent=1), encoding="utf-8")
    overlays = {u["overlay"] for u in units}
    print(f"wrote {args.out}: {len(units)} units in {len(overlays)} overlays")


if __name__ == "__main__":
    main()
