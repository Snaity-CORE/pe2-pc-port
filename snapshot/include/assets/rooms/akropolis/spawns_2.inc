/* rooms/akropolis/stage: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area_flags.h"
#include "stages/akropolis/rooms/helicopter_landing_pad.h"

static AreaObjectSpawn _gAkropolisStageHelicopterLandingPadObjectSpawns[3] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisHelicopterLandingPadLiftTask, .data = { .model = &gAkropolisHelicopterLandingPadLiftModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisHelicopterLandingPadPickupModelTask, .data = { .model = &gAkropolisHelicopterLandingPadAmmunitionPickupModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
