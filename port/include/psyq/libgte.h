/* The geometry library (libgte) the game calls: its vector and matrix
 * types and the routines (port/src/libgte.c implements them). Fixed point:
 * ONE (4096) is 1.0, and a full turn is 4096 in angles. */
#ifndef PSX_LIBGTE_H
#define PSX_LIBGTE_H

#include "sys/types.h"

#define ONE 4096

/* A rotation (or any 3x3, 1.0 = ONE) and a translation. */
typedef struct {
    short m[3][3];
    long t[3];
} MATRIX;

typedef struct {
    long vx, vy;
    long vz, pad;
} VECTOR;

typedef struct {
    short vx, vy;
    short vz, pad;
} SVECTOR;

/* A colour, with the GPU command code in its last byte. */
typedef struct {
    u_char r, g, b, cd;
} CVECTOR;

/* A screen position. */
typedef struct {
    short vx, vy;
} DVECTOR;

extern void InitGeom();

extern MATRIX* MulMatrix0(MATRIX* m0, MATRIX* m1, MATRIX* m2);
extern MATRIX* MulMatrix(MATRIX* m0, MATRIX* m1);
extern MATRIX* MulRotMatrix(MATRIX* m0);
extern MATRIX* CompMatrix(MATRIX* m0, MATRIX* m1, MATRIX* m2);
extern MATRIX* TransposeMatrix(MATRIX* m0, MATRIX* m1);
extern MATRIX* ScaleMatrix(MATRIX* m, VECTOR* v);
extern MATRIX* ScaleMatrixL(MATRIX* m, VECTOR* v);
extern void MatrixNormal(MATRIX* m, MATRIX* n);
extern void MatrixNormal_0(MATRIX* m, MATRIX* n);
extern void MatrixNormal_1(MATRIX* m, MATRIX* n);
extern void MatrixNormal_2(MATRIX* m, MATRIX* n);

extern MATRIX* RotMatrix(SVECTOR* r, MATRIX* m);
extern MATRIX* RotMatrix_gte(SVECTOR* r, MATRIX* m);
extern MATRIX* RotMatrixX(long r, MATRIX* m);
extern MATRIX* RotMatrixY(long r, MATRIX* m);
extern MATRIX* RotMatrixZ(long r, MATRIX* m);
extern MATRIX* RotMatrixZYX(SVECTOR* r, MATRIX* m);
extern MATRIX* RotMatrixZXY(SVECTOR* r, MATRIX* m);

extern VECTOR* ApplyMatrix(MATRIX* m, SVECTOR* v0, VECTOR* v1);
extern SVECTOR* ApplyMatrixSV(MATRIX* m, SVECTOR* v0, SVECTOR* v1);
extern VECTOR* ApplyMatrixLV(MATRIX* m, VECTOR* v0, VECTOR* v1);
extern VECTOR* ApplyTransposeMatrixLV(MATRIX* m, VECTOR* v0, VECTOR* v1);

extern void SetRotMatrix(MATRIX* m);
extern void SetTransMatrix(MATRIX* m);

extern long RotTransPers(SVECTOR* v0, long* sxy, long* p, long* flag);
extern long RotTransPers3(SVECTOR* v0, SVECTOR* v1, SVECTOR* v2, long* sxy0, long* sxy1, long* sxy2, long* p,
                          long* flag);
extern long RotTransPers4(SVECTOR* v0, SVECTOR* v1, SVECTOR* v2, SVECTOR* v3, long* sxy0, long* sxy1, long* sxy2,
                          long* sxy3, long* p, long* flag);
extern void RotTransSV(SVECTOR* v0, SVECTOR* v1, long* flag);

extern long VectorNormal(VECTOR* v0, VECTOR* v1);
extern long VectorNormalS(VECTOR* v0, SVECTOR* v1);
extern long VectorNormalSS(SVECTOR* v0, SVECTOR* v1);

extern long SquareRoot0(long a);
extern long SquareRoot12(long a);
extern int rsin(int a);
extern int rcos(int a);
extern long ratan2(long y, long x);
extern int cln(int a);

extern void LoadAverageCol(u_char* v0, u_char* v1, long p0, long p1, u_char* v2);
extern void gteMIMefunc(SVECTOR* otp, SVECTOR* dfp, long n, long p);

#endif
