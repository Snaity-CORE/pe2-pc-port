/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisHelicopterLandingPadFloorFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionFootstepSounds _gAkropolisHelicopterLandingPadStairFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisHelicopterLandingPadSilentSolidSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisHelicopterLandingPadStairSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisHelicopterLandingPadStairFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisHelicopterLandingPadFloorSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisHelicopterLandingPadFloorFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisHelicopterLandingPadPerimeterSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisHelicopterLandingPadFloorFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisHelicopterLandingPadSurface5Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisHelicopterLandingPadFloorFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisHelicopterLandingPadOuterBoundarySurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAkropolisHelicopterLandingPadSurfacePropertiesByClass[8] = {
    &_gAkropolisHelicopterLandingPadSilentSolidSurface,
    &_gAkropolisHelicopterLandingPadSilentSolidSurface,
    &_gAkropolisHelicopterLandingPadStairSurface,
    &_gAkropolisHelicopterLandingPadFloorSurface,
    &_gAkropolisHelicopterLandingPadPerimeterSurface,
    &_gAkropolisHelicopterLandingPadSurface5Properties,
    &_gAkropolisHelicopterLandingPadOuterBoundarySurface,
    &_gAkropolisHelicopterLandingPadSilentSolidSurface,
};
