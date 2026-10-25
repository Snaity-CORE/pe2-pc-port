/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisEastElevatorHallRoomCollisionResources[1] = {
    { .grid = &_gAcropolisEastElevatorHallCollisionGrid, .viewBoundaryTriggers = _gAcropolisEastElevatorHallViewBoundaryTriggers, .actionTriggers = _gAcropolisEastElevatorHallActionTriggers, .occluders = NULL },
};
u8* gAcropolisEastElevatorHallViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gAcropolisEastElevatorHallViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAcropolisEastElevatorHallRoomLightingTable[1] = {
    { .lights = &_gAcropolisEastElevatorHallRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisEastElevatorHallWarpEntries[1] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
