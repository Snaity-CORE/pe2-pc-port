/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gMistShootingGalleryCollisionGridNormals[22] = {
#include "assets/mist_shooting_gallery_collision_0C3A8_normals.inc"
};
static SVECTOR _gMistShootingGalleryCollisionGridVerts[136] = {
#include "assets/mist_shooting_gallery_collision_0C3A8_verts.inc"
};
static WorldCollisionGridFace _gMistShootingGalleryCollisionGridFaces[57] = {
#include "assets/mist_shooting_gallery_collision_0C3A8_faces.inc"
};
static s16 _gMistShootingGalleryCollisionGridCells[474] = {
#include "assets/mist_shooting_gallery_collision_0C3A8_cells.inc"
};
#define MIST_SHOOTING_GALLERY_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gMistShootingGalleryCollisionGridCells[(halfwordOffset)])
static s16* _gMistShootingGalleryCollisionGridTable[28] = {
#include "assets/mist_shooting_gallery_collision_0C3A8_table.inc"
};
#undef MIST_SHOOTING_GALLERY_CELL_FACE_LIST_AT_OFFSET
WorldCollisionGrid gMistShootingGalleryCollisionGrid = { .viewCoord = NULL, .normals = _gMistShootingGalleryCollisionGridNormals, .vertices = _gMistShootingGalleryCollisionGridVerts, .faces = _gMistShootingGalleryCollisionGridFaces, .cellFaceIds = _gMistShootingGalleryCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
