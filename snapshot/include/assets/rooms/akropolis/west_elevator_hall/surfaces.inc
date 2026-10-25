/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionFootstepSounds _gAcropolisWestElevatorHallFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAcropolisWestElevatorHallSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAcropolisWestElevatorHallFootstepSounds };
WorldCollisionSurfaceProperties* gAcropolisWestElevatorHallSurfacePropertiesByClass[8] = {
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
    &_gAcropolisWestElevatorHallSurfaceProperties,
};
