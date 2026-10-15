/* Absolute value, the PlayStation C library's abs() and its macro form. */
#ifndef PSX_ABS_H
#define PSX_ABS_H

#ifndef ABS
#define ABS(x) (((x) >= 0) ? (x) : (-(x)))
#endif

extern int abs(int);

#endif
