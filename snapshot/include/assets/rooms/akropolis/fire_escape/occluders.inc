/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static WorldCollisionOccluder _gAcropolisFireEscapeOccluders[2] = {
    { .next = NULL, .prevLink = NULL, .origin = { -0, -0, 0, 0 }, .vertices = { { -0, -0, 0, 0 }, { 0, -0, 0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, -0, 0 }, .radius = 0, .flags = 0, .unknown_3B = 0 },
    { .next = NULL, .prevLink = NULL, .origin = { -0, -0, -0, 0 }, .vertices = { { 0, -0, 0, 0 }, { 0, -0, -0, 0 }, { 0, 0, 0, 0 }, { 0, 0, -0, 0 } }, .normal = { -0, 0, 0, 0 }, .radius = 0, .flags = 0, .unknown_3B = 0 },
};
