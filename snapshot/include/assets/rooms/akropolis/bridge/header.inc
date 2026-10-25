/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS_gAcropolisBridgeRoomCollisionResources 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
WorldCollisionRoomResources gAcropolisBridgeRoomCollisionResources[2] = {
    { .grid = &_gAcropolisBridgeRoom1Collision, .viewBoundaryTriggers = _gAcropolisBridgeViewBoundaryTriggers, .actionTriggers = _gAcropolisBridgeRoom1ActionTriggers, .occluders = NULL },
    { .grid = &_gAcropolisBridgeRoom2Collision, .viewBoundaryTriggers = _gAcropolisBridgeViewBoundaryTriggers, .actionTriggers = _gAcropolisBridgeRoom2ActionTriggers, .occluders = NULL },
};
static u8 _gAcropolisBridgeRoom2ViewMap[12] = {
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
u8* gAcropolisBridgeViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAcropolisBridgeRoom2ViewMap,
};
ViewCount gAcropolisBridgeViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAcropolisBridgeRoomLightingTable[2] = {
    { .lights = &_gAcropolisBridgeRoomLights, .ambientTable = NULL },
    { .lights = &_gAcropolisBridgeRoomLights, .ambientTable = NULL },
};
#endif
#undef ROOM_IS_gAcropolisBridgeRoomCollisionResources

#define ROOM_IS_gAcropolisBridgeWarpEntries 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
DirectionWarpEntry gAcropolisBridgeWarpEntries[2] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
#endif
#undef ROOM_IS_gAcropolisBridgeWarpEntries
#undef ROOM_PART
