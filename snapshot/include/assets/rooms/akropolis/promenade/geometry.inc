/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisPromenadeRoom1CollisionGridNormals[30] = {
#include "assets/akropolis_promenade_collision_04E1C_normals.inc"
};
static SVECTOR _gAkropolisPromenadeRoom1CollisionGridVerts[93] = {
#include "assets/akropolis_promenade_collision_04E1C_verts.inc"
};
static WorldCollisionGridFace _gAkropolisPromenadeRoom1CollisionGridFaces[38] = {
#include "assets/akropolis_promenade_collision_04E1C_faces.inc"
};
static s16 _gAkropolisPromenadeRoom1CollisionGridCells[144] = {
#include "assets/akropolis_promenade_collision_04E1C_cells.inc"
};
#define AKROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisPromenadeRoom1CollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisPromenadeRoom1CollisionGridTable[10] = {
#include "assets/akropolis_promenade_collision_04E1C_table.inc"
};
#undef AKROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisPromenadeRoom1CollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisPromenadeRoom1CollisionGridNormals, .vertices = _gAkropolisPromenadeRoom1CollisionGridVerts, .faces = _gAkropolisPromenadeRoom1CollisionGridFaces, .cellFaceIds = _gAkropolisPromenadeRoom1CollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
static SVECTOR _gAkropolisPromenadeRoom2CollisionGridNormals[30] = {
#include "assets/akropolis_promenade_collision_05610_normals.inc"
};
static SVECTOR _gAkropolisPromenadeRoom2CollisionGridVerts[103] = {
#include "assets/akropolis_promenade_collision_05610_verts.inc"
};
static WorldCollisionGridFace _gAkropolisPromenadeRoom2CollisionGridFaces[46] = {
#include "assets/akropolis_promenade_collision_05610_faces.inc"
};
static s16 _gAkropolisPromenadeRoom2CollisionGridCells[172] = {
#include "assets/akropolis_promenade_collision_05610_cells.inc"
};
#define AKROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisPromenadeRoom2CollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisPromenadeRoom2CollisionGridTable[10] = {
#include "assets/akropolis_promenade_collision_05610_table.inc"
};
#undef AKROPOLIS_PROMENADE_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisPromenadeRoom2CollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisPromenadeRoom2CollisionGridNormals, .vertices = _gAkropolisPromenadeRoom2CollisionGridVerts, .faces = _gAkropolisPromenadeRoom2CollisionGridFaces, .cellFaceIds = _gAkropolisPromenadeRoom2CollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
