/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisPlazaDefaultFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisPlazaClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisPlazaDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPlazaDefaultFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisPlazaClass1SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPlazaClass1FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisPlazaSurfacePropertiesByClass[8] = {
    &_gAcropolisPlazaDefaultSurfaceProperties,
    &_gAcropolisPlazaClass1SurfaceProperties,
    &_gAcropolisPlazaDefaultSurfaceProperties,
    &_gAcropolisPlazaDefaultSurfaceProperties,
    &_gAcropolisPlazaDefaultSurfaceProperties,
    &_gAcropolisPlazaDefaultSurfaceProperties,
    &_gAcropolisPlazaDefaultSurfaceProperties,
    &_gAcropolisPlazaDefaultSurfaceProperties,
};
