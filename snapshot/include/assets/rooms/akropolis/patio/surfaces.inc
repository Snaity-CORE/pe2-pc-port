/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisPatioClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisPatioDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisPatioClass1SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisPatioClass2SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisPatioClass3SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisPatioClass4SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPatioClass4FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisPatioSurfacePropertiesByClass[9] = {
    &_gAkropolisPatioDefaultSurfaceProperties,
    &_gAkropolisPatioClass1SurfaceProperties,
    &_gAkropolisPatioClass2SurfaceProperties,
    &_gAkropolisPatioClass3SurfaceProperties,
    &_gAkropolisPatioClass4SurfaceProperties,
    &_gAkropolisPatioDefaultSurfaceProperties,
    &_gAkropolisPatioDefaultSurfaceProperties,
    &_gAkropolisPatioDefaultSurfaceProperties,
    NULL,
};
