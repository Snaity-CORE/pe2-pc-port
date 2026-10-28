/*
 * The primitive and environment helpers of the graphics libraries (libgpu,
 * libgs) the game calls, written for the port from the documented GPU
 * packet formats (psx-spx). They only build packets and ordering tables in
 * memory; the GPU itself is the host's (runtime/gpu). PE2_LIBTEST checks
 * them against the original library.
 */
#include <stdint.h>

typedef struct {
    int16_t x, y, w, h;
} Rect;

/* DR_ENV: a tag and up to 15 command words. */
typedef struct {
    uint32_t tag;
    uint32_t code[15];
} DrEnv;

typedef struct {
    Rect clip;
    int16_t ofs[2];
    Rect tw;
    uint16_t tpage;
    uint8_t dtd, dfe, isbg, r0, g0, b0;
    DrEnv dr_env;
} DrawEnv;

typedef struct {
    Rect disp;
    Rect screen;
    uint8_t isinter, isrgb24, pad0, pad1;
} DispEnv;

/* Packet header: next pointer in the low 24 bits, word count in the top 8;
 * the first command word's top byte is the GPU command code. */
static void set_len(void* p, uint32_t len)
{
    ((uint8_t*)p)[3] = (uint8_t)len;
}

static void set_code(void* p, uint32_t code)
{
    ((uint8_t*)p)[7] = (uint8_t)code;
}

/* ---- ordering tables ----------------------------------------------------- */

/* Links packet p in front of the list at ot. */
void pcrt_AddPrim(uint32_t* ot, uint32_t* p)
{
    *p = (*p & 0xFF000000u) | (*ot & 0x00FFFFFFu);
    *ot = (*ot & 0xFF000000u) | ((uint32_t)(uintptr_t)p & 0x00FFFFFFu);
}

/* Appends p1's words to p0 when the result fits a 16-word packet: 0 on
 * success, -1 otherwise. */
int pcrt_MargePrim(uint32_t* p0, uint32_t* p1)
{
    uint32_t len0 = p0[0] >> 24, len1 = p1[0] >> 24;
    uint32_t len = len0 + len1 + 1;
    if (len > 16) {
        return -1;
    }
    set_len(p0, len);
    p1[0] = 0;
    return 0;
}

/* ---- primitive set-up ---------------------------------------------------- */

void pcrt_SetSprt(void* p)
{
    set_len(p, 4);
    set_code(p, 0x64);
}

void pcrt_SetTile(void* p)
{
    set_len(p, 3);
    set_code(p, 0x60);
}

void pcrt_SetPolyFT4(void* p)
{
    set_len(p, 9);
    set_code(p, 0x2C);
}

void pcrt_SetSemiTrans(void* p, int abe)
{
    uint8_t* code = (uint8_t*)p + 7;
    *code = abe ? (uint8_t)(*code | 2) : (uint8_t)(*code & ~2);
}

void pcrt_SetShadeTex(void* p, int tge)
{
    uint8_t* code = (uint8_t*)p + 7;
    *code = tge ? (uint8_t)(*code | 1) : (uint8_t)(*code & ~1);
}

/* ---- texture page and CLUT ----------------------------------------------- */

/* tp: colour mode, abr: blending, (x, y): page origin in VRAM. */
uint32_t pcrt_GetTPage(int tp, int abr, int x, int y)
{
    return (uint16_t)(((tp & 3) << 7) | ((abr & 3) << 5) | ((y & 0x100) >> 4) | ((x & 0x3FF) >> 6) |
                      ((y & 0x200) << 2));
}

uint32_t pcrt_GetClut(int x, int y)
{
    return (uint16_t)((y << 6) | ((x >> 4) & 0x3F));
}

/* ---- drawing-mode packets ------------------------------------------------ */

void pcrt_SetDrawTPage(uint32_t* p, int dfe, int dtd, int tpage)
{
    set_len(p, 1);
    p[1] = 0xE1000000u | (dtd ? 0x200u : 0) | (dfe ? 0x400u : 0) | ((uint32_t)tpage & 0x9FFu);
}

/* Two words: the command and a NOP. */
void pcrt_SetDrawStp(uint32_t* p, int pbw)
{
    set_len(p, 2);
    p[1] = 0xE6000000u | (pbw ? 1u : 0);
    p[2] = 0;
}

void pcrt_SetDrawOffset(uint32_t* p, uint16_t* ofs)
{
    set_len(p, 2);
    p[1] = 0xE5000000u | (ofs[0] & 0x7FFu) | ((ofs[1] & 0x7FFu) << 11);
    p[2] = 0;
}

/* The drawing area's corners (E3, E4), kept inside the VRAM whose size
 * ResetGraph left in GEnv (+4 width, +6 height; runtime/gpu init_genv),
 * ten bits each. */
extern const uint32_t pc_sym_GEnv;

