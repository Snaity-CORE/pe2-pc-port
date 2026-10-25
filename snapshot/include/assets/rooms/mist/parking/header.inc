/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gMistParkingRoomCollisionResources[4] = {
    { .grid = &gMistParkingCollisionGrid, .viewBoundaryTriggers = _gMistParkingViewBoundaryTriggers, .actionTriggers = _gMistParkingEarlyChapterActionTriggers, .occluders = NULL },
    { .grid = &gMistParkingCollisionGrid, .viewBoundaryTriggers = _gMistParkingViewBoundaryTriggers, .actionTriggers = _gMistParkingEarlyChapterActionTriggers, .occluders = NULL },
    { .grid = &gMistParkingCollisionGrid, .viewBoundaryTriggers = _gMistParkingViewBoundaryTriggers, .actionTriggers = _gMistParkingLaterChapterActionTriggers, .occluders = NULL },
    { .grid = &gMistParkingCollisionGrid, .viewBoundaryTriggers = _gMistParkingViewBoundaryTriggers, .actionTriggers = _gMistParkingLaterChapterActionTriggers, .occluders = NULL },
};
static u8 _gMistParkingEarlyChapterViewMap[20] = {
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
u8* gMistParkingViewMaps[4] = {
    _gMistParkingEarlyChapterViewMap,
    _gMistParkingEarlyChapterViewMap,
    (void*)&gViewIdentityMap,
    (void*)&gViewIdentityMap,
};
ViewCount gMistParkingViewCounts[4] = {
    0,
    0,
    0,
    0,
};
WorldCoordRoomLighting gMistParkingRoomLightingTable[4] = {
    { .lights = &_gMistParkingRoom1Lights, .ambientTable = NULL },
    { .lights = &_gMistParkingRoom2Lights, .ambientTable = _gMistParkingRoom2AmbientByView },
    { .lights = &_gMistParkingRoom1Lights, .ambientTable = NULL },
    { .lights = &_gMistParkingRoom2Lights, .ambientTable = _gMistParkingRoom2AmbientByView },
};
DirectionWarpEntry gMistParkingWarpEntries[4] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
