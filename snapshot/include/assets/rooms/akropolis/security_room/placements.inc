/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/security_monitors.h"

static AreaResource _gAkropolisSecurityRoomActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gSecurityMonitorsRupertTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gSecurityMonitorsSwatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisSecurityRoomAreaVariants[4] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisSecurityRoomVariant1Placements, .resources = _gAkropolisSecurityRoomActorResources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
