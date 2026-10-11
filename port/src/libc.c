/*
 * The Psy-Q C library functions the game calls (pcrt_*), with the console's
 * behavior. Library state that lives in PSX RAM stays there, so it starts
 * from the values loaded from disc.
 */
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "pc/memory.h"

/* Held around CRT calls that take locks: see pc_irq_enter in the Rust runtime. */
void pc_irq_enter(void);
void pc_irq_leave(void);

/* ---- memory and strings ------------------------------------------------- */

/* The console's memcpy copies one byte at a time, front to back, and the game
 * relies on it: overlapping copies replicate a pattern instead of moving it,
 * which the C library's memcpy does not guarantee. A null destination returns
 * NULL. */
void* pcrt_memcpy(void* dst, const void* src, int n)
{
    if (!dst)
        return NULL;
    volatile uint8_t* d = (volatile uint8_t*)dst;
    const volatile uint8_t* s = (const volatile uint8_t*)src;
    for (int i = 0; i < n; i++)
        d[i] = s[i];
    return dst;
}

/* Same contract as the console's memset: NULL for a null destination or a
 * count of zero or less, otherwise `dst`. */
void* pcrt_memset(void* dst, int c, int n)
{
    if (!dst || n <= 0)
        return NULL;
    return memset(dst, c, (size_t)n);
}

int pcrt_strcmp(const char* a, const char* b)
{
    return strcmp(a, b);
}

int pcrt_strncmp(const char* a, const char* b, int n)
{
    return n > 0 ? strncmp(a, b, (size_t)n) : 0;
}

/* ---- formatted output --------------------------------------------------- */

int pcrt_sprintf(char* out, const char* fmt, ...)
{
    va_list ap;
    pc_irq_enter();
    va_start(ap, fmt);
    int n = vsprintf(out, fmt, ap);
    va_end(ap);
    pc_irq_leave();
    return n;
}

/* The game's debug printf goes to stderr. */
int pcrt_printf(const char* fmt, ...)
{
    va_list ap;
    pc_irq_enter();
    va_start(ap, fmt);
    int n = vfprintf(stderr, fmt, ap);
    va_end(ap);
    pc_irq_leave();
    return n;
}

/* ---- rand ---------------------------------------------------------------- */

/* The generator state is a library global in the main executable's data
 * (USA SLUS_010.42), initialized from disc. */
#define PSX_RAND_STATE ((uint32_t*)pc_psx_ptr(0x80073C10u))

int pcrt_rand(void)
{
    uint32_t s = *PSX_RAND_STATE * 0x41C64E6Du + 0x3039u;
    *PSX_RAND_STATE = s;
    return (int)((s >> 16) & 0x7FFF);
}

void pcrt_srand(unsigned seed)
{
    *PSX_RAND_STATE = seed;
}

/* malloc3 / free3 / InitHeap3 are not implemented here on purpose: the game
 * keeps several heaps by writing libapi's _freep cursor before each call, so
 * the original routines run in the interpreter on the real heap state. */
