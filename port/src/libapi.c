/*
 * The heap routines of the system library (libapi) the game calls, written
 * for the port. The allocator is the classic first-fit free list of Kernighan
 * and Ritchie's "The C Programming Language" (section 8.7): blocks carry an
 * 8-byte header {next, size in 8-byte units}, free blocks form a ring in
 * address order closed by a zero-size block at the end of the heap, and
 * allocations come off the end of the block that fits. One difference from
 * K&R: free3 never merges into the closing block, and malloc3(0) returns
 * NULL. PE2_LIBTEST checks it
 * against the original step by step.
 *
 * The search cursor is libapi's own variable at its address in PSX RAM: the
 * game writes it to pick which of its two heaps the next call uses
 * (main/mem.c).
 */
#include <stddef.h>
#include <stdint.h>

typedef struct Header {
    struct Header* next;
    uint32_t size; /* in units of sizeof(Header) */
} Header;

#define FREEP (*(Header* volatile*)(uintptr_t)0x8005ED90u)

/* Makes the `size` bytes at head one free block, followed by the zero-size
 * block that closes the ring. */
void pcrt_InitHeap3(uint32_t* head, uint32_t size)
{
    Header* base = (Header*)head;
    uint32_t units = size / sizeof(Header);
    Header* end = base + (units - 1);
    base->next = end;
    base->size = units - 1;
    end->next = base;
    end->size = 0;
    FREEP = base;
}

void* pcrt_malloc3(size_t nbytes)
{
    if (nbytes == 0) {
        return NULL;
    }
    uint32_t nunits = (uint32_t)((nbytes + sizeof(Header) - 1) / sizeof(Header) + 1);
    Header* prevp = FREEP;
    for (Header* p = prevp->next;; prevp = p, p = p->next) {
        if (p->size >= nunits) {
            if (p->size == nunits) {
                prevp->next = p->next;
            } else {
                p->size -= nunits;
                p += p->size;
                p->size = nunits;
            }
            FREEP = prevp;
            return p + 1;
        }
        if (p == FREEP) {
            return NULL;
        }
    }
}

void pcrt_free3(void* ap)
{
    Header* bp = (Header*)ap - 1;
    Header* p;
    for (p = FREEP; !(bp > p && bp < p->next); p = p->next) {
        if (p >= p->next && (bp > p || bp < p->next)) {
            break; /* freed block at the start or end of the heap */
        }
    }
    /* Unlike K&R's free, a block is never merged into the zero-size block
     * that closes the ring, so that block always stays. */
    if (bp + bp->size == p->next && p->next->size != 0) {
        bp->size += p->next->size;
        bp->next = p->next->next;
    } else {
        bp->next = p->next;
    }
    if (p + p->size == bp) {
        p->size += bp->size;
        p->next = bp->next;
    } else {
        p->next = bp;
    }
    FREEP = p;
}

/* ---- start-up ------------------------------------------------------------ */

/* SetSp moves the MIPS stack; native code runs on the host's stack. */
uint32_t pcrt_SetSp(uint32_t sp)
{
    (void)sp;
    return 0;
}

/* SetMem selects 2 or 8 MB of RAM in the memory controller, which the host
 * does not model. */
void pcrt_SetMem(long megabytes)
{
    (void)megabytes;
}
