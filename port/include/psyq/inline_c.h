/*
 * GTE register loads and stores the game uses, as C on top of the software
 * GTE (port/include/pc/gte.h). On the console these are inline MIPS
 * (lwc2/swc2, mtc2/mfc2, ctc2/cfc2); each macro here moves the same values
 * between memory and the same GTE registers, in the same order. The GTE
 * commands themselves (rtps, ncs, ...) come from the decomp's gte.h.
 *
 * Data registers: 0-5 V0-V2 (xy, z), 6 RGBC, 7 OTZ, 8 IR0, 9-11 IR1-3,
 * 12-15 SXY0-2/SXYP, 16-19 SZ0-3, 20-22 RGB0-2, 24 MAC0, 25-27 MAC1-3,
 * 30 LZCS, 31 LZCR. Control registers: 0-4 RT, 5-7 TR, 8-12 LLM, 13-15 BK,
 * 16-20 LCM, 21-23 FC, 24-25 OFX/OFY, 26 H, 31 FLAG.
 */
#ifndef PSX_INLINE_C_H
#define PSX_INLINE_C_H

#include "pc/gte_macros.h"

/* ---- loads -------------------------------------------------------------- */

/* V0 from an SVECTOR. */
#define gte_ldv0(r0)                                                                                  \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        pcgte_write_data(0, PCMEM_LW(_a0));                                                           \
        pcgte_write_data(1, PCMEM_LW((_a0 + 4)));                                                     \
    } while (0)

/* V0, V1 and V2 from three SVECTORs. */
#define gte_ldv3(r0, r1, r2)                                                                          \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uintptr_t _a1 = (uintptr_t)(r1);                                                              \
        uintptr_t _a2 = (uintptr_t)(r2);                                                              \
        pcgte_write_data(0, PCMEM_LW(_a0));                                                           \
        pcgte_write_data(1, PCMEM_LW((_a0 + 4)));                                                     \
        pcgte_write_data(2, PCMEM_LW(_a1));                                                           \
        pcgte_write_data(3, PCMEM_LW((_a1 + 4)));                                                     \
        pcgte_write_data(4, PCMEM_LW(_a2));                                                           \
        pcgte_write_data(5, PCMEM_LW((_a2 + 4)));                                                     \
    } while (0)

/* V0 from a VECTOR (x and y packed into one register). */
#define gte_ldlv0(r0)                                                                                 \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t1 = PCMEM_LHU((_a0 + 4));                                                          \
        uint32_t _t0 = PCMEM_LHU(_a0);                                                                \
        _t1 = _t1 << 16;                                                                              \
        _t0 = _t0 | _t1;                                                                              \
        pcgte_write_data(0, _t0);                                                                     \
        pcgte_write_data(1, PCMEM_LW((_a0 + 8)));                                                     \
    } while (0)

/* IR1-3 from a matrix column (elements 6 bytes apart). */
#define gte_ldclmv(r0)                                                                                \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LHU(_a0);                                                                \
        uint32_t _t1 = PCMEM_LHU((_a0 + 6));                                                          \
        uint32_t _t2 = PCMEM_LHU((_a0 + 12));                                                         \
        pcgte_write_data(9, _t0);                                                                     \
        pcgte_write_data(10, _t1);                                                                    \
        pcgte_write_data(11, _t2);                                                                    \
    } while (0)

/* IR1-3 from a VECTOR. */
#define gte_ldlvl(r0)                                                                                 \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        pcgte_write_data(9, PCMEM_LW(_a0));                                                           \
        pcgte_write_data(10, PCMEM_LW((_a0 + 4)));                                                    \
        pcgte_write_data(11, PCMEM_LW((_a0 + 8)));                                                    \
    } while (0)

