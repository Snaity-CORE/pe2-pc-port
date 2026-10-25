/* rooms/akropolis/sanctuary/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisSanctuaryRoomCollisionResources[1] = {
    { .grid = &_gAcropolisSanctuaryCollisionGrid, .viewBoundaryTriggers = _gAcropolisSanctuaryViewBoundaryTriggers, .actionTriggers = _gAcropolisSanctuaryActionTriggers, .occluders = NULL },
};
u8* gAcropolisSanctuaryViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gAcropolisSanctuaryViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAcropolisSanctuaryRoomLightingTable[1] = {
    { .lights = &_gAcropolisSanctuaryRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisSanctuaryWarpEntries[3] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
