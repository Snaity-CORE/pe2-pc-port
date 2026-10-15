/* Console output of the PlayStation C library (the debug channel). */
#ifndef PSX_STDIO_H
#define PSX_STDIO_H

#ifndef NULL
#define NULL 0
#endif

#ifndef PSX_SIZE_T
#define PSX_SIZE_T
typedef unsigned int size_t;
#endif

extern int printf(char* fmt, ...);
extern int sprintf(char* buffer, char* fmt, ...);
extern char* gets(char*);
extern void puts(char*);

#endif
