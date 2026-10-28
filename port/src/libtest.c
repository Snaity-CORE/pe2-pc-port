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

typedef uint32_t (*LibFn)(uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t,
                          uint32_t);

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
    void (*setup)(uint32_t args[10]);
} LibCase;

static void s_angle(uint32_t a[10]) { a[0] = (uint32_t)rnd_angle(); }
static void s_value(uint32_t a[10]) { a[0] = (uint32_t)rnd_value(); }
/* A non-negative value (square roots: negative inputs are not meaningful). */
static void s_unsigned(uint32_t a[10]) { a[0] = (uint32_t)rnd_value() & 0x7FFFFFFF; }
static void s_value2(uint32_t a[10]) { a[0] = (uint32_t)rnd_value(); a[1] = (uint32_t)rnd_value(); }
static void s_sangles_m(uint32_t a[10]) { rnd_svector(A0, 1); rnd_matrix(A1); a[0] = A0; a[1] = A1; }
static void s_angle_m(uint32_t a[10]) { rnd_matrix(A1); a[0] = (uint32_t)rnd_angle(); a[1] = A1; }
static void s_m(uint32_t a[10]) { rnd_matrix(A0); a[0] = A0; }
/* A matrix of rotation size (|m| <= 4096). */
static void rnd_rotation_matrix(uint32_t at)
{
    rnd_matrix(at);
    for (int i = 0; i < 9; i++) {
        w16(at + 2 * i, (int32_t)(rnd() % 8193) - 4096);
    }
}
static void s_rot_m_m(uint32_t a[10]) { rnd_rotation_matrix(A0); rnd_matrix(A1); a[0] = A0; a[1] = A1; }
static void s_m_m(uint32_t a[10]) { rnd_matrix(A0); rnd_matrix(A1); a[0] = A0; a[1] = A1; }
static void s_m_m_m(uint32_t a[10]) { rnd_matrix(A0); rnd_matrix(A1); rnd_matrix(A2); a[0] = A0; a[1] = A1; a[2] = A2; }
static void s_m_v(uint32_t a[10]) { rnd_matrix(A0); rnd_vector(A1); a[0] = A0; a[1] = A1; }
static void s_m_v_v(uint32_t a[10]) { rnd_matrix(A0); rnd_vector(A1); rnd_vector(A2); a[0] = A0; a[1] = A1; a[2] = A2; }
static void s_m_sv_v(uint32_t a[10]) { rnd_matrix(A0); rnd_svector(A1, 0); rnd_vector(A2); a[0] = A0; a[1] = A1; a[2] = A2; }
static void s_m_sv_sv(uint32_t a[10]) { rnd_matrix(A0); rnd_svector(A1, 0); rnd_svector(A2, 0); a[0] = A0; a[1] = A1; a[2] = A2; }
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
static void s_normal_v_v(uint32_t a[10])
{
    for (int i = 0; i < 4; i++) { w32(A0 + 4 * i, rnd_normal_component()); }
    rnd_vector(A1); a[0] = A0; a[1] = A1;
}
static void s_normal_v_sv(uint32_t a[10])
{
    for (int i = 0; i < 4; i++) { w32(A0 + 4 * i, rnd_normal_component()); }
    rnd_svector(A1, 0); a[0] = A0; a[1] = A1;
}
static void s_normal_sv_sv(uint32_t a[10])
{
    for (int i = 0; i < 4; i++) { w16(A0 + 2 * i, rnd_normal_component()); }
    rnd_svector(A1, 0); a[0] = A0; a[1] = A1;
}
static void s_v_v(uint32_t a[10]) { rnd_vector(A0); rnd_vector(A1); a[0] = A0; a[1] = A1; }
static void s_v_sv(uint32_t a[10]) { rnd_vector(A0); rnd_svector(A1, 0); a[0] = A0; a[1] = A1; }
static void s_sv_sv(uint32_t a[10]) { rnd_svector(A0, 0); rnd_svector(A1, 0); a[0] = A0; a[1] = A1; }

/* Graphics library set-ups: packets of random words, small integers. */
static void rnd_words(uint32_t at, int n)
{
    for (int i = 0; i < n; i++) {
        w32(at + 4 * i, (int32_t)rnd());
    }
}
static int32_t rnd_small(int n) { return (int32_t)(rnd() % (uint32_t)n); }
static void s_ot_p(uint32_t a[10]) { rnd_words(A0, 4); rnd_words(A1, 8); a[0] = A0; a[1] = A1; }
static void s_p_p(uint32_t a[10])
{
    rnd_words(A0, 16); rnd_words(A1, 16);
    /* Packet lengths that sometimes fit together and sometimes do not. */
    *(uint8_t*)(uintptr_t)(A0 + 3) = (uint8_t)rnd_small(12);
    *(uint8_t*)(uintptr_t)(A1 + 3) = (uint8_t)rnd_small(12);
    a[0] = A0; a[1] = A1;
}
static void s_none(uint32_t a[10]) { (void)a; }
/* DecDCTBufSize: a BS header with a varied size field. */
static void s_bsheader(uint32_t a[10]) { rnd_words(A0, 4); a[0] = A0; }
/* DecDCTin: a run-length buffer whose header word holds a varied command and
 * a small size, and a mode 0-3. */
