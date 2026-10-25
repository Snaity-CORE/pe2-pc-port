/* rooms/akropolis/sanctuary/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gAcropolisSanctuaryBlockerCollisionGridNormals 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static SVECTOR _gAcropolisSanctuaryBlockerCollisionGridNormals[4] = {
#include "assets/acropolis_sanctuary_collision_04D2C_normals.inc"
};
static SVECTOR _gAcropolisSanctuaryBlockerCollisionGridVerts[8] = {
#include "assets/acropolis_sanctuary_collision_04D2C_verts.inc"
};
static WorldCollisionGridFace _gAcropolisSanctuaryBlockerCollisionGridFaces[4] = {
#include "assets/acropolis_sanctuary_collision_04D2C_faces.inc"
};
static s16 _gAcropolisSanctuaryBlockerCollisionGridCells[6] = {
#include "assets/acropolis_sanctuary_collision_04D2C_cells.inc"
};
#define ACROPOLIS_SANCTUARY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisSanctuaryBlockerCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisSanctuaryBlockerCollisionGridTable[1] = {
#include "assets/acropolis_sanctuary_collision_04D2C_table.inc"
};
#undef ACROPOLIS_SANCTUARY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisSanctuaryBlockerCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisSanctuaryBlockerCollisionGridNormals, .vertices = _gAcropolisSanctuaryBlockerCollisionGridVerts, .faces = _gAcropolisSanctuaryBlockerCollisionGridFaces, .cellFaceIds = _gAcropolisSanctuaryBlockerCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
#endif
#undef ROOM_IS__gAcropolisSanctuaryBlockerCollisionGridNormals

#define ROOM_IS__gAcropolisSanctuaryCollisionGridNormals 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static SVECTOR _gAcropolisSanctuaryCollisionGridNormals[42] = {
#include "assets/acropolis_sanctuary_collision_05FA8_normals.inc"
};
static SVECTOR _gAcropolisSanctuaryCollisionGridVerts[136] = {
#include "assets/acropolis_sanctuary_collision_05FA8_verts.inc"
};
static WorldCollisionGridFace _gAcropolisSanctuaryCollisionGridFaces[85] = {
#include "assets/acropolis_sanctuary_collision_05FA8_faces.inc"
};
static s16 _gAcropolisSanctuaryCollisionGridCells[386] = {
#include "assets/acropolis_sanctuary_collision_05FA8_cells.inc"
};
#define ACROPOLIS_SANCTUARY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisSanctuaryCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisSanctuaryCollisionGridTable[9] = {
#include "assets/acropolis_sanctuary_collision_05FA8_table.inc"
};
#undef ACROPOLIS_SANCTUARY_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisSanctuaryCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisSanctuaryCollisionGridNormals, .vertices = _gAcropolisSanctuaryCollisionGridVerts, .faces = _gAcropolisSanctuaryCollisionGridFaces, .cellFaceIds = _gAcropolisSanctuaryCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
#endif
#undef ROOM_IS__gAcropolisSanctuaryCollisionGridNormals
#undef ROOM_PART
