/*
 * The geometry library (libgte) routines the game calls, written for the
 * port from the library's documented behaviour. Results, and what the
 * routines leave in the GTE registers, match the original library bit for
 * bit; PE2_LIBTEST (libtest.c) checks them against it.
 */
#include <math.h>
#include <stdint.h>

#include "pc/gte.h"

typedef struct {
    int16_t m[3][3];
    int16_t pad;
    int32_t t[3];
} Matrix;

typedef struct {
    int32_t vx, vy, vz, pad;
} Vector;

typedef struct {
    int16_t vx, vy, vz, pad;
} SVector;

static uint32_t pack(int16_t lo, int16_t hi)
{
    return (uint16_t)lo | ((uint32_t)(uint16_t)hi << 16);
}

static void load_rotation(const Matrix* m)
{
    pcgte_write_ctrl(PCGTE_RT0, pack(m->m[0][0], m->m[0][1]));
    pcgte_write_ctrl(PCGTE_RT1, pack(m->m[0][2], m->m[1][0]));
    pcgte_write_ctrl(PCGTE_RT2, pack(m->m[1][1], m->m[1][2]));
    pcgte_write_ctrl(PCGTE_RT3, pack(m->m[2][0], m->m[2][1]));
    /* One word load: m[2][2] and the padding after it. */
    pcgte_write_ctrl(PCGTE_RT4, *(const uint32_t*)&m->m[2][2]);
}

static void load_translation(const Matrix* m)
{
    pcgte_write_ctrl(PCGTE_TRX, (uint32_t)m->t[0]);
    pcgte_write_ctrl(PCGTE_TRY, (uint32_t)m->t[1]);
    pcgte_write_ctrl(PCGTE_TRZ, (uint32_t)m->t[2]);
}

/* ---- matrix registers ---------------------------------------------------- */

void pcrt_SetRotMatrix(Matrix* m)
{
    load_rotation(m);
}

void pcrt_SetTransMatrix(Matrix* m)
{
    load_translation(m);
}

/* ---- matrix arithmetic --------------------------------------------------- */

Matrix* pcrt_TransposeMatrix(Matrix* m0, Matrix* m1)
{
    Matrix t = *m0;
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            m1->m[i][j] = t.m[j][i];
        }
    }
    return m1;
}

/* ---- trigonometry -------------------------------------------------------- */

/* sin over a quarter turn, 4096 = 1.0, a full turn being 4096 steps. */
static int16_t sin_quarter[1025];

static void init_sin(void)
{
    if (sin_quarter[1024] != 0) {
        return;
    }
    for (int i = 0; i <= 1024; i++) {
        sin_quarter[i] = (int16_t)lround(sin(i * (3.14159265358979323846 / 2048.0)) * 4096.0);
    }
}

static int sin4096(int a)
{
    init_sin();
    a &= 4095;
    if (a < 1024) {
        return sin_quarter[a];
    }
    if (a < 2048) {
        return sin_quarter[2048 - a];
    }
    if (a < 3072) {
        return -sin_quarter[a - 2048];
    }
    return -sin_quarter[4096 - a];
}

int pcrt_rsin(int a)
{
    return sin4096(a);
}

int pcrt_rcos(int a)
{
    return sin4096(a + 1024);
}

/* ---- GTE helpers --------------------------------------------------------- */

/* MVMVA: multiply a matrix (mx) by a vector (v), optionally add a vector (cv). */
enum { MX_RT = 0, MX_LLM = 1, MX_LCM = 2 };
enum { V_V0 = 0, V_V1 = 1, V_V2 = 2, V_IR = 3 };
enum { CV_TR = 0, CV_BK = 1, CV_FC = 2, CV_NONE = 3 };

static void mvmva(int sf, int mx, int v, int cv, int lm)
{
    pcgte_op(0x4A000012u | (uint32_t)sf << 19 | (uint32_t)mx << 17 | (uint32_t)v << 15 | (uint32_t)cv << 13 |
             (uint32_t)lm << 10);
}

static int32_t ir(int i)
{
    return (int32_t)pcgte_read_data(PCGTE_IR1 + i);
}

static int32_t mac(int i)
{
    return (int32_t)pcgte_read_data(PCGTE_MAC1 + i);
}

static void load_ir(int32_t x, int32_t y, int32_t z)
{
    pcgte_write_data(PCGTE_IR1, (uint32_t)x);
    pcgte_write_data(PCGTE_IR2, (uint32_t)y);
    pcgte_write_data(PCGTE_IR3, (uint32_t)z);
}

/* Word loads, as the library does: vz comes with the padding after it. */
static void load_v0(const SVector* v)
{
    pcgte_write_data(PCGTE_VXY0, *(const uint32_t*)&v->vx);
    pcgte_write_data(PCGTE_VZ0, *(const uint32_t*)&v->vz);
}

