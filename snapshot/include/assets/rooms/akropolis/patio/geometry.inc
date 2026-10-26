/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisPatioCollisionGridNormals[33] = {
#include "assets/akropolis_patio_collision_06838_normals.inc"
};
static SVECTOR _gAkropolisPatioCollisionGridVerts[201] = {
#include "assets/akropolis_patio_collision_06838_verts.inc"
};
static WorldCollisionGridFace _gAkropolisPatioCollisionGridFaces[70] = {
#include "assets/akropolis_patio_collision_06838_faces.inc"
};
static s16 _gAkropolisPatioCollisionGridCells[402] = {
#include "assets/akropolis_patio_collision_06838_cells.inc"
};
#define AKROPOLIS_PATIO_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisPatioCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisPatioCollisionGridTable[28] = {
#include "assets/akropolis_patio_collision_06838_table.inc"
};
#undef AKROPOLIS_PATIO_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisPatioCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisPatioCollisionGridNormals, .vertices = _gAkropolisPatioCollisionGridVerts, .faces = _gAkropolisPatioCollisionGridFaces, .cellFaceIds = _gAkropolisPatioCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
