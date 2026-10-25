/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gMistShootingGalleryStoryBarrierCollisionTemplateNormals 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
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
#endif
#undef ROOM_IS__gMistShootingGalleryStoryBarrierCollisionTemplateNormals

#define ROOM_IS__gMistShootingGalleryCollisionGridNormals 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
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
#endif
#undef ROOM_IS__gMistShootingGalleryCollisionGridNormals
#undef ROOM_PART