/* m[2][2] is stored as a word: the padding after it gets the high half. */
static void store_m22(Matrix* m, int32_t v)
{
    *(int32_t*)&m->m[2][2] = v;
}

/* m2 = RT * m1, a column at a time through V0 (m2 may be m1). The last
 * column's z is a word load of m1[2][2] and the padding. */
static void rotate_columns(const Matrix* m1, Matrix* m2)
{
    int16_t col[3][3];
    for (int j = 0; j < 3; j++) {
        pcgte_write_data(PCGTE_VXY0, pack(m1->m[0][j], m1->m[1][j]));
        pcgte_write_data(PCGTE_VZ0, j == 2 ? *(const uint32_t*)&m1->m[2][2] : (uint32_t)(int32_t)m1->m[2][j]);
        mvmva(1, MX_RT, V_V0, CV_NONE, 0);
        col[j][0] = (int16_t)ir(0);
        col[j][1] = (int16_t)ir(1);
        col[j][2] = (int16_t)ir(2);
    }
    for (int j = 0; j < 3; j++) {
        m2->m[0][j] = col[j][0];
        m2->m[1][j] = col[j][1];
        if (j < 2) {
            m2->m[2][j] = col[j][2];
        }
    }
    store_m22(m2, (int16_t)col[2][2]);
}

/* ---- matrix products ----------------------------------------------------- */

Matrix* pcrt_MulMatrix0(Matrix* m0, Matrix* m1, Matrix* m2)
{
    load_rotation(m0);
    rotate_columns(m1, m2);
    return m2;
}

Matrix* pcrt_MulMatrix(Matrix* m0, Matrix* m1)
{
    load_rotation(m0);
    rotate_columns(m1, m0);
    return m0;
}

Matrix* pcrt_MulRotMatrix(Matrix* m0)
{
    rotate_columns(m0, m0);
    return m0;
}

/* ---- matrix times vector ------------------------------------------------- */

Vector* pcrt_ApplyMatrix(Matrix* m, SVector* v0, Vector* v1)
{
    load_rotation(m);
    load_v0(v0);
    mvmva(1, MX_RT, V_V0, CV_NONE, 0);
    v1->vx = mac(0);
    v1->vy = mac(1);
    v1->vz = mac(2);
    return v1;
}

SVector* pcrt_ApplyMatrixSV(Matrix* m, SVector* v0, SVector* v1)
{
    load_rotation(m);
    load_v0(v0);
    mvmva(1, MX_RT, V_V0, CV_NONE, 0);
    v1->vx = (int16_t)ir(0);
    v1->vy = (int16_t)ir(1);
    v1->vz = (int16_t)ir(2);
    return v1;
}

/* ---- scaling ------------------------------------------------------------- */

Matrix* pcrt_ScaleMatrix(Matrix* m, Vector* v)
{
    const int32_t s[3] = {v->vx, v->vy, v->vz};
    int32_t r[3][3];
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            r[i][j] = (m->m[i][j] * s[j]) >> 12;
        }
    }
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            if (i < 2 || j < 2) {
                m->m[i][j] = (int16_t)r[i][j];
            }
        }
    }
    store_m22(m, r[2][2]);
    return m;
}

/* ---- long vectors -------------------------------------------------------- */

/* v1 = M * v0 for 32-bit v0: v = high * 32768 + low with both truncated
 * toward zero; the high part goes through IR1-3 unshifted, then the low
 * part, shifted by 12, through IR1-3 or (low_in_v0) V0. */
static void apply_long(const Vector* v0, Vector* v1, int low_in_v0)
{
    const int32_t v[3] = {v0->vx, v0->vy, v0->vz};
    load_ir(v[0] / 32768, v[1] / 32768, v[2] / 32768);
    mvmva(0, MX_RT, V_IR, CV_NONE, 0);
    int32_t h0 = mac(0), h1 = mac(1), h2 = mac(2);
    if (low_in_v0) {
        pcgte_write_data(PCGTE_VXY0, pack((int16_t)(v[0] % 32768), (int16_t)(v[1] % 32768)));
        pcgte_write_data(PCGTE_VZ0, (uint32_t)(v[2] % 32768));
        mvmva(1, MX_RT, V_V0, CV_NONE, 0);
    } else {
        load_ir(v[0] % 32768, v[1] % 32768, v[2] % 32768);
        mvmva(1, MX_RT, V_IR, CV_NONE, 0);
    }
    v1->vx = (h0 << 3) + mac(0);
    v1->vy = (h1 << 3) + mac(1);
    v1->vz = (h2 << 3) + mac(2);
}

