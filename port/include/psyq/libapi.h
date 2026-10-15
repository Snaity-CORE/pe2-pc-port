/* The kernel services of the system library (libapi) the game calls: root
 * counters, events, interrupts masking, files, and start-up. */
#ifndef PSX_LIBAPI_H
#define PSX_LIBAPI_H

#include "kernel.h"

struct EXEC;

extern long SetRCnt(unsigned long, unsigned short, long);
extern long GetRCnt(unsigned long);
extern long ResetRCnt(unsigned long);
extern long StartRCnt(unsigned long);
extern long StopRCnt(unsigned long);

extern long OpenEvent(unsigned long, long, long, long (*func)());
extern long CloseEvent(long);
extern long EnableEvent(long);
extern long DisableEvent(long);

extern long open(char*, unsigned long);
extern long close(long);
extern long read(long, void*, long);
extern long write(long, void*, long);
extern long format(char*);
extern long cd(char*);
extern long Load(char*, struct EXEC*);

extern int EnterCriticalSection(void);
extern void ExitCriticalSection(void);

extern unsigned long SetSp(unsigned long);
extern void SetMem(long);

#endif
