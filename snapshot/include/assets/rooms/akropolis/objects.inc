/* rooms/akropolis/stage: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gMapAkropolisHelicopterLandingPadObjectPlaces 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AreaObjectPlace _gMapAkropolisHelicopterLandingPadObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = -0, .y = -0, .z = -0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
#endif
#undef ROOM_IS__gMapAkropolisHelicopterLandingPadObjectPlaces

#define ROOM_IS_gMapAkropolisObjectRooms 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
AreaObjectRoom gMapAkropolisObjectRooms[22] = {
    { .places = { .sentinel = 0 }, .spawns = NULL },
    { .places = { .list = _gMapAkropolisSquareObjectPlaces }, .spawns = (void*)&_gMapAkropolisSquareObjectSpawns },
    { .places = { .list = _gMapAkropolisEastElevatorHallObjectPlaces }, .spawns = (void*)&_gMapAkropolisEastElevatorHallObjectSpawns },
    { .places = { .sentinel = 0 }, .spawns = NULL },
    { .places = { .list = _gMapAkropolisCafeteriaObjectPlaces }, .spawns = (void*)&_gMapAkropolisCafeteriaObjectSpawns },
    { .places = { .sentinel = 0 }, .spawns = NULL },
    { .places = { .list = _gMapAkropolisSecurityRoomObjectPlaces }, .spawns = (void*)&_gMapAkropolisSecurityRoomObjectSpawns },
    { .places = { .list = _gMapAkropolisHallwayObjectPlaces }, .spawns = (void*)&_gMapAkropolisHallwayObjectSpawns },
    { .places = { .list = _gMapAkropolisFountainObjectPlaces }, .spawns = (void*)&_gMapAkropolisFountainObjectSpawns },
    { .places = { .list = _gMapAkropolisForkedRoadObjectPlaces }, .spawns = (void*)&_gMapAkropolisForkedRoadObjectSpawns },
    { .places = { .list = _gMapAkropolisObservatoryObjectPlaces }, .spawns = (void*)&_gMapAkropolisObservatoryObjectSpawns },
    { .places = { .list = _gMapAkropolisPromenadeObjectPlaces }, .spawns = (void*)&_gMapAkropolisPromenadeObjectSpawns },
    { .places = { .list = _gMapAkropolisSanctuaryObjectPlaces }, .spawns = (void*)&_gMapAkropolisSanctuaryObjectSpawns },
    { .places = { .list = _gMapAkropolisRoofGardenObjectPlaces }, .spawns = (void*)&_gMapAkropolisRoofGardenObjectSpawns },
    { .places = { .sentinel = 0 }, .spawns = NULL },
    { .places = { .list = _gMapAkropolisFireEscapeObjectPlaces }, .spawns = (void*)&_gMapAkropolisFireEscapeObjectSpawns },
    { .places = { .list = _gMapAkropolisHelicopterLandingPadObjectPlaces }, .spawns = (void*)&_gMapAkropolisHelicopterLandingPadObjectSpawns },
    { .places = { .list = _gMapAkropolisWestElevatorHallObjectPlaces }, .spawns = (void*)&_gMapAkropolisWestElevatorHallObjectSpawns },
    { .places = { .sentinel = 0 }, .spawns = NULL },
    { .places = { .list = _gMapAkropolisMistParkingObjectPlaces }, .spawns = (void*)&_gMapAkropolisMistParkingObjectSpawns },
    { .places = { .sentinel = 0 }, .spawns = NULL },
    { .places = { .sentinel = -0 }, .spawns = NULL },
};
#endif
#undef ROOM_IS_gMapAkropolisObjectRooms

#define ROOM_IS__gMapAkropolisSquareObjectPlaces 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AreaObjectPlace _gMapAkropolisSquareObjectPlaces[6] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = -0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisEastElevatorHallObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisCafeteriaObjectPlaces[5] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = -0, .y = -0, .z = -0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = -0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisSecurityRoomObjectPlaces[1] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisHallwayObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = -0, .z = -0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisFountainObjectPlaces[4] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisForkedRoadObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisObservatoryObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisPromenadeObjectPlaces[3] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisSanctuaryObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = -0, .y = -0, .z = -0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisRoofGardenObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = -0, .y = 0, .z = -0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisFireEscapeObjectPlaces[2] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = -0, .y = -0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisWestElevatorHallObjectPlaces[1] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
static AreaObjectPlace _gMapAkropolisMistParkingObjectPlaces[10] = {
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = -0, .y = -0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
    { .flagIndex = 0, .kind = 0, .placeKeyHigh = 0, .state = 0, .x = 0, .y = 0, .z = 0, .yaw = 0 },
};
#endif
#undef ROOM_IS__gMapAkropolisSquareObjectPlaces
#undef ROOM_PART
