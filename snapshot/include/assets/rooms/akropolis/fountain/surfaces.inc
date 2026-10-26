/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisFountainClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisFountainClass5FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisFountainDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisFountainClass2SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisFountainClass4SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFountainClass4FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisFountainClass5SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFountainClass5FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisFountainSurfacePropertiesByClass[8] = {
    &_gAkropolisFountainDefaultSurfaceProperties,
    &_gAkropolisFountainDefaultSurfaceProperties,
    &_gAkropolisFountainClass2SurfaceProperties,
    &_gAkropolisFountainDefaultSurfaceProperties,
    &_gAkropolisFountainClass4SurfaceProperties,
    &_gAkropolisFountainClass5SurfaceProperties,
    &_gAkropolisFountainDefaultSurfaceProperties,
    &_gAkropolisFountainDefaultSurfaceProperties,
};
