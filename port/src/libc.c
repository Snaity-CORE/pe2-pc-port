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

/* ---- memory and strings ------------------------------------------------- */

void* pcrt_memcpy(void* dst, const void* src, int n)
{
    return n > 0 ? memcpy(dst, src, (size_t)n) : dst;
}

void* pcrt_memset(void* dst, int c, int n)
{
    return n > 0 ? memset(dst, c, (size_t)n) : dst;
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
    va_start(ap, fmt);
    int n = vsprintf(out, fmt, ap);
    va_end(ap);
    return n;
}

/* The game's debug printf goes to stderr. */
int pcrt_printf(const char* fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    int n = vfprintf(stderr, fmt, ap);
    va_end(ap);
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

/* ---- heap (malloc3 / free3 / InitHeap3) ---------------------------------- */

/* A first-fit free-list allocator over the region the game hands to
 * InitHeap3. Blocks live in PSX RAM, so returned pointers are PSX addresses
 * like on the console. Each block starts with an 8-byte header. */
typedef struct HeapBlock {
    uint32_t size; /* bytes after the header */
    uint32_t free;
} HeapBlock;

static uint8_t* heap_base;
static uint32_t heap_size;

void pcrt_InitHeap3(void* addr, unsigned size)
{
    heap_base = (uint8_t*)(((uintptr_t)addr + 7) & ~(uintptr_t)7);
    heap_size = (uint32_t)(size - (uint32_t)(heap_base - (uint8_t*)addr)) & ~7u;
    HeapBlock* b = (HeapBlock*)heap_base;
    b->size = heap_size - sizeof(HeapBlock);
    b->free = 1;
}

void* pcrt_malloc3(unsigned size)
{
    size = (size + 7) & ~7u;
    for (uint8_t* p = heap_base; heap_base && p < heap_base + heap_size;) {
        HeapBlock* b = (HeapBlock*)p;
        if (b->free && b->size >= size) {
            if (b->size >= size + sizeof(HeapBlock) + 8) {
                HeapBlock* rest = (HeapBlock*)(p + sizeof(HeapBlock) + size);
                rest->size = b->size - size - sizeof(HeapBlock);
                rest->free = 1;
                b->size = size;
            }
            b->free = 0;
            return p + sizeof(HeapBlock);
        }
        p += sizeof(HeapBlock) + b->size;
    }
    return NULL;
}

void pcrt_free3(void* ptr)
{
    if (!ptr)
        return;
    ((HeapBlock*)((uint8_t*)ptr - sizeof(HeapBlock)))->free = 1;
    /* Coalesce adjacent free blocks. */
    for (uint8_t* p = heap_base; p < heap_base + heap_size;) {
        HeapBlock* b = (HeapBlock*)p;
        uint8_t* next = p + sizeof(HeapBlock) + b->size;
        if (b->free && next < heap_base + heap_size && ((HeapBlock*)next)->free) {
            b->size += sizeof(HeapBlock) + ((HeapBlock*)next)->size;
            continue;
        }
        p = next;
    }
}
