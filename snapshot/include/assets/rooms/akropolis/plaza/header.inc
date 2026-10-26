/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

#define ROOM_IS_gAkropolisPlazaRoomCollisionResources 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
WorldCollisionRoomResources gAkropolisPlazaRoomCollisionResources[1] = {
    { .grid = &_gAkropolisPlazaCollisionGrid, .viewBoundaryTriggers = NULL, .actionTriggers = _gAkropolisPlazaHelicopterWreckTrigger, .occluders = NULL },
};
u8* gAkropolisPlazaViewMaps[1] = {
    (void*)&gViewIdentityMap,
};
ViewCount gAkropolisPlazaViewCounts[1] = {
    0,
};
WorldCoordRoomLighting gAkropolisPlazaRoomLightingTable[1] = {
    { .lights = &_gAkropolisPlazaRoomLights, .ambientTable = NULL },
};
#endif
#undef ROOM_IS_gAkropolisPlazaRoomCollisionResources

#define ROOM_IS_gAkropolisPlazaWarpEntries 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
DirectionWarpEntry gAkropolisPlazaWarpEntries[1] = {
    { .player = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_10 = { 0, 0, 0, 0 }, .companion = { .yaw = { .word = 0 }, .x = 0, .y = 0, .z = 0 }, .unknown_24 = { 0, 0, 0, 0 }, .arrivalSound = 0, .departureSound = 0, .blockedSound = 0, .initialView = 0, .flags = 0, .mapFlagId = 0 },
};
#endif
#undef ROOM_IS_gAkropolisPlazaWarpEntries
#undef ROOM_PART
