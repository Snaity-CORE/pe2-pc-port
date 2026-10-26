#!/usr/bin/env python3
"""Write the notices of the third-party code pe2.exe is built from, to ship
with it (their licenses ask for it): every crate linked into the
executable (normal dependencies for the release target, not build scripts
or procedural macros), with the license files it ships, anywhere in it but
its tests (some embed others' work: egui's fonts, gilrs' controller
database, OpenH264). A license of a crate's expression none of its files
holds (all of them, when it ships none) gets its standard text
(tools/licenses/, from the SPDX license list), naming the crate's authors.
The same text is printed once, with every crate under it.

    python tools/third_party.py [OUT]    # default: dist/THIRD-PARTY-NOTICES.txt
"""

import json
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TARGET = "i686-pc-windows-msvc"
LICENSES = ROOT / "tools" / "licenses"
LICENSE_FILE = re.compile(r"(?i)^(licen[cs]e|copying|notice|ofl)|licen[cs]e[^/\\]*\.(txt|md)$")


def shipped_crates(metadata: dict) -> list[dict]:
    """The packages linked into pe2-host's executable, by name."""
    packages = {p["id"]: p for p in metadata["packages"]}
    nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
    root = next(p["id"] for p in metadata["packages"] if p["name"] == "pe2-host")
    seen, stack = set(), [root]
    while stack:
        pid = stack.pop()
        if pid in seen:
            continue
        seen.add(pid)
        for dep in nodes[pid]["deps"]:
            if not any(k["kind"] is None for k in dep["dep_kinds"]):
                continue  # build or dev only
            if any("proc-macro" in t["kind"] for t in packages[dep["pkg"]]["targets"]):
                continue  # runs in the compiler
            stack.append(dep["pkg"])
    seen.discard(root)
    return sorted((packages[p] for p in seen), key=lambda p: (p["name"], p["version"]))


def license_files(package: dict) -> list[Path]:
    """The license files a package ships, outside its tests."""
    top = Path(package["manifest_path"]).parent
    found = []
    for dirpath, dirs, files in os.walk(top):
        dirs[:] = sorted(d for d in dirs if d not in ("tests", "target", ".git"))
        found += [Path(dirpath) / f for f in sorted(files) if LICENSE_FILE.search(f)]
    if package.get("license_file"):
        named = top / package["license_file"]
        if named.is_file() and named not in found:
            found.append(named)
    return found


# A phrase each text of tools/licenses has, to tell whether a license file
# shipped with a crate already is one.
MARKERS = {
    "MIT": "Permission is hereby granted, free of charge",
    "Apache-2.0": "Apache License",
    "BSD-2-Clause": "Redistribution and use in source and binary forms",
    "BSD-3-Clause": "Redistribution and use in source and binary forms",
    "MPL-2.0": "Mozilla Public License",
    "Ubuntu-font-1.0": "UBUNTU FONT LICENCE",
}


def standard_text(package: dict, spdx_ids: list[str]) -> str | None:
    """The standard text of the first of `spdx_ids` that tools/licenses
    has, with the package's authors where it names holders."""
    for spdx in spdx_ids:
        path = LICENSES / f"{spdx}.txt"
        if path.is_file():
            text = path.read_text(encoding="utf-8")
            authors = ", ".join(a.split(" <")[0] for a in package.get("authors", [])) or f"the {package['name']} authors"
            text = text.replace("<year> <copyright holders>", authors).replace("<year> <owner>", authors)
            return f"(No file of this license ships with the crate; it is under {spdx}.)\n\n{text}"
    return None


def missing_terms(package: dict, bodies: list[str]) -> list[list[str]]:
    """The terms of the package's license expression (`A AND (B OR C)`)
    none of whose licenses the shipped `bodies` hold, as their SPDX ids;
    a license tools/licenses lacks is taken as shipped."""
    expression = (package.get("license") or "").replace("/", " OR ")
    terms = [re.findall(r"[A-Za-z0-9.\-]+", t) for t in re.split(r"\s+AND\s+", expression) if t.strip()]
    terms = [[i for i in ids if i not in ("OR", "WITH")] for ids in terms]
    known = lambda i: i in MARKERS
    held = lambda i: any(normalized(MARKERS[i]).lower() in normalized(b).lower() for b in bodies)
    return [ids for ids in terms if ids and all(known(i) for i in ids) and not any(held(i) for i in ids)]


def normalized(text: str) -> str:
    return " ".join(text.split())


def main() -> int:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "dist" / "THIRD-PARTY-NOTICES.txt"
    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--filter-platform", TARGET],
        cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8"))
    crates = shipped_crates(metadata)
    # Text (normalized) -> (text, the crates under it).
    texts: dict[str, tuple[str, list[str]]] = {}
    missing = []
    for c in crates:
        line = f"{c['name']} {c['version']} ({c.get('license') or 'no license given'})"
        if c.get("repository"):
            line += f", {c['repository']}"
        bodies = [f.read_text(encoding="utf-8", errors="replace") for f in license_files(c)]
        for ids in missing_terms(c, bodies):
            bodies.append(standard_text(c, ids))
        if not bodies:
            missing.append(c["name"])
            continue
        for body in bodies:
            texts.setdefault(normalized(body), (body.strip(), []))[1].append(line)
    if missing:
        print(f"third_party: no license text for {', '.join(missing)}: add theirs to tools/licenses/")
        return 1
    parts = [
        "Third-party notices for pe2.exe\n"
        "================================\n\n"
        "pe2.exe is built from the crates below (Rust libraries, some with C\n"
        "code or fonts of their own). Each license text is followed by the\n"
        "crates it applies to. Their sources are on crates.io, and at the\n"
        "repositories named. The project's own code is CC0-1.0 (LICENSE).\n"
    ]
    for body, users in sorted(texts.values(), key=lambda t: t[1][0]):
        parts.append("-" * 72 + "\n\n" + "\n".join(users) + "\n\n" + body + "\n")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\n".join(parts), encoding="utf-8", newline="\n")
    print(f"third_party: {len(crates)} crates, {len(texts)} license texts in {out.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
