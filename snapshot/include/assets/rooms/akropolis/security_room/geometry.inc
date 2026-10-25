/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisSecurityRoomCollisionGridNormals[11] = {
#include "assets/acropolis_security_room_collision_067D4_normals.inc"
};
static SVECTOR _gAcropolisSecurityRoomCollisionGridVerts[50] = {
#include "assets/acropolis_security_room_collision_067D4_verts.inc"
};
static WorldCollisionGridFace _gAcropolisSecurityRoomCollisionGridFaces[24] = {
#include "assets/acropolis_security_room_collision_067D4_faces.inc"
};
static s16 _gAcropolisSecurityRoomCollisionGridCells[46] = {
#include "assets/acropolis_security_room_collision_067D4_cells.inc"
};
#define ACROPOLIS_SECURITY_ROOM_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisSecurityRoomCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisSecurityRoomCollisionGridTable[2] = {
#include "assets/acropolis_security_room_collision_067D4_table.inc"
};
#undef ACROPOLIS_SECURITY_ROOM_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisSecurityRoomCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisSecurityRoomCollisionGridNormals, .vertices = _gAcropolisSecurityRoomCollisionGridVerts, .faces = _gAcropolisSecurityRoomCollisionGridFaces, .cellFaceIds = _gAcropolisSecurityRoomCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
