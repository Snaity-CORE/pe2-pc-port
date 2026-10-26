/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisObservatoryCollisionGridNormals[22] = {
#include "assets/akropolis_observatory_collision_03490_normals.inc"
};
static SVECTOR _gAkropolisObservatoryCollisionGridVerts[155] = {
#include "assets/akropolis_observatory_collision_03490_verts.inc"
};
static WorldCollisionGridFace _gAkropolisObservatoryCollisionGridFaces[62] = {
#include "assets/akropolis_observatory_collision_03490_faces.inc"
};
static s16 _gAkropolisObservatoryCollisionGridCells[232] = {
#include "assets/akropolis_observatory_collision_03490_cells.inc"
};
#define AKROPOLIS_OBSERVATORY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisObservatoryCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisObservatoryCollisionGridTable[9] = {
#include "assets/akropolis_observatory_collision_03490_table.inc"
};
#undef AKROPOLIS_OBSERVATORY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisObservatoryCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisObservatoryCollisionGridNormals, .vertices = _gAkropolisObservatoryCollisionGridVerts, .faces = _gAkropolisObservatoryCollisionGridFaces, .cellFaceIds = _gAkropolisObservatoryCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
