/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "actors/actor_300700.h"
#include "actors/actor_401800.h"
#include "gameplay/area.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisHallwayVariant1Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor300700RatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisHallwayVariant2Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisHallwayVariant3Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisHallwayVariant4Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor300700RatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisHallwayAreaVariants[13] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageHallwayVariant1Placements, .resources = _gAkropolisHallwayVariant1Resources },
    { .placements = gAkropolisStageHallwayVariant2Placements, .resources = _gAkropolisHallwayVariant2Resources },
    { .placements = gAkropolisStageHallwayVariant3Placements, .resources = _gAkropolisHallwayVariant3Resources },
    { .placements = gAkropolisStageHallwayVariant4Placements, .resources = _gAkropolisHallwayVariant4Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
