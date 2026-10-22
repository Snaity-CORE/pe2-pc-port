#!/usr/bin/env python3
"""Rewrite the game's native objects for the identity-mapped PSX memory model.

The port runs the decompiled C natively but keeps the PlayStation's memory map:
PSX RAM is mapped at its original address (0x80000000), and every global the
game uses lives there, loaded from the player's disc exactly as on the console.
Native code is linked normally; this tool rebinds its symbols so that:

* Data defined in C is not used: each data symbol with a known PSX address
  becomes a reference to `psx_<ADDR>`, an absolute symbol at that address.
* Functions keep their native body, renamed `pcfn_<image>_<ADDR>_<name>`. Taking
  a function's address (an i386 DIR32 relocation) yields `psx_<ADDR>`, where
  the runtime writes a `jmp` to the native body once the image is loaded, so
  function pointers stored in game data keep working.
* Calls (REL32) stay direct when the callee is in the same image or in `main`
  (always resident). Calls into another overlay go through the callee's PSX
  address, i.e. to whichever image is loaded there, as on the console.
* Calls to functions that exist in the PSX image but have no C body (Psy-Q
  libraries, the hand-written assembly routines) go to `pcrt_<name>`,
  implemented by the port runtime.

Addresses come from the decomp's matching PSX build (build/USA/out/*.elf), and
the unit -> image mapping from build/units.json (gen_units.py). Each unit is
rewritten once per image that links it. Outputs, under build/link/:

    obj/<image>/<object>.obj   rewritten COFF objects
    psx_abs.s                  absolute psx_<ADDR> symbols
    thunks.c                   per-image tables of (PSX address, native body)
    pcrt_stubs.c               fallback stubs for runtime functions
    report.txt                 summary and anything left unresolved
"""

import argparse
import collections
import concurrent.futures
import functools
import json
import os
import re
import struct
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DECOMP = ROOT / "decomp"
ELF_DIR = DECOMP / "build" / "USA" / "out"
OBJ_DIR = ROOT / "build" / "obj"
LINK_DIR = ROOT / "build" / "link"
MAIN = "main"
MAIN_IMAGE = "SLUS_010.42"

# i386 COFF relocation types.
REL_I386_DIR32 = 0x06
REL_I386_REL32 = 0x14
# COFF storage classes.
CLASS_EXTERNAL = 2
CLASS_STATIC = 3
SCN_CNT_CODE = 0x20

# `extern T x __asm__("D_foo+20")` names a symbol plus an offset.
OFFSET_NAME_RE = re.compile(r"^(\w+)\+(\d+|0x[0-9a-fA-F]+)$")


# --------------------------------------------------------------------------
# ELF (PSX build) symbol tables


@dataclass
class ElfSym:
    name: str
    addr: int
    is_func: bool
    is_abs: bool
    is_local: bool
    file: str
    # STT_FUNC: a real function (is_func also covers labels in code sections)
    typed_func: bool = False


def read_elf_symbols(path: Path) -> list[ElfSym]:
    data = path.read_bytes()
    if data[:4] != b"\x7fELF" or data[4] != 1 or data[5] != 1:
        raise ValueError(f"{path}: not a 32-bit little-endian ELF")
    shoff, = struct.unpack_from("<I", data, 0x20)
    shentsize, shnum = struct.unpack_from("<HH", data, 0x2E)
    sections = [struct.unpack_from("<IIIIIIIIII", data, shoff + i * shentsize) for i in range(shnum)]
    out: list[ElfSym] = []
    for sec in sections:
        if sec[1] != 2:  # SHT_SYMTAB
            continue
        strtab = sections[sec[6]]
        str_off = strtab[4]
        current_file = ""
        for off in range(sec[4], sec[4] + sec[5], 16):
            st_name, st_value, _size, st_info, _other, st_shndx = struct.unpack_from("<IIIBBH", data, off)
            end = data.index(b"\0", str_off + st_name)
            name = data[str_off + st_name:end].decode("latin-1")
            sym_type, bind = st_info & 0xF, st_info >> 4
            if sym_type == 4:  # STT_FILE
                current_file = name
                continue
            if not name or st_shndx == 0 or sym_type == 3:  # undefined / section
                continue
            is_abs = st_shndx == 0xFFF1
            is_func = sym_type == 2 or (not is_abs and st_shndx < len(sections)
                                        and bool(sections[st_shndx][2] & 0x4))  # SHF_EXECINSTR
            out.append(ElfSym(name, st_value, is_func, is_abs, bind == 0, current_file, sym_type == 2))
    return out


