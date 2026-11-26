/* rooms/akropolis/patio/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "gameplay/message.h"

static AnimationBankCopyRequest _gAkropolisPatioAmbushPlayerBankCopyRequest = { .sets = (void*)&_gAkropolisPatioAmbushPlayerAnimationSets, .count = 0 };
static AnimationPlayRequest _gAkropolisPatioRetainedBank1AnimationRequests[4] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gAkropolisPatioAmbushPlayerAnimation9Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gAkropolisPatioAmbushPlayerStartPlacement = { .pos = { -0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisPatioUnusedAmbushTransforms[2] = {
    { .pos = { -0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } },
    { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } },
};
static ActorTransform _gAkropolisPatioAmbushPlayerRunDestination = { .pos = { -0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisPatioAmbushPlayerSkipPlacement = { .pos = { -0, 0, 0, 0 }, .rot = { 0, -0, 0, 0 } };
static AnimationPlayRequest _gAkropolisPatioRetainedBank0Animation1To4Requests[5] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gAkropolisPatioLeapingStranger1AnimationRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisPatioLeapingStranger2AnimationRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisPatioRetainedBank0Animation7And8Requests[2] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static ActorTransform _gAkropolisPatioLeapingStranger1StartPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gAkropolisPatioLeapingStranger1Launch = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisPatioLeapingStranger2StartPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisPatioLeapingStranger2Launch = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisPatioAmbushStranger1BattlePlacement = { .pos = { -0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisPatioAmbushStranger2BattlePlacement = { .pos = { -0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorMotionWalkAnim _gAkropolisPatioStrangerLeapAnimations = { .animationId = 0, .nextAnimId = 0 };
static ActorCommand _gAkropolisPatioLowerStrangerHeadWeightCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisPatioRaiseStrangerHeadWeightCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisPatioPrintStrangerTransformCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
