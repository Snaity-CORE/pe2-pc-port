/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisWestElevatorHallFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisWestElevatorHallSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisWestElevatorHallFootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisWestElevatorHallSurfacePropertiesByClass[8] = {
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
    &_gAkropolisWestElevatorHallSurfaceProperties,
};
