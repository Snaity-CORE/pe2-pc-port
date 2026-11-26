/* rooms/mist/r18/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "actors/actor_213000.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/mist_rupert.h"
#include "stages/akropolis/stage.h"

static AreaResource _gMistR18BriefingActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gMistRupertTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor213000EricBaldwinTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gMistR18AreaVariants[13] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageMistR18Placements, .resources = _gMistR18BriefingActorResources },
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageMistR18Placements, .resources = _gMistR18BriefingActorResources },
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
