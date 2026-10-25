/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gMistParkingFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gMistParkingDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gMistParkingSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gMistParkingFootstepSounds };
static WorldCollisionSurfaceProperties _gMistParkingSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gMistParkingSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gMistParkingFootstepSounds };
WorldCollisionSurfaceProperties* gMistParkingSurfacePropertiesByClass[8] = {
    &_gMistParkingDefaultSurfaceProperties,
    &_gMistParkingSurfaceClass1Properties,
    &_gMistParkingSurfaceClass2Properties,
    &_gMistParkingSurfaceClass3Properties,
    &_gMistParkingDefaultSurfaceProperties,
    &_gMistParkingDefaultSurfaceProperties,
    &_gMistParkingDefaultSurfaceProperties,
    &_gMistParkingDefaultSurfaceProperties,
};
