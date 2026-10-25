#!/usr/bin/env python3
"""Save in snapshot/ what the port's build takes from the decomp's PSX
build, without any of the game's data, so the port builds without the
discs (pclink.from_snapshot).

Run where the discs are, after a normal build (docs/BUILDING.md), whenever
the decomp changes. It saves:

- decomp.txt: the decomp commit it was made from;
- units.json: the game's units and the images that link them;
- symbols/<image>.tsv: each image's symbols (name, address, kind, file):
  the decomp's names and where they are, no bytes;
- images.tsv: each image's fingerprint (hashes the runtime recognizes it
  by in PSX RAM) and a digest of its thunks;
- undefined_funcs_auto.main.txt: the linker's list of the main image's
  functions known by address;
- include/assets/: the asset includes the compile reads, with every value
  taken out: numbers in initializers become 0 and each character of a text
  an x (sizes kept), so names, types, sizes and designators stay.

The game's code reads its data from PSX RAM, loaded from the player's
discs, so the values never mattered to it (DATA_CONST and the wrappers'
static data in build_game.py see to it). It then checks that: no value is
left in the includes, and the port built from the snapshot has the same
machine code as the normal build, unit by unit.

    python tools/make_snapshot.py            # save and check
    python tools/make_snapshot.py --no-check # save only
"""

import argparse
import json
import os
import re
import shutil
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
import pclink  # noqa: E402

DECOMP = ROOT / "decomp"
BUILD = ROOT / "build"
ASSETS = DECOMP / "build" / "include" / "assets"
SNAPSHOT = pclink.SNAPSHOT

# ---- values out --------------------------------------------------------------------

TOKEN = re.compile(
    r'(?P<comment>/\*.*?\*/|//[^\n]*)'
    r'|(?P<string>"(?:[^"\\\n]|\\.)*")'
    r"|(?P<char>'(?:[^'\\\n]|\\.)+')"
    r'|(?P<number>\b(?:0[xX][0-9a-fA-F]+|\d+\.\d*(?:[eE][-+]?\d+)?[fF]?|\d*\.\d+(?:[eE][-+]?\d+)?[fF]?|\d+)[uUlL]*\b)'
    r'|(?P<word>\b[A-Za-z_]\w*\b)'
    r'|(?P<other>[\[\]{};=])',
    re.S,
)


def _values(text: str):
    """Each token of an include that is a value: (match, kind), with the
    rest passed through as (text, None). Values are numbers, texts and
    characters in initializers (after a definition's `=`, inside braces, or
    at the top of a fragment included inside one), not in [...] (sizes,
    designators), enums or preprocessor lines."""
    fragment = not re.search(r"=\s*\{", re.sub(r"/\*.*?\*/", "", text, flags=re.S))
    brackets = braces = 0
    enum_depth, pending_enum = None, False
    # After a top-level `=`, up to its `;`: an initializer, braced or not.
    initializer = False
    for line in text.split("\n"):
        if line.lstrip().startswith("#"):
            yield line + "\n", None
            continue
        pos = 0
        for m in TOKEN.finditer(line):
            yield line[pos:m.start()], None
            tok, kind = m.group(0), m.lastgroup
            value = None
            if kind == "word" and tok == "enum":
                pending_enum = True
            elif kind == "other":
                if tok == "[":
                    brackets += 1
                elif tok == "]":
                    brackets -= 1
                elif tok == "{":
                    braces += 1
                    if pending_enum:
                        enum_depth, pending_enum = braces, False
                elif tok == "}":
                    if enum_depth == braces:
                        enum_depth = None
                    braces -= 1
                elif tok == ";":
                    pending_enum = False
                    if braces == 0:
                        initializer = False
                elif tok == "=" and braces == 0 and enum_depth is None:
                    initializer = True
            elif kind in ("number", "string", "char"):
                if (braces > 0 or fragment or initializer) and brackets == 0 and enum_depth is None:
                    value = kind
            yield tok, value
            pos = m.end()
        yield line[pos:] + "\n", None


def without_values(text: str) -> str:
    """`text` (an asset include) with its values taken out."""
    out = []
    for tok, value in _values(text):
        if value == "number":
            tok = "0"
        elif value == "string":
            tok = '"' + "x" * len(re.sub(r"\\.", "x", tok[1:-1])) + '"'
        elif value == "char":
            tok = "'x'"
        out.append(tok)
    return "".join(out)[:-1]  # the last line's added newline


def values_left(text: str) -> list[str]:
    """The values `text` still holds (an include of the snapshot holds none)."""
    def nonzero(tok: str) -> bool:
        digits = tok.rstrip("uUlLfF") if not tok.lower().startswith("0x") else tok.rstrip("uUlL")
        if digits.lower().startswith("0x"):
            return int(digits, 16) != 0
        if any(c in digits for c in ".eE"):
            return float(digits) != 0
        return int(digits, 10) != 0

    left = []
    for tok, value in _values(text):
        if value == "number" and nonzero(tok):
            left.append(tok)
        elif value == "string" and set(tok[1:-1]) - {"x"}:
            left.append(tok)
        elif value == "char" and tok != "'x'":
            left.append(tok)
    return left


# ---- the snapshot ------------------------------------------------------------------


def used_includes() -> list[Path]:
    """The asset includes the last build compiled with (its depfiles)."""
    sys.path.insert(0, str(ROOT / "tools"))
    import build_game

    used = set()
    for d in (BUILD / "obj").rglob("*.d"):
        for p in build_game.depfile_deps(d):
            p = Path(p).resolve()
            if ASSETS.resolve() in p.parents:
                used.add(p.relative_to(ASSETS.resolve()))
    return sorted(used)


