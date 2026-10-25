/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS_gAcropolisPlazaRoomCollisionResources 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
WorldCollisionRoomResources gAcropolisPlazaRoomCollisionResources[1] = {
    { .grid = &_gAcropolisPlazaCollisionGrid, .viewBoundaryTriggers = NULL, .actionTriggers = _gAcropolisPlazaStreamSceneTrigger, .occluders = NULL },
};
u8* gAcropolisPlazaViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gAcropolisPlazaViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAcropolisPlazaRoomLightingTable[1] = {
    { .lights = &_gAcropolisPlazaRoomLights, .ambientTable = NULL },
};
#endif
#undef ROOM_IS_gAcropolisPlazaRoomCollisionResources

#define ROOM_IS_gAcropolisPlazaWarpEntries 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
DirectionWarpEntry gAcropolisPlazaWarpEntries[1] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
#endif
#undef ROOM_IS_gAcropolisPlazaWarpEntries
#undef ROOM_PART