/* IR1-3 from an SVECTOR. */
#define gte_ldsv(r0)                                                                                  \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LHU(_a0);                                                                \
        uint32_t _t1 = PCMEM_LHU((_a0 + 2));                                                          \
        uint32_t _t2 = PCMEM_LHU((_a0 + 4));                                                          \
        pcgte_write_data(9, _t0);                                                                     \
        pcgte_write_data(10, _t1);                                                                    \
        pcgte_write_data(11, _t2);                                                                    \
    } while (0)

/* RGBC from a CVECTOR. */
#define gte_ldrgb(r0)                                                                                 \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        pcgte_write_data(6, PCMEM_LW(_a0));                                                           \
    } while (0)

/* IR1-3 from a CVECTOR's red, green and blue. */
#define gte_ldcv(r0)                                                                                  \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LBU(_a0);                                                                \
        uint32_t _t1 = PCMEM_LBU((_a0 + 1));                                                          \
        uint32_t _t2 = PCMEM_LBU((_a0 + 2));                                                          \
        pcgte_write_data(9, _t0);                                                                     \
        pcgte_write_data(10, _t1);                                                                    \
        pcgte_write_data(11, _t2);                                                                    \
    } while (0)

/* IR0 (the interpolation factor) from a value. */
#define gte_lddp(r0) do { pcgte_write_data(8, (uint32_t)(r0)); } while (0)

/* LZCS from a value. */
#define gte_ldlzc(r0) do { pcgte_write_data(30, (uint32_t)(r0)); } while (0)

/* Outer-product operands: the rotation diagonal (RT11, RT22, RT33) ... */
#define gte_ldopv1(r0)                                                                                \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LW(_a0);                                                                 \
        uint32_t _t1 = PCMEM_LW((_a0 + 4));                                                           \
        pcgte_write_ctrl(0, _t0);                                                                     \
        uint32_t _t2 = PCMEM_LW((_a0 + 8));                                                           \
        pcgte_write_ctrl(2, _t1);                                                                     \
        pcgte_write_ctrl(4, _t2);                                                                     \
    } while (0)
#define gte_ldopv1SV(r0)                                                                              \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LH(_a0);                                                                 \
        uint32_t _t1 = PCMEM_LH((_a0 + 2));                                                           \
        pcgte_write_ctrl(0, _t0);                                                                     \
        uint32_t _t2 = PCMEM_LH((_a0 + 4));                                                           \
        pcgte_write_ctrl(2, _t1);                                                                     \
        pcgte_write_ctrl(4, _t2);                                                                     \
    } while (0)

/* ... and the other vector, in IR1-3. */
#define gte_ldopv2(r0)                                                                                \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        pcgte_write_data(11, PCMEM_LW((_a0 + 8)));                                                    \
        pcgte_write_data(9, PCMEM_LW(_a0));                                                           \
        pcgte_write_data(10, PCMEM_LW((_a0 + 4)));                                                    \
    } while (0)
#define gte_ldopv2SV(r0)                                                                              \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LH(_a0);                                                                 \
        uint32_t _t1 = PCMEM_LH((_a0 + 2));                                                           \
        uint32_t _t2 = PCMEM_LH((_a0 + 4));                                                           \
        pcgte_write_data(9, _t0);                                                                     \
        pcgte_write_data(10, _t1);                                                                    \
        pcgte_write_data(11, _t2);                                                                    \
    } while (0)

/* The rotation's first row from an SVECTOR (whole words: RT13 comes with
 * the SVECTOR's padding in RT21). */
#define gte_ldsvrtrow0(r0)                                                                            \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LW(_a0);                                                                 \
        uint32_t _t1 = PCMEM_LW((_a0 + 4));                                                           \
        pcgte_write_ctrl(0, _t0);                                                                     \
        pcgte_write_ctrl(1, _t1);                                                                     \
    } while (0)

