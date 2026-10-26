/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"

static AreaResource _gAkropolisBridgeDefaultAreaResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = (void*)&_gAkropolisBridgeEnemyTaskDescStorage },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisBridgeVariant2AreaResources[1] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisBridgeVariant3AreaResources[1] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisBridgePlacementVariants[12] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisBridgeDefaultPlacements, .resources = _gAkropolisBridgeDefaultAreaResources },
    { .placements = (void*)&gMapAkropolisBridgeVariant2Placements, .resources = _gAkropolisBridgeVariant2AreaResources },
    { .placements = (void*)&gMapAkropolisBridgeVariant3Placements, .resources = _gAkropolisBridgeVariant3AreaResources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
