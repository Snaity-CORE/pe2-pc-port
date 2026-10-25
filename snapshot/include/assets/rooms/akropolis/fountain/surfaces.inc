/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisFountainClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisFountainClass5FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisFountainDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisFountainClass2SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisFountainClass4SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFountainClass4FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisFountainClass5SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFountainClass5FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisFountainSurfacePropertiesByClass[8] = {
    &_gAcropolisFountainDefaultSurfaceProperties,
    &_gAcropolisFountainDefaultSurfaceProperties,
    &_gAcropolisFountainClass2SurfaceProperties,
    &_gAcropolisFountainDefaultSurfaceProperties,
    &_gAcropolisFountainClass4SurfaceProperties,
    &_gAcropolisFountainClass5SurfaceProperties,
    &_gAcropolisFountainDefaultSurfaceProperties,
    &_gAcropolisFountainDefaultSurfaceProperties,
};
