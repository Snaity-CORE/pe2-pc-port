/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gMistParkingCollisionGridNormals[13] = {
#include "assets/mist_parking_collision_14C44_normals.inc"
};
static SVECTOR _gMistParkingCollisionGridVerts[125] = {
#include "assets/mist_parking_collision_14C44_verts.inc"
};
static WorldCollisionGridFace _gMistParkingCollisionGridFaces[66] = {
#include "assets/mist_parking_collision_14C44_faces.inc"
};
static s16 _gMistParkingCollisionGridCells[434] = {
#include "assets/mist_parking_collision_14C44_cells.inc"
};
#define MIST_PARKING_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gMistParkingCollisionGridCells[(halfwordOffset)])
static s16* _gMistParkingCollisionGridTable[28] = {
#include "assets/mist_parking_collision_14C44_table.inc"
};
#undef MIST_PARKING_CELL_FACE_LIST_AT_OFFSET
WorldCollisionGrid gMistParkingCollisionGrid = { .viewCoord = NULL, .normals = _gMistParkingCollisionGridNormals, .vertices = _gMistParkingCollisionGridVerts, .faces = _gMistParkingCollisionGridFaces, .cellFaceIds = _gMistParkingCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
