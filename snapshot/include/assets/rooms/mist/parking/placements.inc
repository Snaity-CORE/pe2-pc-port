/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "actors/actor_213100.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/parking_pierce.h"

static AreaResource _gMistParkingActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gParkingPierceTaskDescs },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor213100JodieBouquetTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gMistParkingAreaVariants[13] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisMistParkingPlacements, .resources = _gMistParkingActorResources },
    { .placements = (void*)&gMapAkropolisMistParkingPlacements, .resources = _gMistParkingActorResources },
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
