/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisSecurityRoomFloorFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisSecurityRoomSilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisSecurityRoomFloorSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisSecurityRoomFloorFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisSecurityRoomSurfacePropertiesByClass[8] = {
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
    &_gAkropolisSecurityRoomFloorSurfaceProperties,
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
    &_gAkropolisSecurityRoomSilentSurfaceProperties,
};
