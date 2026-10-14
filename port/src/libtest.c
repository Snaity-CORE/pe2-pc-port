/*
 * PE2_LIBTEST=<names|all>[:iterations]: differential tests of the port's
 * library replacements against the original library code.
 *
 * Each case fills a scratch area of PSX RAM and the GTE registers with
 * random values, runs the original routine (pcrt_stub_X, the MIPS code in
 * the main executable, through the interpreter), then restores the same
 * state and runs the port's routine (pcrt_X). Everything either can touch
 * is compared: the scratch area, all 64 GTE registers and the return value.
 * A routine the port does not replace yet resolves pcrt_X to the stub
 * itself, so its case passes trivially.
 *
 * Runs after the main executable is loaded, before the game starts (main.rs).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "pc/gte.h"
#include "pc/thunks.h"

void pc_runtime_fn_interpret(uint32_t addr, int on);

void pcgte_save(uint32_t out[64]);
void pcgte_restore(const uint32_t in[64]);

/* Scratch area, above the main executable's BSS, and the argument slots. */
#define SCRATCH      0x80100000u
#define SCRATCH_SIZE 0x400u
#define A0           (SCRATCH + 0x000u)
#define A1           (SCRATCH + 0x100u)
#define A2           (SCRATCH + 0x200u)
#define A3           (SCRATCH + 0x300u)

typedef uint32_t (*LibFn)(uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t);

/* ---- random inputs -------------------------------------------------------- */

static uint64_t rng = 0x9E3779B97F4A7C15ull;

static uint32_t rnd(void)
{
    rng ^= rng << 13;
    rng ^= rng >> 7;
    rng ^= rng << 17;
    return (uint32_t)(rng >> 16);
}

/* A value with a varied magnitude: small, medium, 16-bit, full or an edge. */
static int32_t rnd_value(void)
{
    static const int32_t edges[] = {0, 1, -1, 4096, -4096, 0x7FFF, -0x8000, 0x7FFFFFFF, (int32_t)0x80000000};
    switch (rnd() % 6) {
    case 0: return edges[rnd() % (sizeof edges / sizeof edges[0])];
    case 1: return (int32_t)(rnd() % 64) - 32;
    case 2: return (int32_t)(rnd() % 8192) - 4096;
    case 3: return (int16_t)rnd();
    case 4: return (int32_t)(rnd() % 0x200000) - 0x100000;
    default: return (int32_t)rnd();
    }
}

/* An angle: mostly 0..4095 and nearby, sometimes anything. */
static int32_t rnd_angle(void)
{
    return rnd() % 4 ? (int32_t)(rnd() % 12288) - 4096 : rnd_value();
}

static void w16(uint32_t addr, int32_t v) { *(int16_t*)(uintptr_t)addr = (int16_t)v; }
static void w32(uint32_t addr, int32_t v) { *(int32_t*)(uintptr_t)addr = v; }

/* MATRIX: short m[3][3] (and padding), long t[3] at +20. Rotation-sized
 * elements (|m| <= 4096) most of the time, as the game uses them. */
static void rnd_matrix(uint32_t at)
{
    int full = rnd() % 4 == 0;
    for (int i = 0; i < 10; i++) {
        w16(at + 2 * i, full ? (int16_t)rnd() : (int32_t)(rnd() % 8193) - 4096);
    }
    for (int i = 0; i < 3; i++) {
        w32(at + 20 + 4 * i, rnd_value());
    }
}

/* VECTOR: long vx, vy, vz, pad. */
static void rnd_vector(uint32_t at)
{
    for (int i = 0; i < 4; i++) {
        w32(at + 4 * i, rnd_value());
    }
}

/* SVECTOR: short vx, vy, vz, pad. */
static void rnd_svector(uint32_t at, int angles)
{
    for (int i = 0; i < 4; i++) {
        w16(at + 2 * i, angles ? rnd_angle() : rnd_value());
    }
}

/* ---- cases --------------------------------------------------------------- */

typedef struct {
    const char* name;
    LibFn native;
    LibFn original;
    void (*setup)(uint32_t args[8]);
} LibCase;