class ImageSymbols:
    """Addresses of one PSX image (main executable or overlay)."""

    def __init__(self, syms: list[ElfSym]):
        self.by_name: dict[str, list[ElfSym]] = collections.defaultdict(list)
        for s in syms:
            self.by_name[s.name].append(s)

    def lookup(self, name: str, unit_stem: str | None = None) -> ElfSym | None:
        cands = self.by_name.get(name)
        if not cands:
            return None
        defined = [s for s in cands if not s.is_abs]
        pool = defined or cands
        # Another unit's local symbol is not this unit's: a static the PSX
        # build inlined here has no symbol of its own (only another unit's
        # static of the same name has one).
        if unit_stem:
            pool = [s for s in pool if not (s.is_local and s.file and Path(s.file).stem != unit_stem)]
            if not pool:
                return None
        if len(pool) == 1:
            return pool[0]
        if unit_stem:
            mine = [s for s in pool if Path(s.file).stem == unit_stem]
            if len(mine) == 1:
                return mine[0]
        globals_ = [s for s in pool if not s.is_local]
        if len(globals_) == 1:
            return globals_[0]
        return None  # ambiguous

    def address(self, name: str, unit_stem: str | None = None) -> int | None:
        if m := OFFSET_NAME_RE.match(name):
            base = self.lookup(m.group(1), unit_stem)
            return None if base is None else base.addr + int(m.group(2), 0)
        s = self.lookup(name, unit_stem)
        return None if s is None else s.addr


def image_file(image: str) -> Path:
    return ELF_DIR / f"{MAIN_IMAGE if image == MAIN else image}.elf"


# --------------------------------------------------------------------------
# COFF objects


@dataclass
class CoffSym:
    index: int
    name: str
    value: int
    section: int
    type: int
    storage: int
    naux: int


@dataclass
class Coff:
    data: bytearray
    nsections: int
    sym_ptr: int
    syms: list[CoffSym]          # primary records only (aux skipped)
    nrecords: int                # total records including aux
    raw_symtab: bytes
    strtab: bytes
    section_chars: list[int]
    section_names: list[str]
    # (file offset of the relocation record, type, file offset of the patched site)
    relocs: list[tuple[int, int, int]] = field(default_factory=list)


def read_coff(path: Path) -> Coff:
    data = bytearray(path.read_bytes())
    machine, nsec, _ts, sym_ptr, nsym, opt_size, _ch = struct.unpack_from("<HHIIIHH", data, 0)
    if machine != 0x14C:
        raise ValueError(f"{path}: not an i386 COFF object")
    str_off = sym_ptr + nsym * 18
    str_size, = struct.unpack_from("<I", data, str_off)
    strtab = bytes(data[str_off:str_off + str_size])
    if str_off + str_size != len(data):
        raise ValueError(f"{path}: symbol/string table is not at the end of the file")

    def sym_name(raw: bytes) -> str:
        if raw[:4] == b"\0\0\0\0":
            off, = struct.unpack_from("<I", raw, 4)
            return strtab[off:strtab.index(b"\0", off)].decode("latin-1")
        return raw.rstrip(b"\0").decode("latin-1")

    chars, names, relocs = [], [], []
    for i in range(nsec):
        hdr = 20 + opt_size + i * 40
        raw_name = bytes(data[hdr:hdr + 8])
        names.append(sym_name(raw_name) if not raw_name.startswith(b"/") else raw_name.decode("latin-1"))
        ptr_raw, ptr_rel = struct.unpack_from("<II", data, hdr + 20)
        nrel, = struct.unpack_from("<H", data, hdr + 32)
        ch, = struct.unpack_from("<I", data, hdr + 36)
        chars.append(ch)
        if ch & 0x01000000:
            raise ValueError(f"{path}: relocation overflow sections are not supported")
        for r in range(nrel):
            off = ptr_rel + r * 10
            va, = struct.unpack_from("<I", data, off)
            rtype, = struct.unpack_from("<H", data, off + 8)
            relocs.append((off, rtype, ptr_raw + va))

    syms = []
    i = 0
    while i < nsym:
        off = sym_ptr + i * 18
        raw = bytes(data[off:off + 8])
        value, section, stype, storage, naux = struct.unpack_from("<IhHBB", data, off + 8)
        syms.append(CoffSym(i, sym_name(raw), value, section, stype, storage, naux))
        i += 1 + naux
    return Coff(data, nsec, sym_ptr, syms, nsym, bytes(data[sym_ptr:str_off]), strtab, chars, names, relocs)


