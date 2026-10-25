/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static AreaResource _gAcropolisBridgeDefaultAreaResources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = (void*)&_gAcropolisBridgeEnemyTaskDescStorage },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAcropolisBridgeVariant2AreaResources[1] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAcropolisBridgeVariant3AreaResources[1] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAcropolisBridgePlacementVariants[12] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisBridgeDefaultPlacements, .resources = _gAcropolisBridgeDefaultAreaResources },
    { .placements = (void*)&gMapAkropolisBridgeVariant2Placements, .resources = _gAcropolisBridgeVariant2AreaResources },
    { .placements = (void*)&gMapAkropolisBridgeVariant3Placements, .resources = _gAcropolisBridgeVariant3AreaResources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