static int clamp_to(int v, int size)
{
    return v < 0 ? 0 : v >= size ? size - 1 : v;
}

void pcrt_SetDrawArea(uint32_t* p, Rect* r)
{
    const uint8_t* genv = (const uint8_t*)(uintptr_t)pc_sym_GEnv;
    int w = *(const int16_t*)(genv + 4), h = *(const int16_t*)(genv + 6);
    int x0 = clamp_to(r->x, w), y0 = clamp_to(r->y, h);
    /* The far corner is worked out in halfwords: it wraps past 32767. */
    int x1 = clamp_to((int16_t)(r->x + r->w - 1), w), y1 = clamp_to((int16_t)(r->y + r->h - 1), h);
    set_len(p, 2);
    p[1] = 0xE3000000u | (((uint32_t)y0 & 0x3FF) << 10) | ((uint32_t)x0 & 0x3FF);
    p[2] = 0xE4000000u | (((uint32_t)y1 & 0x3FF) << 10) | ((uint32_t)x1 & 0x3FF);
}

/* A texture-cache flush, then the VRAM-to-VRAM copy. */
void pcrt_SetDrawMove(uint32_t* p, Rect* rect, int x, int y)
{
    set_len(p, rect->w && rect->h ? 5 : 0);
    p[1] = 0x01000000u;
    p[2] = 0x80000000u;
    p[3] = *(uint32_t*)&rect->x;
    p[4] = ((uint32_t)y << 16) | ((uint32_t)x & 0xFFFFu);
    p[5] = *(uint32_t*)&rect->w;
}

/* ---- default environments ------------------------------------------------ */

DrawEnv* pcrt_SetDefDrawEnv(DrawEnv* env, int x, int y, int w, int h)
{
    env->clip.x = (int16_t)x;
    env->clip.y = (int16_t)y;
    env->clip.w = (int16_t)w;
    env->clip.h = (int16_t)h;
    env->ofs[0] = (int16_t)x;
    env->ofs[1] = (int16_t)y;
    env->tw.x = env->tw.y = env->tw.w = env->tw.h = 0;
    env->tpage = 10;
    env->dtd = 1;
    env->dfe = h <= 256 ? 1 : 0; /* draw to the displayed area unless interlaced */
    env->isbg = 0;
    env->r0 = env->g0 = env->b0 = 0;
    return env;
}

DispEnv* pcrt_SetDefDispEnv(DispEnv* env, int x, int y, int w, int h)
{
    env->disp.x = (int16_t)x;
    env->disp.y = (int16_t)y;
    env->disp.w = (int16_t)w;
    env->disp.h = (int16_t)h;
    env->screen.x = env->screen.y = env->screen.w = env->screen.h = 0;
    env->isinter = env->isrgb24 = env->pad0 = env->pad1 = 0;
    return env;
}

/* ---- libgs ordering tables ----------------------------------------------- */

typedef struct {
    uint32_t length; /* the table has 1 << length entries */
    uint32_t* org;
    uint32_t offset;
    uint32_t point;
    uint32_t* tag;
} GsOt;

/* Empties the table: each entry links to the one before it, the first ends
 * the list, and drawing starts from the last. */
void pcrt_GsClearOt(uint16_t offset, uint16_t point, GsOt* ot)
{
    uint32_t n = 1u << ot->length;
    ot->offset = offset;
    ot->point = point;
    for (uint32_t i = n - 1; i > 0; i--) {
        ot->org[i] = (uint32_t)(uintptr_t)&ot->org[i - 1] & 0x00FFFFFFu;
    }
    ot->org[0] = 0x00FFFFFFu;
    ot->tag = &ot->org[n - 1];
}

/* ---- drawing control ----------------------------------------------------- */

/* The host's GPU (runtime/gpu): drawing completes within DrawOTag. */
void pcrt_DrawOTag(uint32_t* p);
void pcrt_PutDrawEnv(DrawEnv* env);

/* Suspends the drawing in progress and returns where to resume it. The
 * host's GPU finishes each list before DrawOTag returns, so there is never
 * one: 0, as the original returns with the GPU's DMA idle. */
uint32_t* pcrt_BreakDraw(void)
{
    return 0;
}

/* Draws the list at insaddr (if any), then resumes contaddr: a list
 * BreakDraw interrupted (0 when it found none; -1 is its "could not
 * suspend" value, never a list). */
void pcrt_ContinueDraw(uint32_t* insaddr, uint32_t* contaddr)
{
    if (insaddr) {
        pcrt_DrawOTag(insaddr);
    }
    if (contaddr && contaddr != (uint32_t*)-1) {
        pcrt_DrawOTag(contaddr);
    }
}

/* Draws the list p in the drawing environment env. */
void pcrt_DrawOTagEnv(uint32_t* p, DrawEnv* env)
{
    pcrt_PutDrawEnv(env);
    pcrt_DrawOTag(p);
}
