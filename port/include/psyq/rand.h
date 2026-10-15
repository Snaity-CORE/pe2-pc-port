/* The PlayStation C library's pseudo-random numbers (0 to RAND_MAX). */
#ifndef PSX_RAND_H
#define PSX_RAND_H

#define RAND_MAX 32767

extern int rand(void);
extern void srand(unsigned int);

#endif
