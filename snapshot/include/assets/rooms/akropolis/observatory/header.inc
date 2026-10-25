/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisObservatoryRoomCollisionResources[2] = {
    { .grid = &_gAcropolisObservatoryCollisionGrid, .viewBoundaryTriggers = _gAcropolisObservatoryViewBoundaryTriggers, .actionTriggers = _gAcropolisObservatoryActionTriggers, .occluders = NULL },
    { .grid = &_gAcropolisObservatoryCollisionGrid, .viewBoundaryTriggers = _gAcropolisObservatoryViewBoundaryTriggers, .actionTriggers = _gAcropolisObservatoryActionTriggers, .occluders = NULL },
};
static u8 _gAcropolisObservatoryRoom2ViewMap[8] = {
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};
u8* gAcropolisObservatoryViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAcropolisObservatoryRoom2ViewMap,
};
ViewCount gAcropolisObservatoryViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAcropolisObservatoryRoomLightingTable[2] = {
    { .lights = &_gAcropolisObservatoryRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisObservatoryRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisObservatoryWarpEntries[4] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
