/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisEastElevatorHallFootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisEastElevatorHallDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisEastElevatorHallFootstepSounds };
static WorldCollisionSurfaceProperties _gAkropolisEastElevatorHallClass1SurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
WorldCollisionSurfaceProperties* gAkropolisEastElevatorHallSurfacePropertiesByClass[8] = {
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAkropolisEastElevatorHallClass1SurfaceProperties,
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
    &_gAkropolisEastElevatorHallDefaultSurfaceProperties,
};
