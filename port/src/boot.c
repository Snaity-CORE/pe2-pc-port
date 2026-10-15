/* Booting from the PS-X EXE. See pc/boot.h. */
#include "pc/boot.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

#include "pc/memory.h"
#include "pc/runtime.h"
#include "pc/thunks.h"

/* PS-X EXE header (2 KiB): "PS-X EXE" magic, then load parameters. */
#define EXE_HEADER_SIZE 0x800
#define EXE_OFF_PC0     0x10
#define EXE_OFF_T_ADDR  0x18
#define EXE_OFF_T_SIZE  0x1C

static uint32_t read_u32(const uint8_t* p)
{
    return (uint32_t)p[0] | ((uint32_t)p[1] << 8) | ((uint32_t)p[2] << 16) | ((uint32_t)p[3] << 24);
}

/* One bit per RAM word: set where a thunk to native code is installed, so the
 * MIPS interpreter knows to call native code instead of decoding x86 bytes. */
static uint8_t thunk_bitmap[PC_PSX_RAM_SIZE / 4 / 8];

int pc_thunk_at(uint32_t addr)
{
    uint32_t word = (addr & (PC_PSX_RAM_SIZE - 1)) >> 2;
    return (thunk_bitmap[word >> 3] >> (word & 7)) & 1;
}

static void write_jmp(uint32_t addr, void (*native)(void))
{
    uint8_t* at = (uint8_t*)pc_psx_ptr(addr);
    int32_t rel = (int32_t)((uintptr_t)native - ((uintptr_t)at + 5));
    at[0] = 0xE9; /* jmp rel32 */
    memcpy(at + 1, &rel, sizeof(rel));
}

static void install_thunk(uint32_t addr, void (*native)(void))
{
    write_jmp(addr, native);
    uint32_t word = (addr & (PC_PSX_RAM_SIZE - 1)) >> 2;
    thunk_bitmap[word >> 3] |= (uint8_t)(1u << (word & 7));
}

/* Routines that run in the interpreter still get a thunk (to an
 * interpreter entry), so native code calling them through a PSX address
 * works: library routines without a native implementation, and game
 * functions selected with PE2_INTERPRET. The jmp overwrites their first two
 * MIPS words; the interpreter reads those from this table. */
#define PATCH_SLOTS 65536u /* power of two, open addressing on the address */

typedef struct PatchedCode {
    uint32_t addr; /* 0: empty */
    uint32_t words[2];
} PatchedCode;

static PatchedCode* patched;
static uint8_t patched_bitmap[PC_PSX_RAM_SIZE / 4 / 8];

static PatchedCode* patch_slot(uint32_t addr)
{
    if (!patched)
        patched = (PatchedCode*)calloc(PATCH_SLOTS, sizeof(PatchedCode));
    uint32_t i = (addr >> 2) * 2654435761u;
    for (;;) {
        PatchedCode* slot = &patched[i & (PATCH_SLOTS - 1)];
        if (slot->addr == addr || slot->addr == 0)
            return slot;
        i++;
    }
}

static void set_patched_bits(uint32_t addr, int on)
{
    for (uint32_t a = addr; a < addr + 8; a += 4) {
        uint32_t word = (a & (PC_PSX_RAM_SIZE - 1)) >> 2;
        if (on)
            patched_bitmap[word >> 3] |= (uint8_t)(1u << (word & 7));
        else
            patched_bitmap[word >> 3] &= (uint8_t)~(1u << (word & 7));
    }
}

/* Writes a jmp to `target` over interpreted code at `addr`, keeping the
 * original words for the interpreter. */
static void install_interpreted(uint32_t addr, void (*target)(void))
{
    PatchedCode* slot = patch_slot(addr);
    slot->addr = addr;
    memcpy(slot->words, pc_psx_ptr(addr), sizeof(slot->words));
    set_patched_bits(addr, 1);
    write_jmp(addr, target);
    uint32_t word = (addr & (PC_PSX_RAM_SIZE - 1)) >> 2;
    thunk_bitmap[word >> 3] &= (uint8_t)~(1u << (word & 7));
}

