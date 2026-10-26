/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/shooting_gallery_pierce.h"
#include "stages/akropolis/actors/shooting_gallery_targets.h"

static AreaResource _gMistShootingGalleryVariant1Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gShootingGalleryTargetsTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gShootingGalleryPiercePierceTaskDescs },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gMistShootingGalleryAreaVariants[12] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisMistShootingGalleryPlacements, .resources = _gMistShootingGalleryVariant1Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
