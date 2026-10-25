/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisForkedRoadRoomCollisionResources[3] = {
    { .grid = &_gAcropolisForkedRoadCollisionGrid, .viewBoundaryTriggers = _gAcropolisForkedRoadViewBoundaryTriggers, .actionTriggers = _gAcropolisForkedRoadActionTriggers, .occluders = NULL },
    { .grid = &_gAcropolisForkedRoadCollisionGrid, .viewBoundaryTriggers = _gAcropolisForkedRoadViewBoundaryTriggers, .actionTriggers = _gAcropolisForkedRoadActionTriggers, .occluders = NULL },
    { .grid = &_gAcropolisForkedRoadCollisionGrid, .viewBoundaryTriggers = _gAcropolisForkedRoadViewBoundaryTriggers, .actionTriggers = _gAcropolisForkedRoadActionTriggers, .occluders = NULL },
};
static u8 _gAcropolisForkedRoadRoom2ViewMap[12] = {
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
static u8 _gAcropolisForkedRoadRoom3ViewMap[12] = {
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
u8* gAcropolisForkedRoadViewMaps[3] = {
    (void*)&gViewIdentityMap,
    _gAcropolisForkedRoadRoom2ViewMap,
    _gAcropolisForkedRoadRoom3ViewMap,
};
ViewCount gAcropolisForkedRoadViewCounts[3] = {
    0,
    0,
    0,
};
WorldCoordRoomLighting gAcropolisForkedRoadRoomLightingTable[3] = {
    { .lights = &_gAcropolisForkedRoadRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisForkedRoadRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisForkedRoadRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisForkedRoadWarpEntries[5] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