static void s_dctin(uint32_t a[10])
{
    rnd_words(A0, 40);
    w32(A0, (int32_t)((rnd() & 0xFFFF0000u) | 16u));
    a[0] = A0;
    a[1] = (uint32_t)rnd_small(4);
}
static void s_p(uint32_t a[10]) { rnd_words(A0, 16); a[0] = A0; }
static void s_p_flag(uint32_t a[10]) { rnd_words(A0, 16); a[0] = A0; a[1] = rnd() % 3 ? (uint32_t)rnd_small(2) : rnd(); }
static void s_tpage(uint32_t a[10])
{
    a[0] = (uint32_t)rnd_small(4); a[1] = (uint32_t)rnd_small(4);
    a[2] = rnd() % 4 ? (uint32_t)rnd_small(1024) : rnd(); a[3] = rnd() % 4 ? (uint32_t)rnd_small(1024) : rnd();
}
static void s_clut(uint32_t a[10]) { a[0] = rnd() % 4 ? (uint32_t)rnd_small(1024) : rnd(); a[1] = rnd() % 4 ? (uint32_t)rnd_small(512) : rnd(); }
static void s_drawtpage(uint32_t a[10])
{
    rnd_words(A0, 4); a[0] = A0;
    a[1] = rnd() % 3 ? (uint32_t)rnd_small(2) : rnd(); a[2] = rnd() % 3 ? (uint32_t)rnd_small(2) : rnd(); a[3] = rnd();
}
static void s_drawoffset(uint32_t a[10]) { rnd_words(A0, 4); rnd_words(A1, 2); a[0] = A0; a[1] = A1; }
static void s_drawmove(uint32_t a[10])
{
    rnd_words(A0, 8); rnd_words(A1, 2);
    if (rnd() % 4 == 0) { w16(A1 + 4, 0); }
    a[0] = A0; a[1] = A1; a[2] = (uint32_t)rnd_small(1024); a[3] = (uint32_t)rnd_small(512);
}
static void s_defenv(uint32_t a[10])
{
    rnd_words(A0, 32); a[0] = A0;
    a[1] = (uint32_t)rnd_small(1024); a[2] = (uint32_t)rnd_small(512);
    a[3] = (uint32_t)(rnd() % 2 ? 256 + rnd_small(385) : rnd_small(1024));
    a[4] = (uint32_t)(rnd() % 2 ? 200 + rnd_small(300) : rnd_small(512));
}
static void s_gsclearot(uint32_t a[10])
{
    rnd_words(A0, 8); rnd_words(A1, 64);
    w32(A0, (int32_t)(rnd() % 7));   /* length: up to 64 entries */
    w32(A0 + 4, (int32_t)A1);        /* org */
    a[0] = (uint32_t)rnd() & 0xFFFF; a[1] = (uint32_t)rnd() & 0xFFFF; a[2] = A0;
}

/* Projections. Three times in four, a scene as games set one up
 * (rotation-sized matrix, the points ahead of the camera, a usual
 * projection), so results vary; otherwise the random registers, which
 * mostly saturate. Returns whether it set a scene. */
static int rnd_scene(void)
{
    if (rnd() % 4 == 0) {
        return 0;
    }
    for (int r = 0; r < 5; r++) {
        uint32_t lo = (uint32_t)(rnd() % 8193) - 4096, hi = (uint32_t)(rnd() % 8193) - 4096;
        pcgte_write_ctrl(PCGTE_RT0 + r, (lo & 0xFFFF) | (hi << 16));
    }
    pcgte_write_ctrl(PCGTE_TRX, (uint32_t)(rnd_small(8001) - 4000));
    pcgte_write_ctrl(PCGTE_TRY, (uint32_t)(rnd_small(8001) - 4000));
    pcgte_write_ctrl(PCGTE_TRZ, (uint32_t)(rnd_small(30000) + 200));
    pcgte_write_ctrl(PCGTE_OFX, (uint32_t)rnd_small(321) << 16);
    pcgte_write_ctrl(PCGTE_OFY, (uint32_t)rnd_small(241) << 16);
    pcgte_write_ctrl(PCGTE_H, (uint32_t)(rnd_small(800) + 100));
    return 1;
}

/* n vectors at `at`, scene-sized when `scene`. */
static void rnd_points(uint32_t at, int n, int scene)
{
    for (int i = 0; i < n; i++) {
        rnd_svector(at + 8 * i, 0);
        if (scene) {
            for (int c = 0; c < 3; c++) {
                w16(at + 8 * i + 2 * c, rnd_small(6001) - 3000);
            }
        }
    }
}

/* RotTransPers, RotTransPers3, RotTransPers4: n vectors in A0, their screen
 * positions, depth cue and flags written to A1. */
static void s_rtp(uint32_t a[10], int n)
{
    rnd_points(A0, n, rnd_scene());
    rnd_words(A1, 8);
    for (int i = 0; i < n; i++) {
        a[i] = A0 + 8 * i;
    }
    for (int i = 0; i < n + 2; i++) {
        a[n + i] = A1 + 4 * i;
    }
}
static void s_rtp1(uint32_t a[10]) { s_rtp(a, 1); }
static void s_rtp3(uint32_t a[10]) { s_rtp(a, 3); }
static void s_rtp4(uint32_t a[10]) { s_rtp(a, 4); }

/* RotTransSV: a vector in A0, the result and flags to A1. */
static void s_rtsv(uint32_t a[10])
{
    rnd_points(A0, 1, rnd_scene());
    rnd_words(A1, 4);
    a[0] = A0; a[1] = A1; a[2] = A1 + 8;
}

