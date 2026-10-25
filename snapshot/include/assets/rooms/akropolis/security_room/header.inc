/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisSecurityRoomRoomCollisionResources[1] = {
    { .grid = &_gAcropolisSecurityRoomCollisionGrid, .viewBoundaryTriggers = _gAcropolisSecurityRoomViewBoundaryTriggers, .actionTriggers = _gAcropolisSecurityRoomActionTriggers, .occluders = NULL },
};
u8* gAcropolisSecurityRoomViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gAcropolisSecurityRoomViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAcropolisSecurityRoomRoomLightingTable[1] = {
    { .lights = &_gAcropolisSecurityRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisSecurityRoomWarpEntries[1] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
