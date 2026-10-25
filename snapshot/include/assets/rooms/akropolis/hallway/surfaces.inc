/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisHallwayFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisHallwaySilentBlockingSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAcropolisHallwayWalkablePassThroughSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisHallwayFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisHallwayWalkableBlockingSurface = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisHallwayFootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisHallwaySurfacePropertiesByClass[8] = {
    &_gAcropolisHallwaySilentBlockingSurface,
    &_gAcropolisHallwayWalkablePassThroughSurface,
    &_gAcropolisHallwaySilentBlockingSurface,
    &_gAcropolisHallwaySilentBlockingSurface,
    &_gAcropolisHallwayWalkableBlockingSurface,
    &_gAcropolisHallwaySilentBlockingSurface,
    &_gAcropolisHallwaySilentBlockingSurface,
    &_gAcropolisHallwaySilentBlockingSurface,
};
