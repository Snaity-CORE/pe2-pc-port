/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/evs.h"
#include "gameplay/message.h"

static AnimationBankCopyRequest _gAkropolisWestElevatorHallArrivalAnimationBankCopyRequest = { .sets = (void*)&_gAkropolisWestElevatorHallArrivalAnimationSets, .count = 0 };
static AnimationPlayRequest _gAkropolisWestElevatorHallArrivalAnimationRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gAkropolisWestElevatorHallPlayerArrivalHiddenTransform = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisWestElevatorHallPlayerDoorwayTransform = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisWestElevatorHallUnusedTransform = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisWestElevatorHallPlayerArrivalEndTransform = { .pos = { -0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static AnimationPlayRequest _gAkropolisWestElevatorHallPlayerIdleAnimationRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static EvsCommand _gAkropolisWestElevatorHallArrivalScript[26] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallPlayerIdleAnimationRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallArrivalAnimationBankCopyRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallPlayerArrivalHiddenTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallPlayerDoorwayTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallArrivalAnimationRequest }, .operand4 = { .value = 0 } },
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
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallPlayerArrivalEndTransform }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisWestElevatorHallArrivalSkipScript[13] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallPlayerArrivalEndTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisWestElevatorHallPlayerIdleAnimationRequest }, .operand4 = { .value = 0 } },
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
