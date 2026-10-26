/* rooms/akropolis/sanctuary/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "actors/actor_210700.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/sanctuary_no_9.h"

static AreaResource _gAkropolisSanctuaryNo9AndRupertResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gSanctuaryNo9TaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor210700RupertBroderickTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisSanctuaryRupertResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor210700RupertBroderickTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisSanctuaryAreaVariants[12] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisSanctuaryNo9AndRupertPlacements, .resources = _gAkropolisSanctuaryNo9AndRupertResources },
    { .placements = (void*)&gMapAkropolisSanctuaryRupertPlacements, .resources = _gAkropolisSanctuaryRupertResources },
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
