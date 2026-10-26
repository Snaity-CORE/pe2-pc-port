/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisPlazaCollisionGridNormals[30] = {
#include "assets/akropolis_plaza_collision_1BBC0_normals.inc"
};
static SVECTOR _gAkropolisPlazaCollisionGridVerts[80] = {
#include "assets/akropolis_plaza_collision_1BBC0_verts.inc"
};
static WorldCollisionGridFace _gAkropolisPlazaCollisionGridFaces[32] = {
#include "assets/akropolis_plaza_collision_1BBC0_faces.inc"
};
static s16 _gAkropolisPlazaCollisionGridCells[192] = {
#include "assets/akropolis_plaza_collision_1BBC0_cells.inc"
};
#define AKROPOLIS_PLAZA_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisPlazaCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisPlazaCollisionGridTable[28] = {
#include "assets/akropolis_plaza_collision_1BBC0_table.inc"
};
#undef AKROPOLIS_PLAZA_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisPlazaCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisPlazaCollisionGridNormals, .vertices = _gAkropolisPlazaCollisionGridVerts, .faces = _gAkropolisPlazaCollisionGridFaces, .cellFaceIds = _gAkropolisPlazaCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