Vector* pcrt_ApplyMatrixLV(Matrix* m, Vector* v0, Vector* v1)
{
    load_rotation(m);
    apply_long(v0, v1, 0);
    return v1;
}

/* ---- square roots -------------------------------------------------------- */

/* sqrt(i) for i in 64..255, in 9 fractional bits, rounded down. */
static uint16_t sqrt_table[256];

static void init_sqrt(void)
{
    if (sqrt_table[255] != 0) {
        return;
    }
    for (uint32_t i = 64; i < 256; i++) {
        uint32_t x = i << 18, r = 0;
        for (uint32_t bit = 1u << 30; bit != 0; bit >>= 2) {
            if (x >= r + bit) {
                x -= r + bit;
                r = (r >> 1) + bit;
            } else {
                r >>= 1;
            }
        }
        sqrt_table[i] = (uint16_t)r;
    }
}

/* sqrt(a) << extra: a is scaled by an even power of two into 64..255 (its
 * leading zeros counted with the GTE's LZCS/LZCR), the root looked up, and
 * the result scaled back. Negative a has no meaningful root; the original
 * reads past its table there, this returns 0. */
static long square_root(long a, int extra)
{
    pcgte_write_data(PCGTE_LZCS, (uint32_t)a);
    int lz = (int)pcgte_read_data(PCGTE_LZCR);
    if (a <= 0) {
        return 0;
    }
    init_sqrt();
    int s = 31 - lz - 7;
    s += s & 1;
    uint32_t idx = s >= 0 ? (uint32_t)a >> s : (uint32_t)a << -s;
    int shift = s / 2 + extra - 9;
    uint32_t v = sqrt_table[idx];
    return (long)(shift >= 0 ? v << shift : v >> -shift);
}

long pcrt_SquareRoot0(long a)
{
    return square_root(a, 0);
}

/* sqrt(a) in 12 fractional bits, for a with 12 fractional bits. */
long pcrt_SquareRoot12(long a)
{
    return square_root(a, 6);
}

/* ---- transposed and row scaling ------------------------------------------ */

/* RT = transpose(m); m[2][2] stays a word load with the padding. */
static void load_rotation_transposed(const Matrix* m)
{
    pcgte_write_ctrl(PCGTE_RT0, pack(m->m[0][0], m->m[1][0]));
    pcgte_write_ctrl(PCGTE_RT1, pack(m->m[2][0], m->m[0][1]));
    pcgte_write_ctrl(PCGTE_RT2, pack(m->m[1][1], m->m[2][1]));
    pcgte_write_ctrl(PCGTE_RT3, pack(m->m[0][2], m->m[1][2]));
    pcgte_write_ctrl(PCGTE_RT4, *(const uint32_t*)&m->m[2][2]);
}

Vector* pcrt_ApplyTransposeMatrixLV(Matrix* m, Vector* v0, Vector* v1)
{
    load_rotation_transposed(m);
    apply_long(v0, v1, 1);
    return v1;
}

Matrix* pcrt_ScaleMatrixL(Matrix* m, Vector* v)
{
    const int32_t s[3] = {v->vx, v->vy, v->vz};
    int32_t r[3][3];
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            r[i][j] = (m->m[i][j] * s[i]) >> 12;
        }
    }
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            if (i < 2 || j < 2) {
                m->m[i][j] = (int16_t)r[i][j];
            }
        }
    }
    store_m22(m, r[2][2]);
    return m;
}

/* ---- rotation about one axis --------------------------------------------- */

/* m = R * m, R rotating by r about the axis between rows a and b. */
static Matrix* rotate_rows(long r, Matrix* m, int a, int b, int sign)
{
    int32_t s = sin4096((int)r) * sign, c = sin4096((int)r + 1024);
    for (int j = 0; j < 3; j++) {
        int32_t x = m->m[a][j], y = m->m[b][j];
        m->m[a][j] = (int16_t)((c * x - s * y) >> 12);
        m->m[b][j] = (int16_t)((s * x + c * y) >> 12);
    }
    return m;
}

Matrix* pcrt_RotMatrixX(long r, Matrix* m)
{
    return rotate_rows(r, m, 1, 2, 1);
}

Matrix* pcrt_RotMatrixY(long r, Matrix* m)
{
    return rotate_rows(r, m, 2, 0, 1);
}

Matrix* pcrt_RotMatrixZ(long r, Matrix* m)
{
    return rotate_rows(r, m, 0, 1, 1);
}

/* ---- rotation from three angles ------------------------------------------ */

