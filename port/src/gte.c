/*
 * Software GTE (COP2). See pc/gte.h.
 *
 * Behavior follows the psx-spx hardware documentation: 44-bit MAC1-3
 * accumulation with per-step overflow flags, IR/colour/SZ/SXY saturation,
 * the UNR reciprocal used by RTPS/RTPT, and the register side effects
 * (SXYP/SZ/RGB FIFOs, IRGB/ORGB, LZCS/LZCR, FLAG summary bit).
 */
#include "pc/gte.h"

#include <string.h>

typedef int16_t s16;
typedef uint16_t u16;
typedef int32_t s32;
typedef uint32_t u32;
typedef int64_t s64;

/* Register file: raw 32-bit words, decoded on access. */
static u32 data[32];
static u32 ctrl[32];

/* UNR reciprocal seed table, generated from its closed form at first use. */
static uint8_t unr_table[0x101];
static int unr_ready;

/* FLAG bits. */
enum {
    F_MAC1_POS = 1u << 30, F_MAC2_POS = 1u << 29, F_MAC3_POS = 1u << 28,
    F_MAC1_NEG = 1u << 27, F_MAC2_NEG = 1u << 26, F_MAC3_NEG = 1u << 25,
    F_IR1 = 1u << 24, F_IR2 = 1u << 23, F_IR3 = 1u << 22,
    F_COLOR_R = 1u << 21, F_COLOR_G = 1u << 20, F_COLOR_B = 1u << 19,
    F_SZ3_OTZ = 1u << 18, F_DIVIDE = 1u << 17,
    F_MAC0_POS = 1u << 16, F_MAC0_NEG = 1u << 15,
    F_SX2 = 1u << 14, F_SY2 = 1u << 13, F_IR0 = 1u << 12,
    /* Bits that set the summary error bit 31. */
    F_ERROR_MASK = 0x7F87E000u,
};

static u32 flag;

/* ---- register decoding helpers ---------------------------------------- */

static s16 lo16(u32 v) { return (s16)(v & 0xFFFF); }
static s16 hi16(u32 v) { return (s16)(v >> 16); }

static s16 VX(int n) { return lo16(data[PCGTE_VXY0 + n * 2]); }
static s16 VY(int n) { return hi16(data[PCGTE_VXY0 + n * 2]); }
static s16 VZ(int n) { return lo16(data[PCGTE_VZ0 + n * 2]); }

static s16 IR(int n) { return lo16(data[PCGTE_IR0 + n]); }
static void set_IR(int n, s32 v) { data[PCGTE_IR0 + n] = (u32)(s32)(s16)v; }

static s32 MAC(int n) { return (s32)data[PCGTE_MAC0 + n]; }

static u16 SZ(int n) { return (u16)data[PCGTE_SZ0 + n]; }
static s16 SX(int n) { return lo16(data[PCGTE_SXY0 + n]); }
static s16 SY(int n) { return hi16(data[PCGTE_SXY0 + n]); }

static uint8_t RGBC_R(void) { return (uint8_t)data[PCGTE_RGBC]; }
static uint8_t RGBC_G(void) { return (uint8_t)(data[PCGTE_RGBC] >> 8); }
static uint8_t RGBC_B(void) { return (uint8_t)(data[PCGTE_RGBC] >> 16); }
static uint8_t RGBC_CODE(void) { return (uint8_t)(data[PCGTE_RGBC] >> 24); }

/* A 3x3 matrix element from control registers starting at `base`
 * (RT, LLM or LCM): five words holding nine s16, row-major. */
static s16 mat(int base, int row, int col)
{
    int idx = row * 3 + col;
    u32 w = ctrl[base + idx / 2];
    return (idx & 1) ? hi16(w) : lo16(w);
}

/* ---- saturation / flag helpers ---------------------------------------- */

/* Checks a MAC1-3 intermediate against the 44-bit range and wraps it. */
static s64 A(int n, s64 v)
{
    if (v > 0x7FFFFFFFFFFLL)
        flag |= F_MAC1_POS >> (n - 1);
    else if (v < -0x80000000000LL)
        flag |= F_MAC1_NEG >> (n - 1);
    return (v << 20) >> 20;
}

/* Checks MAC0 against the 32-bit range. */
static s64 F(s64 v)
{
    if (v > 0x7FFFFFFFLL)
        flag |= F_MAC0_POS;
    else if (v < -0x80000000LL)
        flag |= F_MAC0_NEG;
    return v;
}

