/* rooms/akropolis/square/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAkropolisSquareFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisSquareDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisSquareClass4SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisSquareFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisSquareSurfacePropertiesByClass[8] = {
    &_gAkropolisSquareDefaultSurfaceProperties,
    &_gAkropolisSquareDefaultSurfaceProperties,
    &_gAkropolisSquareDefaultSurfaceProperties,
    &_gAkropolisSquareDefaultSurfaceProperties,
    &_gAkropolisSquareClass4SurfaceProperties,
    &_gAkropolisSquareDefaultSurfaceProperties,
    &_gAkropolisSquareDefaultSurfaceProperties,
    &_gAkropolisSquareDefaultSurfaceProperties,
};
