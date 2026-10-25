/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static AreaResource _gAcropolisFireEscapeActorResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = (void*)0 },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAcropolisFireEscapeAreaVariants[5] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisFireEscapePlacements, .resources = _gAcropolisFireEscapeActorResources },
    { .placements = (void*)&gMapAkropolisFireEscapePlacements, .resources = _gAcropolisFireEscapeActorResources },
    { .placements = (void*)&gMapAkropolisFireEscapePlacements, .resources = _gAcropolisFireEscapeActorResources },
    { .placements = NULL, .resources = NULL },
};
