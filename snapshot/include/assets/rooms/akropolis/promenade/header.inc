/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisPromenadeRoomCollisionResources[2] = {
    { .grid = &_gAcropolisPromenadeRoom1CollisionGrid, .viewBoundaryTriggers = _gAcropolisPromenadeViewBoundaryTriggers, .actionTriggers = _gAcropolisPromenadeActionTriggers, .occluders = NULL },
    { .grid = &_gAcropolisPromenadeRoom2CollisionGrid, .viewBoundaryTriggers = _gAcropolisPromenadeViewBoundaryTriggers, .actionTriggers = _gAcropolisPromenadeActionTriggers, .occluders = NULL },
};
static u8 _gAcropolisPromenadeRoom2ViewMap[16] = {
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
u8* gAcropolisPromenadeViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAcropolisPromenadeRoom2ViewMap,
};
ViewCount gAcropolisPromenadeViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAcropolisPromenadeRoomLightingTable[2] = {
    { .lights = &_gAcropolisPromenadeRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisPromenadeRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisPromenadeWarpEntries[5] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
