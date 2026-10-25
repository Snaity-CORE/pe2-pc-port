/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisCafeteriaCollisionRooms[4] = {
    { .grid = &_gAcropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAcropolisCafeteriaRooms1To3ViewBoundaries, .actionTriggers = _gAcropolisCafeteriaRoom1ActionTriggers, .occluders = _gAcropolisCafeteriaSightOccluders },
    { .grid = &_gAcropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAcropolisCafeteriaRooms1To3ViewBoundaries, .actionTriggers = _gAcropolisCafeteriaRooms2To4ActionTriggers, .occluders = _gAcropolisCafeteriaSightOccluders },
    { .grid = &_gAcropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAcropolisCafeteriaRooms1To3ViewBoundaries, .actionTriggers = _gAcropolisCafeteriaRooms2To4ActionTriggers, .occluders = _gAcropolisCafeteriaSightOccluders },
    { .grid = &_gAcropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAcropolisCafeteriaRoom4ViewBoundaries, .actionTriggers = _gAcropolisCafeteriaRooms2To4ActionTriggers, .occluders = _gAcropolisCafeteriaSightOccluders },
};
static u8 _gAcropolisCafeteriaRooms2And3ViewMap[24] = {
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
    0,
    0,
    0,
    0,
};
static u8 _gAcropolisCafeteriaRoom4ViewMap[24] = {
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
    0,
    0,
    0,
    0,
};
u8* gAcropolisCafeteriaViewMaps[4] = {
    (void*)&gViewIdentityMap,
    _gAcropolisCafeteriaRooms2And3ViewMap,
    _gAcropolisCafeteriaRooms2And3ViewMap,
    _gAcropolisCafeteriaRoom4ViewMap,
};
ViewCount gAcropolisCafeteriaViewCounts[4] = {
    0,
    0,
    0,
    0,
};
WorldCoordRoomLighting gAcropolisCafeteriaRoomLightingTable[4] = {
    { .lights = &gAcropolisCafeteriaRoomLights, .ambientTable = gAcropolisCafeteriaViewAmbientColors },
    { .lights = &gAcropolisCafeteriaRoomLights, .ambientTable = gAcropolisCafeteriaViewAmbientColors },
    { .lights = &gAcropolisCafeteriaRoomLights, .ambientTable = gAcropolisCafeteriaViewAmbientColors },
    { .lights = &gAcropolisCafeteriaRoomLights, .ambientTable = gAcropolisCafeteriaViewAmbientColors },
};
DirectionWarpEntry gAcropolisCafeteriaWarps[3] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