/* m = Rx * Ry * Rz for the angles in r. */
Matrix* pcrt_RotMatrix(SVector* r, Matrix* m)
{
    int32_t sx = sin4096(r->vx), cx = sin4096(r->vx + 1024);
    int32_t sy = sin4096(r->vy), cy = sin4096(r->vy + 1024);
    int32_t sz = sin4096(r->vz), cz = sin4096(r->vz + 1024);
    int32_t u = (-sy * cz) >> 12, w = (-sy * sz) >> 12;
    m->m[0][0] = (int16_t)((cy * cz) >> 12);
    m->m[0][1] = (int16_t)((-cy * sz) >> 12);
    m->m[0][2] = (int16_t)sy;
    m->m[1][0] = (int16_t)(-((u * sx) >> 12) + ((cx * sz) >> 12));
    m->m[1][1] = (int16_t)(((w * sx) >> 12) + ((cx * cz) >> 12));
    m->m[1][2] = (int16_t)((-sx * cy) >> 12);
    m->m[2][0] = (int16_t)(((u * cx) >> 12) + ((sx * sz) >> 12));
    m->m[2][1] = (int16_t)(-((w * cx) >> 12) + ((sx * cz) >> 12));
    m->m[2][2] = (int16_t)((cx * cy) >> 12);
    return m;
}

/* GPF: IR1-3 = IR0 * (a, b, c) >> 12, through the GTE (its colour FIFO
 * too). Returns nothing; the products are read back from IR1-3. */
static void gpf(int32_t ir0, int32_t a, int32_t b, int32_t c)
{
    pcgte_write_data(PCGTE_IR0, (uint32_t)ir0);
    load_ir(a, b, c);
    pcgte_op(0x4B98003Du); /* GPF, sf = 1 */
}

/* RotMatrix computed with the GTE: the same rotation as RotMatrix, but the
 * three-angle products come from GPF, so a few elements round differently. */
Matrix* pcrt_RotMatrix_gte(SVector* r, Matrix* m)
{
    int32_t sx = sin4096(r->vx), cx = sin4096(r->vx + 1024);
    int32_t sy = sin4096(r->vy), cy = sin4096(r->vy + 1024);
    int32_t sz = sin4096(r->vz), cz = sin4096(r->vz + 1024);

    gpf(cx, sy, sz, cz);
    int32_t cxsy = ir(0), cxsz = ir(1), cxcz = ir(2);
    gpf(sx, sy, sz, cz);
    int32_t sxsy = ir(0), sxsz = ir(1), sxcz = ir(2);
    gpf(cz, cy, sxsy, cxsy);
    int32_t cycz = ir(0), sxsycz = ir(1), cxsycz = ir(2);
    gpf(sz, cy, sxsy, cxsy);
    int32_t cysz = ir(0), sxsysz = ir(1), cxsysz = ir(2);

    m->m[0][0] = (int16_t)cycz;
    m->m[0][1] = (int16_t)-cysz;
    m->m[0][2] = (int16_t)sy;
    m->m[1][0] = (int16_t)(sxsycz + cxsz);
    m->m[1][1] = (int16_t)(-sxsysz + cxcz);
    m->m[1][2] = (int16_t)-((sx * cy) >> 12);
    m->m[2][0] = (int16_t)(-cxsycz + sxsz);
    m->m[2][1] = (int16_t)(cxsysz + sxcz);
    m->m[2][2] = (int16_t)((cx * cy) >> 12);
    return m;
}

/* m = Rz * Ry * Rx for the angles in r. */
Matrix* pcrt_RotMatrixZYX(SVector* r, Matrix* m)
{
    int32_t sx = sin4096(r->vx), cx = sin4096(r->vx + 1024);
    int32_t sy = sin4096(r->vy), cy = sin4096(r->vy + 1024);
    int32_t sz = sin4096(r->vz), cz = sin4096(r->vz + 1024);
    int32_t u = (sy * sx) >> 12, w = (sy * cx) >> 12;
    m->m[0][0] = (int16_t)((cz * cy) >> 12);
    m->m[0][1] = (int16_t)(((u * cz) >> 12) - ((sz * cx) >> 12));
    m->m[0][2] = (int16_t)(((w * cz) >> 12) + ((sz * sx) >> 12));
    m->m[1][0] = (int16_t)((sz * cy) >> 12);
    m->m[1][1] = (int16_t)(((u * sz) >> 12) + ((cz * cx) >> 12));
    m->m[1][2] = (int16_t)(((w * sz) >> 12) - ((cz * sx) >> 12));
    m->m[2][0] = (int16_t)-sy;
    m->m[2][1] = (int16_t)((cy * sx) >> 12);
    m->m[2][2] = (int16_t)((cy * cx) >> 12);
    return m;
}

