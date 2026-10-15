/* The sound library (libspu) the game calls: voices, keys, reverb, the CD
 * input mix, sound RAM transfers and the SPU interrupt. */
#ifndef PSX_LIBSPU_H
#define PSX_LIBSPU_H

#define SPU_OFF        0
#define SPU_ON         1
#define SPU_OFF_ENV_ON 2
#define SPU_ON_ENV_OFF 3

/* SpuIsTransferCompleted: wait for the transfer, or just look. */
#define SPU_TRANSFER_WAIT 1
#define SPU_TRANSFER_PEEK 0

/* Voice bits: one per voice, all 24 at once. */
#define SPU_KEYCH(x)   (0x1L << (x))
#define SPU_VOICECH(x) SPU_KEYCH(x)
#define SPU_ALLCH      (0xFFFFFFL)

/* SpuVoiceAttr mask bits. */
#define SPU_VOICE_PITCH      (0x01 << 4)
#define SPU_VOICE_WDSA       (0x01 << 7)
#define SPU_VOICE_ADSR_ADSR1 (0x01 << 17)
#define SPU_VOICE_ADSR_ADSR2 (0x01 << 18)
#define SPU_VOICE_DIRECT     0

/* SpuCommonAttr mask bits. */
#define SPU_COMMON_CDVOLL (0x01 << 6)
#define SPU_COMMON_CDVOLR (0x01 << 7)

/* SpuReverbAttr mask bits and modes. */
#define SPU_REV_MODE      (0x01 << 0)
#define SPU_REV_DEPTHL    (0x01 << 1)
#define SPU_REV_DEPTHR    (0x01 << 2)
#define SPU_REV_MODE_OFF  0

typedef struct {
    short left;
    short right;
} SpuVolume;

typedef struct {
    unsigned long voice; /* voice bits */
    unsigned long mask;  /* which attributes to set */
    SpuVolume volume;
    SpuVolume volmode;
    SpuVolume volumex;
    unsigned short pitch;
    unsigned short note;
    unsigned short sample_note;
    short envx;
    unsigned long addr;      /* waveform start in sound RAM */
    unsigned long loop_addr; /* loop start */
    long a_mode;
    long s_mode;
    long r_mode;
    unsigned short ar;
    unsigned short dr;
    unsigned short sr;
    unsigned short rr;
    unsigned short sl;
    unsigned short adsr1;
    unsigned short adsr2;
} SpuVoiceAttr;

typedef struct {
    short voiceNum;
    short pad;
    SpuVoiceAttr attr;
} SpuLVoiceAttr;

typedef struct {
    unsigned long mask;
    long mode;
    SpuVolume depth;
    long delay;    /* echo and delay modes */
    long feedback; /* echo mode */
} SpuReverbAttr;

typedef struct {
    SpuVolume volume;
    long reverb;
    long mix;
} SpuExtAttr;

typedef struct {
    unsigned long mask;
    SpuVolume mvol;
    SpuVolume mvolmode;
    SpuVolume mvolx;
    SpuExtAttr cd;
    SpuExtAttr ext;
} SpuCommonAttr;

typedef void (*SpuIRQCallbackProc)(void);
typedef void (*SpuTransferCallbackProc)(void);

extern void SpuInit(void);
extern long SpuSetReverb(long on_off);
extern long SpuSetReverbModeParam(SpuReverbAttr* attr);
extern long SpuSetReverbDepth(SpuReverbAttr* attr);
extern long SpuReserveReverbWorkArea(long on_off);
extern unsigned long SpuSetReverbVoice(long on_off, unsigned long voice_bit);
extern unsigned long SpuGetReverbVoice(void);
extern long SpuClearReverbWorkArea(long mode);
extern unsigned long SpuWrite(unsigned char* addr, unsigned long size);
extern unsigned long SpuSetTransferStartAddr(unsigned long addr);
extern unsigned long SpuWritePartly(unsigned char* addr, unsigned long size);
extern long SpuIsTransferCompleted(long flag);
extern SpuTransferCallbackProc SpuSetTransferCallback(SpuTransferCallbackProc func);
extern long SpuSetIRQ(long on_off);
extern unsigned long SpuSetIRQAddr(unsigned long);
extern SpuIRQCallbackProc SpuSetIRQCallback(SpuIRQCallbackProc);
extern void SpuSetKey(long on_off, unsigned long voice_bit);
extern void SpuGetAllKeysStatus(char* status);
extern void SpuSetCommonAttr(SpuCommonAttr* attr);
extern void SpuLSetVoiceAttr(int num, SpuLVoiceAttr* argList);

#endif
