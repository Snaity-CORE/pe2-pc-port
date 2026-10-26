/* rooms/akropolis/roof_garden/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisRoofGardenCollisionGridNormals[31] = {
#include "assets/akropolis_roof_garden_collision_07EE4_normals.inc"
};
static SVECTOR _gAkropolisRoofGardenCollisionGridVerts[92] = {
#include "assets/akropolis_roof_garden_collision_07EE4_verts.inc"
};
static WorldCollisionGridFace _gAkropolisRoofGardenCollisionGridFaces[53] = {
#include "assets/akropolis_roof_garden_collision_07EE4_faces.inc"
};
static s16 _gAkropolisRoofGardenCollisionGridCells[136] = {
#include "assets/akropolis_roof_garden_collision_07EE4_cells.inc"
};
#define AKROPOLIS_ROOF_GARDEN_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisRoofGardenCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisRoofGardenCollisionGridTable[9] = {
#include "assets/akropolis_roof_garden_collision_07EE4_table.inc"
};
#undef AKROPOLIS_ROOF_GARDEN_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisRoofGardenCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisRoofGardenCollisionGridNormals, .vertices = _gAkropolisRoofGardenCollisionGridVerts, .faces = _gAkropolisRoofGardenCollisionGridFaces, .cellFaceIds = _gAkropolisRoofGardenCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
