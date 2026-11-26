/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/east_elevator_hall_swat_member.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisEastElevatorHallAreaResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gEastElevatorHallSwatMemberViewFigureTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisEastElevatorHallAreaVariants[3] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageEastElevatorHallVariant1Placements, .resources = _gAkropolisEastElevatorHallAreaResources },
    { .placements = NULL, .resources = NULL },
};
