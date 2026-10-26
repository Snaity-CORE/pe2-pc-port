/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisHallwayCollisionGridNormals[10] = {
#include "assets/akropolis_hallway_collision_01010_normals.inc"
};
static SVECTOR _gAkropolisHallwayCollisionGridVerts[35] = {
#include "assets/akropolis_hallway_collision_01010_verts.inc"
};
static WorldCollisionGridFace _gAkropolisHallwayCollisionGridFaces[17] = {
#include "assets/akropolis_hallway_collision_01010_faces.inc"
};
static s16 _gAkropolisHallwayCollisionGridCells[30] = {
#include "assets/akropolis_hallway_collision_01010_cells.inc"
};
#define AKROPOLIS_HALLWAY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisHallwayCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisHallwayCollisionGridTable[2] = {
#include "assets/akropolis_hallway_collision_01010_table.inc"
};
#undef AKROPOLIS_HALLWAY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisHallwayCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisHallwayCollisionGridNormals, .vertices = _gAkropolisHallwayCollisionGridVerts, .faces = _gAkropolisHallwayCollisionGridFaces, .cellFaceIds = _gAkropolisHallwayCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
