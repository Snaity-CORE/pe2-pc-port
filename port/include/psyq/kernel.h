/* The PlayStation kernel's descriptors and structures the game uses: root
 * counter and event descriptors, and memory card directory entries. */
#ifndef PSX_KERNEL_H
#define PSX_KERNEL_H

#ifndef NULL
#define NULL 0
#endif

/* Descriptor classes. */
#define DescMask 0xff000000
#define DescHW   0xf0000000
#define DescRC   0xf2000000

/* Root counters (DescRC | counter). */
#define RCntCNT0 (DescRC | 0x00) /* pixel clock */
#define RCntCNT1 (DescRC | 0x01) /* horizontal sync */
#define RCntCNT2 (DescRC | 0x02) /* system clock / 8 */
#define RCntCNT3 (DescRC | 0x03) /* vertical sync */

/* Root counter modes. */
#define RCntMdINTR   0x1000
#define RCntMdNOINTR 0x2000
#define RCntMdSC     0x0001

/* Event specifications and modes. */
#define EvSpINT  0x0002 /* interrupt */
#define EvMdINTR 0x1000 /* call the handler from the interrupt */

/* A file entry of a memory card directory. */
struct DIRENTRY {
    char name[20];
    long attr;
    long size;
    struct DIRENTRY* next;
    long head;
    char system[4];
};

#endif
