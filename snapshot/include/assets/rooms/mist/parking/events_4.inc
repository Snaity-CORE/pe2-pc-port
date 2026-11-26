/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "gameplay/message.h"

static ActorTransform _gMistParkingPlayerArrivalAndDepartureTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gMistParkingPlayerArrivalTalkTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gMistParkingPlayerArrivalApproachTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gMistParkingPlayerArrivalClosingTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gMistParkingPlayerDeparturePromptTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static AnimationPlayRequest _gMistParkingUnusedPierceResetRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceConversationRestAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip2To4Requests[3] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingPierceDeparturePromptAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClosingAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip7Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceFollowupTalkAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip9To14Requests[6] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingPierceDepartureReplyAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip16Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip17Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip18Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip19Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip20Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip21To26Requests[6] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingPierceArrivalClip27Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip28Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip29Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip30Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPierceArrivalClip31Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip32To33Requests[2] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingPierceArrivalClip34Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPierceClip35Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gMistParkingPierceArrivalStartTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gMistParkingPierceArrivalTalkTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gMistParkingPiercePostArrivalTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gMistParkingPierceDryfieldDepartureTarget = { .pos = { -0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorTransform _gMistParkingPierceArrivalClosingTransform = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
static ActorMotionWalkAnim _gMistParkingPiercePostArrivalWalkClips = { .animationId = 0, .nextAnimId = 0 };
static ActorMotionWalkAnim _gMistParkingPierceArrivalTalkWalkClips = { .animationId = 0, .nextAnimId = 0 };
static ActorCommand _gMistParkingPierceShowBillboardCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gMistParkingPierceHideBillboardCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gMistParkingPierceAimHeadCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gMistParkingPierceReleaseHeadCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static AnimationPlayRequest _gMistParkingUnusedJodieResetRequest = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingJodieTalkOpeningAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingJodieUnusedAnimations2To4[3] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingJodieConversationRestAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingJodieUnusedAnimation6 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingJodieTalkGreetingAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingJodieShopReplyAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingJodieShopReplyClosingAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static ActorTransform _gMistParkingJodieShopTransform = { .pos = { 0, 0, 0, 0 }, .rot = { 0, 0, 0, 0 } };
