/* rooms/akropolis/sanctuary/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisSanctuaryCollisionGridNormals[42] = {
#include "assets/akropolis_sanctuary_collision_05FA8_normals.inc"
};
static SVECTOR _gAkropolisSanctuaryCollisionGridVerts[136] = {
#include "assets/akropolis_sanctuary_collision_05FA8_verts.inc"
};
static WorldCollisionGridFace _gAkropolisSanctuaryCollisionGridFaces[85] = {
#include "assets/akropolis_sanctuary_collision_05FA8_faces.inc"
};
static s16 _gAkropolisSanctuaryCollisionGridCells[386] = {
#include "assets/akropolis_sanctuary_collision_05FA8_cells.inc"
};
#define AKROPOLIS_SANCTUARY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisSanctuaryCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisSanctuaryCollisionGridTable[9] = {
#include "assets/akropolis_sanctuary_collision_05FA8_table.inc"
};
#undef AKROPOLIS_SANCTUARY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisSanctuaryCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisSanctuaryCollisionGridNormals, .vertices = _gAkropolisSanctuaryCollisionGridVerts, .faces = _gAkropolisSanctuaryCollisionGridFaces, .cellFaceIds = _gAkropolisSanctuaryCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