/* A weight: mostly within 0..4096, sometimes any. */
static uint32_t rnd_weight(void) { return rnd() % 4 ? (uint32_t)rnd_small(4097) : rnd(); }

/* LoadAverageCol: two colours in A0, the average to A1. */
static void s_avgcol(uint32_t a[10])
{
    rnd_words(A0, 2); rnd_words(A1, 1);
    a[0] = A0; a[1] = A0 + 4; a[2] = rnd_weight(); a[3] = rnd_weight(); a[4] = A1;
}

/* gteMIMefunc: 1 to 16 vectors in A0 moved by deltas in A1 (the original
 * counts n down after each vector: 0 would run on past the RAM). */
static void s_mime(uint32_t a[10])
{
    rnd_points(A0, 16, rnd() % 2); rnd_points(A1, 16, rnd() % 2);
    a[0] = A0; a[1] = A1; a[2] = (uint32_t)(1 + rnd_small(16)); a[3] = rnd_weight();
}

/* SetDrawArea: a packet in A0, a rectangle in A1, mostly on screen; the
 * VRAM size in GEnv mostly the console's (1024 x 512), sometimes any. */
extern const unsigned pc_sym_GEnv;
static void s_drawarea(uint32_t a[10])
{
    w16(pc_sym_GEnv + 4, rnd() % 4 ? 1024 : rnd_small(2048));
    w16(pc_sym_GEnv + 6, rnd() % 4 ? 512 : rnd_small(2048));
    rnd_words(A0, 4); rnd_words(A1, 2);
    if (rnd() % 4) {
        for (int i = 0; i < 4; i++) {
            w16(A1 + 2 * i, rnd_small(1200) - 50);
        }
    }
    a[0] = A0; a[1] = A1;
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
    X(RotTransPers, s_rtp1)                         \
    X(RotTransPers3, s_rtp3)                        \
    X(RotTransPers4, s_rtp4)                        \
    X(RotTransSV, s_rtsv)                           \
    X(LoadAverageCol, s_avgcol)                     \
    X(gteMIMefunc, s_mime)                          \
    X(SetDrawArea, s_drawarea)                      \
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
    X(BreakDraw, s_none)                            \
    X(InitGeom, s_none)                             \
    X(DecDCTin, s_dctin)                            \
    X(DecDCTBufSize, s_bsheader)

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
                                        "SetDrawStp", "SetDrawOffset", "SetDrawMove", "GsClearOt",
                                        "InitGeom", "DecDCTin", "RotTransSV", "LoadAverageCol", "gteMIMefunc",
                                        "SetDrawArea"};
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
static int report(const LibCase* c, const uint32_t args[10], const State* before, const State* want,
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
            uint32_t r = c->original(a0, (uint32_t)second, 0, 0, 0, 0, 0, 0, 0, 0);
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
            uint32_t args[10] = {0};
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
            uint32_t ret = c->original(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
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

/* ---- heap scripts -------------------------------------------------------- */

/* libapi's heap search cursor, which the game also writes (main/mem.c). */
#define HEAP_FREEP (*(volatile uint32_t*)(uintptr_t)0x8005ED90u)
#define HEAP_BASE  (SCRATCH + 0x10u)

uint32_t pcrt_InitHeap3(); uint32_t pcrt_stub_InitHeap3();
uint32_t pcrt_malloc3(); uint32_t pcrt_stub_malloc3();
uint32_t pcrt_free3(); uint32_t pcrt_stub_free3();

static uint32_t psx_address_of(LibFn native)
{
    for (unsigned f = 0; f < pc_runtime_fn_count; f++) {
        if ((LibFn)pc_runtime_fns[f].impl == native) {
            return pc_runtime_fns[f].psx_addr;
        }
    }
    return 0;
}

/* Calls a heap routine, the original (through the interpreter) or the port's. */
static uint32_t heap_call(int original, uint32_t (*native)(), uint32_t (*stub)(), uint32_t a0, uint32_t a1)
{
    if (!original) {
        return ((LibFn)native)(a0, a1, 0, 0, 0, 0, 0, 0, 0, 0);
    }
    uint32_t addr = psx_address_of((LibFn)native);
    if (addr && (LibFn)native != (LibFn)stub) {
        pc_runtime_fn_interpret(addr, 1);
    }
    uint32_t r = ((LibFn)stub)(a0, a1, 0, 0, 0, 0, 0, 0, 0, 0);
    if (addr && (LibFn)native != (LibFn)stub) {
        pc_runtime_fn_interpret(addr, 0);
    }
    return r;
}

/* Runs a script of heap operations; records each result, the cursor and
 * the scratch area after every step. Ops: i<size> InitHeap3(HEAP_BASE,
 * size), m<size> malloc3, f<k> free3 of the k-th allocation, b cursor
 * back to the heap base (as the game does before each call). */
typedef struct {
    uint32_t ret, freep;
    uint8_t ram[SCRATCH_SIZE];
} HeapStep;

static int heap_run(const char* script, int original, HeapStep* steps, int max_steps)
{
    uint32_t allocs[256];
    int n_alloc = 0, n = 0;
    for (const char* p = script; *p && n < max_steps; ) {
        char op = *p++;
        long v = strtol(p, (char**)&p, 10);
        uint32_t r = 0;
        switch (op) {
        case 'i': r = heap_call(original, pcrt_InitHeap3, pcrt_stub_InitHeap3, HEAP_BASE, (uint32_t)v); r = 0; break;
        case 'm': r = heap_call(original, pcrt_malloc3, pcrt_stub_malloc3, (uint32_t)v, 0); if (n_alloc < 256) allocs[n_alloc++] = r; break;
        case 'f': if (v < n_alloc && allocs[v] != 0) heap_call(original, pcrt_free3, pcrt_stub_free3, allocs[v], 0); r = 0; break;
        case 'b': HEAP_FREEP = HEAP_BASE; break;
        default: continue;
        }
        steps[n].ret = r;
        steps[n].freep = HEAP_FREEP;
        memcpy(steps[n].ram, (void*)(uintptr_t)SCRATCH, SCRATCH_SIZE);
        n++;
        if (*p == ',') {
            p++;
        }
    }
    return n;
}

/* PE2_LIBTEST=heap:<script> prints the original's results and the words of
 * the heap after each step; heap!<script> compares the port against it. */
static int heap(const char* spec)
{
    static HeapStep want[512], got[512];
    int compare = spec[4] == '!';
    const char* script = spec + 5;
    uint32_t saved_freep = HEAP_FREEP;
    memset((void*)(uintptr_t)SCRATCH, 0xAB, SCRATCH_SIZE);
    static uint8_t start[SCRATCH_SIZE];
    memcpy(start, (void*)(uintptr_t)SCRATCH, SCRATCH_SIZE);
    int n = heap_run(script, 1, want, 512);
    if (!compare) {
        for (int i = 0; i < n; i++) {
            printf("step %d ret %08X freep %08X:", i, want[i].ret, want[i].freep);
            for (uint32_t off = 0; off < SCRATCH_SIZE; off += 4) {
                uint32_t w;
                memcpy(&w, want[i].ram + off, 4);
                if (w != 0xABABABABu) {
                    printf(" +%03X=%08X", off, w);
                }
            }
            printf("\n");
        }
        HEAP_FREEP = saved_freep;
        return 0;
    }
    /* A script can leave the heap unusable (a cursor outside the ring) and
     * hang the original; anything after this line is the port's doing. */
    fprintf(stderr, "[libtest] heap original done\n");
    fflush(stderr);
    memcpy((void*)(uintptr_t)SCRATCH, start, SCRATCH_SIZE);
    HEAP_FREEP = saved_freep;
    heap_run(script, 0, got, 512);
    for (int i = 0; i < n; i++) {
        if (want[i].ret != got[i].ret || want[i].freep != got[i].freep ||
            memcmp(want[i].ram, got[i].ram, SCRATCH_SIZE) != 0) {
            fprintf(stderr, "[libtest] heap step %d differs: ret %08X/%08X freep %08X/%08X\n", i, want[i].ret,
                    got[i].ret, want[i].freep, got[i].freep);
            for (uint32_t off = 0; off < SCRATCH_SIZE; off += 4) {
                uint32_t w, g;
                memcpy(&w, want[i].ram + off, 4);
                memcpy(&g, got[i].ram + off, 4);
                if (w != g) {
                    fprintf(stderr, "    +%03X: original %08X, port %08X\n", off, w, g);
                }
            }
            return 1;
        }
    }
    fprintf(stderr, "[libtest] heap script ok (%d steps)\n", n);
    HEAP_FREEP = saved_freep;
    return 0;
}

/* PE2_LIBTEST=peek:<address>:<words>: prints PSX RAM words after the main
 * executable loads (for comparing data the port computes with the
 * original's, never for copying it). */
static int peek(const char* spec)
{
    unsigned long addr = 0, n = 0;
    if (sscanf(spec, "peek:%lx:%lu", &addr, &n) < 2) {
        return 1;
    }
    for (unsigned long i = 0; i < n; i++) {
        printf("%08lX\n", (unsigned long)*(const uint32_t*)(uintptr_t)(addr + 4 * i));
    }
    fflush(stdout);
    return 0;
}

/* PE2_LIBTEST=mdecreset: DecDCTReset(0) must load the MDEC with the same
 * tables as the original. */
void pc_mdec_tables(uint8_t out[256]);
uint32_t pcrt_DecDCTReset(); uint32_t pcrt_stub_DecDCTReset();

static int mdec_reset(void)
{
    uint8_t want[256], got[256];
    uint32_t addr = psx_address_of((LibFn)pcrt_DecDCTReset);
    if (addr && (LibFn)pcrt_DecDCTReset != (LibFn)pcrt_stub_DecDCTReset) {
        pc_runtime_fn_interpret(addr, 1);
    }
    ((LibFn)pcrt_stub_DecDCTReset)(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    if (addr) {
        pc_runtime_fn_interpret(addr, 0);
    }
    pc_mdec_tables(want);
    ((LibFn)pcrt_DecDCTReset)(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    pc_mdec_tables(got);
    for (int i = 0; i < 256; i++) {
        if (want[i] != got[i]) {
            fprintf(stderr, "[libtest] mdecreset: byte %d original %02X port %02X\n", i, want[i], got[i]);
            return 1;
        }
    }
    fprintf(stderr, "[libtest] mdecreset ok\n");
    return 0;
}

/* PE2_LIBTEST=vlcbuild[!]: DecDCTvlcBuild's table, built at VLC_TABLE:
 * printed for the original, or (with !) compared with the port's. */
#define VLC_TABLE 0x80120000u
#define VLC_TABLE_BYTES 0x11000u
uint32_t pcrt_DecDCTvlcBuild(); uint32_t pcrt_stub_DecDCTvlcBuild();

static int vlc_build(int compare)
{
    static uint8_t want[VLC_TABLE_BYTES];
    uint8_t* table = (uint8_t*)(uintptr_t)VLC_TABLE;
    memset(table, 0xCD, VLC_TABLE_BYTES);
    uint32_t addr = psx_address_of((LibFn)pcrt_DecDCTvlcBuild);
    int native = (LibFn)pcrt_DecDCTvlcBuild != (LibFn)pcrt_stub_DecDCTvlcBuild;
    if (addr && native) {
        pc_runtime_fn_interpret(addr, 1);
    }
    ((LibFn)pcrt_stub_DecDCTvlcBuild)(VLC_TABLE, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    if (addr && native) {
        pc_runtime_fn_interpret(addr, 0);
    }
    if (!compare) {
        for (uint32_t i = 0; i < VLC_TABLE_BYTES / 2; i++) {
            printf("%04X\n", ((const uint16_t*)table)[i]);
        }
        fflush(stdout);
        return 0;
    }
    memcpy(want, table, VLC_TABLE_BYTES);
    memset(table, 0xCD, VLC_TABLE_BYTES);
    ((LibFn)pcrt_DecDCTvlcBuild)(VLC_TABLE, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    for (uint32_t i = 0; i < VLC_TABLE_BYTES / 2; i++) {
        uint16_t w = ((const uint16_t*)want)[i], g = ((const uint16_t*)table)[i];
        if (w != g) {
            fprintf(stderr, "[libtest] vlcbuild: entry %u original %04X port %04X\n", i, w, g);
            return 1;
        }
    }
    fprintf(stderr, "[libtest] vlcbuild ok\n");
    return 0;
}

/* PE2_LIBTEST=vlcfile[!]:<size2>:<file>: decodes a captured bitstream
 * with DecDCTvlcSize2(size2) and DecDCTvlc2 (first with the stream, then
 * NULL until it returns 0); prints the original's calls and output, or
 * (with !) compares the port's with it. Each side uses the table its own
 * DecDCTvlcBuild makes. */
#define VLC_BS     0x80140000u
#define VLC_OUT    0x80160000u
#define VLC_OUT_BYTES 0x40000u
uint32_t pcrt_DecDCTvlcSize2(); uint32_t pcrt_stub_DecDCTvlcSize2();
uint32_t pcrt_DecDCTvlc2(); uint32_t pcrt_stub_DecDCTvlc2();
uint64_t pc_mips_executed(uint64_t* loads);

static uint32_t call_lib(int original, LibFn native, LibFn stub, uint32_t a0, uint32_t a1, uint32_t a2)
{
    if (!original) {
        return native(a0, a1, a2, 0, 0, 0, 0, 0, 0, 0);
    }
    uint32_t addr = psx_address_of(native);
    int replaced = native != stub;
    if (addr && replaced) {
        pc_runtime_fn_interpret(addr, 1);
    }
    uint32_t r = stub(a0, a1, a2, 0, 0, 0, 0, 0, 0, 0);
    if (addr && replaced) {
        pc_runtime_fn_interpret(addr, 0);
    }
    return r;
}

static int vlc_decode(int original, int size2, uint32_t rets[4096])
{
    memset((void*)(uintptr_t)VLC_OUT, 0xCD, VLC_OUT_BYTES);
    memset((void*)(uintptr_t)VLC_TABLE, 0, VLC_TABLE_BYTES);
    uint64_t build_loads0, build_loads1;
    uint64_t build0 = pc_mips_executed(&build_loads0);
    call_lib(original, (LibFn)pcrt_DecDCTvlcBuild, (LibFn)pcrt_stub_DecDCTvlcBuild, VLC_TABLE, 0, 0);
    uint64_t build1 = pc_mips_executed(&build_loads1);
    if (original) {
        fprintf(stderr, "[libtest] DecDCTvlcBuild: %llu instructions, %llu loads\n", (unsigned long long)(build1 - build0),
                (unsigned long long)(build_loads1 - build_loads0));
    }
    call_lib(original, (LibFn)pcrt_DecDCTvlcSize2, (LibFn)pcrt_stub_DecDCTvlcSize2, (uint32_t)size2, 0, 0);
    int n = 0;
    uint32_t r = call_lib(original, (LibFn)pcrt_DecDCTvlc2, (LibFn)pcrt_stub_DecDCTvlc2, VLC_BS, VLC_OUT, VLC_TABLE);
    rets[n++] = r;
    while (r != 0 && n < 4096) {
        if (getenv("PE2_VLCPROGRESS")) {
            uint32_t w = 0;
            while (w < VLC_OUT_BYTES / 4 && ((const uint32_t*)(uintptr_t)VLC_OUT)[w] != 0xCDCDCDCDu) {
                w++;
            }
            fprintf(stderr, "[vlc] after call %d: %u words\n", n, w);
        }
        r = call_lib(original, (LibFn)pcrt_DecDCTvlc2, (LibFn)pcrt_stub_DecDCTvlc2, 0, 0, VLC_TABLE);
        rets[n++] = r;
    }
    return n;
}

static int vlc_file(const char* spec)
{
    int compare = spec[7] == '!';
    int size2 = atoi(spec + 9);
    const char* path = strchr(spec + 9, ':');
    if (!path) {
        return 1;
    }
    path++;
    FILE* f = fopen(path, "rb");
    if (!f) {
        fprintf(stderr, "[libtest] cannot open %s\n", path);
        return 1;
    }
    size_t len = fread((void*)(uintptr_t)VLC_BS, 1, 0x10000, f);
    fclose(f);
    (void)len;
    static uint8_t want[VLC_OUT_BYTES];
    static uint32_t want_rets[4096], got_rets[4096];
    uint64_t loads0, loads1;
    uint64_t insns0 = pc_mips_executed(&loads0);
    int nw = vlc_decode(1, size2, want_rets);
    uint64_t insns1 = pc_mips_executed(&loads1);
    fprintf(stderr, "[libtest] vlcfile %s: original ran %llu instructions, %llu loads; %u input halfwords\n", path,
            (unsigned long long)(insns1 - insns0), (unsigned long long)(loads1 - loads0),
            (unsigned)((const uint16_t*)(uintptr_t)VLC_BS)[0]);
    if (!compare) {
        printf("calls %d:", nw);
        for (int i = 0; i < nw; i++) {
            printf(" %08X", want_rets[i]);
        }
        printf("\n");
        for (uint32_t i = 0; i < VLC_OUT_BYTES / 4; i++) {
            printf("%08X\n", ((const uint32_t*)(uintptr_t)VLC_OUT)[i]);
        }
        fflush(stdout);
        return 0;
    }
    memcpy(want, (void*)(uintptr_t)VLC_OUT, VLC_OUT_BYTES);
    int ng = vlc_decode(0, size2, got_rets);
    /* With a limit the two split the frame at different points: only the
     * decoded data has to match. */
    if (size2 == 0 && (nw != ng || memcmp(want_rets, got_rets, sizeof(uint32_t) * (size_t)nw) != 0)) {
        fprintf(stderr, "[libtest] vlcfile %s: calls differ (%d vs %d)", path, nw, ng);
        for (int i = 0; i < nw && i < 8; i++) {
            fprintf(stderr, " %08X/%08X", want_rets[i], i < ng ? got_rets[i] : 0);
        }
        fprintf(stderr, "\n");
        return 1;
    }
    const uint32_t* g = (const uint32_t*)(uintptr_t)VLC_OUT;
    const uint32_t* w = (const uint32_t*)want;
    for (uint32_t i = 0; i < VLC_OUT_BYTES / 4; i++) {
        if (w[i] != g[i]) {
            fprintf(stderr, "[libtest] vlcfile %s: word %u original %08X port %08X\n", path, i, w[i], g[i]);
            return 1;
        }
    }
    fprintf(stderr, "[libtest] vlcfile ok (%d calls)\n", nw);
    return 0;
}

/* ---- TMD stream handlers ------------------------------------------------ */

/* PE2_LIBTEST=tmd[:iterations]: the TMD stream handlers (game functions,
 * ported from Square's assembly) against the original code, on synthetic
 * scenes: a workspace, geometry, depth cache, ordering table and packets,
 * with sensible GTE matrices or random registers. Each handler runs as the
 * second record of a stream walked by Tmd_DispatchStream, after an empty
 * tmdSkipStreamRecord, so it is entered as in the game (the reversed GT4
 * walk can read the previous handler's result). The whole scene, the GTE
 * and the return value are compared. */
unsigned pc_mips_call(unsigned addr, const unsigned* args);

#define TMD_DISPATCH  0x80010A20u
#define TMD_SKIP      0x800105ACu
#define TMD_REGION    0x80120000u
#define TMD_REGION_SZ 0x4000u
#define TMD_WS        (TMD_REGION + 0x0000u)
#define TMD_STREAM    (TMD_REGION + 0x0100u)
#define TMD_VERTS     (TMD_REGION + 0x0800u)
#define TMD_NORMALS   (TMD_REGION + 0x0A00u)
#define TMD_DEPTHS    (TMD_REGION + 0x1000u)
#define TMD_OT        (TMD_REGION + 0x2000u)
#define TMD_PACKETS   (TMD_REGION + 0x3000u)
#define TMD_ENTRIES   64 /* vertices and normals */

typedef struct {
    uint32_t addr;
    const char* name;
    int words;      /* element words the handler reads */
    int word2_full; /* word 2 is a whole-word normal offset */
    int quad;       /* vertex 3 may repeat the previous vertex 0 */
    int xform;      /* the vertex may repeat the previous element's */
} TmdHandler;

static const TmdHandler tmd_handlers[] = {
    {0x800105CCu, "G3PreXformSemiTrans", 2, 0, 0, 0},
    {0x800105F4u, "G3PreXform", 2, 0, 0, 0},
    {0x800106F0u, "G4PreXformSemiTrans", 2, 0, 0, 0},
    {0x80010718u, "G4PreXform", 2, 0, 0, 0},
    {0x80010A90u, "G3CornerNormals", 4, 0, 0, 0},
    {0x80010BF4u, "G4CornerNormals", 5, 0, 1, 0},
    {0x80010E1Cu, "XformVertsElemColor", 3, 0, 0, 1},
    {0x80010EF4u, "Gt3SemiTrans", 3, 0, 0, 0},
    {0x80010F08u, "Gt3", 3, 0, 0, 0},
    {0x8001108Cu, "Gt4SemiTrans", 4, 0, 1, 0},
    {0x800110A0u, "Gt4", 4, 0, 1, 0},
    {0x800112DCu, "XformVerts", 2, 0, 0, 1},
    {0x800113B4u, "Gt3PreXformSemiTrans", 2, 0, 0, 0},
    {0x800113E8u, "Gt3PreXform", 2, 0, 0, 0},
    {0x800114FCu, "Gt4PreXformSemiTrans", 2, 0, 0, 0},
    {0x80011530u, "Gt4PreXform", 2, 0, 0, 0},
    {0x80011678u, "G3", 3, 0, 0, 0},
    {0x800117BCu, "G4", 4, 1, 1, 0},
    {0x80011994u, "Gt3OneNormalSemiTrans", 2, 0, 0, 0},
    {0x800119A8u, "Gt3OneNormal", 2, 0, 0, 0},
    {0x80011AF4u, "Gt4OneNormal", 3, 1, 1, 0},
    {0x80011B08u, "Gt4OneNormalSemiTrans", 3, 1, 1, 0},
    {0x8001237Cu, "Gt3CornerColors", 6, 0, 0, 0},
    {0x80012520u, "Gt4CornerColors", 8, 0, 1, 0},
};

static uint32_t tmd_offset_pair(void)
{
    return (8u * (rnd() % TMD_ENTRIES)) | ((8u * (rnd() % TMD_ENTRIES)) << 16);
}

static int16_t tmd_short(int range) { return (int16_t)((int32_t)(rnd() % (uint32_t)(2 * range + 1)) - range); }

/* GTE control registers of a plausible model transform; the data
 * registers stay random. */
static void tmd_sensible_gte(uint32_t gte[64])
{
    uint32_t* c = gte + 32;
    for (int r = 0; r < 5; r++) {
        c[r] = (uint16_t)tmd_short(4096) | ((uint32_t)(uint16_t)tmd_short(4096) << 16);
    }
    c[5] = (uint32_t)tmd_short(1000);
    c[6] = (uint32_t)tmd_short(1000);
    c[7] = (uint32_t)(int32_t)(rnd() % 8 ? 200 + rnd() % 6000 : (uint32_t)tmd_short(400));
    for (int r = 8; r < 24; r++) {
        c[r] = (uint16_t)tmd_short(4096) | ((uint32_t)(uint16_t)tmd_short(4096) << 16);
    }
    for (int r = 13; r < 16; r++) {
        c[r] = rnd() % 0x2000;
    }
    for (int r = 21; r < 24; r++) {
        c[r] = rnd() % 0x2000;
    }
    c[24] = (160u + rnd() % 64) << 16;
    c[25] = (120u + rnd() % 64) << 16;
    c[26] = 200 + rnd() % 800;
    c[29] = rnd() % 0x1000;
    c[30] = rnd() % 0x1000;
}

static void tmd_scene(const TmdHandler* h, uint32_t gte[64], uint32_t* flags)
{
    for (uint32_t a = TMD_REGION; a < TMD_REGION + TMD_REGION_SZ; a += 4) {
        w32(a, (int32_t)rnd());
    }
    int sensible = rnd() % 4 != 0;
    for (int i = 0; i < TMD_ENTRIES; i++) {
        uint32_t v = TMD_VERTS + 8u * i, n = TMD_NORMALS + 8u * i;
        int range = sensible ? 1500 : 32767;
        w16(v, tmd_short(range));
        w16(v + 2, tmd_short(range));
        w16(v + 4, tmd_short(range));
        w16(n, tmd_short(4096));
        w16(n + 2, tmd_short(4096));
        w16(n + 4, tmd_short(4096));
    }
    for (uint32_t i = 0; i < 1024; i++) {
        uint32_t d = rnd() & 0xFFFF;
        w32(TMD_DEPTHS + 4 * i, (int32_t)(rnd() % 4 ? d : d | 0x80000000u));
    }

    int stride = h->words + (int)(rnd() % 3);
    int count  = (int)(rnd() % 13);
    uint32_t* s = (uint32_t*)(uintptr_t)TMD_STREAM;
    s[0] = rnd();
    s[1] = TMD_SKIP;
    s[2] = 1 + rnd() % 4; /* no elements */
    s[3] = rnd();
    s[4] = h->addr;
    s[5] = ((uint32_t)count << 16) | (uint32_t)stride;
    uint32_t* e = s + 6;
    for (int i = 0; i < count; i++, e += stride) {
        for (int w = 0; w < stride; w++) {
            e[w] = w < 4 ? tmd_offset_pair() : rnd();
        }
        if (h->word2_full) {
            e[2] = 8u * (rnd() % TMD_ENTRIES);
        }
        if (i > 0 && rnd() % 2) {
            if (h->quad) {
                e[1] = (e[1] & 0xFFFF) | ((e - stride)[0] << 16);
            } else if (h->xform) {
                e[0] = (e[0] & 0xFFFF0000u) | ((e - stride)[0] & 0xFFFF);
            }
        }
    }
    e[0] = (uint32_t)-2;

    uint32_t* ws = (uint32_t*)(uintptr_t)TMD_WS;
    ws[0] = TMD_PACKETS;
    ws[1] = TMD_PACKETS;
    ws[2] = TMD_VERTS;
    ws[3] = TMD_NORMALS;
    ws[4] = TMD_DEPTHS;
    ws[5] = TMD_OT;
    ws[0x84 / 4] = rnd() % 4 ? rnd() % 4 : rnd();

    static const uint32_t flag_choices[] = {0, 0x02, 0x10, 0x12};
    *flags = rnd() % 5 ? flag_choices[rnd() % 4] : rnd();

    for (int r = 0; r < 64; r++) {
        gte[r] = rnd();
    }
    if (sensible) {
        tmd_sensible_gte(gte);
    }
}

static int tmd_test(const char* spec)
{
    const char* colon = strchr(spec, ':');
    int iterations = colon ? atoi(colon + 1) : 2000;
    static uint8_t before[TMD_REGION_SZ], want[TMD_REGION_SZ];
    uint8_t* region = (uint8_t*)(uintptr_t)TMD_REGION;
    int failed = 0;
    typedef uint32_t (*Dispatch)(uint32_t, uint32_t, uint32_t);

    for (size_t k = 0; k < sizeof tmd_handlers / sizeof tmd_handlers[0]; k++) {
        const TmdHandler* h = &tmd_handlers[k];
        int failures = 0;
        for (int it = 0; it < iterations && failures < 3; it++) {
            uint32_t gte[64], gte_want[64], gte_got[64], flags;
            tmd_scene(h, gte, &flags);
            memcpy(before, region, TMD_REGION_SZ);

            uint32_t args[10] = {TMD_WS, flags, TMD_STREAM};
            pcgte_restore(gte);
            pc_runtime_fn_interpret(TMD_DISPATCH, 1);
            pc_runtime_fn_interpret(TMD_SKIP, 1);
            pc_runtime_fn_interpret(h->addr, 1);
            uint32_t ret_want = pc_mips_call(TMD_DISPATCH, args);
            pc_runtime_fn_interpret(h->addr, 0);
            pc_runtime_fn_interpret(TMD_SKIP, 0);
            pc_runtime_fn_interpret(TMD_DISPATCH, 0);
            pcgte_save(gte_want);
            memcpy(want, region, TMD_REGION_SZ);

            memcpy(region, before, TMD_REGION_SZ);
            pcgte_restore(gte);
            uint32_t ret_got = ((Dispatch)(uintptr_t)TMD_DISPATCH)(TMD_WS, flags, TMD_STREAM);
            pcgte_save(gte_got);
            /* The dispatcher keeps its return address: the interpreter's
             * differs from the game's. */
            memcpy(region + 0x2C, want + 0x2C, 4);

            int bad = ret_want != ret_got;
            for (uint32_t i = 0; i < TMD_REGION_SZ; i += 4) {
                bad |= memcmp(want + i, region + i, 4) != 0;
            }
            for (int r = 0; r < 64; r++) {
                bad |= gte_want[r] != gte_got[r];
            }
            if (!bad) {
                continue;
            }
            failures++;
            fprintf(stderr, "[libtest] tmd %s iteration %d: flags %08X, %u elements of %u words, shift %u\n", h->name,
                    it, flags, ((uint32_t*)(uintptr_t)TMD_STREAM)[5] >> 16,
                    ((uint32_t*)(uintptr_t)TMD_STREAM)[5] & 0xFFFF, *(uint32_t*)(before + 0x84));
            if (ret_want != ret_got) {
                fprintf(stderr, "    return: original %08X, port %08X\n", ret_want, ret_got);
            }
            int shown = 0;
            for (uint32_t i = 0; i < TMD_REGION_SZ && shown < 12; i += 4) {
                uint32_t w = *(uint32_t*)(want + i), g = *(uint32_t*)(region + i);
                if (w != g) {
                    fprintf(stderr, "    %08X: before %08X, original %08X, port %08X\n", TMD_REGION + i,
                            *(uint32_t*)(before + i), w, g);
                    shown++;
                }
            }
            for (int r = 0; r < 64; r++) {
                if (gte_want[r] != gte_got[r]) {
                    fprintf(stderr, "    gte %s: before %08X, original %08X, port %08X\n", gte_reg_name(r), gte[r],
                            gte_want[r], gte_got[r]);
                }
            }
        }
        fprintf(stderr, "[libtest] tmd %-22s %s\n", h->name, failures ? "FAILED" : "ok");
        failed += failures != 0;
    }
    return failed;
}

int pc_libtest(const char* spec)
{
    if (strncmp(spec, "tmd", 3) == 0 && (spec[3] == 0 || spec[3] == ':')) {
        return tmd_test(spec);
    }
    if (strncmp(spec, "vlcfile", 7) == 0) {
        return vlc_file(spec);
    }
    if (strncmp(spec, "vlcbuild", 8) == 0) {
        return vlc_build(spec[8] == '!');
    }
    if (strcmp(spec, "mdecreset") == 0) {
        return mdec_reset();
    }
    if (strncmp(spec, "peek:", 5) == 0) {
        return peek(spec);
    }
    if (strncmp(spec, "heap", 4) == 0 && (spec[4] == ':' || spec[4] == '!')) {
        return heap(spec);
    }
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
            uint32_t args[10] = {0};
            uint32_t gte[64];
            for (int r = 0; r < 64; r++) {
                gte[r] = rnd();
            }
            pcgte_restore(gte);
            c->setup(args);
            capture(&before, 0);

            pc_runtime_fn_interpret(psx_addr, 1);
            uint32_t ret = c->original(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
            pc_runtime_fn_interpret(psx_addr, 0);
            capture(&want, ret);

            memcpy((void*)(uintptr_t)SCRATCH, before.ram, SCRATCH_SIZE);
            pcgte_restore(before.gte);
            ret = c->native(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
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
