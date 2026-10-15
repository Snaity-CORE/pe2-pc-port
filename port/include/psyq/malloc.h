/* Heap routines of the PlayStation C library: malloc/free, and the
 * malloc3/free3 heap the game uses (InitHeap3 sets it up). */
#ifndef PSX_MALLOC_H
#define PSX_MALLOC_H

#ifndef NULL
#define NULL 0
#endif

#ifndef PSX_SIZE_T
#define PSX_SIZE_T
typedef unsigned int size_t;
#endif

extern void* malloc(size_t);
extern void free(void*);
extern void InitHeap3(unsigned long*, unsigned long);
extern void* malloc3(size_t);
extern void free3(void*);

#endif
