/* The CD-ROM library (libcd) the game calls: drive commands and status,
 * positions, sector reads, and the streaming ring (St*). */
#ifndef PSX_LIBCD_H
#define PSX_LIBCD_H

#include "sys/types.h"

/* A disc position: minute, second and sector in BCD, and a track number. */
typedef struct {
    u_char minute;
    u_char second;
    u_char sector;
    u_char track;
} CdlLOC;

/* CD audio to SPU volumes: left and right inputs to left and right. */
typedef struct {
    u_char val0; /* CD left -> SPU left */
    u_char val1; /* CD left -> SPU right */
    u_char val2; /* CD right -> SPU left */
    u_char val3; /* CD right -> SPU right */
} CdlATV;

/* Header of a movie frame read through the streaming ring. */
typedef struct {
    u_short id;
    u_short type;
    u_short secCount;
    u_short nSectors;
    u_long frameCount;
    u_long frameSize;
    u_short width;
    u_short height;
    u_long dummy1;
    u_long dummy2;
    CdlLOC loc;
} StHEADER;

typedef void (*CdlCB)(u_char, u_char*);

/* Drive modes (CdlSetmode). */
#define CdlModeStream2 0x120
#define CdlModeSpeed   0x80 /* double speed */
#define CdlModeRT      0x40 /* XA-ADPCM playback */
#define CdlModeSize1   0x20 /* 2340-byte sectors */

/* Drive status bits. */
#define CdlStatShellOpen 0x10
#define CdlStatStandby   0x02 /* spindle motor on */
#define CdlStatError     0x01

/* CdGetDiskType results. */
#define CdlCdromFormat 2

/* Drive commands. */
#define CdlNop     0x01
#define CdlSetloc  0x02
#define CdlReadN   0x06
#define CdlStop    0x08
#define CdlPause   0x09
#define CdlSetmode 0x0e
#define CdlGetlocL 0x10
#define CdlGetTN   0x13
#define CdlSeekL   0x15

/* Interrupt / command results. */
#define CdlNoIntr    0x00
#define CdlComplete  0x02
#define CdlDiskError 0x05

/* Words in a 2048-byte sector. */
#define SECTOR_SIZE (512)

void StSetRing(u_long* ring_addr, u_long ring_size);
void StClearRing(void);
void StUnSetRing(void);
void StSetStream(u_long mode, u_long start_frame, u_long end_frame, void (*func1)(), void (*func2)());
u_long StFreeRing(u_long* base);
u_long StGetNext(u_long** addr, u_long** header);
void StCdInterrupt(void);
int StGetBackloc(CdlLOC* loc);

void CdFlush(void);
CdlLOC* CdIntToPos(int i, CdlLOC* p);
int CdControl(u_char com, u_char* param, u_char* result);
int CdControlB(u_char com, u_char* param, u_char* result);
int CdControlF(u_char com, u_char* param);
int CdGetSector(void* madr, int size);
int CdMix(CdlATV* vol);
int CdPosToInt(CdlLOC* p);
int CdRead2(long mode);
int CdSync(int mode, u_char* result);
CdlCB CdReadyCallback(CdlCB func);
CdlCB CdSyncCallback(CdlCB func);
int CdInit(void);
int CdReset(int mode);
int CdStatus(void);
int CdMode(void);
int CdDiskReady(int mode);
int CdGetDiskType(void);

#endif
