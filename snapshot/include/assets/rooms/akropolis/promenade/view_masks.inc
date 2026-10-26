/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"

static u16 _gAkropolisPromenadeGlowViewMask = 0;
static u16 _gAkropolisPromenadeDripViewMask = 0;
static u16 _gAkropolisPromenadeLampGlowViewMasks[12] = {
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};
