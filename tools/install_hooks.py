#!/usr/bin/env python3
"""Install the git pre-commit hook that blocks committing game data."""

import stat
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

HOOK = """#!/bin/sh
exec python tools/check_no_game_data.py --staged
"""


def main() -> None:
    git_dir = subprocess.run(["git", "rev-parse", "--git-dir"], cwd=ROOT, check=True,
                             capture_output=True, text=True).stdout.strip()
    hook = (ROOT / git_dir / "hooks" / "pre-commit")
    hook.parent.mkdir(parents=True, exist_ok=True)
    hook.write_text(HOOK, newline="\n")
    hook.chmod(hook.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    print(f"installed {hook}")


if __name__ == "__main__":
    main()
