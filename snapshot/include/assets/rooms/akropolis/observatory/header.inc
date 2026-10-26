/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisObservatoryRoomCollisionResources[2] = {
    { .grid = &_gAkropolisObservatoryCollisionGrid, .viewBoundaryTriggers = _gAkropolisObservatoryViewBoundaryTriggers, .actionTriggers = _gAkropolisObservatoryActionTriggers, .occluders = NULL },
    { .grid = &_gAkropolisObservatoryCollisionGrid, .viewBoundaryTriggers = _gAkropolisObservatoryViewBoundaryTriggers, .actionTriggers = _gAkropolisObservatoryActionTriggers, .occluders = NULL },
};
static u8 _gAkropolisObservatoryRoom2ViewMap[8] = {
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};
u8* gAkropolisObservatoryViewMaps[2] = {
    (void*)&gViewIdentityMap,
    _gAkropolisObservatoryRoom2ViewMap,
};
ViewCount gAkropolisObservatoryViewCounts[2] = {
    0,
    0,
};
WorldCoordRoomLighting gAkropolisObservatoryRoomLightingTable[2] = {
    { .lights = &_gAkropolisObservatoryRoomLights, .ambientTable = NULL },
    { .lights = &_gAkropolisObservatoryRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAkropolisObservatoryWarpEntries[4] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