class CoffWriter:
    """Applies symbol renames / new symbols / relocation retargets to a Coff."""

    def __init__(self, coff: Coff):
        self.coff = coff
        self.symtab = bytearray(coff.raw_symtab)
        self.strtab = bytearray(coff.strtab)
        self.nrecords = coff.nrecords
        self.names: dict[str, int] = {}

    def _name_field(self, name: str) -> bytes:
        enc = name.encode("latin-1")
        if len(enc) <= 8:
            return enc.ljust(8, b"\0")
        if name not in self.names:
            self.names[name] = len(self.strtab)
            self.strtab += enc + b"\0"
        return b"\0\0\0\0" + struct.pack("<I", self.names[name])

    def set_symbol(self, index: int, name: str, value: int, section: int, storage: int) -> None:
        off = index * 18
        self.symtab[off:off + 8] = self._name_field(name)
        struct.pack_into("<IhHBB", self.symtab, off + 8, value, section,
                         struct.unpack_from("<H", self.symtab, off + 14)[0], storage, 0)

    def add_undefined(self, name: str) -> int:
        index = self.nrecords
        self.symtab += self._name_field(name) + struct.pack("<IhHBB", 0, 0, 0, CLASS_EXTERNAL, 0)
        self.nrecords += 1
        return index

    def retarget(self, reloc_off: int, index: int) -> None:
        struct.pack_into("<I", self.coff.data, reloc_off + 4, index)

    def finish(self) -> bytes:
        out = self.coff.data[:self.coff.sym_ptr]
        struct.pack_into("<I", out, 12, self.nrecords)
        struct.pack_into("<I", self.strtab, 0, len(self.strtab))
        return bytes(out + self.symtab + self.strtab)


# --------------------------------------------------------------------------
# Rewriting


def c_name(sym: str) -> str:
    """i386 C symbols carry a leading underscore."""
    return sym[1:] if sym.startswith("_") else sym


def native_runtime_names() -> set[str]:
    """Library routines the port implements natively (pcrt_<name> defined in
    the Rust host or in port/src)."""
    names: set[str] = set()
    for path in [*(ROOT / "crates").rglob("*.rs"), *(ROOT / "port" / "src").glob("*.c")]:
        text = path.read_text(encoding="utf-8", errors="replace")
        names |= set(re.findall(r"\bfn pcrt_(\w+)\s*\(", text))
        names |= set(re.findall(r"^[\w\s\*]*\bpcrt_(\w+)\s*\([^;]*$", text, re.M))
    return names


def psx_sym(addr: int) -> str:
    return f"_psx_{addr:08X}"


def pcjmp_sym(addr: int) -> str:
    """Trampoline that jumps to a PSX address. Calls (REL32) must not target
    an absolute symbol directly: MSVC's linker adds the image base to it."""
    return f"_pcjmp_{addr:08X}"


def native_fn(image: str, addr: int, name: str) -> str:
    return f"_pcfn_{image}_{addr:08X}_{name}"


def runtime_fn(name: str) -> str:
    return f"_pcrt_{name}"