/* m = Rz * Rx * Ry for the angles in r. */
Matrix* pcrt_RotMatrixZXY(SVector* r, Matrix* m)
{
    int32_t sx = sin4096(r->vx), cx = sin4096(r->vx + 1024);
    int32_t sy = sin4096(r->vy), cy = sin4096(r->vy + 1024);
    int32_t sz = sin4096(r->vz), cz = sin4096(r->vz + 1024);
    int32_t u = (sx * sy) >> 12, w = (sx * cy) >> 12;
    m->m[0][0] = (int16_t)(((cz * cy) >> 12) - ((u * sz) >> 12));
    m->m[0][1] = (int16_t)((-sz * cx) >> 12);
    m->m[0][2] = (int16_t)(((cz * sy) >> 12) + ((w * sz) >> 12));
    m->m[1][0] = (int16_t)(((sz * cy) >> 12) + ((u * cz) >> 12));
    m->m[1][1] = (int16_t)((cz * cx) >> 12);
    m->m[1][2] = (int16_t)(((sz * sy) >> 12) - ((w * cz) >> 12));
    m->m[2][0] = (int16_t)((-cx * sy) >> 12);
    m->m[2][1] = (int16_t)sx;
    m->m[2][2] = (int16_t)((cx * cy) >> 12);
    return m;
}

/* ---- arctangent ---------------------------------------------------------- */

/* atan(i / 1024) over the first octant, a full turn being 4096, sampled at
 * i + 3/8 and rounded. */
static int16_t atan_table[1025];

static void init_atan(void)
{
    if (atan_table[1024] != 0) {
        return;
    }
    for (int i = 0; i <= 1024; i++) {
        atan_table[i] = (int16_t)lround(atan((i + 0.375) / 1024.0) * (2048.0 / 3.14159265358979323846));
    }
}

/* Angle of a / b for 0 <= a < b (or a == b): the table index is
 * (a << 10) / b, or a / (b >> 10) when a << 10 would overflow. All
 * arithmetic is 32-bit, as on the console: an input of 0x80000000 stays
 * negative after negation and lands on index 0. */
static int32_t atan_octant(int32_t a, int32_t b)
{
    int32_t i;
    if (a & 0x7FE00000) {
        i = a / (b >> 10);
    } else {
        int32_t n = (int32_t)((uint32_t)a << 10);
        i = b != 0 ? n / b : 0;
    }
    if (i < 0) {
        i = 0;
    } else if (i > 1024) {
        i = 1024;
    }
    return atan_table[i];
}

/* atan2(y, x), a full turn being 4096. */
long pcrt_ratan2(long y, long x)
{
    init_atan();
    int neg_x = x < 0, neg_y = y < 0;
    int32_t ax = neg_x ? (int32_t)(0u - (uint32_t)x) : (int32_t)x;
    int32_t ay = neg_y ? (int32_t)(0u - (uint32_t)y) : (int32_t)y;
    if (ax == 0 && ay == 0) {
        return 0;
    }
    int32_t t = ay < ax ? atan_octant(ay, ax) : 1024 - atan_octant(ax, ay);
    if (neg_x) {
        t = 2048 - t;
    }
    return neg_y ? -t : t;
}

/* ---- normalization ------------------------------------------------------- */

/* 32768 / sqrt(i) for i in 64..255, rounded down. */
static uint16_t rsqrt_table[256];

static void init_rsqrt(void)
{
    if (rsqrt_table[64] != 0) {
        return;
    }
    for (int i = 64; i < 256; i++) {
        rsqrt_table[i] = (uint16_t)floor(32768.0 / sqrt((double)i));
    }
}

/* Scales the vector (its components' low 16 bits) to length 4096. The sum
 * of squares, also returned, comes from the GTE's SQR and its leading zeros
 * from LZCS/LZCR; it is scaled by an even power of two into 2^30..2^32 to
 * look up 1/sqrt, and GPF applies that to the vector. A sum of 2^31 or
 * more makes LZCR count leading ones and the original read past its table;
 * this takes the sum as unsigned instead. */
static int32_t normalize(int32_t x, int32_t y, int32_t z, int32_t out[3])
{
    init_rsqrt();
    load_ir(x, y, z);
    pcgte_op(0x4AA00428u); /* SQR, sf = 0 */
    uint32_t sum = (uint32_t)mac(0) + (uint32_t)mac(1) + (uint32_t)mac(2);
    pcgte_write_data(PCGTE_LZCS, sum);
    int lz = (int)pcgte_read_data(PCGTE_LZCR);
    if ((int32_t)sum < 0) {
        lz = 0;
    }
    int32_t scale = 0;
    if (sum != 0) {
        int e = lz & ~1;
        scale = rsqrt_table[(sum << e) >> 24];
    }
    pcgte_write_data(PCGTE_IR0, (uint32_t)scale);
    load_ir(x, y, z);
    pcgte_op(0x4A90003Du); /* GPF, sf = 0 */
    int shift = 15 - (lz >> 1);
    if (shift < 0) {
        shift = 0;
    }
    for (int i = 0; i < 3; i++) {
        out[i] = mac(i) >> shift;
    }
    return (int32_t)sum;
}

