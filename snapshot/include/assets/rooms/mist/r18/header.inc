/* rooms/mist/r18/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gMistR18CollisionRoomResources[1] = {
    { .grid = &_gMistR18FloorCollisionGrid, .viewBoundaryTriggers = NULL, .actionTriggers = NULL, .occluders = NULL },
};
u8* gMistR18ViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gMistR18ViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gMistR18RoomLightingTable[1] = {
    { .lights = &_gMistR18RoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gMistR18WarpEntries[1] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
