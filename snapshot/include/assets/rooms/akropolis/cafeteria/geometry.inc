/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisCafeteriaCollisionGridNormals[36] = {
#include "assets/akropolis_cafeteria_collision_0B1E8_normals.inc"
};
static SVECTOR _gAkropolisCafeteriaCollisionGridVerts[207] = {
#include "assets/akropolis_cafeteria_collision_0B1E8_verts.inc"
};
static WorldCollisionGridFace _gAkropolisCafeteriaCollisionGridFaces[120] = {
#include "assets/akropolis_cafeteria_collision_0B1E8_faces.inc"
};
static s16 _gAkropolisCafeteriaCollisionGridCells[480] = {
#include "assets/akropolis_cafeteria_collision_0B1E8_cells.inc"
};
#define AKROPOLIS_CAFETERIA_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisCafeteriaCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisCafeteriaCollisionGridTable[9] = {
#include "assets/akropolis_cafeteria_collision_0B1E8_table.inc"
};
#undef AKROPOLIS_CAFETERIA_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisCafeteriaCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisCafeteriaCollisionGridNormals, .vertices = _gAkropolisCafeteriaCollisionGridVerts, .faces = _gAkropolisCafeteriaCollisionGridFaces, .cellFaceIds = _gAkropolisCafeteriaCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
