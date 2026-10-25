/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisForkedRoadCollisionGridNormals[18] = {
#include "assets/acropolis_forked_road_collision_05630_normals.inc"
};
static SVECTOR _gAcropolisForkedRoadCollisionGridVerts[118] = {
#include "assets/acropolis_forked_road_collision_05630_verts.inc"
};
static WorldCollisionGridFace _gAcropolisForkedRoadCollisionGridFaces[38] = {
#include "assets/acropolis_forked_road_collision_05630_faces.inc"
};
static s16 _gAcropolisForkedRoadCollisionGridCells[244] = {
#include "assets/acropolis_forked_road_collision_05630_cells.inc"
};
#define ACROPOLIS_FORKED_ROAD_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisForkedRoadCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisForkedRoadCollisionGridTable[24] = {
#include "assets/acropolis_forked_road_collision_05630_table.inc"
};
#undef ACROPOLIS_FORKED_ROAD_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisForkedRoadCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisForkedRoadCollisionGridNormals, .vertices = _gAcropolisForkedRoadCollisionGridVerts, .faces = _gAcropolisForkedRoadCollisionGridFaces, .cellFaceIds = _gAcropolisForkedRoadCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
