/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gAcropolisFountainClimbTrigger 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static WorldCollisionTrigger _gAcropolisFountainClimbTrigger = { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, 0, 0 }, { -0, 0, -0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { -0, 0, 0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 };
#endif
#undef ROOM_IS__gAcropolisFountainClimbTrigger

#define ROOM_IS__gAcropolisFountainActionTriggers 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static WorldCollisionTrigger _gAcropolisFountainActionTriggers[9] = {
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { -0, -0, 0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, -0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, 0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, -0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { -0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, 0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { -0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, -0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, -0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, -0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { -0, 0, 0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, -0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { -0, 0, 0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
    { .next = NULL, .prevLink = NULL, .coord = NULL, .origin = { 0, -0, -0, 0 }, .vertices = { { -0, 0, -0, 0 }, { 0, 0, -0, 0 }, { -0, 0, 0, 0 }, { 0, 0, 0, 0 } }, .normal = { 0, 0, 0, 0 }, .facingNormal = { 0, 0, -0, 0 }, .radius = 0, .control = 0, .parameter0 = 0, .parameter1 = 0, .flags = 0, .hit = 0 },
};
#endif
#undef ROOM_IS__gAcropolisFountainActionTriggers
#undef ROOM_PART