def skip_symbol(s: CoffSym) -> bool:
    return (s.naux != 0 or s.section < 0 or s.storage not in (CLASS_EXTERNAL, CLASS_STATIC)
            or s.name.startswith(("@", "??_C", "__real@", "__xmm@", "$", ".")))


@dataclass
class Result:
    image: str
    obj: str
    thunks: list[tuple[int, str]] = field(default_factory=list)
    runtime: set[str] = field(default_factory=set)
    absolutes: set[int] = field(default_factory=set)
    jumps: set[int] = field(default_factory=set)
    unresolved: set[str] = field(default_factory=set)
    unmapped_defs: list[str] = field(default_factory=list)
    section_relative: int = 0


class Linker:
    def __init__(self, units: list[dict], route_calls: bool = False):
        self.units = units
        self.route_calls = route_calls
        images = sorted({img for u in units for img in u["overlays"]})
        with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
            self.elf = dict(zip(images, pool.map(lambda i: ImageSymbols(read_elf_symbols(image_file(i))), images)))
        # Which images define each global natively (from the objects themselves).
        self.native: dict[str, set[str]] = collections.defaultdict(set)
        self.objects: dict[str, Coff] = {}
        with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
            for u, coff in zip(units, pool.map(lambda u: read_coff(OBJ_DIR / f"{u['object']}.o"), units)):
                self.objects[u["object"]] = coff
                for s in coff.syms:
                    if not skip_symbol(s) and s.storage == CLASS_EXTERNAL and s.section > 0:
                        for img in u["overlays"]:
                            self.native[c_name(s.name)].add(img)

    def fallback_address(self, name: str, image: str) -> int | None:
        """Address of a global the PSX image has no symbol of its own for.

        Sub-word globals the C code defines inside a larger PSX object carry
        their address in the decomp's symbol files, and unnamed ones in their
        name (D_<image>_<ADDR>).
        """
        addr = self.symbol_files(image).get(name)
        if addr is None and (m := re.search(r"_([0-9A-F]{8})$", name)):
            addr = int(m.group(1), 16)
        if addr is not None and not 0x80000000 <= addr < 0x80200000:
            return None
        return addr

    @functools.cache
    def symbol_files(self, image: str) -> dict[str, int]:
        sym_dir = DECOMP / "configs" / "USA"
        if image in (MAIN, "gameplay", "title"):
            files = [sym_dir / f"sym.{image}.txt"]
        else:
            files = list((sym_dir / "sym").glob(f"*/{image}.txt"))
        out = {}
        for path in files:
            for m in re.finditer(r"^\s*(\w+)\s*=\s*(0x[0-9A-Fa-f]+)\s*;", path.read_text(encoding="utf-8"), re.M):
                out[m.group(1)] = int(m.group(2), 16)
        return out

    def library_functions(self) -> list[str]:
        """Routines in the main image with no C body: Psy-Q library code and
        the hand-written assembly, known to the PSX build only by address.

        Only symbols the decomp's linker script declares as functions
        (undefined_funcs_auto) qualify. Its data symbols (undefined_syms_auto)
        include the library's lookup tables (rsin_tbl, SQRT, ratan_tbl...):
        a thunk written over one of those corrupts every result that uses it."""
        funcs_file = DECOMP / "linkers" / "USA" / "undefined_funcs_auto.main.txt"
        declared = set(re.findall(r"^(\w+)\s*=", funcs_file.read_text(encoding="utf-8"), re.M))
        # The decomp lists a few functions among its data symbols (callback
        # setters such as DMACallback). Those the port implements natively are
        # functions by definition and need their thunk.
        declared |= native_runtime_names()
        # The game's own hand-written assembly (src/main/hasm) is defined in
        # the image, in code, rather than declared by address.
        for name, syms in self.elf[MAIN].by_name.items():
            if any(s.is_abs or s.is_local or not s.is_func for s in syms):
                continue
            if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name) and not name.startswith(("D_", "jtbl_")):
                declared.add(name)
        out = []
        for name in sorted(declared):
            if self.native.get(name):
                continue
            addr = self.elf[MAIN].address(name)
            if addr is not None and 0x80010000 <= addr < self.main_end:
                out.append(name)
        return out

    def is_import_only(self, name: str, image: str) -> bool:
        """True when `name` is a call into an overlay slot by address: no image
        defines it and its address lies past the main executable. (Psy-Q
        library functions are absolute symbols too, but inside main's range:
        those have no C body and belong to the runtime.)"""
        for img in (image, MAIN):
            for s in self.elf[img].by_name.get(name, []):
                if not s.is_abs:
                    return False
        addr = self.elf[image].address(name) or self.elf[MAIN].address(name)
        return addr is not None and addr >= self.main_end

    @functools.cached_property
    def main_end(self) -> int:
        end = self.elf[MAIN].address("main_VRAM_END")
        if end is None:
            raise SystemExit("main image lacks main_VRAM_END")
        return end

    def address_anywhere(self, name: str, image: str) -> int | None:
        """Address of a global as seen from `image` (its own ELF first)."""
        addr = self.elf[image].address(name)
        if addr is None:
            addr = self.elf[MAIN].address(name)
        if addr is None:
            owners = self.native.get(name, set())
            addrs = {self.elf[o].address(name) for o in owners} - {None}
            if len(addrs) == 1:
                addr = addrs.pop()
        return addr

    def rewrite(self, unit: dict, image: str) -> Result:
        coff = read_coff(OBJ_DIR / f"{unit['object']}.o")  # fresh copy per image
        w = CoffWriter(coff)
        res = Result(image, unit["object"])
        stem = Path(unit["object"]).name
        elf = self.elf[image]

        by_index = {s.index: s for s in coff.syms}
        dir32 = collections.defaultdict(list)
        rel32 = collections.defaultdict(list)
        all_refs = collections.defaultdict(list)
        for off, rtype, _site in coff.relocs:
            idx, = struct.unpack_from("<I", coff.data, off + 4)
            all_refs[idx].append(off)
            if rtype == REL_I386_DIR32:
                dir32[idx].append(off)
            elif rtype == REL_I386_REL32:
                rel32[idx].append(off)

        # section number -> [(offset in section, symbol index)] of remapped data
        remapped_data: dict[int, list[tuple[int, int]]] = collections.defaultdict(list)
        for s in coff.syms:
            if skip_symbol(s):
                continue
            name = c_name(s.name)
            defined = s.section > 0 or (s.section == 0 and s.value > 0)  # value>0: COMMON
            if defined:
                is_code = s.section > 0 and bool(coff.section_chars[s.section - 1] & SCN_CNT_CODE)
                info = elf.lookup(name, stem)
                if info is not None and not info.is_abs:
                    addr = info.addr
                elif s.storage == CLASS_EXTERNAL and (addr := self.fallback_address(name, image)) is not None:
                    pass
                else:
                    if s.storage == CLASS_EXTERNAL:
                        res.unmapped_defs.append(name)
                    continue
                res.absolutes.add(addr)
                if is_code:
                    fn = native_fn(image, addr, name)
                    w.set_symbol(s.index, fn, s.value, s.section, CLASS_EXTERNAL)
                    res.thunks.append((addr, fn))
                    if dir32.get(s.index):
                        psx = w.add_undefined(psx_sym(addr))
                        for off in dir32[s.index]:
                            w.retarget(off, psx)
                else:
                    # Keep the native definition under a dead, unique name (a
                    # COMDAT section must keep its defining symbol; the linker
                    # drops it as unreferenced) and send every reference to
                    # the PSX address instead.
                    psx = w.add_undefined(psx_sym(addr))
                    if s.section > 0:
                        remapped_data[s.section].append((s.value, psx))
                    w.set_symbol(s.index, f"_pcdead_{image}_{addr:08X}_{name}", s.value, s.section,
                                 CLASS_EXTERNAL if s.section > 0 else s.storage)
                    for off in all_refs.get(s.index, []):
                        w.retarget(off, psx)
                continue

            # Undefined reference.
            addr = self.address_anywhere(name, image)
            owners = self.native.get(name, set())
            if rel32.get(s.index):
                if self.route_calls and addr is not None and owners:
                    # Debug builds: every call to a game function goes through
                    # the dispatcher, so PE2_INTERPRET can swap any function.
                    target = pcjmp_sym(addr)
                    res.jumps.add(addr)
                    res.absolutes.add(addr)
                elif image in owners and addr is not None:
                    target = native_fn(image, self.elf[image].address(name), name)
                elif MAIN in owners and self.elf[MAIN].address(name) is not None:
                    target = native_fn(MAIN, self.elf[MAIN].address(name), name)
                elif addr is not None and (owners or self.is_import_only(name, image)):
                    # Code in another image (or only known as an absolute
                    # import): dispatch through its PSX address to whatever
                    # is loaded there.
                    target = pcjmp_sym(addr)
                    res.jumps.add(addr)
                    res.absolutes.add(addr)
                elif self.elf[MAIN].lookup(name) is not None:
                    target = runtime_fn(name)  # PSX code with no C body
                    res.runtime.add(name)
                else:
                    target = None  # port runtime symbol (e.g. pcgte_*) or CRT
                if target is not None:
                    w.set_symbol(s.index, target, 0, 0, CLASS_EXTERNAL)
                    if dir32.get(s.index):
                        if addr is None:
                            res.unresolved.add(name)
                        else:
                            psx = w.add_undefined(psx_sym(addr))
                            res.absolutes.add(addr)
                            for off in dir32[s.index]:
                                w.retarget(off, psx)
            elif addr is not None:
                w.set_symbol(s.index, psx_sym(addr), 0, 0, CLASS_EXTERNAL)
                res.absolutes.add(addr)
            elif self.elf[MAIN].lookup(name) is None and not owners:
                pass  # port runtime / CRT symbol: leave as is
            else:
                res.unresolved.add(name)

        # A reference through a section symbol plus an offset (how clang
        # addresses some internal data) must follow the data to its PSX
        # address too. With -fdata-sections each object has its own section,
        # so the containing symbol is the last one at or before the offset.
        for off, rtype, site in coff.relocs:
            idx, = struct.unpack_from("<I", coff.data, off + 4)
            sym = by_index.get(idx)
            if not (sym and sym.naux and sym.section in remapped_data and rtype == REL_I386_DIR32):
                continue
            addend, = struct.unpack_from("<i", coff.data, site)
            owners = [(v, i) for v, i in remapped_data[sym.section] if v <= addend]
            if not owners:
                continue  # anonymous data before any remapped object: stays native
            value, owner = max(owners)
            struct.pack_into("<i", coff.data, site, addend - value)
            w.retarget(off, owner)
            res.section_relative += 1

        out = LINK_DIR / "obj" / image / f"{unit['object']}.obj"
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(w.finish())
        return res


