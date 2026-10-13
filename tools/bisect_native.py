#!/usr/bin/env python3
"""Find which native game function misbehaves, by bisection against the
original MIPS code.

Needs a debug build where calls go through the dispatcher
(`python tools/build_game.py --route-calls` then `cargo build --release`).
Each step runs the game with PE2_INTERPRET set to half of the candidate
functions (they then run as original MIPS code in the interpreter) and
measures a symptom; the half that makes the symptom disappear holds the
culprit.

The symptom is the number of oversized polygons the GPU receives
(PE2_TRACE=gpubig) during a fixed time after the gameplay overlay loads,
which flags broken 3D geometry. Reaching gameplay depends on scripted
button presses landing at the right moment, which varies from run to run,
so a run that has not got there by --reach seconds is abandoned and
retried rather than counted:

    python tools/bisect_native.py --image gameplay --disc "Disc 1.chd" \\
        --input start@20,start@21,start@26,cross@28,cross@30
"""

import argparse
import os
import re
import subprocess
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXE = ROOT / "target" / "i686-pc-windows-msvc" / "release" / "pe2.exe"
THUNKS_C = ROOT / "build" / "link" / "thunks.c"
GAMEPLAY_MARK = b"[images] gameplay loaded"


def image_functions(image: str) -> list[tuple[int, str]]:
    text = THUNKS_C.read_text(encoding="utf-8")
    m = re.search(rf"static const PcThunk thunks_{re.escape(image)}\[\] = \{{(.*?)\n\}};", text, re.S)
    if not m:
        raise SystemExit(f"no thunk table for image {image}")
    return [(int(a, 16), n) for a, n in re.findall(r"\{0x([0-9A-F]+)u, (\w+)\}", m.group(1))]


def run_once(spec: str, args) -> int | None:
    """Oversized polygons drawn in the --measure seconds after the gameplay
    overlay loads, or None if the run did not get there in time."""
    env = dict(os.environ, PE2_INTERPRET=spec, PE2_TRACE="gpubig,images", PE2_INPUT=args.input)
    fd, path = tempfile.mkstemp(suffix=".log")
    os.close(fd)
    try:
        # Log to a file: output captured through a pipe can be lost on
        # Windows when the process is killed.
        with open(path, "wb") as log:
            proc = subprocess.Popen([str(EXE), args.disc], env=env, stdout=log, stderr=log)
        start = time.monotonic()
        reached_at = None
        complete = False
        while proc.poll() is None:
            time.sleep(2)
            now = time.monotonic()
            if reached_at is None:
                if GAMEPLAY_MARK in Path(path).read_bytes():
                    reached_at = now
                elif now - start > args.reach:
                    break
            elif now - reached_at > args.measure:
                complete = True
                break
        exited_early = proc.poll() is not None
        proc.kill()
        proc.wait()
        data = Path(path).read_bytes()
    finally:
        os.unlink(path)
    if exited_early:
        # Crashed or closed: the measurement window is incomplete.
        tail = data[-300:].decode("utf-8", "replace").strip().splitlines()
        print(f"    (run ended early: {tail[-1] if tail else 'no output'})", flush=True)
        return None
    at = data.find(GAMEPLAY_MARK)
    if at < 0 or not complete:
        return None
    return data[at:].count(b"[gpubig]")


def notes_in_window(spec: str, args) -> int:
    """--metric notes: SPU key-ons logged between the --window times (seconds
    since the first key-on) of a run of --total seconds."""
    env = dict(os.environ, PE2_INTERPRET=spec, PE2_TRACE="spu", PE2_INPUT=args.input)
    fd, path = tempfile.mkstemp(suffix=".log")
    os.close(fd)
    try:
        with open(path, "wb") as log:
            proc = subprocess.Popen([str(EXE), args.disc], env=env, stdout=log, stderr=log)
        try:
            proc.wait(timeout=args.total)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait()
        text = Path(path).read_text(encoding="utf-8", errors="replace")
    finally:
        os.unlink(path)
    lo, hi = args.window
    times = [float(m) for m in re.findall(r"\[spu\s+([0-9.]+)\] key on", text)]
    return sum(1 for t in times if lo <= t <= hi)


def symptom(spec: str, args) -> int:
    if args.metric == "notes":
        return notes_in_window(spec, args)
    for attempt in range(args.retries):
        n = run_once(spec, args)
        if n is not None:
            return n
        print(f"    (run did not reach gameplay, retrying {attempt + 1}/{args.retries})", flush=True)
    raise SystemExit(f"interpreting {spec or 'nothing'}: gameplay never reached; "
                     "raise --reach or adjust --input")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--image", default="main")
    parser.add_argument("--disc", required=True)
    parser.add_argument("--input", default="")
    parser.add_argument("--reach", type=int, default=150,
                        help="seconds a run may take to reach gameplay before it is retried")
    parser.add_argument("--measure", type=int, default=60,
                        help="seconds of gameplay to measure")
    parser.add_argument("--retries", type=int, default=8)
    parser.add_argument("--threshold", type=int, default=50,
                        help="symptom count at or below which a run counts as fixed")
    parser.add_argument("--from-addr", type=lambda v: int(v, 16), default=0,
                        help="only consider functions at or above this address (hex)")
    parser.add_argument("--to-addr", type=lambda v: int(v, 16), default=0xFFFFFFFF,
                        help="only consider functions below this address (hex)")
    parser.add_argument("--skip-baseline", action="store_true")
    parser.add_argument("--metric", choices=["gpubig", "notes"], default="gpubig",
                        help="gpubig: oversized polygons after gameplay loads (fixed when few); "
                             "notes: SPU key-ons inside --window (fixed when at least --min-notes)")
    parser.add_argument("--window", type=lambda v: tuple(float(x) for x in v.split(",")), default=(0.0, 1e9),
                        help="notes metric: 'start,end' in seconds since the first key-on")
    parser.add_argument("--total", type=int, default=260, help="notes metric: run length in seconds")
    parser.add_argument("--min-notes", type=int, default=10)
    args = parser.parse_args()

    funcs = [f for f in image_functions(args.image) if args.from_addr <= f[0] < args.to_addr]
    print(f"{len(funcs)} candidate functions in {args.image}", flush=True)
    if not args.skip_baseline:
        print(f"native: {symptom('', args)} oversized polygons", flush=True)

    cands = funcs
    while len(cands) > 1:
        half = cands[: len(cands) // 2]
        spec = f"{half[0][0]:08X}-{half[-1][0] + 1:08X}"
        n = symptom(spec, args)
        fixed = n >= args.min_notes if args.metric == "notes" else n <= args.threshold
        print(f"  interpret {len(half)} functions {spec}: {n} -> {'fixed' if fixed else 'still broken'}",
              flush=True)
        cands = half if fixed else cands[len(cands) // 2:]
    addr, name = cands[0]
    n = symptom(f"{addr:08X}", args)
    print(f"culprit: 0x{addr:08X} {name} (interpreting it alone: {n})", flush=True)


if __name__ == "__main__":
    main()
