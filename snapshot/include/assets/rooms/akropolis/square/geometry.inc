/* rooms/akropolis/square/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisSquareCollisionGridNormals[82] = {
#include "assets/akropolis_square_collision_07BDC_normals.inc"
};
static SVECTOR _gAkropolisSquareCollisionGridVerts[262] = {
#include "assets/akropolis_square_collision_07BDC_verts.inc"
};
static WorldCollisionGridFace _gAkropolisSquareCollisionGridFaces[115] = {
#include "assets/akropolis_square_collision_07BDC_faces.inc"
};
static s16 _gAkropolisSquareCollisionGridCells[506] = {
#include "assets/akropolis_square_collision_07BDC_cells.inc"
};
#define AKROPOLIS_SQUARE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisSquareCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisSquareCollisionGridTable[16] = {
#include "assets/akropolis_square_collision_07BDC_table.inc"
};
#undef AKROPOLIS_SQUARE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisSquareCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisSquareCollisionGridNormals, .vertices = _gAkropolisSquareCollisionGridVerts, .faces = _gAkropolisSquareCollisionGridFaces, .cellFaceIds = _gAkropolisSquareCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