long pcrt_VectorNormal(Vector* v0, Vector* v1)
{
    int32_t out[3];
    int32_t sum = normalize((int16_t)v0->vx, (int16_t)v0->vy, (int16_t)v0->vz, out);
    v1->vx = out[0];
    v1->vy = out[1];
    v1->vz = out[2];
    return sum;
}

long pcrt_VectorNormalS(Vector* v0, SVector* v1)
{
    int32_t out[3];
    int32_t sum = normalize((int16_t)v0->vx, (int16_t)v0->vy, (int16_t)v0->vz, out);
    v1->vx = (int16_t)out[0];
    v1->vy = (int16_t)out[1];
    v1->vz = (int16_t)out[2];
    return sum;
}

long pcrt_VectorNormalSS(SVector* v0, SVector* v1)
{
    int32_t out[3];
    int32_t sum = normalize(v0->vx, v0->vy, v0->vz, out);
    v1->vx = (int16_t)out[0];
    v1->vy = (int16_t)out[1];
    v1->vz = (int16_t)out[2];
    return sum;
}

/* ---- orthonormalization -------------------------------------------------- */

/* (a x b) >> 12 in 32-bit arithmetic, each component kept to 16 bits. */
static void cross12(const int16_t a[3], const int16_t b[3], int16_t out[3])
{
    out[0] = (int16_t)((int32_t)((uint32_t)(a[1] * b[2]) - (uint32_t)(a[2] * b[1])) >> 12);
    out[1] = (int16_t)((int32_t)((uint32_t)(a[2] * b[0]) - (uint32_t)(a[0] * b[2])) >> 12);
    out[2] = (int16_t)((int32_t)((uint32_t)(a[0] * b[1]) - (uint32_t)(a[1] * b[0])) >> 12);
}

static void normalize_row(const int16_t v[3], int16_t out[3])
{
    int32_t r[3];
    normalize(v[0], v[1], v[2], r);
    out[0] = (int16_t)r[0];
    out[1] = (int16_t)r[1];
    out[2] = (int16_t)r[2];
}

enum { ROW_KEEP, ROW_C, ROW_D };

/* Rebuilds an orthonormal rotation from m: row `keep` is m's row
 * normalized, row `second` the normalized c = m[a] x m[b], and row `third`
 * the normalized d = m[keep] x c (or c x m[keep] with third_ab). The three
 * normalizations run in `order`, which shows in the GTE state left behind.
 * m and n may be the same. */
static void matrix_normal(const Matrix* m, Matrix* n, int keep, int a, int b, int second, int third, int third_ab,
                          const int order[3])
{
    int16_t rows[3][3], c[3], d[3], out[3][3];
    /* The rotation registers are saved and restored around the work; RT4
     * reads back as its low half sign-extended. */
    uint32_t rt4 = (uint32_t)(int32_t)(int16_t)pcgte_read_ctrl(PCGTE_RT4);
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            rows[i][j] = m->m[i][j];
        }
    }
    cross12(rows[a], rows[b], c);
    if (third_ab == 0) {
        cross12(rows[keep], c, d);
    } else {
        cross12(c, rows[keep], d);
    }
    for (int k = 0; k < 3; k++) {
        switch (order[k]) {
        case ROW_KEEP: normalize_row(rows[keep], out[keep]); break;
        case ROW_C: normalize_row(c, out[second]); break;
        default: normalize_row(d, out[third]); break;
        }
    }
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            n->m[i][j] = out[i][j];
        }
    }
    pcgte_write_ctrl(PCGTE_RT4, rt4);
}

void pcrt_MatrixNormal_0(Matrix* m, Matrix* n)
{
    static const int order[3] = {ROW_D, ROW_KEEP, ROW_C};
    matrix_normal(m, n, 1, 0, 1, 2, 0, 0, order);
}

void pcrt_MatrixNormal_1(Matrix* m, Matrix* n)
{
    static const int order[3] = {ROW_C, ROW_KEEP, ROW_D};
    matrix_normal(m, n, 1, 1, 2, 0, 2, 1, order);
}

void pcrt_MatrixNormal_2(Matrix* m, Matrix* n)
{
    static const int order[3] = {ROW_D, ROW_C, ROW_KEEP};
    matrix_normal(m, n, 2, 2, 0, 1, 0, 1, order);
}

/* MatrixNormal_0's result; this version also leaves m's middle row in the
 * GTE's VXY0, VZ0 and VXY1, one element each. For matrices far outside
 * rotation range (elements well above 4096), where the cross products
 * overflow 16 bits, the original's first row differs from MatrixNormal_0's
 * and this one does not follow it; rotations never get there. */
