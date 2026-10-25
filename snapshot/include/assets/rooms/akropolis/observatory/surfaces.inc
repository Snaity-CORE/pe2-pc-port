/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisObservatoryFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisObservatorySilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisObservatoryFootstepSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisObservatoryFootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisObservatorySurfacePropertiesByClass[8] = {
    &_gAcropolisObservatorySilentSurfaceProperties,
    &_gAcropolisObservatoryFootstepSurfaceProperties,
    &_gAcropolisObservatorySilentSurfaceProperties,
    &_gAcropolisObservatorySilentSurfaceProperties,
    &_gAcropolisObservatorySilentSurfaceProperties,
    &_gAcropolisObservatorySilentSurfaceProperties,
    &_gAcropolisObservatorySilentSurfaceProperties,
    &_gAcropolisObservatorySilentSurfaceProperties,
};
