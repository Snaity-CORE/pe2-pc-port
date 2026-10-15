/* Memory block routines of the PlayStation C library. They are declared
 * without parameter lists, as in the library: callers pass byte pointers,
 * and memset's fill value goes through the default argument promotions. */
#ifndef PSX_MEMORY_H
#define PSX_MEMORY_H

#ifndef NULL
#define NULL 0
#endif

#ifndef PSX_SIZE_T
#define PSX_SIZE_T
typedef unsigned int size_t;
#endif

extern void* memcpy();
extern void* memset();

#endif
