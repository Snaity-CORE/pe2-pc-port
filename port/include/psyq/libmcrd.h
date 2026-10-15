/* The memory card library (libmcrd) the game calls. */
#ifndef PSX_LIBMCRD_H
#define PSX_LIBMCRD_H

#include "kernel.h"

typedef void (*MemCB)(long cmds, long rslt);

void MemCardInit(long val);
void MemCardStart(void);
long MemCardExist(long chan);
long MemCardAccept(long chan);
long MemCardOpen(long chan, char* file, long flag);
void MemCardClose(void);
long MemCardReadData(unsigned long* adrs, long ofs, long bytes);
long MemCardWriteData(unsigned long* adrs, long ofs, long bytes);
long MemCardWriteFile(long chan, char* file, unsigned long* adrs, long ofs, long bytes);
long MemCardCreateFile(long chan, char* file, long blocks);
long MemCardFormat(long chan);
long MemCardSync(long mode, long* cmds, long* rslt);
long MemCardGetDirentry(long chan, char* name, struct DIRENTRY* dir, long* files, long ofs, long max);

#endif
