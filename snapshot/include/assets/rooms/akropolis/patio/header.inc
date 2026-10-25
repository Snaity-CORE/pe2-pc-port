/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisPatioRoomCollisionResources[3] = {
    { .grid = &_gAcropolisPatioCollisionGrid, .viewBoundaryTriggers = _gAcropolisPatioViewBoundaryTriggers, .actionTriggers = _gAcropolisPatioRoom1ActionTriggers, .occluders = _gAcropolisPatioOccluders },
    { .grid = &_gAcropolisPatioCollisionGrid, .viewBoundaryTriggers = _gAcropolisPatioViewBoundaryTriggers, .actionTriggers = _gAcropolisPatioRooms2And3ActionTriggers, .occluders = _gAcropolisPatioOccluders },
    { .grid = &_gAcropolisPatioCollisionGrid, .viewBoundaryTriggers = _gAcropolisPatioViewBoundaryTriggers, .actionTriggers = _gAcropolisPatioRooms2And3ActionTriggers, .occluders = _gAcropolisPatioOccluders },
};
static u8 _gAcropolisPatioRoom2ViewMap[20] = {
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};
static u8 _gAcropolisPatioRoom3ViewMap[20] = {
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};
u8* gAcropolisPatioViewMaps[3] = {
    (void*)&gViewIdentityMap,
    _gAcropolisPatioRoom2ViewMap,
    _gAcropolisPatioRoom3ViewMap,
};
ViewCount gAcropolisPatioViewCounts[3] = {
    0,
    0,
    0,
};
WorldCoordRoomLighting gAcropolisPatioRoomLightingTable[3] = {
    { .lights = &_gAcropolisPatioRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisPatioRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisPatioRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisPatioWarpEntries[4] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
