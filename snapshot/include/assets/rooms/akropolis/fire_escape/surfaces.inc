/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisFireEscapeSurfaceClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisFireEscapeSurfaceClass3FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisFireEscapeDefaultFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisFireEscapeDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFireEscapeDefaultFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisFireEscapeSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFireEscapeSurfaceClass1FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisFireEscapeSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFireEscapeDefaultFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisFireEscapeSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFireEscapeSurfaceClass3FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisFireEscapeSurfaceClass6Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisFireEscapeDefaultFootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisFireEscapeSurfacePropertiesByClass[8] = {
    &_gAcropolisFireEscapeDefaultSurfaceProperties,
    &_gAcropolisFireEscapeSurfaceClass1Properties,
    &_gAcropolisFireEscapeSurfaceClass2Properties,
    &_gAcropolisFireEscapeSurfaceClass3Properties,
    &_gAcropolisFireEscapeDefaultSurfaceProperties,
    &_gAcropolisFireEscapeDefaultSurfaceProperties,
    &_gAcropolisFireEscapeSurfaceClass6Properties,
    &_gAcropolisFireEscapeDefaultSurfaceProperties,
};
