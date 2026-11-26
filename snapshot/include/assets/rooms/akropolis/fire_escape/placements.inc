/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "enemies/stranger/lesser.h"
#include "gameplay/area.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisFireEscapeActorResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gLesserStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisFireEscapeAreaVariants[5] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageFireEscapePlacements, .resources = _gAkropolisFireEscapeActorResources },
    { .placements = gAkropolisStageFireEscapePlacements, .resources = _gAkropolisFireEscapeActorResources },
    { .placements = gAkropolisStageFireEscapePlacements, .resources = _gAkropolisFireEscapeActorResources },
    { .placements = NULL, .resources = NULL },
};