static void s_angle(uint32_t a[8]) { a[0] = (uint32_t)rnd_angle(); }
static void s_value(uint32_t a[8]) { a[0] = (uint32_t)rnd_value(); }
/* A non-negative value (square roots: negative inputs are not meaningful). */
static void s_unsigned(uint32_t a[8]) { a[0] = (uint32_t)rnd_value() & 0x7FFFFFFF; }
static void s_value2(uint32_t a[8]) { a[0] = (uint32_t)rnd_value(); a[1] = (uint32_t)rnd_value(); }
static void s_sangles_m(uint32_t a[8]) { rnd_svector(A0, 1); rnd_matrix(A1); a[0] = A0; a[1] = A1; }
static void s_angle_m(uint32_t a[8]) { rnd_matrix(A1); a[0] = (uint32_t)rnd_angle(); a[1] = A1; }
static void s_m(uint32_t a[8]) { rnd_matrix(A0); a[0] = A0; }
/* A matrix of rotation size (|m| <= 4096). */
static void rnd_rotation_matrix(uint32_t at)
{
    rnd_matrix(at);
    for (int i = 0; i < 9; i++) {
        w16(at + 2 * i, (int32_t)(rnd() % 8193) - 4096);
    }
}
static void s_rot_m_m(uint32_t a[8]) { rnd_rotation_matrix(A0); rnd_matrix(A1); a[0] = A0; a[1] = A1; }
static void s_m_m(uint32_t a[8]) { rnd_matrix(A0); rnd_matrix(A1); a[0] = A0; a[1] = A1; }
static void s_m_m_m(uint32_t a[8]) { rnd_matrix(A0); rnd_matrix(A1); rnd_matrix(A2); a[0] = A0; a[1] = A1; a[2] = A2; }
static void s_m_v(uint32_t a[8]) { rnd_matrix(A0); rnd_vector(A1); a[0] = A0; a[1] = A1; }
static void s_m_v_v(uint32_t a[8]) { rnd_matrix(A0); rnd_vector(A1); rnd_vector(A2); a[0] = A0; a[1] = A1; a[2] = A2; }
static void s_m_sv_v(uint32_t a[8]) { rnd_matrix(A0); rnd_svector(A1, 0); rnd_vector(A2); a[0] = A0; a[1] = A1; a[2] = A2; }
static void s_m_sv_sv(uint32_t a[8]) { rnd_matrix(A0); rnd_svector(A1, 0); rnd_svector(A2, 0); a[0] = A0; a[1] = A1; a[2] = A2; }
/* Vectors to normalize: any high bits (the routines use the low 16), low
 * halves small enough that the sum of squares stays below 2^31. */
static int32_t rnd_normal_component(void)
{
    int32_t low = (int32_t)(rnd() % 52001) - 26000;
    if (rnd() % 4 == 0) {
        low = (int32_t)(rnd() % 64) - 32;
    }
    return (int32_t)((rnd() << 16) | (uint16_t)low);
}
static void s_normal_v_v(uint32_t a[8])
{
    for (int i = 0; i < 4; i++) { w32(A0 + 4 * i, rnd_normal_component()); }
    rnd_vector(A1); a[0] = A0; a[1] = A1;
}
static void s_normal_v_sv(uint32_t a[8])
{
    for (int i = 0; i < 4; i++) { w32(A0 + 4 * i, rnd_normal_component()); }
    rnd_svector(A1, 0); a[0] = A0; a[1] = A1;
}
static void s_normal_sv_sv(uint32_t a[8])
{
    for (int i = 0; i < 4; i++) { w16(A0 + 2 * i, rnd_normal_component()); }
    rnd_svector(A1, 0); a[0] = A0; a[1] = A1;
}
static void s_v_v(uint32_t a[8]) { rnd_vector(A0); rnd_vector(A1); a[0] = A0; a[1] = A1; }
static void s_v_sv(uint32_t a[8]) { rnd_vector(A0); rnd_svector(A1, 0); a[0] = A0; a[1] = A1; }
static void s_sv_sv(uint32_t a[8]) { rnd_svector(A0, 0); rnd_svector(A1, 0); a[0] = A0; a[1] = A1; }