/* IR1-3 saturation: [-0x8000 or 0, 0x7FFF]. */
static s32 Lm_B(int n, s32 v, int lm)
{
    s32 lo = lm ? 0 : -0x8000;
    if (v < lo) { flag |= F_IR1 >> (n - 1); return lo; }
    if (v > 0x7FFF) { flag |= F_IR1 >> (n - 1); return 0x7FFF; }
    return v;
}

/* Colour FIFO component saturation: [0, 0xFF]. */
static uint8_t Lm_C(int n, s32 v)
{
    if (v < 0) { flag |= F_COLOR_R >> (n - 1); return 0; }
    if (v > 0xFF) { flag |= F_COLOR_R >> (n - 1); return 0xFF; }
    return (uint8_t)v;
}

/* SZ3/OTZ saturation: [0, 0xFFFF]. */
static u16 Lm_D(s64 v)
{
    if (v < 0) { flag |= F_SZ3_OTZ; return 0; }
    if (v > 0xFFFF) { flag |= F_SZ3_OTZ; return 0xFFFF; }
    return (u16)v;
}

/* SX2/SY2 saturation: [-0x400, 0x3FF]. */
static s16 Lm_G(int n, s64 v)
{
    if (v < -0x400) { flag |= n == 1 ? F_SX2 : F_SY2; return -0x400; }
    if (v > 0x3FF) { flag |= n == 1 ? F_SX2 : F_SY2; return 0x3FF; }
    return (s16)v;
}

/* IR0 saturation: [0, 0x1000]. */
static s16 Lm_H(s64 v)
{
    if (v < 0) { flag |= F_IR0; return 0; }
    if (v > 0x1000) { flag |= F_IR0; return 0x1000; }
    return (s16)v;
}

/* ---- FIFOs ------------------------------------------------------------- */

static void push_sz(u16 z)
{
    data[PCGTE_SZ0] = data[PCGTE_SZ1];
    data[PCGTE_SZ1] = data[PCGTE_SZ2];
    data[PCGTE_SZ2] = data[PCGTE_SZ3];
    data[PCGTE_SZ3] = z;
}

static void push_sxy(s16 x, s16 y)
{
    data[PCGTE_SXY0] = data[PCGTE_SXY1];
    data[PCGTE_SXY1] = data[PCGTE_SXY2];
    data[PCGTE_SXY2] = (u16)x | ((u32)(u16)y << 16);
}

/* Pushes MAC1-3 / 16 as a colour, with the CODE byte from RGBC. */
static void push_color(void)
{
    u32 r = Lm_C(1, MAC(1) >> 4);
    u32 g = Lm_C(2, MAC(2) >> 4);
    u32 b = Lm_C(3, MAC(3) >> 4);
    data[PCGTE_RGB0] = data[PCGTE_RGB1];
    data[PCGTE_RGB1] = data[PCGTE_RGB2];
    data[PCGTE_RGB2] = r | (g << 8) | (b << 16) | ((u32)RGBC_CODE() << 24);
}

/* ---- common result stores ---------------------------------------------- */

static void set_MAC(int n, s64 v, int sf)
{
    data[PCGTE_MAC0 + n] = (u32)(s32)(A(n, v) >> (sf * 12));
}

/* MAC1-3 = v >> sf, IR1-3 = Lm_B(MAC1-3, lm). */
static void set_MAC_IR(const s64 v[3], int sf, int lm)
{
    for (int i = 1; i <= 3; i++) {
        set_MAC(i, v[i - 1], sf);
        set_IR(i, Lm_B(i, MAC(i), lm));
    }
}

/* ---- UNR division (RTPS/RTPT) ------------------------------------------ */

static void init_unr(void)
{
    for (int i = 0; i < 0x101; i++) {
        int v = (0x40000 / (i + 0x100) + 1) / 2 - 0x101;
        unr_table[i] = (uint8_t)(v < 0 ? 0 : v);
    }
    unr_ready = 1;
}

static int clz16(u16 v)
{
    int n = 0;
    if (v == 0)
        return 16;
    while (!(v & 0x8000)) {
        v <<= 1;
        n++;
    }
    return n;
}