/* The background colour (BK) from three values, all evaluated first. */
#define gte_ldbkdir(r0, r1, r2)                                                                       \
    do {                                                                                              \
        uint32_t _v0 = (uint32_t)(r0);                                                                \
        uint32_t _v1 = (uint32_t)(r1);                                                                \
        uint32_t _v2 = (uint32_t)(r2);                                                                \
        pcgte_write_ctrl(13, _v0);                                                                    \
        pcgte_write_ctrl(14, _v1);                                                                    \
        pcgte_write_ctrl(15, _v2);                                                                    \
    } while (0)

/* ---- matrices and vectors ----------------------------------------------- */

/* A MATRIX's 3x3 part into the rotation (RT) or light colour (LCM)
 * registers, or its translation into TR. */
#define PSX_GTE_SET_MATRIX(r0, first)                                                                 \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LW(_a0);                                                                 \
        uint32_t _t1 = PCMEM_LW((_a0 + 4));                                                           \
        pcgte_write_ctrl((first), _t0);                                                               \
        pcgte_write_ctrl((first) + 1, _t1);                                                           \
        _t0 = PCMEM_LW((_a0 + 8));                                                                    \
        _t1 = PCMEM_LW((_a0 + 12));                                                                   \
        uint32_t _t2 = PCMEM_LW((_a0 + 16));                                                          \
        pcgte_write_ctrl((first) + 2, _t0);                                                           \
        pcgte_write_ctrl((first) + 3, _t1);                                                           \
        pcgte_write_ctrl((first) + 4, _t2);                                                           \
    } while (0)
#define gte_SetRotMatrix(r0)   PSX_GTE_SET_MATRIX(r0, 0)
#define gte_SetColorMatrix(r0) PSX_GTE_SET_MATRIX(r0, 16)
#define gte_SetTransMatrix(r0)                                                                        \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LW((_a0 + 20));                                                          \
        uint32_t _t1 = PCMEM_LW((_a0 + 24));                                                          \
        pcgte_write_ctrl(5, _t0);                                                                     \
        uint32_t _t2 = PCMEM_LW((_a0 + 28));                                                          \
        pcgte_write_ctrl(6, _t1);                                                                     \
        pcgte_write_ctrl(7, _t2);                                                                     \
    } while (0)

/* TR from a VECTOR. */
#define gte_SetTransVector(r0)                                                                        \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = PCMEM_LW(_a0);                                                                 \
        uint32_t _t1 = PCMEM_LW((_a0 + 4));                                                           \
        uint32_t _t2 = PCMEM_LW((_a0 + 8));                                                           \
        pcgte_write_ctrl(5, _t0);                                                                     \
        pcgte_write_ctrl(6, _t1);                                                                     \
        pcgte_write_ctrl(7, _t2);                                                                     \
    } while (0)

/* The screen offset (OFX, OFY in 16.16) and the projection distance H. */
#define gte_SetGeomOffset(r0, r1)                                                                     \
    do {                                                                                              \
        uint32_t _t0 = (uint32_t)(r0);                                                                \
        uint32_t _t1 = (uint32_t)(r1);                                                                \
        _t0 = _t0 << 16;                                                                              \
        _t1 = _t1 << 16;                                                                              \
        pcgte_write_ctrl(24, _t0);                                                                    \
        pcgte_write_ctrl(25, _t1);                                                                    \
    } while (0)
#define gte_SetGeomScreen(r0) do { pcgte_write_ctrl(26, (uint32_t)(r0)); } while (0)

