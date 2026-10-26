/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisHelicopterLandingPadCollisionRoomResources[1] = {
    { .grid = &_gAkropolisHelicopterLandingPadCollisionGrid, .viewBoundaryTriggers = _gAkropolisHelicopterLandingPadViewBoundaryTriggers, .actionTriggers = gAkropolisHelicopterLandingPadActionTriggers, .occluders = gAkropolisHelicopterLandingPadSightOccluders },
};
static u8 _gAkropolisHelicopterLandingPadViewMap[28] = {
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
u8* gAkropolisHelicopterLandingPadViewMaps[1] = {
    _gAkropolisHelicopterLandingPadViewMap,
};
ViewCount gAkropolisHelicopterLandingPadViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAkropolisHelicopterLandingPadRoomLightingTable[1] = {
    { .lights = &gAkropolisHelicopterLandingPadRoomLights, .ambientTable = NULL },
};
DirectionWarpEntry gAkropolisHelicopterLandingPadWarps[3] = {
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = 0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
