/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisForkedRoadCollisionGridNormals[18] = {
#include "assets/akropolis_forked_road_collision_05630_normals.inc"
};
static SVECTOR _gAkropolisForkedRoadCollisionGridVerts[118] = {
#include "assets/akropolis_forked_road_collision_05630_verts.inc"
};
static WorldCollisionGridFace _gAkropolisForkedRoadCollisionGridFaces[38] = {
#include "assets/akropolis_forked_road_collision_05630_faces.inc"
};
static s16 _gAkropolisForkedRoadCollisionGridCells[244] = {
#include "assets/akropolis_forked_road_collision_05630_cells.inc"
};
#define AKROPOLIS_FORKED_ROAD_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisForkedRoadCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisForkedRoadCollisionGridTable[24] = {
#include "assets/akropolis_forked_road_collision_05630_table.inc"
};
#undef AKROPOLIS_FORKED_ROAD_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisForkedRoadCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisForkedRoadCollisionGridNormals, .vertices = _gAkropolisForkedRoadCollisionGridVerts, .faces = _gAkropolisForkedRoadCollisionGridFaces, .cellFaceIds = _gAkropolisForkedRoadCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
