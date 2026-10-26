/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/room.h"

static WorldCollisionFootstepSounds _gAkropolisForkedRoadSurfaceClass4FootstepSounds = { .walk = 0, .run = 0, .scriptedJump = 0 };
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadDefaultSurfaceProperties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadSurfaceClass2Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadSurfaceClass3Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = NULL };
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadSurfaceClass4Properties = { .field_0 = 0, .probePassThrough = 0, .weaponImpactEnabled = 0, .suppressPushback = 0, .footstepSounds = &_gAkropolisForkedRoadSurfaceClass4FootstepSounds };
WorldCollisionSurfaceProperties* gAkropolisForkedRoadSurfacePropertiesByClass[8] = {
    &_gAkropolisForkedRoadDefaultSurfaceProperties,
    &_gAkropolisForkedRoadDefaultSurfaceProperties,
    &_gAkropolisForkedRoadSurfaceClass2Properties,
    &_gAkropolisForkedRoadSurfaceClass3Properties,
    &_gAkropolisForkedRoadSurfaceClass4Properties,
    &_gAkropolisForkedRoadDefaultSurfaceProperties,
    &_gAkropolisForkedRoadDefaultSurfaceProperties,
    &_gAkropolisForkedRoadDefaultSurfaceProperties,
};
