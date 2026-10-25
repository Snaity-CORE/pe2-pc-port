/* rooms/mist/r18/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gMistR18FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gMistR18SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gMistR18FootstepSounds };
WorldCollisionSurfaceProperties* gMistR18SurfacePropertiesByClass[8] = {
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
    &_gMistR18SurfaceProperties,
};
