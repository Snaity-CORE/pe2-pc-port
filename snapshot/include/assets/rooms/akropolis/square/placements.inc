/* rooms/akropolis/square/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/square_grinning_stranger.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisSquareVariant1Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gSquareGrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisSquareAreaVariants[3] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageSquareVariant1Placements, .resources = _gAkropolisSquareVariant1Resources },
    { .placements = NULL, .resources = NULL },
};