uint32_t pc_code_word(uint32_t addr)
{
    uint32_t word = (addr & (PC_PSX_RAM_SIZE - 1)) >> 2;
    if (addr >= PC_PSX_RAM_BASE && addr < PC_PSX_RAM_BASE + PC_PSX_RAM_SIZE &&
        (patched_bitmap[word >> 3] >> (word & 7)) & 1) {
        uint32_t base = addr & ~3u;
        PatchedCode* slot = patch_slot(base);
        if (slot->addr == base)
            return slot->words[0];
        slot = patch_slot(base - 4);
        if (slot->addr == base - 4)
            return slot->words[1];
    }
    return *(const uint32_t*)pc_psx_ptr(addr & ~3u);
}

void pc_clear_thunks(uint32_t start, uint32_t size)
{
    for (uint32_t addr = start & ~3u; addr < start + size; addr += 4) {
        uint32_t word = (addr & (PC_PSX_RAM_SIZE - 1)) >> 2;
        thunk_bitmap[word >> 3] &= (uint8_t)~(1u << (word & 7));
        patched_bitmap[word >> 3] &= (uint8_t)~(1u << (word & 7));
    }
}

/* Differential tests (libtest.c): makes the interpreter run the original
 * code of the library routine at `addr`, which has a native implementation,
 * instead of following its thunk (on), or restores the thunk (off). */
void pc_runtime_fn_interpret(uint32_t addr, int on)
{
    uint32_t word = (addr & (PC_PSX_RAM_SIZE - 1)) >> 2;
    set_patched_bits(addr, on);
    if (on)
        thunk_bitmap[word >> 3] &= (uint8_t)~(1u << (word & 7));
    else
        thunk_bitmap[word >> 3] |= (uint8_t)(1u << (word & 7));
}

int pc_install_runtime_thunks(void)
{
    int n = 0;
    for (unsigned i = 0; i < pc_runtime_fn_count; i++) {
        const PcRuntimeFn* f = &pc_runtime_fns[i];
        if (f->impl != f->stub) {
            /* Keep the words the jmp replaces, for pc_runtime_fn_interpret. */
            PatchedCode* slot = patch_slot(f->psx_addr);
            slot->addr = f->psx_addr;
            memcpy(slot->words, pc_psx_ptr(f->psx_addr), sizeof(slot->words));
            install_thunk(f->psx_addr, f->impl);
            n++;
        } else {
            install_interpreted(f->psx_addr, f->stub);
        }
    }
    return n;
}

/* ---- PE2_INTERPRET: run selected game functions as original MIPS code ---
 *
 * A differential debugging aid: "all", image names ("gameplay") and PSX
 * addresses ("8009ABCD") separated by commas. Selected functions run their
 * original code in the interpreter instead of the decompiled C, so a
 * rendering or logic difference can be bisected to one function. */

void pc_interp_entry(void); /* dispatch.s: eax = PSX address */

static const char* interpret_spec;
static int interpret_ready;
static uint8_t* entry_pool;
static unsigned entry_used;

static int should_interpret(const char* image, uint32_t addr)
{
    if (!interpret_ready) {
        interpret_spec = getenv("PE2_INTERPRET");
        interpret_ready = 1;
    }
    if (!interpret_spec || !*interpret_spec)
        return 0;
    const char* p = interpret_spec;
    while (*p) {
        const char* end = strchr(p, ',');
        size_t len = end ? (size_t)(end - p) : strlen(p);
        char tok[64];
        if (len >= sizeof(tok))
            len = sizeof(tok) - 1;
        memcpy(tok, p, len);
        tok[len] = 0;
        if (strcmp(tok, "all") == 0 || strcmp(tok, image) == 0)
            return 1;
        char* rest;
        unsigned long a = strtoul(tok, &rest, 16);
        if (*rest == 0 && len > 0 && (uint32_t)a == addr)
            return 1;
        /* "A-B": every function with A <= address < B (hex). */
        if (*rest == '-') {
            unsigned long b = strtoul(rest + 1, &rest, 16);
            if (*rest == 0 && addr >= (uint32_t)a && addr < (uint32_t)b)
                return 1;
        }
        if (!end)
            break;
        p = end + 1;
    }
    return 0;
}