static u32 unr_divide(u16 h, u16 sz3)
{
    if (!unr_ready)
        init_unr();
    if (h >= (u32)sz3 * 2) {
        flag |= F_DIVIDE;
        return 0x1FFFF;
    }
    int z = clz16(sz3);
    uint64_t n = (uint64_t)h << z;
    u32 d = (u32)sz3 << z;
    u32 u = unr_table[(d - 0x7FC0) >> 7] + 0x101;
    d = (0x2000080 - d * u) >> 8;
    d = (0x0000080 + d * u) >> 8;
    uint64_t q = (n * d + 0x8000) >> 16;
    if (q > 0x1FFFF) {
        flag |= F_DIVIDE;
        return 0x1FFFF;
    }
    return (u32)q;
}

/* ---- commands ---------------------------------------------------------- */

/* Rotate, translate and perspective-transform vector n. */
static void rtp(int n, int sf, int lm, int last)
{
    s64 v[3];
    for (int r = 0; r < 3; r++) {
        s64 acc = (s64)(s32)ctrl[PCGTE_TRX + r] * 0x1000;
        acc = A(r + 1, acc + (s64)mat(PCGTE_RT0, r, 0) * VX(n));
        acc = A(r + 1, acc + (s64)mat(PCGTE_RT0, r, 1) * VY(n));
        acc = A(r + 1, acc + (s64)mat(PCGTE_RT0, r, 2) * VZ(n));
        v[r] = acc;
    }
    set_MAC(1, v[0], sf);
    set_MAC(2, v[1], sf);
    set_MAC(3, v[2], sf);
    set_IR(1, Lm_B(1, MAC(1), lm));
    set_IR(2, Lm_B(2, MAC(2), lm));
    /* IR3's saturation flag is checked on the unshifted value >> 12
     * regardless of sf; the stored value still comes from MAC3. */
    {
        s32 m3 = MAC(3);
        s32 lo = lm ? 0 : -0x8000;
        s64 z12 = v[2] >> 12;
        if (z12 < -0x8000 || z12 > 0x7FFF)
            flag |= F_IR3;
        set_IR(3, m3 < lo ? lo : (m3 > 0x7FFF ? 0x7FFF : m3));
    }
    push_sz(Lm_D(v[2] >> 12));

    s64 q = unr_divide((u16)ctrl[PCGTE_H], SZ(3));
    s64 m0;
    m0 = F(q * IR(1) + (s32)ctrl[PCGTE_OFX]);
    s16 sx = Lm_G(1, m0 >> 16);
    m0 = F(q * IR(2) + (s32)ctrl[PCGTE_OFY]);
    s16 sy = Lm_G(2, m0 >> 16);
    push_sxy(sx, sy);
    if (last) {
        m0 = F(q * (s16)ctrl[PCGTE_DQA] + (s32)ctrl[PCGTE_DQB]);
        data[PCGTE_MAC0] = (u32)(s32)m0;
        data[PCGTE_IR0] = (u32)(s32)Lm_H(m0 >> 12);
    } else {
        data[PCGTE_MAC0] = (u32)(s32)m0;
    }
}

static void cmd_nclip(void)
{
    s64 v = (s64)SX(0) * SY(1) + (s64)SX(1) * SY(2) + (s64)SX(2) * SY(0)
          - (s64)SX(0) * SY(2) - (s64)SX(1) * SY(0) - (s64)SX(2) * SY(1);
    data[PCGTE_MAC0] = (u32)(s32)F(v);
}

static void cmd_op(int sf, int lm)
{
    s64 d1 = mat(PCGTE_RT0, 0, 0), d2 = mat(PCGTE_RT0, 1, 1), d3 = mat(PCGTE_RT0, 2, 2);
    s64 v[3] = {
        d2 * IR(3) - d3 * IR(2),
        d3 * IR(1) - d1 * IR(3),
        d1 * IR(2) - d2 * IR(1),
    };
    set_MAC_IR(v, sf, lm);
}

/* Depth-cue interpolation of MAC1-3 (already set, unshifted scale) towards
 * the far colour, then push the colour. */
static void interpolate(s64 m1, s64 m2, s64 m3, int sf, int lm)
{
    s64 in[3] = { m1, m2, m3 };
    s64 fc[3];
    for (int i = 0; i < 3; i++) {
        fc[i] = ((s64)(s32)ctrl[PCGTE_RFC + i] << 12) - in[i];
        set_MAC(i + 1, fc[i], sf);
        set_IR(i + 1, Lm_B(i + 1, MAC(i + 1), 0));
    }
    s64 out[3];
    for (int i = 0; i < 3; i++)
        out[i] = A(i + 1, (s64)IR(i + 1) * IR(0) + in[i]);
    set_MAC_IR(out, sf, lm);
    push_color();
}

