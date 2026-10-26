/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/view.h"

ViewCamera gAkropolisObservatoryViewCameras[8] = {
    { .transform = { .m = { { 0, 0, 0 }, { 0, 0, -0 }, { 0, 0, 0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { 0, 0, 0 }, { 0, 0, -0 }, { -0, 0, 0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { 0, 0, 0 }, { 0, 0, -0 }, { -0, 0, 0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { -0, 0, 0 }, { 0, 0, 0 }, { -0, 0, -0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { -0, 0, 0 }, { 0, 0, 0 }, { -0, 0, -0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { 0, 0, -0 }, { -0, 0, -0 }, { 0, 0, 0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { -0, 0, -0 }, { -0, 0, 0 }, { 0, 0, -0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
    { .transform = { .m = { { 0, 0, 0 }, { 0, 0, -0 }, { -0, 0, 0 } }, .t = { 0, 0, 0 } }, .screenDistance = 0 },
};
