/* rooms/akropolis/roof_garden/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisRoofGardenFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisRoofGardenSilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisRoofGardenSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisRoofGardenFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisRoofGardenSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisRoofGardenFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisRoofGardenSurfacePropertiesByClass[8] = {
    &_gAkropolisRoofGardenSilentSurfaceProperties,
    &_gAkropolisRoofGardenSurfaceClass1Properties,
    &_gAkropolisRoofGardenSurfaceClass2Properties,
    &_gAkropolisRoofGardenSilentSurfaceProperties,
    &_gAkropolisRoofGardenSilentSurfaceProperties,
    &_gAkropolisRoofGardenSilentSurfaceProperties,
    &_gAkropolisRoofGardenSilentSurfaceProperties,
    &_gAkropolisRoofGardenSilentSurfaceProperties,
};
