/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisPatioRoomCollisionResources[3] = {
    { .grid = &_gAkropolisPatioCollisionGrid, .viewBoundaryTriggers = _gAkropolisPatioViewBoundaryTriggers, .actionTriggers = _gAkropolisPatioRoom1ActionTriggers, .occluders = _gAkropolisPatioOccluders },
    { .grid = &_gAkropolisPatioCollisionGrid, .viewBoundaryTriggers = _gAkropolisPatioViewBoundaryTriggers, .actionTriggers = _gAkropolisPatioRooms2And3ActionTriggers, .occluders = _gAkropolisPatioOccluders },
    { .grid = &_gAkropolisPatioCollisionGrid, .viewBoundaryTriggers = _gAkropolisPatioViewBoundaryTriggers, .actionTriggers = _gAkropolisPatioRooms2And3ActionTriggers, .occluders = _gAkropolisPatioOccluders },
};
static u8 _gAkropolisPatioRoom2ViewMap[20] = {
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
static u8 _gAkropolisPatioRoom3ViewMap[20] = {
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
u8* gAkropolisPatioViewMaps[3] = {
    (void*)&gViewIdentityMap,
    _gAkropolisPatioRoom2ViewMap,
    _gAkropolisPatioRoom3ViewMap,
};
ViewCount gAkropolisPatioViewCounts[3] = {
    0,
    0,
    0,
};
WorldCoordRoomLighting gAkropolisPatioRoomLightingTable[3] = {
    { .lights = &_gAkropolisPatioRoomLights, .ambientTable = NULL },
    { .lights = &_gAkropolisPatioRoomLights, .ambientTable = NULL },
    { .lights = &_gAkropolisPatioRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAkropolisPatioWarpEntries[4] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
