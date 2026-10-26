/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisPlazaDefaultFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisPlazaClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisPlazaDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPlazaDefaultFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisPlazaClass1SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPlazaClass1FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisPlazaSurfacePropertiesByClass[8] = {
    &_gAkropolisPlazaDefaultSurfaceProperties,
    &_gAkropolisPlazaClass1SurfaceProperties,
    &_gAkropolisPlazaDefaultSurfaceProperties,
    &_gAkropolisPlazaDefaultSurfaceProperties,
    &_gAkropolisPlazaDefaultSurfaceProperties,
    &_gAkropolisPlazaDefaultSurfaceProperties,
    &_gAkropolisPlazaDefaultSurfaceProperties,
    &_gAkropolisPlazaDefaultSurfaceProperties,
};
