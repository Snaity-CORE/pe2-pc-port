/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisPatioCollisionGridNormals[33] = {
#include "assets/acropolis_patio_collision_06838_normals.inc"
};
static SVECTOR _gAcropolisPatioCollisionGridVerts[201] = {
#include "assets/acropolis_patio_collision_06838_verts.inc"
};
static WorldCollisionGridFace _gAcropolisPatioCollisionGridFaces[70] = {
#include "assets/acropolis_patio_collision_06838_faces.inc"
};
static s16 _gAcropolisPatioCollisionGridCells[402] = {
#include "assets/acropolis_patio_collision_06838_cells.inc"
};
#define ACROPOLIS_PATIO_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisPatioCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisPatioCollisionGridTable[28] = {
#include "assets/acropolis_patio_collision_06838_table.inc"
};
#undef ACROPOLIS_PATIO_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisPatioCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisPatioCollisionGridNormals, .vertices = _gAcropolisPatioCollisionGridVerts, .faces = _gAcropolisPatioCollisionGridFaces, .cellFaceIds = _gAcropolisPatioCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
