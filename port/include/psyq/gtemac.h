/*
 * Compound GTE operations the game uses, built from the register moves in
 * inline_c.h and the GTE commands of the decomp's gte.h.
 */
#ifndef PSX_GTEMAC_H
#define PSX_GTEMAC_H

/* Rotate, translate and project v (r1): screen position (r2), depth cue
 * (r3), flags (r4) and ordering-table depth (r5). */
#define gte_RotTransPers(r1, r2, r3, r4, r5)                                                          \
    {                                                                                                 \
        gte_ldv0(r1);                                                                                 \
        gte_rtps();                                                                                   \
        gte_stsxy(r2);                                                                                \
        gte_stdp(r3);                                                                                 \
        gte_stflg(r4);                                                                                \
        gte_stszotz(r5);                                                                              \
    }

/* r2 = r1's rotation applied to SVECTOR r2's source (r2 -> r3). */
#define gte_ApplyMatrixSV(r1, r2, r3)                                                                 \
    {                                                                                                 \
        gte_SetRotMatrix(r1);                                                                         \
        gte_ldv0(r2);                                                                                 \
        gte_rtv0();                                                                                   \
        gte_stsv(r3);                                                                                 \
    }

/* r3 = r1 * r2, a column at a time. */
#define gte_MulMatrix0(r1, r2, r3)                                                                    \
    {                                                                                                 \
        gte_SetRotMatrix(r1);                                                                         \
        gte_ldclmv(r2);                                                                               \
        gte_rtir();                                                                                   \
        gte_stclmv(r3);                                                                               \
        gte_ldclmv((char*)r2 + 2);                                                                    \
        gte_rtir();                                                                                   \
        gte_stclmv((char*)r3 + 2);                                                                    \
        gte_ldclmv((char*)r2 + 4);                                                                    \
        gte_rtir();                                                                                   \
        gte_stclmv((char*)r3 + 4);                                                                    \
    }

/* r3 = r1 * r2 with translations: rotation as gte_MulMatrix0, then r2's
 * translation through r1 plus r1's translation. */
#define gte_CompMatrix(r1, r2, r3)                                                                    \
    {                                                                                                 \
        gte_MulMatrix0(r1, r2, r3);                                                                   \
        gte_SetTransMatrix(r1);                                                                       \
        gte_ldlv0((char*)r2 + 20);                                                                    \
        gte_rt();                                                                                     \
        gte_stlvnl((char*)r3 + 20);                                                                   \
    }

/* r5 = (r1 * r3 + r2 * r4) >> 12 for SVECTORs r1, r2 and weights r3, r4. */
#define gte_LoadAverageShort12(r1, r2, r3, r4, r5)                                                    \
    {                                                                                                 \
        gte_lddp(r3);                                                                                 \
        gte_ldsv(r1);                                                                                 \
        gte_gpf12();                                                                                  \
        gte_lddp(r4);                                                                                 \
        gte_ldsv(r2);                                                                                 \
        gte_gpl12();                                                                                  \
        gte_stsv(r5);                                                                                 \
    }

/* r2 = leading zeros (or ones) of r1. */
#define gte_Lzc(r1, r2)                                                                               \
    {                                                                                                 \
        gte_ldlzc(r1);                                                                                \
        gte_nop();                                                                                    \
        gte_nop();                                                                                    \
        gte_stlzc(r2);                                                                                \
    }

#endif
