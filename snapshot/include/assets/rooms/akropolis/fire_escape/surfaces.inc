/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisFireEscapeSurfaceClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisFireEscapeSurfaceClass3FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisFireEscapeDefaultFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFireEscapeDefaultFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFireEscapeSurfaceClass1FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFireEscapeDefaultFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFireEscapeSurfaceClass3FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass6Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisFireEscapeDefaultFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisFireEscapeSurfacePropertiesByClass[8] = {
    &_gAkropolisFireEscapeDefaultSurfaceProperties,
    &_gAkropolisFireEscapeSurfaceClass1Properties,
    &_gAkropolisFireEscapeSurfaceClass2Properties,
    &_gAkropolisFireEscapeSurfaceClass3Properties,
    &_gAkropolisFireEscapeDefaultSurfaceProperties,
    &_gAkropolisFireEscapeDefaultSurfaceProperties,
    &_gAkropolisFireEscapeSurfaceClass6Properties,
    &_gAkropolisFireEscapeDefaultSurfaceProperties,
};
