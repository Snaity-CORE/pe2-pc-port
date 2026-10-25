/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

SVECTOR gAcropolisHelicopterLandingPadCollisionGridNormals[15] = {
#include "assets/acropolis_helicopter_landing_pad_collision_083D8_normals.inc"
};
SVECTOR gAcropolisHelicopterLandingPadCollisionGridVerts[92] = {
#include "assets/acropolis_helicopter_landing_pad_collision_083D8_verts.inc"
};
WorldCollisionGridFace gAcropolisHelicopterLandingPadCollisionGridFaces[37] = {
#include "assets/acropolis_helicopter_landing_pad_collision_083D8_faces.inc"
};
static s16 _gAcropolisHelicopterLandingPadCollisionGridCells[512] = {
#include "assets/acropolis_helicopter_landing_pad_collision_083D8_cells.inc"
};
#define ACROPOLIS_HELICOPTER_LANDING_PAD_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisHelicopterLandingPadCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisHelicopterLandingPadCollisionGridTable[36] = {
#include "assets/acropolis_helicopter_landing_pad_collision_083D8_table.inc"
};
#undef ACROPOLIS_HELICOPTER_LANDING_PAD_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisHelicopterLandingPadCollisionGrid = { .viewCoord = NULL, .normals = gAcropolisHelicopterLandingPadCollisionGridNormals, .vertices = gAcropolisHelicopterLandingPadCollisionGridVerts, .faces = gAcropolisHelicopterLandingPadCollisionGridFaces, .cellFaceIds = _gAcropolisHelicopterLandingPadCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
