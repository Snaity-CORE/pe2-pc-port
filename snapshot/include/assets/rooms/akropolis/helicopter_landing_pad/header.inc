/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

WorldCollisionRoomResources gAcropolisHelicopterLandingPadCollisionRoomResources[1] = {
    { .grid = &_gAcropolisHelicopterLandingPadCollisionGrid, .viewBoundaryTriggers = _gAcropolisHelicopterLandingPadViewBoundaryTriggers, .actionTriggers = gAcropolisHelicopterLandingPadActionTriggers, .occluders = gAcropolisHelicopterLandingPadSightOccluders },
};
static u8 _gAcropolisHelicopterLandingPadViewMap[28] = {
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
    0,
    0,
    0,
    0,
};
u8* gAcropolisHelicopterLandingPadViewMaps[1] = {
    _gAcropolisHelicopterLandingPadViewMap,
};
ViewCount gAcropolisHelicopterLandingPadViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAcropolisHelicopterLandingPadRoomLightingTable[1] = {
    { .lights = &gAcropolisHelicopterLandingPadRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAcropolisHelicopterLandingPadWarps[3] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
