/* rooms/akropolis/roof_garden/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisRoofGardenCollisionGridNormals[31] = {
#include "assets/acropolis_roof_garden_collision_07EE4_normals.inc"
};
static SVECTOR _gAcropolisRoofGardenCollisionGridVerts[92] = {
#include "assets/acropolis_roof_garden_collision_07EE4_verts.inc"
};
static WorldCollisionGridFace _gAcropolisRoofGardenCollisionGridFaces[53] = {
#include "assets/acropolis_roof_garden_collision_07EE4_faces.inc"
};
static s16 _gAcropolisRoofGardenCollisionGridCells[136] = {
#include "assets/acropolis_roof_garden_collision_07EE4_cells.inc"
};
#define ACROPOLIS_ROOF_GARDEN_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisRoofGardenCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisRoofGardenCollisionGridTable[9] = {
#include "assets/acropolis_roof_garden_collision_07EE4_table.inc"
};
#undef ACROPOLIS_ROOF_GARDEN_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisRoofGardenCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisRoofGardenCollisionGridNormals, .vertices = _gAcropolisRoofGardenCollisionGridVerts, .faces = _gAcropolisRoofGardenCollisionGridFaces, .cellFaceIds = _gAcropolisRoofGardenCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
