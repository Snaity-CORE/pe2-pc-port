/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisEastElevatorHallFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisEastElevatorHallDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisEastElevatorHallFootstepSounds };
static WorldCollisionSurfaceProperties _gAcropolisEastElevatorHallClass1SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAcropolisEastElevatorHallSurfacePropertiesByClass[8] = {
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAcropolisEastElevatorHallClass1SurfaceProperties,
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAcropolisEastElevatorHallDefaultSurfaceProperties,
};
