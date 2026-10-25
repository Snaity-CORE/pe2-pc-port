/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisFountainCollisionGridNormals[32] = {
#include "assets/acropolis_fountain_collision_0204C_normals.inc"
};
static SVECTOR _gAcropolisFountainCollisionGridVerts[181] = {
#include "assets/acropolis_fountain_collision_0204C_verts.inc"
};
static WorldCollisionGridFace _gAcropolisFountainCollisionGridFaces[66] = {
#include "assets/acropolis_fountain_collision_0204C_faces.inc"
};
static s16 _gAcropolisFountainCollisionGridCells[326] = {
#include "assets/acropolis_fountain_collision_0204C_cells.inc"
};
#define ACROPOLIS_FOUNTAIN_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisFountainCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisFountainCollisionGridTable[16] = {
#include "assets/acropolis_fountain_collision_0204C_table.inc"
};
#undef ACROPOLIS_FOUNTAIN_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisFountainCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisFountainCollisionGridNormals, .vertices = _gAcropolisFountainCollisionGridVerts, .faces = _gAcropolisFountainCollisionGridFaces, .cellFaceIds = _gAcropolisFountainCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