def save() -> None:
    if pclink.from_snapshot():
        raise SystemExit("the decomp's PSX build is not there: build it first (docs/BUILDING.md)")
    units_file = BUILD / "units.json"
    fingerprints = BUILD / "link" / pclink.FINGERPRINTS
    if not units_file.exists() or not fingerprints.exists():
        raise SystemExit("build the port first: python tools/gen_units.py && python tools/build_game.py")
    if SNAPSHOT.exists():
        shutil.rmtree(SNAPSHOT)
    (SNAPSHOT / "symbols").mkdir(parents=True)
    commit = subprocess.run(["git", "-C", str(DECOMP), "rev-parse", "HEAD"], capture_output=True, text=True,
                            check=True).stdout.strip()
    (SNAPSHOT / "decomp.txt").write_text(commit + "\n", encoding="utf-8", newline="\n")
    units = json.loads(units_file.read_text(encoding="utf-8"))
    (SNAPSHOT / "units.json").write_text(json.dumps(units, indent=1) + "\n", encoding="utf-8", newline="\n")
    shutil.copyfile(fingerprints, SNAPSHOT / pclink.FINGERPRINTS)
    shutil.copyfile(DECOMP / "linkers" / "USA" / "undefined_funcs_auto.main.txt",
                    SNAPSHOT / "undefined_funcs_auto.main.txt")
    images = sorted({i for u in units for i in u["overlays"]})
    for image in images:
        lines = [pclink.symbol_line(s) for s in pclink.read_elf_symbols(pclink.image_file(image))]
        (SNAPSHOT / "symbols" / f"{image}.tsv").write_text("".join(lines), encoding="utf-8", newline="\n")
    includes = used_includes()
    for rel in includes:
        text = (ASSETS / rel).read_text(encoding="utf-8", errors="surrogateescape")
        if rel.suffix == ".inc":
            text = without_values(text)
            if left := values_left(text):
                raise SystemExit(f"{rel}: values left after taking them out: {left[:5]}")
        out = SNAPSHOT / "include" / "assets" / rel
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(text, encoding="utf-8", errors="surrogateescape", newline="\n")
    print(f"snapshot/ of decomp {commit[:9]}: {len(units)} units, {len(images)} images, {len(includes)} includes")


# ---- the check ---------------------------------------------------------------------


def code_of(path: Path) -> list:
    """An object's code: each code section's bytes, its relocations by
    target name (compiler-made names' numbers dropped: they count in the
    order made), and its switch tables' contents, sorted."""
    d = path.read_bytes()
    nul = bytes(1)
    nsec = struct.unpack_from("<H", d, 2)[0]
    symptr, nsym = struct.unpack_from("<II", d, 8)
    opt = struct.unpack_from("<H", d, 16)[0]
    strtab = symptr + 18 * nsym

    def long_name(off: int) -> str:
        return d[strtab + off:d.index(nul, strtab + off)].decode("latin-1")

    names, i = {}, 0
    while i < nsym:
        raw = d[symptr + 18 * i:symptr + 18 * i + 8]
        names[i] = long_name(struct.unpack_from("<I", raw, 4)[0]) if raw[:4] == nul * 4 else raw.rstrip(nul).decode(
            "latin-1")
        i += 1 + d[symptr + 18 * i + 17]
    out, tables = [], []
    for k in range(nsec):
        sname, _vs, _va, size, raw, relocs, _ln, nrel, _nln, chars = struct.unpack_from(
            "<8sIIIIIIHHI", d, 20 + opt + 40 * k)
        sname = sname.rstrip(nul).decode("latin-1")
        if sname.startswith("/"):
            sname = long_name(int(sname[1:]))
        if chars & 0x20:  # code
            rel = [(off, re.sub(r"\.\d+$", "", names.get(idx, "?")), typ)
                   for off, idx, typ in (struct.unpack_from("<IIH", d, relocs + 10 * r) for r in range(nrel))]
            out.append((d[raw:raw + size], rel))
        elif "switch.table" in sname:
            tables.append(d[raw:raw + size])
    out.append((b"".join(sorted(tables)), []))
    return out


def build(snapshot: bool) -> None:
    env = dict(os.environ, PE2_SNAPSHOT="1" if snapshot else "0")
    env.pop("PE2_ASSET_INCLUDES", None)
    for tool in ("gen_units.py", "build_game.py"):
        args = [sys.executable, str(ROOT / "tools" / tool)] + (["--clean"] if tool == "build_game.py" else [])
        subprocess.run(args, cwd=ROOT, env=env, check=True, stdout=subprocess.DEVNULL)


def check() -> bool:
    """Builds from the snapshot and compares its code with the normal
    build's; then builds normally again."""
    objects = BUILD / "link" / "obj"
    with tempfile.TemporaryDirectory() as tmp:
        normal = Path(tmp) / "obj"
        shutil.copytree(objects, normal)
        print("building from the snapshot...")
        try:
            build(snapshot=True)
            differ = [p.relative_to(normal) for p in sorted(normal.rglob("*.obj"))
                      if not (objects / p.relative_to(normal)).exists()
                      or code_of(p) != code_of(objects / p.relative_to(normal))]
        finally:
            print("building normally again...")
            build(snapshot=False)
    total = sum(1 for _ in objects.rglob("*.obj"))
    if differ:
        print(f"FAILED: {len(differ)} of {total} objects differ, e.g. {[str(p) for p in differ[:5]]}")
        return False
    print(f"ok: all {total} objects have the same code built from the snapshot")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--no-check", action="store_true", help="save only")
    args = parser.parse_args()
    save()
    return 0 if args.no_check or check() else 1


if __name__ == "__main__":
    sys.exit(main())
