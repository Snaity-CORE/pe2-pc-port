/* Basic unsigned integer types of the PlayStation C library, as the game
 * uses them. Written for the port (the native build does not use the SDK's
 * headers). */
#ifndef PSX_SYS_TYPES_H
#define PSX_SYS_TYPES_H

typedef unsigned char u_char;
typedef unsigned short u_short;
typedef unsigned int u_int;
typedef unsigned long u_long;
typedef unsigned int uint;

#ifndef PSX_SIZE_T
#define PSX_SIZE_T
typedef unsigned int size_t;
#endif

#endif
