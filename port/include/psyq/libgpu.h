/* The graphics library (libgpu) the game uses: GPU packet ("primitive")
 * layouts and the macros that fill them, drawing and display environments,
 * and the library calls (port/src/libgpu.c and runtime/gpu implement them).
 * Packet formats follow the GPU's documented command words. */
#ifndef PSX_LIBGPU_H
#define PSX_LIBGPU_H

#include "sys/types.h"

typedef struct {
    short x, y;
    short w, h;
} RECT;

/* Packet header: the next packet's address, then the packet's length in
 * words; the first command word ends in the GPU command code. */
typedef struct {
    unsigned addr : 24;
    unsigned len : 8;
    u_char r0, g0, b0, code;
} P_TAG;

#define setaddr(p, _addr) (((P_TAG*)(p))->addr = (u_long)(_addr))
#define getaddr(p)        (u_long)(((P_TAG*)(p))->addr)
#define setlen(p, _len)   (((P_TAG*)(p))->len = (u_char)(_len))
#define setcode(p, _code) (((P_TAG*)(p))->code = (u_char)(_code))
#define getcode(p)        (u_char)(((P_TAG*)(p))->code)

/* Linking packets into an ordering table entry. */
#define addPrim(ot, p)       setaddr(p, getaddr(ot)), setaddr(ot, p)
#define addPrims(ot, p0, p1) setaddr(p1, getaddr(ot)), setaddr(ot, p0)

/* Texture page and CLUT words. */
#define getTPage(tp, abr, x, y)                                                                       \
    ((((tp) & 0x3) << 7) | (((abr) & 0x3) << 5) | (((y) & 0x100) >> 4) | (((x) & 0x3ff) >> 6) |       \
     (((y) & 0x200) << 2))
#define getClut(x, y) (((y) << 6) | (((x) >> 4) & 0x3f))

/* Field setters. */
#define setVector(v, _x, _y, _z)   (v)->vx = (_x), (v)->vy = (_y), (v)->vz = (_z)
#define copyVector(v0, v1)         (v0)->vx = (v1)->vx, (v0)->vy = (v1)->vy, (v0)->vz = (v1)->vz
#define setRECT(r, _x, _y, _w, _h) (r)->x = (_x), (r)->y = (_y), (r)->w = (_w), (r)->h = (_h)
#define setClut(p, x, y)           ((p)->clut = getClut(x, y))
#define setRGB0(p, _r, _g, _b)     (p)->r0 = (_r), (p)->g0 = (_g), (p)->b0 = (_b)
#define setRGB1(p, _r, _g, _b)     (p)->r1 = (_r), (p)->g1 = (_g), (p)->b1 = (_b)
#define setRGB2(p, _r, _g, _b)     (p)->r2 = (_r), (p)->g2 = (_g), (p)->b2 = (_b)
#define setRGB3(p, _r, _g, _b)     (p)->r3 = (_r), (p)->g3 = (_g), (p)->b3 = (_b)
#define setXY0(p, _x0, _y0)        (p)->x0 = (_x0), (p)->y0 = (_y0)
#define setXY4(p, _x0, _y0, _x1, _y1, _x2, _y2, _x3, _y3)                                             \
    (p)->x0 = (_x0), (p)->y0 = (_y0), (p)->x1 = (_x1), (p)->y1 = (_y1), (p)->x2 = (_x2),               \
    (p)->y2 = (_y2), (p)->x3 = (_x3), (p)->y3 = (_y3)
#define setXYWH(p, _x0, _y0, _w, _h)                                                                  \
    (p)->x0 = (_x0), (p)->y0 = (_y0), (p)->x1 = (_x0) + (_w), (p)->y1 = (_y0), (p)->x2 = (_x0),        \
    (p)->y2 = (_y0) + (_h), (p)->x3 = (_x0) + (_w), (p)->y3 = (_y0) + (_h)
#define setWH(p, _w, _h)    (p)->w = _w, (p)->h = _h
#define setUV0(p, _u0, _v0) (p)->u0 = (_u0), (p)->v0 = (_v0)
#define setUV4(p, _u0, _v0, _u1, _v1, _u2, _v2, _u3, _v3)                                             \
    (p)->u0 = (_u0), (p)->v0 = (_v0), (p)->u1 = (_u1), (p)->v1 = (_v1), (p)->u2 = (_u2),               \
    (p)->v2 = (_v2), (p)->u3 = (_u3), (p)->v3 = (_v3)
