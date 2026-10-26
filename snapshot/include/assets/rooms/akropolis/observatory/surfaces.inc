/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisObservatoryFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisObservatorySilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisObservatoryFootstepSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisObservatoryFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisObservatorySurfacePropertiesByClass[8] = {
    &_gAkropolisObservatorySilentSurfaceProperties,
    &_gAkropolisObservatoryFootstepSurfaceProperties,
    &_gAkropolisObservatorySilentSurfaceProperties,
    &_gAkropolisObservatorySilentSurfaceProperties,
    &_gAkropolisObservatorySilentSurfaceProperties,
    &_gAkropolisObservatorySilentSurfaceProperties,
    &_gAkropolisObservatorySilentSurfaceProperties,
    &_gAkropolisObservatorySilentSurfaceProperties,
};