/* Graphics library set-ups: packets of random words, small integers. */
static void rnd_words(uint32_t at, int n)
{
    for (int i = 0; i < n; i++) {
        w32(at + 4 * i, (int32_t)rnd());
    }
}
static int32_t rnd_small(int n) { return (int32_t)(rnd() % (uint32_t)n); }
static void s_ot_p(uint32_t a[8]) { rnd_words(A0, 4); rnd_words(A1, 8); a[0] = A0; a[1] = A1; }
static void s_p_p(uint32_t a[8])
{
    rnd_words(A0, 16); rnd_words(A1, 16);
    /* Packet lengths that sometimes fit together and sometimes do not. */
    *(uint8_t*)(uintptr_t)(A0 + 3) = (uint8_t)rnd_small(12);
    *(uint8_t*)(uintptr_t)(A1 + 3) = (uint8_t)rnd_small(12);
    a[0] = A0; a[1] = A1;
}
static void s_none(uint32_t a[8]) { (void)a; }
static void s_p(uint32_t a[8]) { rnd_words(A0, 16); a[0] = A0; }
static void s_p_flag(uint32_t a[8]) { rnd_words(A0, 16); a[0] = A0; a[1] = rnd() % 3 ? (uint32_t)rnd_small(2) : rnd(); }
static void s_tpage(uint32_t a[8])
{
    a[0] = (uint32_t)rnd_small(4); a[1] = (uint32_t)rnd_small(4);
    a[2] = rnd() % 4 ? (uint32_t)rnd_small(1024) : rnd(); a[3] = rnd() % 4 ? (uint32_t)rnd_small(1024) : rnd();
}
static void s_clut(uint32_t a[8]) { a[0] = rnd() % 4 ? (uint32_t)rnd_small(1024) : rnd(); a[1] = rnd() % 4 ? (uint32_t)rnd_small(512) : rnd(); }
static void s_drawtpage(uint32_t a[8])
{
    rnd_words(A0, 4); a[0] = A0;
    a[1] = rnd() % 3 ? (uint32_t)rnd_small(2) : rnd(); a[2] = rnd() % 3 ? (uint32_t)rnd_small(2) : rnd(); a[3] = rnd();
}
static void s_drawoffset(uint32_t a[8]) { rnd_words(A0, 4); rnd_words(A1, 2); a[0] = A0; a[1] = A1; }
static void s_drawmove(uint32_t a[8])
{
    rnd_words(A0, 8); rnd_words(A1, 2);
    if (rnd() % 4 == 0) { w16(A1 + 4, 0); }
    a[0] = A0; a[1] = A1; a[2] = (uint32_t)rnd_small(1024); a[3] = (uint32_t)rnd_small(512);
}
static void s_defenv(uint32_t a[8])
{
    rnd_words(A0, 32); a[0] = A0;
    a[1] = (uint32_t)rnd_small(1024); a[2] = (uint32_t)rnd_small(512);
    a[3] = (uint32_t)(rnd() % 2 ? 256 + rnd_small(385) : rnd_small(1024));
    a[4] = (uint32_t)(rnd() % 2 ? 200 + rnd_small(300) : rnd_small(512));
}
static void s_gsclearot(uint32_t a[8])
{
    rnd_words(A0, 8); rnd_words(A1, 64);
    w32(A0, (int32_t)(rnd() % 7));   /* length: up to 64 entries */
    w32(A0 + 4, (int32_t)A1);        /* org */
    a[0] = (uint32_t)rnd() & 0xFFFF; a[1] = (uint32_t)rnd() & 0xFFFF; a[2] = A0;
}

