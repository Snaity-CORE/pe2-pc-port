/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisSecurityRoomFloorFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisSecurityRoomSilentSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisSecurityRoomFloorSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisSecurityRoomFloorFootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisSecurityRoomSurfacePropertiesByClass[8] = {
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
    &_gAcropolisSecurityRoomFloorSurfaceProperties,
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
    &_gAcropolisSecurityRoomSilentSurfaceProperties,
};
