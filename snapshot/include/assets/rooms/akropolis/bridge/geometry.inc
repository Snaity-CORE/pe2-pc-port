/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisBridgeRoom1CollisionNormals[19] = {
#include "assets/akropolis_bridge_collision_0D2DC_normals.inc"
};
static SVECTOR _gAkropolisBridgeRoom1CollisionVerts[193] = {
#include "assets/akropolis_bridge_collision_0D2DC_verts.inc"
};
static WorldCollisionGridFace _gAkropolisBridgeRoom1CollisionFaces[92] = {
#include "assets/akropolis_bridge_collision_0D2DC_faces.inc"
};
static s16 _gAkropolisBridgeRoom1CollisionCells[274] = {
#include "assets/akropolis_bridge_collision_0D2DC_cells.inc"
};
#define AKROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisBridgeRoom1CollisionCells[(halfwordOffset)])
static s16* _gAkropolisBridgeRoom1CollisionTable[25] = {
#include "assets/akropolis_bridge_collision_0D2DC_table.inc"
};
#undef AKROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisBridgeRoom1Collision = { .viewCoord = NULL, .normals = _gAkropolisBridgeRoom1CollisionNormals, .vertices = _gAkropolisBridgeRoom1CollisionVerts, .faces = _gAkropolisBridgeRoom1CollisionFaces, .cellFaceIds = _gAkropolisBridgeRoom1CollisionTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
static SVECTOR _gAkropolisBridgeRoom2CollisionNormals[17] = {
#include "assets/akropolis_bridge_collision_0E0D4_normals.inc"
};
static SVECTOR _gAkropolisBridgeRoom2CollisionVerts[205] = {
#include "assets/akropolis_bridge_collision_0E0D4_verts.inc"
};
static WorldCollisionGridFace _gAkropolisBridgeRoom2CollisionFaces[93] = {
#include "assets/akropolis_bridge_collision_0E0D4_faces.inc"
};
static s16 _gAkropolisBridgeRoom2CollisionCells[274] = {
#include "assets/akropolis_bridge_collision_0E0D4_cells.inc"
};
#define AKROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisBridgeRoom2CollisionCells[(halfwordOffset)])
static s16* _gAkropolisBridgeRoom2CollisionTable[25] = {
#include "assets/akropolis_bridge_collision_0E0D4_table.inc"
};
#undef AKROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisBridgeRoom2Collision = { .viewCoord = NULL, .normals = _gAkropolisBridgeRoom2CollisionNormals, .vertices = _gAkropolisBridgeRoom2CollisionVerts, .faces = _gAkropolisBridgeRoom2CollisionFaces, .cellFaceIds = _gAkropolisBridgeRoom2CollisionTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
