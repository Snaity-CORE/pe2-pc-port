/* rooms/akropolis/sanctuary/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisSanctuarySurfaceClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisSanctuarySurfaceClass2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisSanctuaryDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisSanctuarySurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisSanctuarySurfaceClass1FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisSanctuarySurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisSanctuarySurfaceClass2FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisSanctuarySurfacePropertiesByClass[8] = {
    &_gAkropolisSanctuaryDefaultSurfaceProperties,
    &_gAkropolisSanctuarySurfaceClass1Properties,
    &_gAkropolisSanctuarySurfaceClass2Properties,
    &_gAkropolisSanctuaryDefaultSurfaceProperties,
    &_gAkropolisSanctuaryDefaultSurfaceProperties,
    &_gAkropolisSanctuaryDefaultSurfaceProperties,
    &_gAkropolisSanctuaryDefaultSurfaceProperties,
    &_gAkropolisSanctuaryDefaultSurfaceProperties,
};
