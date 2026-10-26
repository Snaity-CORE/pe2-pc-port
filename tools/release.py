#!/usr/bin/env python3
"""Build a release and publish it as a draft GitHub Release.

Pushing a tag vX.Y.Z makes one on GitHub (.github/workflows/build.yml:
built from snapshot/, then `release.py X.Y.Z --ci`). This makes one on
this machine instead, from the decomp's PSX build. For version X.Y.Z:

1. checks: both trees clean (port and decomp), their commits on GitHub,
   the tag new, crates/pe2-host/Cargo.toml at version X.Y.Z, and the GitHub
   CLI (gh) logged in;
2. builds: units, the game library, the host (release), and its tests;
3. checks the build carries no game data: the repository
   (check_no_game_data.py) and the exe (check_exe_game_data.py);
4. packages dist/pe2-pc-port-vX.Y.Z-windows-x86.zip: pe2.exe, the example
   settings, README, the modding guide and the embedded fonts' licenses;
5. tags the commit vX.Y.Z, pushes the tag and opens a draft release with
   the zip, to review and publish on GitHub.

    python tools/release.py 0.2.0             # all of it
    python tools/release.py 0.2.0 --dry-run   # 1-4 only: no tag, no release
    python tools/release.py 0.2.0 --ci        # on GitHub, for a pushed tag:
                                              # 3-5 of a finished build
    python tools/release.py --nightly         # on GitHub, for a push to main:
                                              # 3-4, then the nightly pre-release

The nightly pre-release keeps main's last build under one link: its tag
`nightly` moves to the built commit and its zip is replaced.
"""

import argparse
import hashlib
import re
import shutil
import subprocess
import sys
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"
TARGET = "i686-pc-windows-msvc"
EXE = ROOT / "target" / TARGET / "release" / "pe2.exe"
DIST = ROOT / "dist"
BRANCH = "main"
NIGHTLY = "nightly"
DISCS = ("Needs your own Parasite Eve II (USA) discs: the game reads them at runtime "
         "(see README.md in the zip).")
# What goes into the zip: (path in the repository, name in the zip).
PACKAGE = [
    (EXE, "pe2.exe"),
    (ROOT / "pe2.example.toml", "pe2.example.toml"),
    (ROOT / "README.md", "README.md"),
    (ROOT / "docs" / "MODDING.md", "MODDING.md"),
    # The fonts pe2.exe embeds: their licenses go with them.
    (ROOT / "crates" / "pe2-host" / "res" / "fonts" / "LICENSE-DejaVu.txt", "LICENSE-DejaVu.txt"),
    (ROOT / "crates" / "pe2-host" / "res" / "fonts" / "LICENSE-CourierPrime.txt", "LICENSE-CourierPrime.txt"),
]


class Failed(Exception):
    pass


def run(cmd: list, cwd: Path = ROOT, capture: bool = False) -> str:
    """Runs `cmd`; its output when `capture`. Raises Failed on an error."""
    shown = " ".join(str(c) for c in cmd)
    print(f"$ {shown}", flush=True)
    result = subprocess.run(cmd, cwd=cwd, text=True, capture_output=capture)
    if result.returncode != 0:
        detail = (result.stderr or result.stdout or "").strip() if capture else ""
        raise Failed(f"{shown} failed ({result.returncode}){': ' + detail if detail else ''}")
    return result.stdout if capture else ""


def git(*args: str, cwd: Path = ROOT) -> str:
    return run(["git", *args], cwd=cwd, capture=True).strip()


def check_tree(path: Path, name: str, branch: str | None) -> None:
    """`path` has no changes, and its HEAD is on GitHub (on `branch` if given)."""
    if git("status", "--porcelain", cwd=path):
        raise Failed(f"{name} has uncommitted changes: commit or stash them first")
    git("fetch", "--quiet", "origin", cwd=path)
    if branch:
        if subprocess.run(["git", "merge-base", "--is-ancestor", "HEAD", f"origin/{branch}"], cwd=path).returncode:
            raise Failed(f"{name}'s HEAD is not on origin/{branch}: push it first")
    elif not git("branch", "-r", "--contains", "HEAD", cwd=path):
        raise Failed(f"{name}'s HEAD is on no branch of origin: push it first")


def check_version(version: str) -> None:
    manifest = tomllib.loads((ROOT / "crates" / "pe2-host" / "Cargo.toml").read_text(encoding="utf-8"))
    have = manifest["package"]["version"]
    if have != version:
        raise Failed(f"crates/pe2-host/Cargo.toml is at version {have}, not {version}: set it and commit first")


def check_tag(tag: str) -> None:
    if git("tag", "--list", tag):
        raise Failed(f"tag {tag} exists here already")
    if git("ls-remote", "--tags", "origin", f"refs/tags/{tag}"):
        raise Failed(f"tag {tag} exists on GitHub already")


def check_gh() -> None:
    if shutil.which("gh") is None:
        raise Failed("the GitHub CLI (gh) is missing: `winget install GitHub.cli`, then `gh auth login`")
    run(["gh", "auth", "status"], capture=True)


