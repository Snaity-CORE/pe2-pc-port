/* The system library's video timing, video mode and pad-bit definitions
 * (libetc) the game uses. */
#ifndef PSX_LIBETC_H
#define PSX_LIBETC_H

/* Controller buttons, bit layout of the button word read from the pad. */
#define PADLup    (1 << 12)
#define PADLdown  (1 << 14)
#define PADLleft  (1 << 15)
#define PADLright (1 << 13)
#define PADRup    (1 << 4)
#define PADRdown  (1 << 6)
#define PADRleft  (1 << 7)
#define PADRright (1 << 5)
#define PADL1     (1 << 2)
#define PADL2     (1 << 0)
#define PADR1     (1 << 3)
#define PADR2     (1 << 1)
#define PADi      (1 << 9)  /* L3 */
#define PADj      (1 << 10) /* R3 */
#define PADselect (1 << 8)
#define PADstart  (1 << 11)

#define MODE_NTSC 0
#define MODE_PAL  1

int ResetCallback(void);
int VSync(int mode);
int VSyncCallback(void (*f)(void));
long SetVideoMode(long mode);

#endif
