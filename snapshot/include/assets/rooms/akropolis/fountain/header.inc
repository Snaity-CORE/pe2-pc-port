/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisFountainRoomCollisionResources[2] = {
    { .grid = &_gAkropolisFountainCollisionGrid, .viewBoundaryTriggers = _gAkropolisFountainViewBoundaryTriggers, .actionTriggers = _gAkropolisFountainActionTriggers, .occluders = NULL },
    { .grid = &_gAkropolisFountainCollisionGrid, .viewBoundaryTriggers = _gAkropolisFountainViewBoundaryTriggers, .actionTriggers = _gAkropolisFountainActionTriggers, .occluders = NULL },
};
static u8 _gAkropolisFountainRoom2ViewMap[24] = {
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
u8* gAkropolisFountainViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAkropolisFountainRoom2ViewMap,
};
ViewCount gAkropolisFountainViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAkropolisFountainRoomLightingTable[2] = {
    { .lights = &_gAkropolisFountainRoomLights, .ambientTable = NULL },
    { .lights = &_gAkropolisFountainRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAkropolisFountainWarpEntries[5] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