def build() -> None:
    run([sys.executable, "tools/gen_units.py"])
    run([sys.executable, "tools/build_game.py"])
    run(["cargo", "build", "--release", "--target", TARGET, "-p", "pe2-host"])
    run(["cargo", "test", "--release", "--target", TARGET, "-p", "pe2-host"])


def check_no_game_data() -> None:
    run([sys.executable, "tools/check_no_game_data.py"])
    sys.path.insert(0, str(ROOT / "tools"))
    import pclink

    if pclink.from_snapshot():
        # Built from snapshot/, where the game's data never was: there is
        # nothing to compare the exe with (no discs here) and nothing it
        # could have taken.
        print("built from snapshot/: no game data to find in the exe")
        return
    run([sys.executable, "tools/check_exe_game_data.py", EXE])


def package(tag: str) -> tuple[Path, str]:
    """The zip and its SHA-256."""
    DIST.mkdir(exist_ok=True)
    out = DIST / f"pe2-pc-port-{tag}-windows-x86.zip"
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as z:
        for path, name in PACKAGE:
            z.write(path, name)
    digest = hashlib.sha256(out.read_bytes()).hexdigest()
    print(f"packaged {out.relative_to(ROOT)} ({out.stat().st_size // 1024} KiB, sha256 {digest})")
    return out, digest


def publish(tag: str, zip_path: Path, digest: str, tagged: bool = False) -> None:
    if not tagged:
        git("tag", "-a", tag, "-m", f"pe2-pc-port {tag}")
        run(["git", "push", "origin", f"refs/tags/{tag}"])
    notes = DIST / f"{tag}-notes.md"
    notes.write_text(f"{DISCS}\n\n`{zip_path.name}` SHA-256: `{digest}`\n", encoding="utf-8")
    # The notes count the commits since the last release, not since nightly.
    last = run(["gh", "release", "list", "--exclude-pre-releases", "--exclude-drafts", "--limit", "1",
                "--json", "tagName", "--jq", ".[].tagName"], capture=True).strip()
    run(["gh", "release", "create", tag, str(zip_path), "--draft", "--verify-tag",
         "--title", f"pe2-pc-port {tag}", "--notes-file", str(notes), "--generate-notes"]
        + (["--notes-start-tag", last] if last else []))
    print(f"draft release {tag} created: review and publish it on GitHub")


def nightly() -> None:
    """Publishes this build as the nightly pre-release: tag `nightly` moved
    to HEAD, its zip replaced (same name, so the same download link)."""
    check_no_game_data()
    zip_path, digest = package(NIGHTLY)
    commit = git("rev-parse", "HEAD")
    notes = DIST / f"{NIGHTLY}-notes.md"
    notes.write_text(
        f"The last build of `{BRANCH}`, from {commit}: not a release, only built and tested by CI.\n\n"
        f"{DISCS}\n\n`{zip_path.name}` SHA-256: `{digest}`\n",
        encoding="utf-8",
    )
    if subprocess.run(["gh", "release", "view", NIGHTLY], cwd=ROOT, capture_output=True).returncode:
        run(["gh", "release", "create", NIGHTLY, str(zip_path), "--prerelease", "--latest=false",
             "--target", commit, "--title", "pe2-pc-port nightly", "--notes-file", str(notes)])
    else:
        run(["gh", "api", "-X", "PATCH", f"repos/{{owner}}/{{repo}}/git/refs/tags/{NIGHTLY}",
             "-f", f"sha={commit}", "-F", "force=true"], capture=True)
        run(["gh", "release", "upload", NIGHTLY, str(zip_path), "--clobber"])
        run(["gh", "release", "edit", NIGHTLY, "--prerelease", "--latest=false", "--notes-file", str(notes)])
    print(f"nightly pre-release now at {commit[:9]}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("version", nargs="?", help="X.Y.Z (or vX.Y.Z)")
    parser.add_argument("--dry-run", action="store_true", help="build, check and package; no tag, no release")
    parser.add_argument("--ci", action="store_true",
                        help="for a pushed tag, after the workflow's build: check, package, draft release")
    parser.add_argument("--nightly", action="store_true",
                        help="for a push to main, after the workflow's build: check, package, nightly pre-release")
    args = parser.parse_args()
    if args.nightly:
        try:
            nightly()
        except Failed as e:
            print(f"release: {e}")
            return 1
        return 0
    if args.version is None:
        parser.error("the version is needed (or --nightly)")
    version = args.version.removeprefix("v")
    if not re.fullmatch(r"\d+\.\d+\.\d+", version):
        print(f"version {args.version!r} is not X.Y.Z")
        return 2
    tag = f"v{version}"
    try:
        check_version(version)
        if args.ci:
            check_no_game_data()
            zip_path, digest = package(tag)
            publish(tag, zip_path, digest, tagged=True)
            return 0
        if not args.dry_run:
            check_tree(ROOT, "the port", BRANCH)
            check_tree(DECOMP, "the decomp", None)
            check_tag(tag)
            check_gh()
        build()
        check_no_game_data()
        zip_path, digest = package(tag)
        if args.dry_run:
            print("dry run: no tag, no release")
            return 0
        publish(tag, zip_path, digest)
    except Failed as e:
        print(f"release: {e}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
