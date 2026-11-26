/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gMistShootingGalleryStoryBarrierCollisionTemplateNormals[3] = {
#include "assets/mist_shooting_gallery_collision_07BD8_normals.inc"
};
static SVECTOR _gMistShootingGalleryStoryBarrierCollisionTemplateVerts[8] = {
#include "assets/mist_shooting_gallery_collision_07BD8_verts.inc"
};
static WorldCollisionGridFace _gMistShootingGalleryStoryBarrierCollisionTemplateFaces[3] = {
#include "assets/mist_shooting_gallery_collision_07BD8_faces.inc"
};
static s16 _gMistShootingGalleryStoryBarrierCollisionTemplateCells[4] = {
#include "assets/mist_shooting_gallery_collision_07BD8_cells.inc"
};
#define MIST_SHOOTING_GALLERY_BARRIER_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gMistShootingGalleryStoryBarrierCollisionTemplateCells[(halfwordOffset)])
static s16* _gMistShootingGalleryStoryBarrierCollisionTemplateTable[1] = {
#include "assets/mist_shooting_gallery_collision_07BD8_table.inc"
};
#undef MIST_SHOOTING_GALLERY_BARRIER_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gMistShootingGalleryStoryBarrierCollisionTemplate = { .viewCoord = NULL, .normals = _gMistShootingGalleryStoryBarrierCollisionTemplateNormals, .vertices = _gMistShootingGalleryStoryBarrierCollisionTemplateVerts, .faces = _gMistShootingGalleryStoryBarrierCollisionTemplateFaces, .cellFaceIds = _gMistShootingGalleryStoryBarrierCollisionTemplateTable, .xBias = 0, .zBias = -0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
static SVECTOR _gMistShootingGalleryTrainingBarrierCollisionTemplateNormals[1] = {
#include "assets/mist_shooting_gallery_collision_07C38_normals.inc"
};
static SVECTOR _gMistShootingGalleryTrainingBarrierCollisionTemplateVerts[4] = {
#include "assets/mist_shooting_gallery_collision_07C38_verts.inc"
};
static WorldCollisionGridFace _gMistShootingGalleryTrainingBarrierCollisionTemplateFaces[1] = {
#include "assets/mist_shooting_gallery_collision_07C38_faces.inc"
};
static s16 _gMistShootingGalleryTrainingBarrierCollisionTemplateCells[2] = {
#include "assets/mist_shooting_gallery_collision_07C38_cells.inc"
};
#define MIST_SHOOTING_GALLERY_BARRIER_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gMistShootingGalleryTrainingBarrierCollisionTemplateCells[(halfwordOffset)])
static s16* _gMistShootingGalleryTrainingBarrierCollisionTemplateTable[1] = {
#include "assets/mist_shooting_gallery_collision_07C38_table.inc"
};
#undef MIST_SHOOTING_GALLERY_BARRIER_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gMistShootingGalleryTrainingBarrierCollisionTemplate = { .viewCoord = NULL, .normals = _gMistShootingGalleryTrainingBarrierCollisionTemplateNormals, .vertices = _gMistShootingGalleryTrainingBarrierCollisionTemplateVerts, .faces = _gMistShootingGalleryTrainingBarrierCollisionTemplateFaces, .cellFaceIds = _gMistShootingGalleryTrainingBarrierCollisionTemplateTable, .xBias = 0, .zBias = -0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
