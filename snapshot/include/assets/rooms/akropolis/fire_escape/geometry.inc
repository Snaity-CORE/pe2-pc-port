/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

SVECTOR gAcropolisFireEscapeCollisionGridNormals[10] = {
#include "assets/acropolis_fire_escape_collision_04CE8_normals.inc"
};
SVECTOR gAcropolisFireEscapeCollisionGridVerts[55] = {
#include "assets/acropolis_fire_escape_collision_04CE8_verts.inc"
};
WorldCollisionGridFace gAcropolisFireEscapeCollisionGridFaces[23] = {
#include "assets/acropolis_fire_escape_collision_04CE8_faces.inc"
};
static s16 _gAcropolisFireEscapeCollisionGridCells[122] = {
#include "assets/acropolis_fire_escape_collision_04CE8_cells.inc"
};
#define ACROPOLIS_FIRE_ESCAPE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisFireEscapeCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisFireEscapeCollisionGridTable[9] = {
#include "assets/acropolis_fire_escape_collision_04CE8_table.inc"
};
#undef ACROPOLIS_FIRE_ESCAPE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisFireEscapeCollisionGrid = { .viewCoord = NULL, .normals = gAcropolisFireEscapeCollisionGridNormals, .vertices = gAcropolisFireEscapeCollisionGridVerts, .faces = gAcropolisFireEscapeCollisionGridFaces, .cellFaceIds = _gAcropolisFireEscapeCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
