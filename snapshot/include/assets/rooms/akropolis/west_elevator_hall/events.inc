/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static AnimationBankCopyRequest _gAcropolisWestElevatorHallArrivalAnimationBankCopyRequest = { .sets = (void*)&_gAcropolisWestElevatorHallArrivalAnimationSets, .count = 0 };
static AnimationPlayRequest _gAcropolisWestElevatorHallArrivalAnimationRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gAcropolisWestElevatorHallPlayerArrivalHiddenTransform = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAcropolisWestElevatorHallPlayerDoorwayTransform = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
ActorTransform D_acropolis_west_elevator_hall_801845DC = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAcropolisWestElevatorHallPlayerArrivalEndTransform = { .pos = { -0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static AnimationPlayRequest _gAcropolisWestElevatorHallPlayerIdleAnimationRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static EvsCommand _gAcropolisWestElevatorHallArrivalScript[26] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallPlayerIdleAnimationRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallArrivalAnimationBankCopyRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallPlayerArrivalHiddenTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallPlayerDoorwayTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallArrivalAnimationRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallPlayerArrivalEndTransform }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAcropolisWestElevatorHallArrivalSkipScript[13] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallPlayerArrivalEndTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAcropolisWestElevatorHallPlayerIdleAnimationRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
