/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisPromenadeRoom1CollisionGridNormals[30] = {
#include "assets/acropolis_promenade_collision_04E1C_normals.inc"
};
static SVECTOR _gAcropolisPromenadeRoom1CollisionGridVerts[93] = {
#include "assets/acropolis_promenade_collision_04E1C_verts.inc"
};
static WorldCollisionGridFace _gAcropolisPromenadeRoom1CollisionGridFaces[38] = {
#include "assets/acropolis_promenade_collision_04E1C_faces.inc"
};
static s16 _gAcropolisPromenadeRoom1CollisionGridCells[144] = {
#include "assets/acropolis_promenade_collision_04E1C_cells.inc"
};
#define ACROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisPromenadeRoom1CollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisPromenadeRoom1CollisionGridTable[10] = {
#include "assets/acropolis_promenade_collision_04E1C_table.inc"
};
#undef ACROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisPromenadeRoom1CollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisPromenadeRoom1CollisionGridNormals, .vertices = _gAcropolisPromenadeRoom1CollisionGridVerts, .faces = _gAcropolisPromenadeRoom1CollisionGridFaces, .cellFaceIds = _gAcropolisPromenadeRoom1CollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
static SVECTOR _gAcropolisPromenadeRoom2CollisionGridNormals[30] = {
#include "assets/acropolis_promenade_collision_05610_normals.inc"
};
static SVECTOR _gAcropolisPromenadeRoom2CollisionGridVerts[103] = {
#include "assets/acropolis_promenade_collision_05610_verts.inc"
};
static WorldCollisionGridFace _gAcropolisPromenadeRoom2CollisionGridFaces[46] = {
#include "assets/acropolis_promenade_collision_05610_faces.inc"
};
static s16 _gAcropolisPromenadeRoom2CollisionGridCells[172] = {
#include "assets/acropolis_promenade_collision_05610_cells.inc"
};
#define ACROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisPromenadeRoom2CollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisPromenadeRoom2CollisionGridTable[10] = {
#include "assets/acropolis_promenade_collision_05610_table.inc"
};
#undef ACROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisPromenadeRoom2CollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisPromenadeRoom2CollisionGridNormals, .vertices = _gAcropolisPromenadeRoom2CollisionGridVerts, .faces = _gAcropolisPromenadeRoom2CollisionGridFaces, .cellFaceIds = _gAcropolisPromenadeRoom2CollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
