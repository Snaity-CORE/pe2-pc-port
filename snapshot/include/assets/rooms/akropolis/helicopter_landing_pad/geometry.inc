/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

SVECTOR gAkropolisHelicopterLandingPadCollisionGridNormals[15] = {
#include "assets/akropolis_helicopter_landing_pad_collision_083D8_normals.inc"
};
SVECTOR gAkropolisHelicopterLandingPadCollisionGridVerts[92] = {
#include "assets/akropolis_helicopter_landing_pad_collision_083D8_verts.inc"
};
WorldCollisionGridFace gAkropolisHelicopterLandingPadCollisionGridFaces[37] = {
#include "assets/akropolis_helicopter_landing_pad_collision_083D8_faces.inc"
};
static s16 _gAkropolisHelicopterLandingPadCollisionGridCells[512] = {
#include "assets/akropolis_helicopter_landing_pad_collision_083D8_cells.inc"
};
#define AKROPOLIS_HELICOPTER_LANDING_PAD_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisHelicopterLandingPadCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisHelicopterLandingPadCollisionGridTable[36] = {
#include "assets/akropolis_helicopter_landing_pad_collision_083D8_table.inc"
};
#undef AKROPOLIS_HELICOPTER_LANDING_PAD_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisHelicopterLandingPadCollisionGrid = { .viewCoord = NULL, .normals = gAkropolisHelicopterLandingPadCollisionGridNormals, .vertices = gAkropolisHelicopterLandingPadCollisionGridVerts, .faces = gAkropolisHelicopterLandingPadCollisionGridFaces, .cellFaceIds = _gAkropolisHelicopterLandingPadCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
