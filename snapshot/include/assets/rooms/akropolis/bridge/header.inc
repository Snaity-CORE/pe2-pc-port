/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

#define ROOM_IS_gAkropolisBridgeRoomCollisionResources 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
WorldCollisionRoomResources gAkropolisBridgeRoomCollisionResources[2] = {
    { .grid = &_gAkropolisBridgeRoom1Collision, .viewBoundaryTriggers = _gAkropolisBridgeViewBoundaryTriggers, .actionTriggers = _gAkropolisBridgeRoom1ActionTriggers, .occluders = NULL },
    { .grid = &_gAkropolisBridgeRoom2Collision, .viewBoundaryTriggers = _gAkropolisBridgeViewBoundaryTriggers, .actionTriggers = _gAkropolisBridgeRoom2ActionTriggers, .occluders = NULL },
};
static u8 _gAkropolisBridgeRoom2ViewMap[12] = {
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
u8* gAkropolisBridgeViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAkropolisBridgeRoom2ViewMap,
};
ViewCount gAkropolisBridgeViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAkropolisBridgeRoomLightingTable[2] = {
    { .lights = &_gAkropolisBridgeRoomLights, .ambientTable = NULL },
    { .lights = &_gAkropolisBridgeRoomLights, .ambientTable = NULL },
};
#endif
#undef ROOM_IS_gAkropolisBridgeRoomCollisionResources

#define ROOM_IS_gAkropolisBridgeWarpEntries 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
DirectionWarpEntry gAkropolisBridgeWarpEntries[2] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
#endif
#undef ROOM_IS_gAkropolisBridgeWarpEntries
#undef ROOM_PART