/* The rotation and translation back into a MATRIX. */
#define gte_ReadRotMatrix(r0)                                                                         \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_ctrl(0);                                                            \
        uint32_t _t1 = pcgte_read_ctrl(1);                                                            \
        PCMEM_SW(_a0, _t0);                                                                           \
        PCMEM_SW((_a0 + 4), _t1);                                                                     \
        _t0 = pcgte_read_ctrl(2);                                                                     \
        _t1 = pcgte_read_ctrl(3);                                                                     \
        uint32_t _t2 = pcgte_read_ctrl(4);                                                            \
        PCMEM_SW((_a0 + 8), _t0);                                                                     \
        PCMEM_SW((_a0 + 12), _t1);                                                                    \
        PCMEM_SW((_a0 + 16), _t2);                                                                    \
        _t0 = pcgte_read_ctrl(5);                                                                     \
        _t1 = pcgte_read_ctrl(6);                                                                     \
        _t2 = pcgte_read_ctrl(7);                                                                     \
        PCMEM_SW((_a0 + 20), _t0);                                                                    \
        PCMEM_SW((_a0 + 24), _t1);                                                                    \
        PCMEM_SW((_a0 + 28), _t2);                                                                    \
    } while (0)

/* ---- stores ------------------------------------------------------------- */

#define PSX_GTE_ST1(r0, reg, off)                                                                     \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        PCMEM_SW((_a0 + (off)), pcgte_read_data(reg));                                                \
    } while (0)

/* IR1-3 into a matrix column (elements 6 bytes apart) or an SVECTOR. */
#define gte_stclmv(r0)                                                                                \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_data(9);                                                            \
        uint32_t _t1 = pcgte_read_data(10);                                                           \
        uint32_t _t2 = pcgte_read_data(11);                                                           \
        PCMEM_SH(_a0, _t0);                                                                           \
        PCMEM_SH((_a0 + 6), _t1);                                                                     \
        PCMEM_SH((_a0 + 12), _t2);                                                                    \
    } while (0)
#define gte_stsv(r0)                                                                                  \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_data(9);                                                            \
        uint32_t _t1 = pcgte_read_data(10);                                                           \
        uint32_t _t2 = pcgte_read_data(11);                                                           \
        PCMEM_SH(_a0, _t0);                                                                           \
        PCMEM_SH((_a0 + 2), _t1);                                                                     \
        PCMEM_SH((_a0 + 4), _t2);                                                                     \
    } while (0)

/* IR1-3 into a CVECTOR's red, green and blue. */
#define gte_stcv(r0)                                                                                  \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_data(9);                                                            \
        uint32_t _t1 = pcgte_read_data(10);                                                           \
        uint32_t _t2 = pcgte_read_data(11);                                                           \
        PCMEM_SB(_a0, _t0);                                                                           \
        PCMEM_SB((_a0 + 1), _t1);                                                                     \
        PCMEM_SB((_a0 + 2), _t2);                                                                     \
    } while (0)

/* IR1-3 or MAC1-3 into a VECTOR. */
#define gte_stlvl(r0)                                                                                 \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        PCMEM_SW(_a0, pcgte_read_data(9));                                                            \
        PCMEM_SW((_a0 + 4), pcgte_read_data(10));                                                     \
        PCMEM_SW((_a0 + 8), pcgte_read_data(11));                                                     \
    } while (0)
#define gte_stlvnl(r0)                                                                                \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        PCMEM_SW(_a0, pcgte_read_data(25));                                                           \
        PCMEM_SW((_a0 + 4), pcgte_read_data(26));                                                     \
        PCMEM_SW((_a0 + 8), pcgte_read_data(27));                                                     \
    } while (0)
#define gte_stlvnl0(r0) PSX_GTE_ST1(r0, 25, 0)

/* Single registers. */
#define gte_strgb(r0)  PSX_GTE_ST1(r0, 22, 0)
#define gte_stdp(r0)   PSX_GTE_ST1(r0, 8, 0)
#define gte_stotz(r0)  PSX_GTE_ST1(r0, 7, 0)
#define gte_stopz(r0)  PSX_GTE_ST1(r0, 24, 0)
#define gte_stsz(r0)   PSX_GTE_ST1(r0, 19, 0)
#define gte_stsxy(r0)  PSX_GTE_ST1(r0, 14, 0)
#define gte_stsxy2(r0) PSX_GTE_ST1(r0, 14, 0)
#define gte_stlzc(r0)  PSX_GTE_ST1(r0, 31, 0)

