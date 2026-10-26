/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisPromenadeRoomCollisionResources[2] = {
    { .grid = &_gAkropolisPromenadeRoom1CollisionGrid, .viewBoundaryTriggers = _gAkropolisPromenadeViewBoundaryTriggers, .actionTriggers = _gAkropolisPromenadeActionTriggers, .occluders = NULL },
    { .grid = &_gAkropolisPromenadeRoom2CollisionGrid, .viewBoundaryTriggers = _gAkropolisPromenadeViewBoundaryTriggers, .actionTriggers = _gAkropolisPromenadeActionTriggers, .occluders = NULL },
};
static u8 _gAkropolisPromenadeRoom2ViewMap[16] = {
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
u8* gAkropolisPromenadeViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAkropolisPromenadeRoom2ViewMap,
};
ViewCount gAkropolisPromenadeViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAkropolisPromenadeRoomLightingTable[2] = {
    { .lights = &_gAkropolisPromenadeRoomLights, .ambientTable = NULL },
    { .lights = &_gAkropolisPromenadeRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAkropolisPromenadeWarpEntries[5] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