#define LIB_CASES(X)                                \
    X(rsin, s_angle)                                \
    X(rcos, s_angle)                                \
    X(ratan2, s_value2)                             \
    X(SquareRoot0, s_unsigned)                         \
    X(SquareRoot12, s_unsigned)                        \
    X(RotMatrix, s_sangles_m)                       \
    X(RotMatrix_gte, s_sangles_m)                   \
    X(RotMatrixX, s_angle_m)                        \
    X(RotMatrixY, s_angle_m)                        \
    X(RotMatrixZ, s_angle_m)                        \
    X(RotMatrixZYX, s_sangles_m)                    \
    X(RotMatrixZXY, s_sangles_m)                    \
    X(MulMatrix, s_m_m)                             \
    X(MulMatrix0, s_m_m_m)                          \
    X(MulRotMatrix, s_m)                            \
    X(TransposeMatrix, s_m_m)                       \
    X(ScaleMatrix, s_m_v)                           \
    X(ScaleMatrixL, s_m_v)                          \
    X(SetRotMatrix, s_m)                            \
    X(SetTransMatrix, s_m)                          \
    X(ApplyMatrix, s_m_sv_v)                        \
    X(ApplyMatrixLV, s_m_v_v)                       \
    X(ApplyMatrixSV, s_m_sv_sv)                     \
    X(ApplyTransposeMatrixLV, s_m_v_v)              \
    X(MatrixNormal, s_rot_m_m)                          \
    X(MatrixNormal_0, s_m_m)                        \
    X(MatrixNormal_1, s_m_m)                        \
    X(MatrixNormal_2, s_m_m)                        \
    X(VectorNormal, s_normal_v_v)                          \
    X(VectorNormalS, s_normal_v_sv)                        \
    X(VectorNormalSS, s_normal_sv_sv)              \
    X(AddPrim, s_ot_p)                              \
    X(MargePrim, s_p_p)                             \
    X(SetSprt, s_p)                                 \
    X(SetTile, s_p)                                 \
    X(SetPolyFT4, s_p)                              \
    X(SetSemiTrans, s_p_flag)                       \
    X(SetShadeTex, s_p_flag)                        \
    X(GetTPage, s_tpage)                            \
    X(GetClut, s_clut)                              \
    X(SetDrawTPage, s_drawtpage)                    \
    X(SetDrawStp, s_p_flag)                         \
    X(SetDrawOffset, s_drawoffset)                  \
    X(SetDrawMove, s_drawmove)                      \
    X(SetDefDrawEnv, s_defenv)                      \
    X(SetDefDispEnv, s_defenv)                      \
    X(GsClearOt, s_gsclearot)                       \
    X(BreakDraw, s_none)

#define DECLARE(n, s) uint32_t pcrt_##n(); uint32_t pcrt_stub_##n();
LIB_CASES(DECLARE)
#define ENTRY(n, s) {#n, (LibFn)pcrt_##n, (LibFn)pcrt_stub_##n, s},

static const LibCase cases[] = {LIB_CASES(ENTRY)};

/* Routines declared void: their return value is not compared. */
static int is_void(const char* name)
{
    static const char* const names[] = {"SetRotMatrix", "SetTransMatrix", "MatrixNormal", "MatrixNormal_0",
                                        "MatrixNormal_1", "MatrixNormal_2", "AddPrim", "SetSprt",
                                        "SetTile", "SetPolyFT4", "SetSemiTrans", "SetShadeTex", "SetDrawTPage",
                                        "SetDrawStp", "SetDrawOffset", "SetDrawMove", "GsClearOt"};
    for (size_t i = 0; i < sizeof names / sizeof names[0]; i++) {
        if (strcmp(name, names[i]) == 0) {
            return 1;
        }
    }
    return 0;
}

/* ---- runner -------------------------------------------------------------- */

typedef struct {
    uint8_t ram[SCRATCH_SIZE];
    uint32_t gte[64];
    uint32_t ret;
} State;

static void capture(State* s, uint32_t ret)
{
    memcpy(s->ram, (void*)(uintptr_t)SCRATCH, SCRATCH_SIZE);
    pcgte_save(s->gte);
    s->ret = ret;
}

static const char* gte_reg_name(int i)
{
    static char name[16];
    snprintf(name, sizeof name, i < 32 ? "data %d" : "ctrl %d", i < 32 ? i : i - 32);
    return name;
}

/* Prints how `got` (port) differs from `want` (original), after the inputs
 * in `before`; returns 1 if it does. */
