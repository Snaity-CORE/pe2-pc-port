/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisPatioClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisPatioDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisPatioClass1SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisPatioClass2SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisPatioClass3SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisPatioClass4SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPatioClass4FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisPatioSurfacePropertiesByClass[9] = {
    &_gAcropolisPatioDefaultSurfaceProperties,
    &_gAcropolisPatioClass1SurfaceProperties,
    &_gAcropolisPatioClass2SurfaceProperties,
    &_gAcropolisPatioClass3SurfaceProperties,
    &_gAcropolisPatioClass4SurfaceProperties,
    &_gAcropolisPatioDefaultSurfaceProperties,
    &_gAcropolisPatioDefaultSurfaceProperties,
    &_gAcropolisPatioDefaultSurfaceProperties,
    NULL,
};
