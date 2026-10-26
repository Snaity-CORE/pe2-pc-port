/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

SVECTOR gAkropolisFireEscapeCollisionGridNormals[10] = {
#include "assets/akropolis_fire_escape_collision_04CE8_normals.inc"
};
SVECTOR gAkropolisFireEscapeCollisionGridVerts[55] = {
#include "assets/akropolis_fire_escape_collision_04CE8_verts.inc"
};
WorldCollisionGridFace gAkropolisFireEscapeCollisionGridFaces[23] = {
#include "assets/akropolis_fire_escape_collision_04CE8_faces.inc"
};
static s16 _gAkropolisFireEscapeCollisionGridCells[122] = {
#include "assets/akropolis_fire_escape_collision_04CE8_cells.inc"
};
#define AKROPOLIS_FIRE_ESCAPE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisFireEscapeCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisFireEscapeCollisionGridTable[9] = {
#include "assets/akropolis_fire_escape_collision_04CE8_table.inc"
};
#undef AKROPOLIS_FIRE_ESCAPE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisFireEscapeCollisionGrid = { .viewCoord = NULL, .normals = gAkropolisFireEscapeCollisionGridNormals, .vertices = gAkropolisFireEscapeCollisionGridVerts, .faces = gAkropolisFireEscapeCollisionGridFaces, .cellFaceIds = _gAkropolisFireEscapeCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