static int report(const LibCase* c, const uint32_t args[8], const State* before, const State* want,
                  const State* got)
{
    int differs = want->ret != got->ret || memcmp(want->ram, got->ram, SCRATCH_SIZE) != 0 ||
                  memcmp(want->gte, got->gte, sizeof want->gte) != 0;
    if (!differs) {
        return 0;
    }
    fprintf(stderr, "[libtest] %s(0x%08X, 0x%08X, 0x%08X, 0x%08X, 0x%08X) differs:\n", c->name, args[0],
            args[1], args[2], args[3], args[4]);
    for (int area = 0; area < 4; area++) {
        const int32_t* w = (const int32_t*)(before->ram + area * 0x100);
        fprintf(stderr, "    in A%d:", area);
        for (int i = 0; i < 8; i++) {
            fprintf(stderr, " %08X", (uint32_t)w[i]);
        }
        fprintf(stderr, "\n");
    }
    if (want->ret != got->ret) {
        fprintf(stderr, "    return: original 0x%08X, port 0x%08X\n", want->ret, got->ret);
    }
    for (uint32_t off = 0; off < SCRATCH_SIZE; off += 4) {
        uint32_t w, g;
        memcpy(&w, want->ram + off, 4);
        memcpy(&g, got->ram + off, 4);
        if (w != g) {
            fprintf(stderr, "    ram +0x%03X: original 0x%08X, port 0x%08X\n", off, w, g);
        }
    }
    for (int i = 0; i < 64; i++) {
        if (want->gte[i] != got->gte[i]) {
            fprintf(stderr, "    gte %s: original 0x%08X, port 0x%08X\n", gte_reg_name(i), want->gte[i],
                    got->gte[i]);
        }
    }
    return 1;
}

static int selected(const char* filter, const char* name)
{
    if (strncmp(filter, "all", 3) == 0 && (filter[3] == '\0' || filter[3] == ':')) {
        return 1;
    }
    size_t n = strlen(name);
    for (const char* p = filter; (p = strstr(p, name)) != NULL; p += n) {
        if ((p == filter || p[-1] == ',') && (p[n] == ',' || p[n] == ':' || p[n] == '\0')) {
            return 1;
        }
    }
    return 0;
}

/* PE2_LIBTEST=probe:<name>:<first>:<count>[:<step>]: prints the original
 * routine's results for a0 = first, first + step, ... (one argument), for
 * working out how it computes them. */
static int probe(const char* spec)
{
    char name[64];
    long long first = 0, count = 0, step = 1;
    long long second = 0;
    const char* comma = strchr(spec, ',');
    if (comma) {
        /* probe:<name>:<first>:<count>:<step>,<a1> passes a fixed a1. */
        second = atoll(comma + 1);
    }
    if (sscanf(spec, "probe:%63[^:]:%lli:%lli:%lli", name, &first, &count, &step) < 3) {
        fprintf(stderr, "[libtest] usage: probe:<name>:<first>:<count>[:<step>]\n");
        return 1;
    }
    for (size_t i = 0; i < sizeof cases / sizeof cases[0]; i++) {
        const LibCase* c = &cases[i];
        if (strcmp(c->name, name) != 0) {
            continue;
        }
        uint32_t psx_addr = 0;
        for (unsigned f = 0; f < pc_runtime_fn_count; f++) {
            if ((LibFn)pc_runtime_fns[f].impl == c->native) {
                psx_addr = pc_runtime_fns[f].psx_addr;
            }
        }
        if (psx_addr) {
            pc_runtime_fn_interpret(psx_addr, 1);
        }
        for (long long k = 0; k < count; k++) {
            uint32_t a0 = (uint32_t)(first + k * step);
            uint32_t r = c->original(a0, (uint32_t)second, 0, 0, 0, 0, 0, 0);
            printf("%u %d\n", a0, (int32_t)r);
        }
        if (psx_addr) {
            pc_runtime_fn_interpret(psx_addr, 0);
        }
        fflush(stdout);
        return 0;
    }
    fprintf(stderr, "[libtest] no case %s\n", name);
    return 1;
}

/* PE2_LIBTEST=dump:<name>:<count>: runs the original routine on <count>
 * random inputs and prints, one line each: the four arguments, the first
 * 32 bytes of the argument areas before (in) and after (out), the return
 * value and the GTE registers after, all in hex. */
