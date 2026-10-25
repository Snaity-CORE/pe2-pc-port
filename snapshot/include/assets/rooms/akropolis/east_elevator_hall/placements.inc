/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static AreaResource _gAcropolisEastElevatorHallAreaResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = (void*)0 },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAcropolisEastElevatorHallAreaVariants[3] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisEastElevatorHallVariant1Placements, .resources = _gAcropolisEastElevatorHallAreaResources },
    { .placements = NULL, .resources = NULL },
};
