/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "actors/actor_00700.h"
#include "actors/actor_01900.h"
#include "actors/actor_300700.h"
#include "actors/actor_401000.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/patio_leaping_strangers.h"
#include "stages/akropolis/actors/patio_swat_member.h"

static AreaResource _gAkropolisPatioVariant1ActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101900GrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gPatioSwatMemberTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisPatioVariant2ActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gPatioLeapingStrangersTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101900GrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisPatioVariant3ActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor00700RatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor00700MothTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisPatioVariant4ActorResources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor300700MothTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401000StrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisPatioVariant5ActorResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401000StrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisPatioVariant10ActorResources[1] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisPatioAreaVariants[12] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisPatioVariant1Placements, .resources = _gAkropolisPatioVariant1ActorResources },
    { .placements = (void*)&gMapAkropolisPatioVariant2Placements, .resources = _gAkropolisPatioVariant2ActorResources },
    { .placements = (void*)&gMapAkropolisPatioVariant3Placements, .resources = _gAkropolisPatioVariant3ActorResources },
    { .placements = (void*)&gMapAkropolisPatioVariant4Placements, .resources = _gAkropolisPatioVariant4ActorResources },
    { .placements = (void*)&gMapAkropolisPatioVariant5Placements, .resources = _gAkropolisPatioVariant5ActorResources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisPatioVariant10Placements, .resources = _gAkropolisPatioVariant10ActorResources },
    { .placements = NULL, .resources = NULL },
};