static void cmd_dpcs(int sf, int lm, u32 rgb)
{
    interpolate((s64)(rgb & 0xFF) << 16, (s64)((rgb >> 8) & 0xFF) << 16,
                (s64)((rgb >> 16) & 0xFF) << 16, sf, lm);
}

static void cmd_intpl(int sf, int lm)
{
    interpolate((s64)IR(1) << 12, (s64)IR(2) << 12, (s64)IR(3) << 12, sf, lm);
}

static void cmd_dcpl(int sf, int lm)
{
    interpolate(((s64)RGBC_R() * IR(1)) << 4, ((s64)RGBC_G() * IR(2)) << 4,
                ((s64)RGBC_B() * IR(3)) << 4, sf, lm);
}

static void cmd_sqr(int sf, int lm)
{
    s64 v[3] = { (s64)IR(1) * IR(1), (s64)IR(2) * IR(2), (s64)IR(3) * IR(3) };
    set_MAC_IR(v, sf, lm);
}

static void cmd_avsz3(void)
{
    s64 v = F((s64)(s16)ctrl[PCGTE_ZSF3] * ((s64)SZ(1) + SZ(2) + SZ(3)));
    data[PCGTE_MAC0] = (u32)(s32)v;
    data[PCGTE_OTZ] = Lm_D(v >> 12);
}

static void cmd_avsz4(void)
{
    s64 v = F((s64)(s16)ctrl[PCGTE_ZSF4] * ((s64)SZ(0) + SZ(1) + SZ(2) + SZ(3)));
    data[PCGTE_MAC0] = (u32)(s32)v;
    data[PCGTE_OTZ] = Lm_D(v >> 12);
}

static void cmd_gpf(int sf, int lm)
{
    s64 v[3] = { (s64)IR(1) * IR(0), (s64)IR(2) * IR(0), (s64)IR(3) * IR(0) };
    set_MAC_IR(v, sf, lm);
    push_color();
}

static void cmd_gpl(int sf, int lm)
{
    s64 v[3];
    for (int i = 0; i < 3; i++)
        v[i] = A(i + 1, ((s64)MAC(i + 1) << (sf * 12)) + (s64)IR(i + 1) * IR(0));
    set_MAC_IR(v, sf, lm);
    push_color();
}

/* Matrix x vector + translation, the core of MVMVA and the lighting
 * commands. mx: 0 RT, 1 LLM, 2 LCM, 3 garbage. vec: 0-2 Vn, 3 IR.
 * cv: 0 TR, 1 BK, 2 FC (hardware bug), 3 none. */
static void mvmva(int sf, int lm, int mx, int vec, int cv)
{
    s64 vx, vy, vz;
    if (vec == 3) {
        vx = IR(1); vy = IR(2); vz = IR(3);
    } else {
        vx = VX(vec); vy = VY(vec); vz = VZ(vec);
    }

    s64 m[3][3];
    if (mx == 3) {
        /* Reserved matrix: reads garbage made of RGBC.R, IR0 and RT. */
        s64 r = (s64)RGBC_R() << 4;
        m[0][0] = -r; m[0][1] = r; m[0][2] = IR(0);
        m[1][0] = m[1][1] = m[1][2] = mat(PCGTE_RT0, 0, 2);
        m[2][0] = m[2][1] = m[2][2] = mat(PCGTE_RT0, 1, 1);
    } else {
        int base = mx == 0 ? PCGTE_RT0 : mx == 1 ? PCGTE_L0 : PCGTE_LC0;
        for (int r = 0; r < 3; r++)
            for (int c = 0; c < 3; c++)
                m[r][c] = mat(base, r, c);
    }

    s64 t[3] = { 0, 0, 0 };
    if (cv != 3) {
        int base = cv == 0 ? PCGTE_TRX : cv == 1 ? PCGTE_RBK : PCGTE_RFC;
        for (int i = 0; i < 3; i++)
            t[i] = (s64)(s32)ctrl[base + i];
    }

    s64 out[3];
    for (int r = 0; r < 3; r++) {
        if (cv == 2) {
            /* FC translation is broken: the first column's product is only
             * used for flags, and the result keeps the other two. */
            A(r + 1, (t[r] << 12) + m[r][0] * vx);
            s64 acc = A(r + 1, m[r][1] * vy);
            acc = A(r + 1, acc + m[r][2] * vz);
            out[r] = acc;
        } else {
            s64 acc = A(r + 1, (t[r] << 12) + m[r][0] * vx);
            acc = A(r + 1, acc + m[r][1] * vy);
            acc = A(r + 1, acc + m[r][2] * vz);
            out[r] = acc;
        }
    }
    set_MAC_IR(out, sf, lm);
}

