/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

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
