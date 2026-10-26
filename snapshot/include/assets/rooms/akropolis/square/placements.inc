/* rooms/akropolis/square/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/square_grinning_stranger.h"

static AreaResource _gAkropolisSquareVariant1Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gSquareGrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisSquareAreaVariants[3] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisSquareVariant1Placements, .resources = _gAkropolisSquareVariant1Resources },
    { .placements = NULL, .resources = NULL },
};
