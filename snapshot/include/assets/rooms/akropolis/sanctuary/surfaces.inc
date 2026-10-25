/* rooms/akropolis/sanctuary/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisSanctuarySurfaceClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisSanctuarySurfaceClass2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisSanctuaryDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisSanctuarySurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisSanctuarySurfaceClass1FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisSanctuarySurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisSanctuarySurfaceClass2FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisSanctuarySurfacePropertiesByClass[8] = {
    &_gAcropolisSanctuaryDefaultSurfaceProperties,
    &_gAcropolisSanctuarySurfaceClass1Properties,
    &_gAcropolisSanctuarySurfaceClass2Properties,
    &_gAcropolisSanctuaryDefaultSurfaceProperties,
    &_gAcropolisSanctuaryDefaultSurfaceProperties,
    &_gAcropolisSanctuaryDefaultSurfaceProperties,
    &_gAcropolisSanctuaryDefaultSurfaceProperties,
    &_gAcropolisSanctuaryDefaultSurfaceProperties,
};
