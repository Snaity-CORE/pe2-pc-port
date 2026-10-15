/* The data-compression library's MDEC calls (libpress) the game makes. */
#ifndef PSX_LIBPRESS_H
#define PSX_LIBPRESS_H

#include "sys/types.h"

/* DecDCTvlc2's lookup table. */
typedef u_short DECDCTTAB[34816];

extern void DecDCTReset(int mode);
extern int DecDCTBufSize(u_long* bs);
extern int DecDCTvlc2(u_long* bs, u_long* buf, DECDCTTAB table);
extern int DecDCTvlcSize2(int size);
extern void DecDCTvlcBuild(u_short* table);
extern void DecDCTin(u_long* buf, int mode);
extern void DecDCTout(u_long* buf, int size);
extern int DecDCToutCallback(void (*func)());

#endif
