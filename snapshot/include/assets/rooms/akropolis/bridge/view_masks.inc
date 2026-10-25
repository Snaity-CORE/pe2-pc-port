/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gAcropolisBridgeStarGlowViewMasks 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static u16 _gAcropolisBridgeStarGlowViewMasks[8] = {
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
};
#endif
#undef ROOM_IS__gAcropolisBridgeStarGlowViewMasks

#define ROOM_IS__gAcropolisBridgeLampGlowViewMasks 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static u16 _gAcropolisBridgeLampGlowViewMasks[12] = {
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
#endif
#undef ROOM_IS__gAcropolisBridgeLampGlowViewMasks
#undef ROOM_PART
