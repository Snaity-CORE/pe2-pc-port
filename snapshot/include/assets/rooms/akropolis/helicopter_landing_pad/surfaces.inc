/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisHelicopterLandingPadFloorFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAcropolisHelicopterLandingPadStairFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisHelicopterLandingPadSilentSolidSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisHelicopterLandingPadStairSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisHelicopterLandingPadStairFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisHelicopterLandingPadFloorSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisHelicopterLandingPadFloorFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisHelicopterLandingPadPerimeterSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisHelicopterLandingPadFloorFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisHelicopterLandingPadSurface5Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisHelicopterLandingPadFloorFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisHelicopterLandingPadOuterBoundarySurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAcropolisHelicopterLandingPadSurfacePropertiesByClass[8] = {
    &_gAcropolisHelicopterLandingPadSilentSolidSurface,
    &_gAcropolisHelicopterLandingPadSilentSolidSurface,
    &_gAcropolisHelicopterLandingPadStairSurface,
    &_gAcropolisHelicopterLandingPadFloorSurface,
    &_gAcropolisHelicopterLandingPadPerimeterSurface,
    &_gAcropolisHelicopterLandingPadSurface5Properties,
    &_gAcropolisHelicopterLandingPadOuterBoundarySurface,
    &_gAcropolisHelicopterLandingPadSilentSolidSurface,
};