/* Lighting stage shared by NCS/NCCS/NCDS (and CC/CDP from step 2). */
enum { TAIL_NONE, TAIL_COLOR, TAIL_DEPTH };

static void light_tail(int sf, int lm, int tail)
{
    /* Background colour + light colour matrix x IR. */
    mvmva(sf, lm, 2, 3, 1);
    if (tail == TAIL_NONE) {
        push_color();
        return;
    }
    s64 m1 = ((s64)RGBC_R() * IR(1)) << 4;
    s64 m2 = ((s64)RGBC_G() * IR(2)) << 4;
    s64 m3 = ((s64)RGBC_B() * IR(3)) << 4;
    if (tail == TAIL_COLOR) {
        s64 v[3] = { m1, m2, m3 };
        set_MAC_IR(v, sf, lm);
        push_color();
    } else {
        interpolate(m1, m2, m3, sf, lm);
    }
}

static void normal_color(int n, int sf, int lm, int tail)
{
    mvmva(sf, lm, 1, n, 3); /* LLM x Vn */
    light_tail(sf, lm, tail);
}

void pcgte_op(uint32_t instruction)
{
    int sf = (instruction >> 19) & 1;
    int mx = (instruction >> 17) & 3;
    int v = (instruction >> 15) & 3;
    int cv = (instruction >> 13) & 3;
    int lm = (instruction >> 10) & 1;

    flag = 0;
    switch (instruction & 0x3F) {
    case 0x01: rtp(0, sf, lm, 1); break;                      /* RTPS */
    case 0x06: cmd_nclip(); break;                            /* NCLIP */
    case 0x0C: cmd_op(sf, lm); break;                         /* OP */
    case 0x10: cmd_dpcs(sf, lm, data[PCGTE_RGBC]); break;     /* DPCS */
    case 0x11: cmd_intpl(sf, lm); break;                      /* INTPL */
    case 0x12: mvmva(sf, lm, mx, v, cv); break;               /* MVMVA */
    case 0x13: normal_color(0, sf, lm, TAIL_DEPTH); break;    /* NCDS */
    case 0x14: light_tail(sf, lm, TAIL_DEPTH); break;         /* CDP */
    case 0x16:                                                /* NCDT */
        for (int i = 0; i < 3; i++)
            normal_color(i, sf, lm, TAIL_DEPTH);
        break;
    case 0x1B: normal_color(0, sf, lm, TAIL_COLOR); break;    /* NCCS */
    case 0x1C: light_tail(sf, lm, TAIL_COLOR); break;         /* CC */
    case 0x1E: normal_color(0, sf, lm, TAIL_NONE); break;     /* NCS */
    case 0x20:                                                /* NCT */
        for (int i = 0; i < 3; i++)
            normal_color(i, sf, lm, TAIL_NONE);
        break;
    case 0x28: cmd_sqr(sf, lm); break;                        /* SQR */
    case 0x29: cmd_dcpl(sf, lm); break;                       /* DCPL */
    case 0x2A:                                                /* DPCT */
        for (int i = 0; i < 3; i++)
            cmd_dpcs(sf, lm, data[PCGTE_RGB0]);
        break;
    case 0x2D: cmd_avsz3(); break;                            /* AVSZ3 */
    case 0x2E: cmd_avsz4(); break;                            /* AVSZ4 */
    case 0x30:                                                /* RTPT */
        rtp(0, sf, lm, 0);
        rtp(1, sf, lm, 0);
        rtp(2, sf, lm, 1);
        break;
    case 0x3D: cmd_gpf(sf, lm); break;                        /* GPF */
    case 0x3E: cmd_gpl(sf, lm); break;                        /* GPL */
    case 0x3F:                                                /* NCCT */
        for (int i = 0; i < 3; i++)
            normal_color(i, sf, lm, TAIL_COLOR);
        break;
    default:
        break;
    }
    if (flag & F_ERROR_MASK)
        flag |= 0x80000000u;
    ctrl[PCGTE_FLAG] = flag;
}