FNV_OFFSET = 0xCBF29CE484222325
FNV_PRIME = 0x100000001B3
PREFIX_BYTES = 16
EDGE_BYTES = 4096


def masked_fnv(data: bytes, start: int, end: int, masked: list[tuple[int, int]]) -> int:
    """FNV-1a 64 over data[start:end], with thunk-covered byte ranges read as 0.

    The runtime computes the same over PSX RAM to recognize a loaded image,
    before or after its thunks are written."""
    h = FNV_OFFSET
    mi = 0
    for i in range(start, end):
        while mi < len(masked) and masked[mi][1] <= i:
            mi += 1
        b = 0 if mi < len(masked) and masked[mi][0] <= i < masked[mi][1] else data[i]
        h = ((h ^ b) * FNV_PRIME) & 0xFFFFFFFFFFFFFFFF
    return h


def image_fingerprint(linker: "Linker", image: str, thunk_addrs: list[int]) -> tuple[int, int, int, int, int]:
    """Load address, size, the size of its constant part and the two hashes
    the runtime uses to recognize `image` in PSX RAM (only hashes reach the
    binary, never image bytes)."""
    elf = linker.elf[image]
    if image == MAIN:
        load = elf.address("main_VRAM")
        raw = (ELF_DIR / MAIN_IMAGE).read_bytes()[0x800:]
    else:
        load = elf.address(f"{image}_VRAM")
        raw = (ELF_DIR / image).read_bytes()
    if load is None:
        raise SystemExit(f"{image}: no load address")
    size = len(raw)
    masked = sorted((a - load, a - load + 5) for a in thunk_addrs if 0 <= a - load < size)
    # Hash the constant part only: rodata then code, which the game never
    # writes. The data after it changes as soon as the game uses the image, so
    # hashing it can miss an image that is really there.
    rodata = elf.address(f"{image}_RODATA_SIZE")
    text = elf.address(f"{image}_TEXT_SIZE")
    code_size = size if rodata is None or text is None else min(size, rodata + text)
    full = masked_fnv(raw, 0, code_size, masked)
    return load, size, code_size, masked_fnv(raw, 0, min(PREFIX_BYTES, size), masked), full


