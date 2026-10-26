/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

WorldCollisionRoomResources gAkropolisCafeteriaCollisionRooms[4] = {
    { .grid = &_gAkropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAkropolisCafeteriaRooms1To3ViewBoundaries, .actionTriggers = _gAkropolisCafeteriaRoom1ActionTriggers, .occluders = _gAkropolisCafeteriaSightOccluders },
    { .grid = &_gAkropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAkropolisCafeteriaRooms1To3ViewBoundaries, .actionTriggers = _gAkropolisCafeteriaRooms2To4ActionTriggers, .occluders = _gAkropolisCafeteriaSightOccluders },
    { .grid = &_gAkropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAkropolisCafeteriaRooms1To3ViewBoundaries, .actionTriggers = _gAkropolisCafeteriaRooms2To4ActionTriggers, .occluders = _gAkropolisCafeteriaSightOccluders },
    { .grid = &_gAkropolisCafeteriaCollisionGrid, .viewBoundaryTriggers = _gAkropolisCafeteriaRoom4ViewBoundaries, .actionTriggers = _gAkropolisCafeteriaRooms2To4ActionTriggers, .occluders = _gAkropolisCafeteriaSightOccluders },
};
static u8 _gAkropolisCafeteriaRooms2And3ViewMap[24] = {
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
static u8 _gAkropolisCafeteriaRoom4ViewMap[24] = {
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
u8* gAkropolisCafeteriaViewMaps[4] = {
    (void*)&gViewIdentityMap,
    _gAkropolisCafeteriaRooms2And3ViewMap,
    _gAkropolisCafeteriaRooms2And3ViewMap,
    _gAkropolisCafeteriaRoom4ViewMap,
};
ViewCount gAkropolisCafeteriaViewCounts[4] = {
    0,
    0,
    0,
    0,
};
WorldCoordRoomLighting gAkropolisCafeteriaRoomLightingTable[4] = {
    { .lights = &gAkropolisCafeteriaRoomLights, .ambientTable = gAkropolisCafeteriaViewAmbientColors },
    { .lights = &gAkropolisCafeteriaRoomLights, .ambientTable = gAkropolisCafeteriaViewAmbientColors },
    { .lights = &gAkropolisCafeteriaRoomLights, .ambientTable = gAkropolisCafeteriaViewAmbientColors },
    { .lights = &gAkropolisCafeteriaRoomLights, .ambientTable = gAkropolisCafeteriaViewAmbientColors },
};
DirectionWarpEntry gAkropolisCafeteriaWarps[3] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
    { .player = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = -0, .y = -0, .z = -0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
