/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisBridgeRoom1CollisionNormals[19] = {
#include "assets/acropolis_bridge_collision_0D2DC_normals.inc"
};
static SVECTOR _gAcropolisBridgeRoom1CollisionVerts[193] = {
#include "assets/acropolis_bridge_collision_0D2DC_verts.inc"
};
static WorldCollisionGridFace _gAcropolisBridgeRoom1CollisionFaces[92] = {
#include "assets/acropolis_bridge_collision_0D2DC_faces.inc"
};
static s16 _gAcropolisBridgeRoom1CollisionCells[274] = {
#include "assets/acropolis_bridge_collision_0D2DC_cells.inc"
};
#define ACROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisBridgeRoom1CollisionCells[(halfwordOffset)])
static s16* _gAcropolisBridgeRoom1CollisionTable[25] = {
#include "assets/acropolis_bridge_collision_0D2DC_table.inc"
};
#undef ACROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisBridgeRoom1Collision = { .viewCoord = NULL, .normals = _gAcropolisBridgeRoom1CollisionNormals, .vertices = _gAcropolisBridgeRoom1CollisionVerts, .faces = _gAcropolisBridgeRoom1CollisionFaces, .cellFaceIds = _gAcropolisBridgeRoom1CollisionTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
static SVECTOR _gAcropolisBridgeRoom2CollisionNormals[17] = {
#include "assets/acropolis_bridge_collision_0E0D4_normals.inc"
};
static SVECTOR _gAcropolisBridgeRoom2CollisionVerts[205] = {
#include "assets/acropolis_bridge_collision_0E0D4_verts.inc"
};
static WorldCollisionGridFace _gAcropolisBridgeRoom2CollisionFaces[93] = {
#include "assets/acropolis_bridge_collision_0E0D4_faces.inc"
};
static s16 _gAcropolisBridgeRoom2CollisionCells[274] = {
#include "assets/acropolis_bridge_collision_0E0D4_cells.inc"
};
#define ACROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisBridgeRoom2CollisionCells[(halfwordOffset)])
static s16* _gAcropolisBridgeRoom2CollisionTable[25] = {
#include "assets/acropolis_bridge_collision_0E0D4_table.inc"
};
#undef ACROPOLIS_BRIDGE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisBridgeRoom2Collision = { .viewCoord = NULL, .normals = _gAcropolisBridgeRoom2CollisionNormals, .vertices = _gAcropolisBridgeRoom2CollisionVerts, .faces = _gAcropolisBridgeRoom2CollisionFaces, .cellFaceIds = _gAcropolisBridgeRoom2CollisionTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
