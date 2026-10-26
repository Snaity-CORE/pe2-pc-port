/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisHallwayFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisHallwaySilentBlockingSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisHallwayWalkablePassThroughSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisHallwayFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisHallwayWalkableBlockingSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisHallwayFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisHallwaySurfacePropertiesByClass[8] = {
    &_gAkropolisHallwaySilentBlockingSurface,
    &_gAkropolisHallwayWalkablePassThroughSurface,
    &_gAkropolisHallwaySilentBlockingSurface,
    &_gAkropolisHallwaySilentBlockingSurface,
    &_gAkropolisHallwayWalkableBlockingSurface,
    &_gAkropolisHallwaySilentBlockingSurface,
    &_gAkropolisHallwaySilentBlockingSurface,
    &_gAkropolisHallwaySilentBlockingSurface,
};
