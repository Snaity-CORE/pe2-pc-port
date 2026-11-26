/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area.h"
#include "stages/akropolis/stage.h"

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
    { .placements = gAkropolisStageBridgeDefaultPlacements, .resources = _gAkropolisBridgeDefaultAreaResources },
    { .placements = gAkropolisStageBridgeVariant2Placements, .resources = _gAkropolisBridgeVariant2AreaResources },
    { .placements = gAkropolisStageBridgeVariant3Placements, .resources = _gAkropolisBridgeVariant3AreaResources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