void pcrt_MatrixNormal(Matrix* m, Matrix* n)
{
    int32_t r0 = m->m[1][0], r1 = m->m[1][1], r2 = m->m[1][2];
    pcrt_MatrixNormal_0(m, n);
    pcgte_write_data(PCGTE_VXY0, (uint32_t)r0);
    pcgte_write_data(PCGTE_VZ0, (uint32_t)r1);
    pcgte_write_data(PCGTE_VXY1, (uint32_t)r2);
}

/* ---- perspective transformation ------------------------------------------ */

/* One point through the rotation, translation and perspective (RTPS): its
 * screen position to sxy (with its exact one, pgxp.c), its depth cue (IR0)
 * to p and the GTE flags to flag. Returns its depth, a quarter (SZ3 / 4). */
long pcrt_RotTransPers(SVector* v0, long* sxy, long* p, long* flag)
{
    load_v0(v0);
    pcgte_op(0x4A180001u); /* RTPS */
    pcgte_store_sxy(PCGTE_SXY2, (uint32_t*)sxy);
    *p = (long)pcgte_read_data(PCGTE_IR0);
    *flag = (long)pcgte_read_ctrl(PCGTE_FLAG);
    return (long)(pcgte_read_data(PCGTE_SZ3) >> 2);
}

/* Three points at once (RTPT), as RotTransPers: the last one's depth cue,
 * the flags and depth. */
long pcrt_RotTransPers3(SVector* v0, SVector* v1, SVector* v2, long* sxy0, long* sxy1, long* sxy2, long* p,
                        long* flag)
{
    pcgte_write_data(PCGTE_VXY0, *(const uint32_t*)&v0->vx);
    pcgte_write_data(PCGTE_VZ0, *(const uint32_t*)&v0->vz);
    pcgte_write_data(PCGTE_VXY1, *(const uint32_t*)&v1->vx);
    pcgte_write_data(PCGTE_VZ1, *(const uint32_t*)&v1->vz);
    pcgte_write_data(PCGTE_VXY2, *(const uint32_t*)&v2->vx);
    pcgte_write_data(PCGTE_VZ2, *(const uint32_t*)&v2->vz);
    pcgte_op(0x4A280030u); /* RTPT */
    pcgte_store_sxy(PCGTE_SXY0, (uint32_t*)sxy0);
    pcgte_store_sxy(PCGTE_SXY1, (uint32_t*)sxy1);
    pcgte_store_sxy(PCGTE_SXY2, (uint32_t*)sxy2);
    *p = (long)pcgte_read_data(PCGTE_IR0);
    *flag = (long)pcgte_read_ctrl(PCGTE_FLAG);
    return (long)(pcgte_read_data(PCGTE_SZ3) >> 2);
}

/* v0 rotated and translated (MVMVA with RT and TR, by 4096) into v1, as
 * halfwords (IR1-3, saturated to them; vz as a word, over the padding after
 * it); the GTE flags to flag. */
void pcrt_RotTransSV(SVector* v0, SVector* v1, long* flag)
{
    load_v0(v0);
    mvmva(1, 0, V_V0, CV_TR, 0);
    v1->vx = (int16_t)ir(0);
    v1->vy = (int16_t)ir(1);
    *(int32_t*)&v1->vz = ir(2);
    *flag = (long)pcgte_read_ctrl(PCGTE_FLAG);
}

/* ---- interpolation ------------------------------------------------------- */

/* v2's RGB = (v0 * p0 + v1 * p1) / 4096: the products summed in the GTE
 * (GPF then GPL, unshifted), each MAC / 4096 kept to its low byte. v2's
 * fourth byte is left as it is. */
void pcrt_LoadAverageCol(uint8_t* v0, uint8_t* v1, long p0, long p1, uint8_t* v2)
{
    pcgte_write_data(PCGTE_IR0, (uint32_t)p0);
    load_ir(v0[0], v0[1], v0[2]);
    pcgte_op(0x4A00003Du); /* GPF, sf=0 */
    pcgte_write_data(PCGTE_IR0, (uint32_t)p1);
    load_ir(v1[0], v1[1], v1[2]);
    pcgte_op(0x4A00003Eu); /* GPL, sf=0 */
    for (int i = 0; i < 3; i++) {
        v2[i] = (uint8_t)(mac(i) >> 12);
    }
}

/* Morphing: each of the n vectors of otp (n >= 1) moves by its dfp delta
 * times p / 4096, the products in the GTE (GPF, saturated to IR's halfwords),
 * the sums on the CPU, wrapping to halfwords. */
