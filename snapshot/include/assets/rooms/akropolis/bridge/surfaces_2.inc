/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionSurfaceProperties _gAkropolisBridgeBlockingSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisBridgeSurface2FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisBridgeSurface4FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface7Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAkropolisBridgeSurfacePropertiesByClass[8] = {
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeSurface1Properties,
    &_gAkropolisBridgeSurface2Properties,
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeSurface4Properties,
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeSurface7Properties,
};
