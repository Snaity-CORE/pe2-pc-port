/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include <psyq/sys/types.h>
#include <psyq/libgte.h>

#define ROOM_IS__gAkropolisBridgeStarGlowOffsets 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static SVECTOR _gAkropolisBridgeStarGlowOffsets[7] = {
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
};
static SVECTOR _gAkropolisBridgeGroundGlowOffsets[7] = {
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
};
static SVECTOR _gAkropolisBridgeLampGlowOffsets[12] = {
    { .vx = 0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = 0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = 0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = 0, .vy = -0, .vz = 0, .pad = 0 },
    { .vx = 0, .vy = -0, .vz = 0, .pad = 0 },
    { .vx = 0, .vy = -0, .vz = 0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = 0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = 0, .pad = 0 },
};
#endif
#undef ROOM_IS__gAkropolisBridgeStarGlowOffsets

#define ROOM_IS__gAkropolisBridgePlayerWaterPartPositions 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static SVECTOR _gAkropolisBridgePlayerWaterPartPositions[2] = {
    { .vx = 0, .vy = 0, .vz = 0, .pad = 0 },
    { .vx = 0, .vy = 0, .vz = 0, .pad = 0 },
};
static SVECTOR _gAkropolisBridgeWarmDiscWorldPosition = { .vx = -0, .vy = -0, .vz = -0, .pad = 0 };
static SVECTOR _gAkropolisBridgeCoolDiscWorldPosition = { .vx = -0, .vy = -0, .vz = -0, .pad = 0 };
#endif
#undef ROOM_IS__gAkropolisBridgePlayerWaterPartPositions
#undef ROOM_PART
