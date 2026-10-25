/* rooms/akropolis/roof_garden/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisRoofGardenFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisRoofGardenSilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisRoofGardenSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisRoofGardenFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisRoofGardenSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisRoofGardenFootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisRoofGardenSurfacePropertiesByClass[8] = {
    &_gAcropolisRoofGardenSilentSurfaceProperties,
    &_gAcropolisRoofGardenSurfaceClass1Properties,
    &_gAcropolisRoofGardenSurfaceClass2Properties,
    &_gAcropolisRoofGardenSilentSurfaceProperties,
    &_gAcropolisRoofGardenSilentSurfaceProperties,
    &_gAcropolisRoofGardenSilentSurfaceProperties,
    &_gAcropolisRoofGardenSilentSurfaceProperties,
    &_gAcropolisRoofGardenSilentSurfaceProperties,
};
