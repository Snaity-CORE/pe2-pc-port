/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisPromenadeSurfaceClass1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisPromenadeSurfaceClass2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisPromenadeSurfaceClass3FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisPromenadeSurfaceClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisPromenadeSurfaceClass7FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisPromenadeDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisPromenadeSurfaceClass1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPromenadeSurfaceClass1FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisPromenadeSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPromenadeSurfaceClass2FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisPromenadeSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPromenadeSurfaceClass3FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisPromenadeSurfaceClass4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPromenadeSurfaceClass4FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisPromenadeSurfaceClass7Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisPromenadeSurfaceClass7FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisPromenadeSurfacePropertiesByClass[8] = {
    &_gAcropolisPromenadeDefaultSurfaceProperties,
    &_gAcropolisPromenadeSurfaceClass1Properties,
    &_gAcropolisPromenadeSurfaceClass2Properties,
    &_gAcropolisPromenadeSurfaceClass3Properties,
    &_gAcropolisPromenadeSurfaceClass4Properties,
    &_gAcropolisPromenadeDefaultSurfaceProperties,
    &_gAcropolisPromenadeDefaultSurfaceProperties,
    &_gAcropolisPromenadeSurfaceClass7Properties,
};
