/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisFountainCollisionGridNormals[32] = {
#include "assets/akropolis_fountain_collision_0204C_normals.inc"
};
static SVECTOR _gAkropolisFountainCollisionGridVerts[181] = {
#include "assets/akropolis_fountain_collision_0204C_verts.inc"
};
static WorldCollisionGridFace _gAkropolisFountainCollisionGridFaces[66] = {
#include "assets/akropolis_fountain_collision_0204C_faces.inc"
};
static s16 _gAkropolisFountainCollisionGridCells[326] = {
#include "assets/akropolis_fountain_collision_0204C_cells.inc"
};
#define AKROPOLIS_FOUNTAIN_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisFountainCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisFountainCollisionGridTable[16] = {
#include "assets/akropolis_fountain_collision_0204C_table.inc"
};
#undef AKROPOLIS_FOUNTAIN_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisFountainCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisFountainCollisionGridNormals, .vertices = _gAkropolisFountainCollisionGridVerts, .faces = _gAkropolisFountainCollisionGridFaces, .cellFaceIds = _gAkropolisFountainCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
