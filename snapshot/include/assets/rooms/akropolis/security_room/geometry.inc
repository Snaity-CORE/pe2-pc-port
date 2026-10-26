/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisSecurityRoomCollisionGridNormals[11] = {
#include "assets/akropolis_security_room_collision_067D4_normals.inc"
};
static SVECTOR _gAkropolisSecurityRoomCollisionGridVerts[50] = {
#include "assets/akropolis_security_room_collision_067D4_verts.inc"
};
static WorldCollisionGridFace _gAkropolisSecurityRoomCollisionGridFaces[24] = {
#include "assets/akropolis_security_room_collision_067D4_faces.inc"
};
static s16 _gAkropolisSecurityRoomCollisionGridCells[46] = {
#include "assets/akropolis_security_room_collision_067D4_cells.inc"
};
#define AKROPOLIS_SECURITY_ROOM_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisSecurityRoomCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisSecurityRoomCollisionGridTable[2] = {
#include "assets/akropolis_security_room_collision_067D4_table.inc"
};
#undef AKROPOLIS_SECURITY_ROOM_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisSecurityRoomCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisSecurityRoomCollisionGridNormals, .vertices = _gAkropolisSecurityRoomCollisionGridVerts, .faces = _gAkropolisSecurityRoomCollisionGridFaces, .cellFaceIds = _gAkropolisSecurityRoomCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
