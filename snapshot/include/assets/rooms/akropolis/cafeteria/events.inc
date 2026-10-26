/* rooms/akropolis/cafeteria/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include <psyq/libgpu.h>
#include "gameplay/evs.h"
#include "gameplay/message.h"
#include "gameplay/scene_combat.h"

#define ROOM_IS__gAkropolisCafeteriaPlayerAnimation1 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AnimationPlayRequest _gAkropolisCafeteriaPlayerAnimation1 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaUnusedPlayerAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerPostBattleAnimation8 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerPostBattleAnimation9 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaOpeningWomanAnimation1 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gAkropolisCafeteriaPlayerOpeningScenePlacement = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaPlayerOpeningResumePlacement = { .pos = { -0, -0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaPlayerFollowUpStartPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaPlayerBeforeRupertArrivalPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaPlayerBattleStartPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaPlayerFollowUpWalkStartPlacement = { .pos = { -0, -0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaPlayerFollowUpWalkTarget = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaUnusedPlayerPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static AnimationPlayRequest _gAkropolisCafeteriaUnusedActorAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
#endif
#undef ROOM_IS__gAkropolisCafeteriaPlayerAnimation1

#define ROOM_IS__gAkropolisCafeteriaOpeningPlayerBankCopy 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AnimationBankCopyRequest _gAkropolisCafeteriaOpeningPlayerBankCopy = { .sets = (void*)&_gAkropolisCafeteriaOpeningPlayerAnimationSets, .count = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerOpeningAnimation47 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
#endif
#undef ROOM_IS__gAkropolisCafeteriaOpeningPlayerBankCopy

#define ROOM_IS__gAkropolisCafeteriaFollowUpPlayerBankCopy 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AnimationBankCopyRequest _gAkropolisCafeteriaFollowUpPlayerBankCopy = { .sets = (void*)&_gAkropolisCafeteriaFollowUpPlayerAnimationSets, .count = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerFollowUpAnimation47 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerFollowUpAnimation48 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerFollowUpAnimation49 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaPlayerFollowUpAnimation50 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gAkropolisCafeteriaStrangerBattleIntroPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerPostBattlePlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerFollowUpStartPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerFollowUpAnimation48Placement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerRecoilPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerFollowUpPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerFollowUpBurnPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaStrangerBattleStartPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaGrinningStrangerScenePlacement = { .pos = { -0, -0, 0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorCommand _gAkropolisCafeteriaUnusedActorCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerStartCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerPostBattleCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerFollowUpTripleHitCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerRecoilCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerFollowUpCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerSettleCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerFollowUpBurnCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gAkropolisCafeteriaStrangerFollowUpClipCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorTransform _gAkropolisCafeteriaRupertArrivalPlacement = { .pos = { -0, -0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaRupertConversationPlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gAkropolisCafeteriaRupertDeparturePlacement = { .pos = { -0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static AnimationPlayRequest _gAkropolisCafeteriaRupertShootingAnimation1 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaRupertArrivalAnimation2 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaRupertReloadAnimation3 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gAkropolisCafeteriaRupertDepartureAnimation4 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static EvsSceneKey _gAkropolisCafeteriaUnusedSceneKey = { .group = 0, .streamId = 0, .subId = 0 };
static EvsSceneKey _gAkropolisCafeteriaFollowUpSceneKey = { .group = 0, .streamId = 0, .subId = 0 };
static GameActorMoveAnim _gAkropolisCafeteriaPlayerFollowUpWalkAnimations = { .approachAnimId = 0, .arrivalAnimId = 0 };
static EvsCommand _gAkropolisCafeteriaOpeningSceneScript[35] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_akropolisCafeteriaCancelPeEffects }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaOpeningPlayerBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaOpeningWomanAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerOpeningAnimation47 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerOpeningScenePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_akropolisCafeteriaSetPlayerSurface }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerOpeningResumePlacement }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisCafeteriaOpeningSceneSkipScript[14] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerOpeningResumePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisCafeteriaPostBattleSceneScript[19] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_akropolisCafeteriaPreparePostBattleScene }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerPostBattleAnimation9 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_akropolisCafeteriaCopyPostBattleVram }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerPostBattlePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerPostBattleCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerPostBattleAnimation8 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisCafeteriaPostBattleSceneSkipScript[15] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerRecoilCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_akropolisCafeteriaCopyPostBattleVram }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerPostBattlePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerPostBattleAnimation8 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisCafeteriaFollowUpSceneScript[80] = {
    { .opcode = 0, .operand0 = { .storage = &_gAkropolisCafeteriaFollowUpSceneKey }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpStartPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaFollowUpPlayerBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpStartPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpAnimation47 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpClipCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerRecoilCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerBeforeRupertArrivalPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertArrivalPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertArrivalAnimation2 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpAnimation48Placement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpTripleHitCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpAnimation48 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerRecoilPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertConversationPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpClipCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertShootingAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpWalkStartPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpWalkTarget }, .operand4 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpWalkAnimations } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpWalkTarget }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpBurnPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerFollowUpBurnCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertConversationPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertReloadAnimation3 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertConversationPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpAnimation49 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaGrinningStrangerScenePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerStartCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertDeparturePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertDepartureAnimation4 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaRupertDeparturePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerFollowUpAnimation50 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
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
static EvsCommand _gAkropolisCafeteriaFollowUpSceneSkipScript[10] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
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
static EvsCommand _gAkropolisCafeteriaCap13ShakeScript[6] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisCafeteriaBattleIntroScript[19] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerAnimation1 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerBattleIntroPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerStartCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&SetDispMask }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&sceneEngageBattle }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaPlayerBattleStartPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerBattleStartPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gAkropolisCafeteriaGrinningStrangerAppearanceScript[4] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaGrinningStrangerScenePlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gAkropolisCafeteriaStrangerStartCommand }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
#endif
#undef ROOM_IS__gAkropolisCafeteriaFollowUpPlayerBankCopy
#undef ROOM_PART
