/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisCafeteriaSurface1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisCafeteriaSurface2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisCafeteriaSurfaces3And4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisCafeteriaSilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisCafeteriaSurface1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisCafeteriaSurface1FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisCafeteriaSurface2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisCafeteriaSurface2FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisCafeteriaSurface3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisCafeteriaSurfaces3And4FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisCafeteriaSurface4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisCafeteriaSurfaces3And4FootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisCafeteriaSurfacePropertiesByClass[8] = {
    &_gAcropolisCafeteriaSilentSurfaceProperties,
    &_gAcropolisCafeteriaSurface1Properties,
    &_gAcropolisCafeteriaSurface2Properties,
    &_gAcropolisCafeteriaSurface3Properties,
    &_gAcropolisCafeteriaSurface4Properties,
    &_gAcropolisCafeteriaSilentSurfaceProperties,
    &_gAcropolisCafeteriaSilentSurfaceProperties,
    &_gAcropolisCafeteriaSilentSurfaceProperties,
};
