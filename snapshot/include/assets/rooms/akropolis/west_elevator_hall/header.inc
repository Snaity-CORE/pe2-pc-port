/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisWestElevatorHallCollisionRoomResources[1] = {
    { .grid = &_gAkropolisWestElevatorHallCollisionGrid, .viewBoundaryTriggers = _gAkropolisWestElevatorHallViewBoundaryTriggers, .actionTriggers = _gAkropolisWestElevatorHallActionTriggers, .occluders = NULL },
};
u8* gAkropolisWestElevatorHallViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gAkropolisWestElevatorHallViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAkropolisWestElevatorHallRoomLightingTable[1] = {
    { .lights = &_gAkropolisWestElevatorHallRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAkropolisWestElevatorHallWarpEntries[2] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
