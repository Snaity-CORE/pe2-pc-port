/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/room.h"
#include "gameplay/view.h"

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
