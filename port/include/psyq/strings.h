/* String routines of the PlayStation C library the game calls. strcmp is
 * declared without a parameter list, as in the library. */
#ifndef PSX_STRINGS_H
#define PSX_STRINGS_H

#include "memory.h"

extern int strcmp();
extern int strncmp(char*, char*, int);
extern char* index(char*, char);

#endif
