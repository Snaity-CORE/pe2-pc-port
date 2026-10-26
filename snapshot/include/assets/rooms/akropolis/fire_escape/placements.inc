/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "enemies/stranger/lesser.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"

static AreaResource _gAkropolisFireEscapeActorResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gLesserStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisFireEscapeAreaVariants[5] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisFireEscapePlacements, .resources = _gAkropolisFireEscapeActorResources },
    { .placements = (void*)&gMapAkropolisFireEscapePlacements, .resources = _gAkropolisFireEscapeActorResources },
    { .placements = (void*)&gMapAkropolisFireEscapePlacements, .resources = _gAkropolisFireEscapeActorResources },
    { .placements = NULL, .resources = NULL },
};
