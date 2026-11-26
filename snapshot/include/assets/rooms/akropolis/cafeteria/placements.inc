/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "actors/actor_01200.h"
#include "actors/actor_05700.h"
#include "actors/actor_202900.h"
#include "actors/actor_210600.h"
#include "actors/actor_310600.h"
#include "actors/actor_401800.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/cafeteria_boss_stranger.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisCafeteriaOpeningSceneResources[4] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gCafeteriaBossStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor202900WomanTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor310600RupertBroderickTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisCafeteriaBattleSceneResources[4] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gCafeteriaBossStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor210600GrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor310600RupertBroderickTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisCafeteriaSingleOddStrangerResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisCafeteriaPairOddStrangerResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisCafeteriaBoneSucklerResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101200BoneSucklerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisCafeteriaRookGolemResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor05700RookGolemTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisCafeteriaAreaVariants[11] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageCafeteriaVariant1Placements, .resources = _gAkropolisCafeteriaOpeningSceneResources },
    { .placements = gAkropolisStageCafeteriaVariant2Placements, .resources = _gAkropolisCafeteriaBattleSceneResources },
    { .placements = gAkropolisStageCafeteriaVariant3Placements, .resources = _gAkropolisCafeteriaSingleOddStrangerResources },
    { .placements = gAkropolisStageCafeteriaVariant4Placements, .resources = _gAkropolisCafeteriaPairOddStrangerResources },
    { .placements = gAkropolisStageCafeteriaVariant5Placements, .resources = _gAkropolisCafeteriaBoneSucklerResources },
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageCafeteriaVariant7Placements, .resources = _gAkropolisCafeteriaRookGolemResources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
