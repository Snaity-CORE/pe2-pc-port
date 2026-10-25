/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gAcropolisBridgeSurface2FootstepSounds 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static WorldCollisionFootstepSounds _gAcropolisBridgeSurface2FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisBridgeSurface4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
#endif
#undef ROOM_IS__gAcropolisBridgeSurface2FootstepSounds

#define ROOM_IS__gAcropolisBridgeBlockingSurfaceProperties 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static WorldCollisionSurfaceProperties _gAcropolisBridgeBlockingSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisBridgeSurface1Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisBridgeSurface2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisBridgeSurface2FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisBridgeSurface4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisBridgeSurface4FootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisBridgeSurface7Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAcropolisBridgeSurfacePropertiesByClass[8] = {
    &_gAcropolisBridgeBlockingSurfaceProperties,
    &_gAcropolisBridgeSurface1Properties,
    &_gAcropolisBridgeSurface2Properties,
    &_gAcropolisBridgeBlockingSurfaceProperties,
    &_gAcropolisBridgeSurface4Properties,
    &_gAcropolisBridgeBlockingSurfaceProperties,
    &_gAcropolisBridgeBlockingSurfaceProperties,
    &_gAcropolisBridgeSurface7Properties,
};
#endif
#undef ROOM_IS__gAcropolisBridgeBlockingSurfaceProperties
#undef ROOM_PART