void pcgte_op_dmpsx_mvmva(uint32_t placeholder)
{
    u32 sf = (placeholder >> 25) & 1;
    u32 mx = (placeholder >> 23) & 3;
    u32 v = (placeholder >> 21) & 3;
    u32 cv = (placeholder >> 19) & 3;
    u32 lm = (placeholder >> 18) & 1;
    pcgte_op(0x4A000012u | (sf << 19) | (mx << 17) | (v << 15) | (cv << 13) | (lm << 10));
}

/* ---- register access --------------------------------------------------- */

static u32 sat5(s16 ir)
{
    s32 v = ir >> 7;
    return v < 0 ? 0 : v > 0x1F ? 0x1F : (u32)v;
}

uint32_t pcgte_read_data(unsigned reg)
{
    reg &= 31;
    switch (reg) {
    case PCGTE_VZ0: case PCGTE_VZ1: case PCGTE_VZ2:
    case PCGTE_IR0: case PCGTE_IR1: case PCGTE_IR2: case PCGTE_IR3:
        return (u32)(s32)(s16)data[reg];
    case PCGTE_OTZ: case PCGTE_SZ0: case PCGTE_SZ1: case PCGTE_SZ2: case PCGTE_SZ3:
        return data[reg] & 0xFFFF;
    case PCGTE_SXYP:
        return data[PCGTE_SXY2];
    case PCGTE_IRGB: case PCGTE_ORGB:
        return sat5(IR(1)) | (sat5(IR(2)) << 5) | (sat5(IR(3)) << 10);
    default:
        return data[reg & 31];
    }
}

static u32 leading_count(u32 v)
{
    /* Leading zeros of a positive value, leading ones of a negative one. */
    if (v & 0x80000000u)
        v = ~v;
    u32 n = 0;
    while (n < 32 && !(v & 0x80000000u)) {
        v <<= 1;
        n++;
    }
    return n;
}

void pcgte_write_data(unsigned reg, uint32_t value)
{
    switch (reg & 31) {
    case PCGTE_SXYP:
        data[PCGTE_SXY0] = data[PCGTE_SXY1];
        data[PCGTE_SXY1] = data[PCGTE_SXY2];
        data[PCGTE_SXY2] = value;
        break;
    case PCGTE_IRGB:
        data[PCGTE_IRGB] = value & 0x7FFF;
        data[PCGTE_IR1] = (value & 0x1F) << 7;
        data[PCGTE_IR2] = ((value >> 5) & 0x1F) << 7;
        data[PCGTE_IR3] = ((value >> 10) & 0x1F) << 7;
        break;
    case PCGTE_ORGB:
    case PCGTE_LZCR:
        break; /* read-only */
    case PCGTE_LZCS:
        data[PCGTE_LZCS] = value;
        data[PCGTE_LZCR] = leading_count(value);
        break;
    default:
        data[reg & 31] = value;
        break;
    }
}

uint32_t pcgte_read_ctrl(unsigned reg)
{
    reg &= 31;
    switch (reg) {
    case PCGTE_RT4: case PCGTE_L4: case PCGTE_LC4:
    case PCGTE_H: case PCGTE_DQA: case PCGTE_ZSF3: case PCGTE_ZSF4:
        /* 16-bit registers read back sign-extended (H included). */
        return (u32)(s32)(s16)ctrl[reg];
    default:
        return ctrl[reg & 31];
    }
}

void pcgte_write_ctrl(unsigned reg, uint32_t value)
{
    if ((reg & 31) == PCGTE_FLAG) {
        value &= 0x7FFFF000u;
        if (value & F_ERROR_MASK)
            value |= 0x80000000u;
    }
    ctrl[reg & 31] = value;
}

void pcgte_reset(void)
{
    memset(data, 0, sizeof(data));
    memset(ctrl, 0, sizeof(ctrl));
}

/* Snapshot / restore of the whole register file (64 words), for the runtime's
 * interrupt delivery. Raw copies: no register write side effects. */
void pcgte_save(uint32_t out[64])
{
    memcpy(out, data, sizeof(data));
    memcpy(out + 32, ctrl, sizeof(ctrl));
}

void pcgte_restore(const uint32_t in[64])
{
    memcpy(data, in, sizeof(data));
    memcpy(ctrl, in + 32, sizeof(ctrl));
}