void pcrt_gteMIMefunc(SVector* otp, SVector* dfp, long n, long p)
{
    pcgte_write_data(PCGTE_IR0, (uint32_t)p);
    for (long i = 0; i < n; i++) {
        load_ir(dfp[i].vx, dfp[i].vy, dfp[i].vz);
        pcgte_op(0x4A08003Du); /* GPF, sf=1 */
        otp[i].vx = (int16_t)(otp[i].vx + ir(0));
        otp[i].vy = (int16_t)(otp[i].vy + ir(1));
        otp[i].vz = (int16_t)(otp[i].vz + ir(2));
    }
}

/* ---- four points ---------------------------------------------------------- */

/* Four points through the rotation, translation and perspective in the GTE:
 * v0..v2 together (RTPT), then v3 (RTPS). Their screen positions go to
 * sxy0..sxy3 (with their exact ones, pgxp.c); p gets the last point's depth
 * cue (IR0) and flag both projections' GTE flags together. Returns the last
 * point's depth, a quarter (SZ3 / 4): not the four's average, which the
 * name of its result in the library's documentation suggests, and no AVSZ4,
 * so OTZ and MAC0 keep what they held. */
long pcrt_RotTransPers4(SVector* v0, SVector* v1, SVector* v2, SVector* v3, long* sxy0, long* sxy1, long* sxy2,
                        long* sxy3, long* p, long* flag)
{
    uint32_t first_flags;

    pcgte_write_data(PCGTE_VXY0, *(const uint32_t*)&v0->vx);
    pcgte_write_data(PCGTE_VZ0, *(const uint32_t*)&v0->vz);
    pcgte_write_data(PCGTE_VXY1, *(const uint32_t*)&v1->vx);
    pcgte_write_data(PCGTE_VZ1, *(const uint32_t*)&v1->vz);
    pcgte_write_data(PCGTE_VXY2, *(const uint32_t*)&v2->vx);
    pcgte_write_data(PCGTE_VZ2, *(const uint32_t*)&v2->vz);
    pcgte_op(0x4A280030u); /* RTPT */
    pcgte_store_sxy(PCGTE_SXY0, (uint32_t*)sxy0);
    pcgte_store_sxy(PCGTE_SXY1, (uint32_t*)sxy1);
    pcgte_store_sxy(PCGTE_SXY2, (uint32_t*)sxy2);
    first_flags = pcgte_read_ctrl(PCGTE_FLAG);
    load_v0(v3);
    pcgte_op(0x4A180001u); /* RTPS */
    pcgte_store_sxy(PCGTE_SXY2, (uint32_t*)sxy3);
    *p = (long)pcgte_read_data(PCGTE_IR0);
    *flag = (long)(first_flags | pcgte_read_ctrl(PCGTE_FLAG));
    return (long)(pcgte_read_data(PCGTE_SZ3) >> 2);
}

/* ---- start-up ------------------------------------------------------------ */

/* The geometry defaults: screen offset 0, projection distance 1000, depth
 * cueing over the depth range, and the 1/3 and 1/4 scales that average three
 * and four Z values. The original also patches the BIOS exception handler
 * to save the GTE; the host's interrupts do not need that. */
void pcrt_InitGeom(void)
{
    pcgte_write_ctrl(PCGTE_OFX, 0);
    pcgte_write_ctrl(PCGTE_OFY, 0);
    pcgte_write_ctrl(PCGTE_H, 1000);
    pcgte_write_ctrl(PCGTE_DQA, (uint32_t)-0x1062);
    pcgte_write_ctrl(PCGTE_DQB, 0x1400000u);
    pcgte_write_ctrl(PCGTE_ZSF3, 0x155);
    pcgte_write_ctrl(PCGTE_ZSF4, 0x100);
}

/* ---- natural logarithm --------------------------------------------------- */

/* ln(a / 4096) * 4096. The exponent comes from the leading zeros (GTE
 * LZCS/LZCR) as (k - 12) * 2839, 2839 being ln 2 in this scale, so powers of
 * two are exact; the mantissa's share is ln(m) * 4096 rounded down and kept
 * below 2839. The original takes that share from tables and is up to a few
 * units lower; the game only divides the result by 2839 (an integer log2),
 * where the two always agree. a <= 0 has no logarithm: 0 here, a value read
 * from past the original's tables there. */
int pcrt_cln(int a)
{
    pcgte_write_data(PCGTE_LZCS, (uint32_t)a);
    int lz = (int)pcgte_read_data(PCGTE_LZCR);
    if (a <= 0) {
        return 0;
    }
    int k = 31 - lz;
    double m = (double)a / (double)(1u << k);
    int f = (int)floor(log(m) * 4096.0);
    if (f > 2838) {
        f = 2838;
    }
    return (k - 12) * 2839 + f;
}
