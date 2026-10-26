/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass3FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass7FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisPromenadeDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPromenadeSurfaceClass1FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPromenadeSurfaceClass2FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPromenadeSurfaceClass3FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPromenadeSurfaceClass4FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass7Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisPromenadeSurfaceClass7FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisPromenadeSurfacePropertiesByClass[8] = {
    &_gAkropolisPromenadeDefaultSurfaceProperties,
    &_gAkropolisPromenadeSurfaceClass1Properties,
    &_gAkropolisPromenadeSurfaceClass2Properties,
    &_gAkropolisPromenadeSurfaceClass3Properties,
    &_gAkropolisPromenadeSurfaceClass4Properties,
    &_gAkropolisPromenadeDefaultSurfaceProperties,
    &_gAkropolisPromenadeDefaultSurfaceProperties,
    &_gAkropolisPromenadeSurfaceClass7Properties,
};
