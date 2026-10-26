/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisCafeteriaSurface1FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisCafeteriaSurface2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisCafeteriaSurfaces3And4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisCafeteriaSilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisCafeteriaSurface1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisCafeteriaSurface1FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisCafeteriaSurface2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisCafeteriaSurface2FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisCafeteriaSurface3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisCafeteriaSurfaces3And4FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisCafeteriaSurface4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisCafeteriaSurfaces3And4FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisCafeteriaSurfacePropertiesByClass[8] = {
    &_gAkropolisCafeteriaSilentSurfaceProperties,
    &_gAkropolisCafeteriaSurface1Properties,
    &_gAkropolisCafeteriaSurface2Properties,
    &_gAkropolisCafeteriaSurface3Properties,
    &_gAkropolisCafeteriaSurface4Properties,
    &_gAkropolisCafeteriaSilentSurfaceProperties,
    &_gAkropolisCafeteriaSilentSurfaceProperties,
    &_gAkropolisCafeteriaSilentSurfaceProperties,
};