/* FLAG. */
#define gte_stflg(r0)                                                                                 \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_ctrl(31);                                                           \
        PCMEM_SW(_a0, _t0);                                                                           \
    } while (0)

/* SZ3 / 4 (an ordering-table depth). */
#define gte_stszotz(r0)                                                                               \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_data(19);                                                           \
        _t0 = (uint32_t)((int32_t)_t0 >> 2);                                                          \
        PCMEM_SW(_a0, _t0);                                                                           \
    } while (0)

/* TR into a VECTOR. */
#define gte_sttr(r0)                                                                                  \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uint32_t _t0 = pcgte_read_ctrl(5);                                                            \
        uint32_t _t1 = pcgte_read_ctrl(6);                                                            \
        uint32_t _t2 = pcgte_read_ctrl(7);                                                            \
        PCMEM_SW(_a0, _t0);                                                                           \
        PCMEM_SW((_a0 + 4), _t1);                                                                     \
        PCMEM_SW((_a0 + 8), _t2);                                                                     \
    } while (0)

/* SXY0-2 into three places, or into a primitive's vertices (x0, x1, x2
 * at the given offsets). */
#define gte_stsxy3(r0, r1, r2)                                                                        \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        uintptr_t _a1 = (uintptr_t)(r1);                                                              \
        uintptr_t _a2 = (uintptr_t)(r2);                                                              \
        PCMEM_SW(_a0, pcgte_read_data(12));                                                           \
        PCMEM_SW(_a1, pcgte_read_data(13));                                                           \
        PCMEM_SW(_a2, pcgte_read_data(14));                                                           \
    } while (0)
#define PSX_GTE_STSXY3_AT(r0, o0, o1, o2)                                                             \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        PCMEM_SW((_a0 + (o0)), pcgte_read_data(12));                                                  \
        PCMEM_SW((_a0 + (o1)), pcgte_read_data(13));                                                  \
        PCMEM_SW((_a0 + (o2)), pcgte_read_data(14));                                                  \
    } while (0)
#define gte_stsxy3_f3(r0)  PSX_GTE_STSXY3_AT(r0, 8, 12, 16)
#define gte_stsxy3_f4(r0)  PSX_GTE_STSXY3_AT(r0, 8, 12, 16)
#define gte_stsxy3_g3(r0)  PSX_GTE_STSXY3_AT(r0, 8, 16, 24)
#define gte_stsxy3_g4(r0)  PSX_GTE_STSXY3_AT(r0, 8, 16, 24)
#define gte_stsxy3_ft3(r0) PSX_GTE_STSXY3_AT(r0, 8, 16, 24)
#define gte_stsxy3_ft4(r0) PSX_GTE_STSXY3_AT(r0, 8, 16, 24)
#define gte_stsxy3_gt3(r0) PSX_GTE_STSXY3_AT(r0, 8, 20, 32)
#define gte_stsxy3_gt4(r0) PSX_GTE_STSXY3_AT(r0, 8, 20, 32)

/* RGB0-2 into a gouraud-textured primitive's three colours. */
#define PSX_GTE_STRGB3_AT(r0, o0, o1, o2)                                                             \
    do {                                                                                              \
        uintptr_t _a0 = (uintptr_t)(r0);                                                              \
        PCMEM_SW((_a0 + (o0)), pcgte_read_data(20));                                                  \
        PCMEM_SW((_a0 + (o1)), pcgte_read_data(21));                                                  \
        PCMEM_SW((_a0 + (o2)), pcgte_read_data(22));                                                  \
    } while (0)
#define gte_strgb3_gt3(r0) PSX_GTE_STRGB3_AT(r0, 4, 16, 28)
#define gte_strgb3_gt4(r0) PSX_GTE_STRGB3_AT(r0, 4, 16, 28)

/* No GTE operation (a delay slot on the console). */
#define gte_nop()

#endif