def write_outputs(results: list[Result], linker: "Linker") -> None:
    absolutes = sorted({a for r in results for a in r.absolutes})
    with open(LINK_DIR / "psx_abs.s", "w", newline="\n") as f:
        f.write("# GENERATED by tools/pclink.py: PSX addresses as absolute symbols.\n")
        for a in absolutes:
            f.write(f".globl {psx_sym(a)}\n.set {psx_sym(a)}, 0x{a:08X}\n")
        # Trampolines for native calls into other images (see pcjmp_sym),
        # through port/src/dispatch.s. eax is free at a cdecl call.
        f.write("\n.text\n")
        for a in sorted({a for r in results for a in r.jumps}):
            f.write(f".globl {pcjmp_sym(a)}\n{pcjmp_sym(a)}:\n    movl $0x{a:08X}, %eax\n    jmp _pc_psx_dispatch\n")

    by_image: dict[str, list[tuple[int, str]]] = collections.defaultdict(list)
    for r in results:
        by_image[r.image].extend(r.thunks)
    with open(LINK_DIR / "thunks.c", "w", newline="\n") as f:
        f.write("/* GENERATED by tools/pclink.py: native bodies of each image's functions. */\n")
        f.write('#include "pc/thunks.h"\n\n')
        for image, thunks in sorted(by_image.items()):
            for _, fn in sorted(set(thunks)):
                f.write(f"extern void {fn[1:]}(void);\n")
        for image, thunks in sorted(by_image.items()):
            f.write(f"\nstatic const PcThunk thunks_{image}[] = {{\n")
            for addr, fn in sorted(set(thunks)):
                f.write(f"    {{0x{addr:08X}u, {fn[1:]}}},\n")
            f.write("};\n")
        f.write("\nconst PcImageThunks pc_image_thunks[] = {\n")
        for image, thunks in sorted(by_image.items()):
            load, size, code_size, prefix_hash, full_hash = image_fingerprint(
                linker, image, sorted({a for a, _ in thunks}))
            f.write(f'    {{"{image}", thunks_{image}, {len(set(thunks))}, 0x{load:08X}u, 0x{size:X}u, '
                    f'0x{code_size:X}u, 0x{prefix_hash:016X}ull, 0x{full_hash:016X}ull}},\n')
        f.write("};\n")
        f.write(f"const unsigned pc_image_thunk_count = {len(by_image)};\n")

        # Library data the native runtime initializes the way the replaced
        # library routines did (addresses only).
        for name in ("GEnv",):
            addr = linker.elf[MAIN].address(name)
            if addr is None:
                raise SystemExit(f"main image lacks {name}")
            f.write(f"const unsigned pc_sym_{name} = 0x{addr:08X}u;\n")

        # Boot facts about the main executable: its C entry point and the BSS
        # range crt0 clears (addresses only; the image itself comes from disc).
        main_syms = linker.elf[MAIN]
        entry = main_syms.address("main")
        bss_start = main_syms.address("main_BSS_START")
        bss_end = main_syms.address("main_BSS_END")
        if None in (entry, bss_start, bss_end):
            raise SystemExit("main image lacks main / main_BSS_START / main_BSS_END")
        f.write(f"\nconst PcMainImage pc_main_image = {{\n"
                f"    (int (*)(void)){native_fn(MAIN, entry, 'main')[1:]},\n"
                f"    0x{bss_start:08X}u,\n"
                f"    0x{bss_end:08X}u,\n"
                f"}};\n")

    # Runtime functions: those the C code calls, plus every other library
    # routine in the main image, so that a native implementation of any of
    # them also serves calls from interpreted library code (via a thunk at
    # its PSX address).
    referenced = {n for r in results for n in r.runtime}
    runtime = sorted(referenced | set(linker.library_functions()))
    with open(LINK_DIR / "pcrt_stubs.c", "w", newline="\n") as f:
        f.write("/* GENERATED by tools/pclink.py: default implementations of the runtime\n"
                " * functions. Each pcrt_X resolves to pcrt_stub_X through /alternatename\n"
                " * unless the runtime defines pcrt_X natively; the stub runs the original\n"
                " * MIPS code in PSX RAM with the interpreter. */\n")
        f.write('#include "pc/thunks.h"\n\n')
        f.write("unsigned pc_mips_call(unsigned addr, const unsigned* args);\n\n")
        for n in runtime:
            addr = linker.elf[MAIN].address(n)
            if addr is None:
                raise SystemExit(f"runtime function {n} has no address in the main image")
            f.write(f"unsigned pcrt_{n}();\n")
            f.write(f"unsigned pcrt_stub_{n}(unsigned a0, unsigned a1, unsigned a2, unsigned a3,\n"
                    f"    unsigned a4, unsigned a5, unsigned a6, unsigned a7)\n"
                    f"{{\n"
                    f"    const unsigned args[8] = {{a0, a1, a2, a3, a4, a5, a6, a7}};\n"
                    f"    return pc_mips_call(0x{addr:08X}u, args);\n"
                    f"}}\n")
            f.write(f'#pragma comment(linker, "/alternatename:_pcrt_{n}=_pcrt_stub_{n}")\n')
        f.write("\nconst PcRuntimeFn pc_runtime_fns[] = {\n")
        for n in runtime:
            addr = linker.elf[MAIN].address(n)
            f.write(f'    {{0x{addr:08X}u, (void (*)(void))pcrt_{n}, (void (*)(void))pcrt_stub_{n}, "{n}"}},\n')
        f.write("};\n")
        f.write(f"const unsigned pc_runtime_fn_count = {len(runtime)};\n")

    unresolved = collections.Counter(n for r in results for n in r.unresolved)
    unmapped = collections.Counter(n for r in results for n in r.unmapped_defs)
    with open(LINK_DIR / "report.txt", "w", newline="\n") as f:
        f.write(f"objects rewritten: {len(results)}\n")
        f.write(f"absolute PSX symbols: {len(absolutes)}\n")
        f.write(f"thunks: {sum(len(set(t)) for t in by_image.values())} in {len(by_image)} images\n")
        f.write(f"runtime functions (pcrt_): {len(runtime)}\n")
        f.write(f"section-relative data references retargeted: {sum(r.section_relative for r in results)}\n")
        f.write(f"\nunmapped global definitions (no PSX address): {len(unmapped)}\n")
        for n, c in unmapped.most_common():
            f.write(f"  {n} ({c})\n")
        f.write(f"\nunresolved references: {len(unresolved)}\n")
        for n, c in unresolved.most_common():
            f.write(f"  {n} ({c})\n")
        f.write("\nruntime functions:\n")
        for n in runtime:
            f.write(f"  {n}\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--units", type=Path, default=ROOT / "build" / "units.json")
    parser.add_argument("--route-calls", action="store_true",
                        help="send every call between game functions through the dispatcher (debugging)")
    args = parser.parse_args()

    units = json.loads(args.units.read_text())
    linker = Linker(units, route_calls=args.route_calls)
    jobs = [(u, img) for u in units for img in u["overlays"]]
    with concurrent.futures.ThreadPoolExecutor(os.cpu_count()) as pool:
        results = list(pool.map(lambda j: linker.rewrite(*j), jobs))
    write_outputs(results, linker)
    print((LINK_DIR / "report.txt").read_text().split("\nunmapped")[0])


if __name__ == "__main__":
    main()
