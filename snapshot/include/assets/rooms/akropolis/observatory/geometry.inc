/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisObservatoryCollisionGridNormals[22] = {
#include "assets/acropolis_observatory_collision_03490_normals.inc"
};
static SVECTOR _gAcropolisObservatoryCollisionGridVerts[155] = {
#include "assets/acropolis_observatory_collision_03490_verts.inc"
};
static WorldCollisionGridFace _gAcropolisObservatoryCollisionGridFaces[62] = {
#include "assets/acropolis_observatory_collision_03490_faces.inc"
};
static s16 _gAcropolisObservatoryCollisionGridCells[232] = {
#include "assets/acropolis_observatory_collision_03490_cells.inc"
};
#define ACROPOLIS_OBSERVATORY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisObservatoryCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisObservatoryCollisionGridTable[9] = {
#include "assets/acropolis_observatory_collision_03490_table.inc"
};
#undef ACROPOLIS_OBSERVATORY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisObservatoryCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisObservatoryCollisionGridNormals, .vertices = _gAcropolisObservatoryCollisionGridVerts, .faces = _gAcropolisObservatoryCollisionGridFaces, .cellFaceIds = _gAcropolisObservatoryCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