#define setUVWH(p, _u0, _v0, _w, _h)                                                                  \
    (p)->u0 = (_u0), (p)->v0 = (_v0), (p)->u1 = (_u0) + (_w), (p)->v1 = (_v0), (p)->u2 = (_u0),        \
    (p)->v2 = (_v0) + (_h), (p)->u3 = (_u0) + (_w), (p)->v3 = (_v0) + (_h)

/* Command code bits: semi-transparency and raw (unshaded) texture. */
#define setSemiTrans(p, abe) ((abe) ? setcode(p, getcode(p) | 0x02) : setcode(p, getcode(p) & ~0x02))
#define setShadeTex(p, tge)  ((tge) ? setcode(p, getcode(p) | 0x01) : setcode(p, getcode(p) & ~0x01))

/* Primitive set-up: packet length and command code. */
#define setPolyF3(p)  setlen(p, 4), setcode(p, 0x20)
#define setPolyF4(p)  setlen(p, 5), setcode(p, 0x28)
#define setPolyFT4(p) setlen(p, 9), setcode(p, 0x2c)
#define setPolyG3(p)  setlen(p, 6), setcode(p, 0x30)
#define setPolyG4(p)  setlen(p, 8), setcode(p, 0x38)
#define setPolyGT4(p) setlen(p, 12), setcode(p, 0x3c)
#define setLineF2(p)  setlen(p, 3), setcode(p, 0x40)
#define setLineF3(p)  setlen(p, 5), setcode(p, 0x48), (p)->pad = 0x55555555
#define setLineG2(p)  setlen(p, 4), setcode(p, 0x50)
#define setLineG3(p)  setlen(p, 7), setcode(p, 0x58), (p)->pad = 0x55555555, (p)->p2 = 0
#define setTile1(p)   setlen(p, 2), setcode(p, 0x68)
#define setTile(p)    setlen(p, 3), setcode(p, 0x60)
#define setSprt(p)    setlen(p, 4), setcode(p, 0x64)

/* Drawing-mode packets: texture page (GP0 E1) and texture window (E2). */
#define _get_mode(dfe, dtd, tpage) ((0xe1000000) | ((dtd) ? 0x0200 : 0) | ((dfe) ? 0x0400 : 0) | ((tpage) & 0x9ff))
#define setDrawTPage(p, dfe, dtd, tpage) setlen(p, 1), ((u_long*)(p))[1] = _get_mode(dfe, dtd, tpage)
#define _get_tw(tw)                                                                                   \
    (tw ? ((0xe2000000) | ((((tw)->y & 0xff) >> 3) << 15) | ((((tw)->x & 0xff) >> 3) << 10) |          \
           (((~((tw)->h - 1) & 0xff) >> 3) << 5) | (((~((tw)->w - 1) & 0xff) >> 3)))                  \
        : 0)
#define setTexWindow(p, tw) setlen(p, 2), ((u_long*)(p))[1] = _get_tw(tw), ((u_long*)(p))[2] = 0

/* Polygons. */
typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    short x1, y1;
    short x2, y2;
} POLY_F3;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    short x1, y1;
    short x2, y2;
    short x3, y3;
} POLY_F4;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
    short x1, y1;
    u_char u1, v1;
    u_short tpage;
    short x2, y2;
    u_char u2, v2;
    u_short pad1;
} POLY_FT3;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
    short x1, y1;
    u_char u1, v1;
    u_short tpage;
    short x2, y2;
    u_char u2, v2;
    u_short pad1;
    short x3, y3;
    u_char u3, v3;
    u_short pad2;
} POLY_FT4;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char r1, g1, b1, pad1;
    short x1, y1;
    u_char r2, g2, b2, pad2;
    short x2, y2;
} POLY_G3;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char r1, g1, b1, pad1;
    short x1, y1;
    u_char r2, g2, b2, pad2;
    short x2, y2;
    u_char r3, g3, b3, pad3;
    short x3, y3;
} POLY_G4;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
    u_char r1, g1, b1, p1;
    short x1, y1;
    u_char u1, v1;
    u_short tpage;
    u_char r2, g2, b2, p2;
    short x2, y2;
    u_char u2, v2;
    u_short pad2;
} POLY_GT3;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
    u_char r1, g1, b1, p1;
    short x1, y1;
    u_char u1, v1;
    u_short tpage;
    u_char r2, g2, b2, p2;
    short x2, y2;
    u_char u2, v2;
    u_short pad2;
    u_char r3, g3, b3, p3;
    short x3, y3;
    u_char u3, v3;
    u_short pad3;
} POLY_GT4;

