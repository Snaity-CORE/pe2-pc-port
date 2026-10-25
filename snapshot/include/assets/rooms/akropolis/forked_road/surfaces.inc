/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisForkedRoadSurfaceClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisForkedRoadDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisForkedRoadSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisForkedRoadSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisForkedRoadSurfaceClass4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisForkedRoadSurfaceClass4FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisForkedRoadSurfacePropertiesByClass[8] = {
    &_gAcropolisForkedRoadDefaultSurfaceProperties,
    &_gAcropolisForkedRoadDefaultSurfaceProperties,
    &_gAcropolisForkedRoadSurfaceClass2Properties,
    &_gAcropolisForkedRoadSurfaceClass3Properties,
    &_gAcropolisForkedRoadSurfaceClass4Properties,
    &_gAcropolisForkedRoadDefaultSurfaceProperties,
    &_gAcropolisForkedRoadDefaultSurfaceProperties,
    &_gAcropolisForkedRoadDefaultSurfaceProperties,
};