/* A per-function entry: mov eax, addr; jmp pc_interp_entry. */
static void (*interp_entry_for(uint32_t addr))(void)
{
    if (!entry_pool)
        entry_pool = (uint8_t*)VirtualAlloc(NULL, 1 << 20, MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    if (!entry_pool || entry_used + 10 > (1u << 20))
        return NULL;
    uint8_t* e = entry_pool + entry_used;
    entry_used += 10;
    e[0] = 0xB8; /* mov eax, imm32 */
    memcpy(e + 1, &addr, 4);
    e[5] = 0xE9; /* jmp rel32 */
    int32_t rel = (int32_t)((uintptr_t)pc_interp_entry - ((uintptr_t)e + 10));
    memcpy(e + 6, &rel, 4);
    return (void (*)(void))e;
}

/* ---- PE2_SHADOW: check native game functions against the original -----
 *
 * "addr,addr,..." or "A-B" ranges (hex), like PE2_INTERPRET; an address
 * ending in "v" is a void function, whose return value is not compared
 * (both leave whatever is in the return register). Each call of a
 * selected function first runs the original MIPS code in the interpreter,
 * then restores PSX RAM, the scratchpad and the GTE as they were and runs
 * the native body; the two results (memory, GTE registers, return value)
 * must be equal, and the first differences are reported on stderr. The
 * native result is kept. Interrupts are held off meanwhile, so nothing else
 * changes memory between the two runs. State outside PSX memory (host
 * buffers, the GPU) is not compared. */

void pc_shadow_entry(void); /* dispatch.s: eax = shadow slot */
void pc_irq_enter(void);
void pc_irq_leave(void);
unsigned pc_mips_call(unsigned addr, const unsigned* args);
void pcgte_save(uint32_t out[64]);
void pcgte_restore(const uint32_t in[64]);

#define SHADOW_SLOTS 256
#define SCRATCHPAD 0x1F800000u
#define SCRATCHPAD_SIZE 0x400u

static struct {
    uint32_t addr;
    void (*native)(void);
    int is_void;
    unsigned calls, reports;
} shadow[SHADOW_SLOTS];
static unsigned shadow_used;
static const char* shadow_spec;
static int shadow_ready;

/* 0: not shadowed, 1: shadowed, 2: shadowed, void. */
static int should_shadow(uint32_t addr)
{
    if (!shadow_ready) {
        shadow_spec = getenv("PE2_SHADOW");
        shadow_ready = 1;
    }
    if (!shadow_spec || !*shadow_spec)
        return 0;
    for (const char* p = shadow_spec; *p;) {
        char* rest;
        unsigned long a = strtoul(p, &rest, 16);
        if (*rest == '-') {
            unsigned long b = strtoul(rest + 1, &rest, 16);
            if (addr >= (uint32_t)a && addr < (uint32_t)b)
                return 1;
        } else if ((uint32_t)a == addr && rest != p) {
            return *rest == 'v' ? 2 : 1;
        }
        p = strchr(rest, ',');
        if (!p)
            break;
        p++;
    }
    return 0;
}

static void (*shadow_entry_for(uint32_t addr, void (*native)(void), int is_void))(void)
{
    if (shadow_used >= SHADOW_SLOTS)
        return NULL;
    if (!entry_pool)
        entry_pool = (uint8_t*)VirtualAlloc(NULL, 1 << 20, MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    if (!entry_pool || entry_used + 10 > (1u << 20))
        return NULL;
    unsigned slot = shadow_used++;
    shadow[slot].addr = addr;
    shadow[slot].native = native;
    shadow[slot].is_void = is_void;
    uint8_t* e = entry_pool + entry_used;
    entry_used += 10;
    e[0] = 0xB8; /* mov eax, slot */
    memcpy(e + 1, &slot, 4);
    e[5] = 0xE9; /* jmp rel32 */
    int32_t rel = (int32_t)((uintptr_t)pc_shadow_entry - ((uintptr_t)e + 10));
    memcpy(e + 6, &rel, 4);
    return (void (*)(void))e;
}

static uint8_t shadow_ram_before[PC_PSX_RAM_SIZE], shadow_ram_original[PC_PSX_RAM_SIZE];
static uint8_t shadow_pad_before[SCRATCHPAD_SIZE], shadow_pad_original[SCRATCHPAD_SIZE];

static void shadow_report_range(const char* what, uint32_t base, const uint8_t* before, const uint8_t* want,
                                const uint8_t* got, uint32_t size)
{
    int shown = 0;
    for (uint32_t off = 0; off < size && shown < 8; off += 4) {
        uint32_t b, w, g;
        memcpy(&b, before + off, 4);
        memcpy(&w, want + off, 4);
        memcpy(&g, got + off, 4);
        if (w != g) {
            fprintf(stderr, "    %s 0x%08X: before %08X, original %08X, native %08X\n", what, base + off, b, w, g);
            shown++;
        }
    }
}

void pc_runtime_fn_interpret(uint32_t addr, int on);
static int shadow_depth;

unsigned pc_shadow_call(unsigned slot, const unsigned* args)
{
    typedef unsigned (*Fn8)(unsigned, unsigned, unsigned, unsigned, unsigned, unsigned, unsigned, unsigned);
    Fn8 native = (Fn8)shadow[slot].native;
    if (shadow_depth > 0) {
        /* Called from within a comparison (the original calling it): no
         * nested snapshot, just the native body. */
        return native(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
    }
    shadow_depth++;
    uint32_t gte_before[64], gte_original[64], gte_native[64];
    pc_irq_enter();
    uint8_t* ram = (uint8_t*)pc_psx_ptr(PC_PSX_RAM_BASE);
    uint8_t* pad = (uint8_t*)pc_psx_ptr(SCRATCHPAD);
    memcpy(shadow_ram_before, ram, PC_PSX_RAM_SIZE);
    memcpy(shadow_pad_before, pad, SCRATCHPAD_SIZE);
    pcgte_save(gte_before);

    unsigned a[8];
    memcpy(a, args, sizeof a);
    pc_runtime_fn_interpret(shadow[slot].addr, 1);
    unsigned want = pc_mips_call(shadow[slot].addr, a);
    pc_runtime_fn_interpret(shadow[slot].addr, 0);
    memcpy(shadow_ram_original, ram, PC_PSX_RAM_SIZE);
    memcpy(shadow_pad_original, pad, SCRATCHPAD_SIZE);
    pcgte_save(gte_original);

    memcpy(ram, shadow_ram_before, PC_PSX_RAM_SIZE);
    memcpy(pad, shadow_pad_before, SCRATCHPAD_SIZE);
    pcgte_restore(gte_before);
    unsigned got = native(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
    pcgte_save(gte_native);

    unsigned call = ++shadow[slot].calls;
    if (shadow[slot].is_void)
        want = got;
    int differs = want != got || memcmp(shadow_ram_original, ram, PC_PSX_RAM_SIZE) != 0 ||
                  memcmp(shadow_pad_original, pad, SCRATCHPAD_SIZE) != 0 ||
                  memcmp(gte_original, gte_native, sizeof gte_native) != 0;
    if (differs && shadow[slot].reports < 5) {
        shadow[slot].reports++;
        fprintf(stderr, "[shadow] 0x%08X call %u (args %08X %08X %08X %08X) differs:\n", shadow[slot].addr, call,
                args[0], args[1], args[2], args[3]);
        if (want != got)
            fprintf(stderr, "    return: original %08X, native %08X\n", want, got);
        shadow_report_range("ram", PC_PSX_RAM_BASE, shadow_ram_before, shadow_ram_original, ram, PC_PSX_RAM_SIZE);
        /* PE2_SHADOW_SHOW=addr,...: words to print whatever they hold. */
        for (const char* p = getenv("PE2_SHADOW_SHOW"); p && *p;) {
            char* rest;
            uint32_t a = (uint32_t)strtoul(p, &rest, 16) & ~3u;
            if (a >= PC_PSX_RAM_BASE && a < PC_PSX_RAM_BASE + PC_PSX_RAM_SIZE) {
                uint32_t off = a - PC_PSX_RAM_BASE, b, w, g;
                memcpy(&b, shadow_ram_before + off, 4);
                memcpy(&w, shadow_ram_original + off, 4);
                memcpy(&g, ram + off, 4);
                fprintf(stderr, "    show 0x%08X: before %08X, original %08X, native %08X\n", a, b, w, g);
            }
            p = strchr(rest, ',');
            if (!p)
                break;
            p++;
        }
        shadow_report_range("scratchpad", SCRATCHPAD, shadow_pad_before, shadow_pad_original, pad, SCRATCHPAD_SIZE);
        for (int r = 0; r < 64; r++) {
            if (gte_original[r] != gte_native[r])
                fprintf(stderr, "    gte %s %d: original %08X, native %08X\n", r < 32 ? "data" : "ctrl", r & 31,
                        gte_original[r], gte_native[r]);
        }
    } else if (!differs && (call == 1 || call % 1000 == 0) && shadow[slot].reports == 0) {
        fprintf(stderr, "[shadow] 0x%08X: %u calls equal\n", shadow[slot].addr, call);
    }
    pc_irq_leave();
    shadow_depth--;
    return got;
}

int pc_install_thunks(const char* image)
{
    for (unsigned i = 0; i < pc_image_thunk_count; i++) {
        const PcImageThunks* t = &pc_image_thunks[i];
        if (strcmp(t->image, image) != 0)
            continue;
        for (unsigned j = 0; j < t->count; j++) {
            uint32_t addr = t->thunks[j].psx_addr;
            void (*entry)(void) = should_interpret(image, addr) ? interp_entry_for(addr) : NULL;
            int shadowed = entry ? 0 : should_shadow(addr);
            void (*shadow_entry)(void) = shadowed ? shadow_entry_for(addr, t->thunks[j].native, shadowed == 2) : NULL;
            if (entry) {
                install_interpreted(addr, entry);
                continue;
            }
            /* A native thunk (to the function or to the comparison), keeping
             * the original words so pc_runtime_fn_interpret can still run
             * the MIPS code. */
            PatchedCode* slot = patch_slot(addr);
            slot->addr = addr;
            memcpy(slot->words, pc_psx_ptr(addr), sizeof(slot->words));
            install_thunk(addr, shadow_entry ? shadow_entry : t->thunks[j].native);
        }
        return (int)t->count;
    }
    return -1;
}

int pc_boot_load(const uint8_t* exe, size_t size)
{
    pc_crash_handler_install();
    if (pc_memory_init() != 0)
        return PC_BOOT_ERR_MEMORY;
    if (size < EXE_HEADER_SIZE || memcmp(exe, "PS-X EXE", 8) != 0)
        return PC_BOOT_ERR_EXE;

    uint32_t t_addr = read_u32(exe + EXE_OFF_T_ADDR);
    uint32_t t_size = read_u32(exe + EXE_OFF_T_SIZE);
    if (t_addr < PC_PSX_RAM_BASE || t_size > size - EXE_HEADER_SIZE ||
        t_addr + t_size > PC_PSX_RAM_BASE + PC_PSX_RAM_SIZE)
        return PC_BOOT_ERR_EXE;

    memcpy(pc_psx_ptr(t_addr), exe + EXE_HEADER_SIZE, t_size);
    memset(pc_psx_ptr(pc_main_image.bss_start), 0, pc_main_image.bss_end - pc_main_image.bss_start);

    if (pc_install_thunks("main") < 0)
        return PC_BOOT_ERR_THUNKS;
    pc_install_runtime_thunks();
    return PC_BOOT_OK;
}

int pc_boot_run(void)
{
    return pc_main_image.entry();
}
