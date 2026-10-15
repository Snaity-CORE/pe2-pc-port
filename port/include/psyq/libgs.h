/* The parts of the extended graphics library (libgs) the game uses:
 * coordinate systems, lights and ordering tables. Include libgte.h first. */
#ifndef PSX_LIBGS_H
#define PSX_LIBGS_H

#ifndef NULL
#define NULL 0
#endif

typedef struct {
    VECTOR scale;
    SVECTOR rotate;
    VECTOR trans;
} GsCOORD2PARAM;

/* A coordinate system: its matrix relative to its parent (super), and the
 * composed world matrix. */
typedef struct _GsCOORDINATE2 {
    unsigned long flg;
    MATRIX coord;
    MATRIX workm;
    GsCOORD2PARAM* param;
    struct _GsCOORDINATE2* super;
    struct _GsCOORDINATE2* sub;
} GsCOORDINATE2;

/* A directional light. */
typedef struct {
    int vx, vy, vz;
    unsigned char r, g, b;
} GsF_LIGHT;

/* An ordering table entry: the next packet and a count. */
typedef struct {
    unsigned p : 24;
    unsigned char num : 8;
} GsOT_TAG;

/* An ordering table of 1 << length entries. */
typedef struct {
    unsigned long length;
    GsOT_TAG* org;
    unsigned long offset;
    unsigned long point;
    GsOT_TAG* tag;
} GsOT;

/* The current world-to-screen matrix, and the world-to-screen matrix of
 * the light directions. */
extern MATRIX GsWSMATRIX;
extern MATRIX GsLIGHTWSMATRIX;

void GsClearOt(unsigned short offset, unsigned short point, GsOT* otp);

#endif
