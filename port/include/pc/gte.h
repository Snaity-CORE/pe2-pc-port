/*
 * Software implementation of the PlayStation Geometry Transformation Engine
 * (COP2), written from the public hardware documentation (psx-spx).
 *
 * The game drives the GTE through Psy-Q macros that, on the PSX, are inline
 * MIPS assembly. For the PC build those macros are regenerated as C (see
 * tools/gen_port_headers.py) on top of the four register accessors and the
 * command entry point declared here.
 */
#ifndef PC_GTE_H
#define PC_GTE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Data registers (cop2r0-31). */
enum {
    PCGTE_VXY0, PCGTE_VZ0, PCGTE_VXY1, PCGTE_VZ1, PCGTE_VXY2, PCGTE_VZ2,
    PCGTE_RGBC, PCGTE_OTZ, PCGTE_IR0, PCGTE_IR1, PCGTE_IR2, PCGTE_IR3,
    PCGTE_SXY0, PCGTE_SXY1, PCGTE_SXY2, PCGTE_SXYP,
    PCGTE_SZ0, PCGTE_SZ1, PCGTE_SZ2, PCGTE_SZ3,
    PCGTE_RGB0, PCGTE_RGB1, PCGTE_RGB2, PCGTE_RES1,
    PCGTE_MAC0, PCGTE_MAC1, PCGTE_MAC2, PCGTE_MAC3,
    PCGTE_IRGB, PCGTE_ORGB, PCGTE_LZCS, PCGTE_LZCR,
};

/* Control registers (cop2r32-63), numbered from 0. */
enum {
    PCGTE_RT0, PCGTE_RT1, PCGTE_RT2, PCGTE_RT3, PCGTE_RT4,
    PCGTE_TRX, PCGTE_TRY, PCGTE_TRZ,
    PCGTE_L0, PCGTE_L1, PCGTE_L2, PCGTE_L3, PCGTE_L4,
    PCGTE_RBK, PCGTE_GBK, PCGTE_BBK,
    PCGTE_LC0, PCGTE_LC1, PCGTE_LC2, PCGTE_LC3, PCGTE_LC4,
    PCGTE_RFC, PCGTE_GFC, PCGTE_BFC,
    PCGTE_OFX, PCGTE_OFY, PCGTE_H, PCGTE_DQA, PCGTE_DQB,
    PCGTE_ZSF3, PCGTE_ZSF4, PCGTE_FLAG,
};

uint32_t pcgte_read_data(unsigned reg);
void pcgte_write_data(unsigned reg, uint32_t value);
uint32_t pcgte_read_ctrl(unsigned reg);
void pcgte_write_ctrl(unsigned reg, uint32_t value);

/* Executes one GTE command, given as its full COP2 instruction word
 * (e.g. 0x4A180001 for RTPS). */
void pcgte_op(uint32_t instruction);

/* Executes an MVMVA given in Psy-Q's DMPSX placeholder encoding
 * (0x13BF | sf<<25 | mx<<23 | v<<21 | cv<<19 | lm<<18), as built at run time
 * by the gte_mvmva() macro. */
void pcgte_op_dmpsx_mvmva(uint32_t placeholder);

/* Stores SXY register `reg` (12-15) at `dst`, recording the projection's
 * exact position against that word for the renderer (pgxp.c). */
void pcgte_store_sxy(unsigned reg, uint32_t* dst);

/* Loads SXY register `reg` (12-15, 15 pushing the FIFO) from `src`, with
 * the exact position recorded for that word, if any. */
void pcgte_load_sxy(unsigned reg, const uint32_t* src);

/* Gives the vertex just written to V0-V2 (n) an identity, kept with its
 * projection's exact position (pgxp.c): the same vertex of the same model
 * part has the same key from frame to frame. Writing the vector clears it,
 * so set it after. 0: none. */
void pcgte_set_vector_key(unsigned n, uint32_t key);

/* Copies the word at `src` to `dst` with its recorded exact position. */
void pc_pgxp_copy(uint32_t dst, uint32_t src);

/* Clears every register (power-on state). */
void pcgte_reset(void);

#ifdef __cplusplus
}
#endif

#endif /* PC_GTE_H */