static int dump(const char* spec)
{
    char name[64];
    int count = 0;
    if (sscanf(spec, "dump:%63[^:]:%d", name, &count) < 2) {
        fprintf(stderr, "[libtest] usage: dump:<name>:<count>\n");
        return 1;
    }
    for (size_t i = 0; i < sizeof cases / sizeof cases[0]; i++) {
        const LibCase* c = &cases[i];
        if (strcmp(c->name, name) != 0) {
            continue;
        }
        uint32_t psx_addr = 0;
        for (unsigned f = 0; f < pc_runtime_fn_count; f++) {
            if ((LibFn)pc_runtime_fns[f].impl == c->native) {
                psx_addr = pc_runtime_fns[f].psx_addr;
            }
        }
        static State before, after;
        for (int n = 0; n < count; n++) {
            uint32_t args[8] = {0};
            uint32_t gte[64];
            for (int r = 0; r < 64; r++) {
                gte[r] = rnd();
            }
            pcgte_restore(gte);
            c->setup(args);
            capture(&before, 0);
            if (psx_addr) {
                pc_runtime_fn_interpret(psx_addr, 1);
            }
            uint32_t ret = c->original(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            if (psx_addr) {
                pc_runtime_fn_interpret(psx_addr, 0);
            }
            capture(&after, ret);
            printf("args %08X %08X %08X %08X %08X", args[0], args[1], args[2], args[3], args[4]);
            for (int area = 0; area < 4; area++) {
                printf(" in%d", area);
                for (int w = 0; w < 8; w++) {
                    printf(" %08X", *(const uint32_t*)(before.ram + area * 0x100 + w * 4));
                }
            }
            for (int area = 0; area < 4; area++) {
                printf(" out%d", area);
                for (int w = 0; w < 8; w++) {
                    printf(" %08X", *(const uint32_t*)(after.ram + area * 0x100 + w * 4));
                }
            }
            printf(" ret %08X gte", ret);
            for (int r = 0; r < 64; r++) {
                printf(" %08X", after.gte[r]);
            }
            printf("\n");
        }
        fflush(stdout);
        return 0;
    }
    fprintf(stderr, "[libtest] no case %s\n", name);
    return 1;
}

int pc_libtest(const char* spec)
{
    if (strncmp(spec, "dump:", 5) == 0) {
        return dump(spec);
    }
    if (strncmp(spec, "probe:", 6) == 0) {
        return probe(spec);
    }
    const char* colon = strchr(spec, ':');
    int iterations = colon ? atoi(colon + 1) : 2000;
    int failed_cases = 0;
    static State before, want, got;

    for (size_t i = 0; i < sizeof cases / sizeof cases[0]; i++) {
        const LibCase* c = &cases[i];
        if (!selected(spec, c->name)) {
            continue;
        }
        if (c->native == c->original) {
            fprintf(stderr, "[libtest] %-24s not replaced yet\n", c->name);
            continue;
        }
        /* The original routine's PSX address, to run it past its thunk. */
        uint32_t psx_addr = 0;
        for (unsigned f = 0; f < pc_runtime_fn_count; f++) {
            if ((LibFn)pc_runtime_fns[f].impl == c->native) {
                psx_addr = pc_runtime_fns[f].psx_addr;
            }
        }
        if (psx_addr == 0) {
            fprintf(stderr, "[libtest] %-24s no PSX address\n", c->name);
            failed_cases++;
            continue;
        }
        int failures = 0;
        for (int it = 0; it < iterations; it++) {
            uint32_t args[8] = {0};
            uint32_t gte[64];
            for (int r = 0; r < 64; r++) {
                gte[r] = rnd();
            }
            pcgte_restore(gte);
            c->setup(args);
            capture(&before, 0);

            pc_runtime_fn_interpret(psx_addr, 1);
            uint32_t ret = c->original(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            pc_runtime_fn_interpret(psx_addr, 0);
            capture(&want, ret);

            memcpy((void*)(uintptr_t)SCRATCH, before.ram, SCRATCH_SIZE);
            pcgte_restore(before.gte);
            ret = c->native(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
            capture(&got, ret);
            if (is_void(c->name)) {
                got.ret = want.ret;
            }

            if (report(c, args, &before, &want, &got) && ++failures >= 3) {
                break;
            }
        }
        fprintf(stderr, "[libtest] %-24s %s\n", c->name, failures ? "FAILED" : "ok");
        failed_cases += failures != 0;
    }
    return failed_cases;
}
