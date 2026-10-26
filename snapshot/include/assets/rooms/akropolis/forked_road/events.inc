/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/evs.h"
#include "gameplay/message.h"
#include "gameplay/scene_combat.h"

static AnimationBankCopyRequest _gAkropolisForkedRoadAmbushPlayerBankCopyRequest = { .sets = (void*)&_gAkropolisForkedRoadAmbushPlayerAnimationSets, .count = 0 };
static AnimationPlayRequest _gAkropolisForkedRoadUnusedBank1Animation1Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisForkedRoadAmbushPlayerStandRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisForkedRoadUnusedBank1AnimationRequests[2] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
EvsCommand gAkropolisForkedRoadAmbushScript[8] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisForkedRoadAmbushPlayerBankCopyRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisForkedRoadAmbushPlayerStandRequest }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_akropolisForkedRoadReleaseMaggotCaterpillarEntrance }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&sceneEngageBattle }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
