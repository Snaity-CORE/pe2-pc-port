/* rooms/mist/r18/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gMistR18FloorCollisionGridNormals[1] = {
#include "assets/mist_r18_collision_09138_normals.inc"
};
static SVECTOR _gMistR18FloorCollisionGridVerts[4] = {
#include "assets/mist_r18_collision_09138_verts.inc"
};
static WorldCollisionGridFace _gMistR18FloorCollisionGridFaces[1] = {
#include "assets/mist_r18_collision_09138_faces.inc"
};
static s16 _gMistR18FloorCollisionGridCells[24] = {
#include "assets/mist_r18_collision_09138_cells.inc"
};
#define MIST_R18_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gMistR18FloorCollisionGridCells[(halfwordOffset)])
static s16* _gMistR18FloorCollisionGridTable[12] = {
#include "assets/mist_r18_collision_09138_table.inc"
};
#undef MIST_R18_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gMistR18FloorCollisionGrid = { .viewCoord = NULL, .normals = _gMistR18FloorCollisionGridNormals, .vertices = _gMistR18FloorCollisionGridVerts, .faces = _gMistR18FloorCollisionGridFaces, .cellFaceIds = _gMistR18FloorCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