/* Lines. */
typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    short x1, y1;
} LINE_F2;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char r1, g1, b1, p1;
    short x1, y1;
} LINE_G2;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    short x1, y1;
    short x2, y2;
    u_long pad;
} LINE_F3;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char r1, g1, b1, p1;
    short x1, y1;
    u_char r2, g2, b2, p2;
    short x2, y2;
    u_long pad;
} LINE_G3;

/* Sprites and rectangles. */
typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
    short w, h;
} SPRT;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
} SPRT_16;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    u_char u0, v0;
    u_short clut;
} SPRT_8;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
    short w, h;
} TILE;

typedef struct {
    u_long tag;
    u_char r0, g0, b0, code;
    short x0, y0;
} TILE_1;

/* Drawing-mode packets. */
typedef struct {
    u_long tag;
    u_long code[2];
} DR_MODE;

typedef struct {
    u_long tag;
    u_long code[2];
} DR_AREA;

typedef struct {
    u_long tag;
    u_long code[2];
} DR_OFFSET;

typedef struct {
    u_long tag;
    u_long code[5];
} DR_MOVE;

typedef struct {
    u_long tag;
    u_long code[1];
} DR_TPAGE;

typedef struct {
    u_long tag;
    u_long code[2];
} DR_STP;

typedef struct {
    u_long tag;
    u_long code[15];
} DR_ENV;

/* Drawing environment: clip area, offset, texture window and page,
 * dithering, drawing to the displayed area, background clear. */
typedef struct {
    RECT clip;
    short ofs[2];
    RECT tw;
    u_short tpage;
    u_char dtd;
    u_char dfe;
    u_char isbg;
    u_char r0, g0, b0;
    DR_ENV dr_env;
} DRAWENV;

/* Display environment: the displayed VRAM area and the screen window. */
typedef struct {
    RECT disp;
    RECT screen;
    u_char isinter;
    u_char isrgb24;
    u_char pad0, pad1;
} DISPENV;

/* The library's debug print, a function pointer (printf by default). */
extern int (*GPU_printf)(char* fmt, ...);

extern int ResetGraph(int mode);
extern void SetDispMask(int mask);
extern int DrawSync(int mode);
extern int IsIdleGPU(int max_count);
extern int ClearImage(RECT* rect, u_char r, u_char g, u_char b);
extern int LoadImage(RECT* rect, u_long* p);
extern int LoadImage2(RECT* rect, u_long* p);
extern int StoreImage(RECT* rect, u_long* p);
extern int StoreImage2(RECT* rect, u_long* p);
extern int MoveImage(RECT* rect, int x, int y);
extern int MoveImage2(RECT* rect, int x, int y);
extern u_long* ClearOTagR(u_long* ot, int n);
extern void DrawOTag(u_long* p);
extern void DrawPrim(void* p);
extern u_long* BreakDraw(void);
extern void ContinueDraw(u_long* insaddr, u_long* contaddr);
extern DISPENV* PutDispEnv(DISPENV* env);
extern DISPENV* SetDefDispEnv(DISPENV* env, int x, int y, int w, int h);
extern DRAWENV* PutDrawEnv(DRAWENV* env);
extern DRAWENV* SetDefDrawEnv(DRAWENV* env, int x, int y, int w, int h);
extern void AddPrim(void* ot, void* p);
extern int MargePrim(void* p0, void* p1);
extern u_short GetClut(int x, int y);
extern u_short GetTPage(int tp, int abr, int x, int y);
extern void SetPolyFT4(POLY_FT4* p);
extern void SetSprt(SPRT* p);
extern void SetTile(TILE* p);
extern void SetSemiTrans(void* p, int abe);
extern void SetShadeTex(void* p, int tge);
extern void SetDrawArea(DR_AREA* p, RECT* r);
extern void SetDrawMove(DR_MOVE* p, RECT* rect, int x, int y);
extern void SetDrawOffset(DR_OFFSET* p, u_short* ofs);
extern void SetDrawStp(DR_STP* p, int pbw);
extern void SetDrawTPage(DR_TPAGE* p, int dfe, int dtd, int tpage);

#endif
