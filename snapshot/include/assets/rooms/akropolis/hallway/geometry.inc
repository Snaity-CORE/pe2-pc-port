/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisHallwayCollisionGridNormals[10] = {
#include "assets/acropolis_hallway_collision_01010_normals.inc"
};
static SVECTOR _gAcropolisHallwayCollisionGridVerts[35] = {
#include "assets/acropolis_hallway_collision_01010_verts.inc"
};
static WorldCollisionGridFace _gAcropolisHallwayCollisionGridFaces[17] = {
#include "assets/acropolis_hallway_collision_01010_faces.inc"
};
static s16 _gAcropolisHallwayCollisionGridCells[30] = {
#include "assets/acropolis_hallway_collision_01010_cells.inc"
};
#define ACROPOLIS_HALLWAY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisHallwayCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisHallwayCollisionGridTable[2] = {
#include "assets/acropolis_hallway_collision_01010_table.inc"
};
#undef ACROPOLIS_HALLWAY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisHallwayCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisHallwayCollisionGridNormals, .vertices = _gAcropolisHallwayCollisionGridVerts, .faces = _gAcropolisHallwayCollisionGridFaces, .cellFaceIds = _gAcropolisHallwayCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
