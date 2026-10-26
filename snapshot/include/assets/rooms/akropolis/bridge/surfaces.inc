/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

#define ROOM_IS__gAkropolisBridgeSurface2FootstepSounds 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static WorldCollisionFootstepSounds _gAkropolisBridgeSurface2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisBridgeSurface4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
#endif
#undef ROOM_IS__gAkropolisBridgeSurface2FootstepSounds

#define ROOM_IS__gAkropolisBridgeBlockingSurfaceProperties 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static WorldCollisionSurfaceProperties _gAkropolisBridgeBlockingSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisBridgeSurface2FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisBridgeSurface4FootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface7Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAkropolisBridgeSurfacePropertiesByClass[8] = {
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeSurface1Properties,
    &_gAkropolisBridgeSurface2Properties,
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeSurface4Properties,
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeBlockingSurfaceProperties,
    &_gAkropolisBridgeSurface7Properties,
};
#endif
#undef ROOM_IS__gAkropolisBridgeBlockingSurfaceProperties
#undef ROOM_PART
