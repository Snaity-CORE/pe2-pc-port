/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gMistShootingGallerySurface1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gMistShootingGallerySurface2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gMistShootingGallerySilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gMistShootingGallerySurface1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gMistShootingGallerySurface1FootstepSounds };
static WorldCollisionSurfaceProperties _gMistShootingGallerySurface2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gMistShootingGallerySurface2FootstepSounds };
WorldCollisionSurfaceProperties* gMistShootingGallerySurfacePropertiesByClass[8] = {
    &_gMistShootingGallerySilentSurfaceProperties,
    &_gMistShootingGallerySurface1Properties,
    &_gMistShootingGallerySurface2Properties,
    &_gMistShootingGallerySilentSurfaceProperties,
    &_gMistShootingGallerySilentSurfaceProperties,
    &_gMistShootingGallerySilentSurfaceProperties,
    &_gMistShootingGallerySilentSurfaceProperties,
    &_gMistShootingGallerySilentSurfaceProperties,
};
