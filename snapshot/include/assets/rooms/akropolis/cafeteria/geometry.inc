/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisCafeteriaCollisionGridNormals[36] = {
#include "assets/acropolis_cafeteria_collision_0B1E8_normals.inc"
};
static SVECTOR _gAcropolisCafeteriaCollisionGridVerts[207] = {
#include "assets/acropolis_cafeteria_collision_0B1E8_verts.inc"
};
static WorldCollisionGridFace _gAcropolisCafeteriaCollisionGridFaces[120] = {
#include "assets/acropolis_cafeteria_collision_0B1E8_faces.inc"
};
static s16 _gAcropolisCafeteriaCollisionGridCells[480] = {
#include "assets/acropolis_cafeteria_collision_0B1E8_cells.inc"
};
#define ACROPOLIS_CAFETERIA_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisCafeteriaCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisCafeteriaCollisionGridTable[9] = {
#include "assets/acropolis_cafeteria_collision_0B1E8_table.inc"
};
#undef ACROPOLIS_CAFETERIA_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisCafeteriaCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisCafeteriaCollisionGridNormals, .vertices = _gAcropolisCafeteriaCollisionGridVerts, .faces = _gAcropolisCafeteriaCollisionGridFaces, .cellFaceIds = _gAcropolisCafeteriaCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
