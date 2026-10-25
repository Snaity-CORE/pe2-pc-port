/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gAcropolisBridgeStarGlowOffsets 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static SVECTOR _gAcropolisBridgeStarGlowOffsets[7] = {
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = -0, .vz = -0, .pad = 0 },
};
static SVECTOR _gAcropolisBridgeGroundGlowOffsets[7] = {
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
    { .vx = -0, .vy = 0, .vz = -0, .pad = 0 },
};
static SVECTOR _gAcropolisBridgeLampGlowOffsets[12] = {
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
#undef ROOM_IS__gAcropolisBridgeStarGlowOffsets

#define ROOM_IS__gAcropolisBridgePlayerWaterPartPositions 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static SVECTOR _gAcropolisBridgePlayerWaterPartPositions[2] = {
    { .vx = 0, .vy = 0, .vz = 0, .pad = 0 },
    { .vx = 0, .vy = 0, .vz = 0, .pad = 0 },
};
static SVECTOR _gAcropolisBridgeWarmDiscWorldPosition = { .vx = -0, .vy = -0, .vz = -0, .pad = 0 };
static SVECTOR _gAcropolisBridgeCoolDiscWorldPosition = { .vx = -0, .vy = -0, .vz = -0, .pad = 0 };
#endif
#undef ROOM_IS__gAcropolisBridgePlayerWaterPartPositions
#undef ROOM_PART
