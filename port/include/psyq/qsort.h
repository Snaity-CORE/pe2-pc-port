/* The PlayStation C library's sort. */
#ifndef PSX_QSORT_H
#define PSX_QSORT_H

#ifndef NULL
#define NULL 0
#endif

#ifndef PSX_SIZE_T
#define PSX_SIZE_T
typedef unsigned int size_t;
#endif

extern void qsort(void*, size_t, size_t, int (*)());

#endif
