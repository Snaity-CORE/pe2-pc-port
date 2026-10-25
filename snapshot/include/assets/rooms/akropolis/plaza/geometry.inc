/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisPlazaCollisionGridNormals[30] = {
#include "assets/acropolis_plaza_collision_1BBC0_normals.inc"
};
static SVECTOR _gAcropolisPlazaCollisionGridVerts[80] = {
#include "assets/acropolis_plaza_collision_1BBC0_verts.inc"
};
static WorldCollisionGridFace _gAcropolisPlazaCollisionGridFaces[32] = {
#include "assets/acropolis_plaza_collision_1BBC0_faces.inc"
};
static s16 _gAcropolisPlazaCollisionGridCells[192] = {
#include "assets/acropolis_plaza_collision_1BBC0_cells.inc"
};
#define ACROPOLIS_PLAZA_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisPlazaCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisPlazaCollisionGridTable[28] = {
#include "assets/acropolis_plaza_collision_1BBC0_table.inc"
};
#undef ACROPOLIS_PLAZA_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisPlazaCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisPlazaCollisionGridNormals, .vertices = _gAcropolisPlazaCollisionGridVerts, .faces = _gAcropolisPlazaCollisionGridFaces, .cellFaceIds = _gAcropolisPlazaCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
