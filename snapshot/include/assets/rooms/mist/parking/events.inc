/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#define ROOM_IS__gMistParkingRoom2PlayerPlacement 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static ActorTransform _gMistParkingRoom2PlayerPlacement = { .pos = { 0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gMistParkingRoom1PlayerPlacement = { .pos = { 0, 0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
static ActorTransform _gMistParkingCommand1PlayerPlacement = { .pos = { 0, 0, -0, 0 }, .rot = { 0, -0, 0, 0 } };
#endif
#undef ROOM_IS__gMistParkingRoom2PlayerPlacement

#define ROOM_IS__gMistParkingCommand1PlayerBankCopy 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AnimationBankCopyRequest _gMistParkingCommand1PlayerBankCopy = { .sets = (void*)&_gMistParkingCommand1PlayerAnimationSets, .count = 0 };
static AnimationPlayRequest _gMistParkingCommand1PlayerAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerRoomTransitionAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static EvsCommand _gMistParkingSelectRoom2Script[15] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerRoomTransitionAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingSpawnCapEventIfIdle }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingRoom2PlayerPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingSelectChapterRoom }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingSelectRoom1Script[13] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerRoomTransitionAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingRoom1PlayerPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingSelectChapterRoom }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingCommand1Script[12] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingCommand1PlayerPlacement }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingCommand1PlayerBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingCommand1PlayerAnimation }, .operand4 = { .value = 0 } },
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
#endif
#undef ROOM_IS__gMistParkingCommand1PlayerBankCopy

#define ROOM_IS_gMistParkingPlayerConversationBankCopy 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
AnimationBankCopyRequest gMistParkingPlayerConversationBankCopy = { .sets = (void*)&_gMistParkingPlayerConversationAnimationSets, .count = 0 };
static AnimationPlayRequest _gMistParkingUnusedPlayerClip1Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerConversationResetAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPlayerClip49Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalCueAnimation50 = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerCutsceneModelCueAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip52Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip53Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip54Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip55Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip56Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip57Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPlayerClip58Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalTalkResetAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip60Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip61Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPlayerClip62Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip63Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerTalkAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPlayerClip65Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip66Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip67Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerStayOrGalleryReplyAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedPlayerClip69Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerDepartureReplyAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerArrivalClip71Request = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
#endif
#undef ROOM_IS_gMistParkingPlayerConversationBankCopy

#define ROOM_IS__gMistParkingPlayerArrivalAndDepartureTransform 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
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
#endif
#undef ROOM_IS__gMistParkingPlayerArrivalAndDepartureTransform

#define ROOM_IS_gMistParkingArrivalConversationScript 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
EvsCommand gMistParkingArrivalConversationScript[155] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalAndDepartureTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalStartTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieShopTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceShowBillboardCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingSpawnCutsceneModel }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerCutsceneModelCueAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingReleaseCutsceneModel }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalApproachTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip52Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalTalkTransform }, .operand4 = { .storage = &_gMistParkingPierceArrivalTalkWalkClips } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip53Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip54Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip28Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip55Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip27Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip34Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip56Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip17Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip18Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip57Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceAimHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceHideBillboardCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip20Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip66Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip27Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalCueAnimation50 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip67Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip29Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip31Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip66Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip27Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip60Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip61Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip34Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip67Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClosingTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClip19Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip67Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClosingTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClosingAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip71Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerConversationResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalClosingAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerConversationResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip67Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalClip63Request }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSetPierceCollisionPatchLowered }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPiercePostArrivalTransform }, .operand4 = { .storage = &_gMistParkingPiercePostArrivalWalkClips } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceReleaseHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSetConversationProgress }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingArrivalSkipScript[23] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingReleaseCutsceneModel }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSetPierceCollisionPatchLowered }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceHideBillboardCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPiercePostArrivalTransform }, .operand4 = { .storage = &_gMistParkingPiercePostArrivalWalkClips } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceReleaseHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSetConversationProgress }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingArrivalRoomEntryScript[8] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceArrivalTalkTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceHideBillboardCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPiercePostArrivalTransform }, .operand4 = { .storage = &_gMistParkingPiercePostArrivalWalkClips } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieShopTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingFirstFollowupScript[10] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPackedDialogue }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceFollowupTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingSecondFollowupScript[10] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPackedDialogue }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceFollowupTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingUnusedThirdFollowupScript[10] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPackedDialogue }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceFollowupTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingDeparturePromptScript[13] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceAimHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSelectDialogueResource }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerDeparturePromptTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceDeparturePromptAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingStayReplyScript[13] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPackedDialogue }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerStayOrGalleryReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerStayOrGalleryReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceReleaseHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSelectDialogueResource }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingDryfieldDepartureScript[24] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPackedDialogue }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerArrivalAndDepartureTransform }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceDryfieldDepartureTarget }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceReleaseHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingQueueDelayedDisplayModeExit }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSelectDialogueResource }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartDepartureMovie }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingQueueDryfieldDeparture }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingShootingGalleryDepartureScript[16] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartPackedDialogue }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerStayOrGalleryReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingControlPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingQueueDelayedDisplayModeExit }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingReloadAcropolisArea }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&mistParkingSelectDialogueResource }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingJodieTalkPoseScript[7] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerConversationResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieTalkOpeningAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieTalkGreetingAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingJodieShopSessionScript[10] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&shopOpenSession }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerConversationResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieShopReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieShopReplyClosingAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingJodieShopDeclineScript[8] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerConversationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerConversationResetAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieShopReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieShopReplyClosingAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingJodieConversationRestAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
#endif
#undef ROOM_IS_gMistParkingArrivalConversationScript

#define ROOM_IS__gMistParkingCutsceneModelPlacement 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static ActorTransform _gMistParkingCutsceneModelPlacement = { .pos = { 0, -0, -0, 0 }, .rot = { 0, 0, 0, 0 } };
#endif
#undef ROOM_IS__gMistParkingCutsceneModelPlacement

#define ROOM_IS_gMistParkingShopPlayerAnimationBankCopy 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
AnimationBankCopyRequest gMistParkingShopPlayerAnimationBankCopy = { .sets = (void*)&_gMistParkingShopPlayerAnimationSets, .count = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank1Clip1Animation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
AnimationPlayRequest gMistParkingPlayerShopTalkAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerDepartureMenuAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingPlayerStayAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
#endif
#undef ROOM_IS_gMistParkingShopPlayerAnimationBankCopy

#define ROOM_IS__gMistParkingUnusedBank0Clip1ResetAnimation 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static AnimationPlayRequest _gMistParkingUnusedBank0Clip1ResetAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank0Clips1To5Animations[5] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingPierceDepartureMenuAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank0Clips7To14Animations[8] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
static AnimationPlayRequest _gMistParkingShopPierceDepartureReplyAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank0Clips16To35Animations[20] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
#endif
#undef ROOM_IS__gMistParkingUnusedBank0Clip1ResetAnimation

#define ROOM_IS__gMistParkingShopPierceAimHeadCommand 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static ActorCommand _gMistParkingShopPierceAimHeadCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static ActorCommand _gMistParkingShopPierceReleaseHeadCommand = { .context = { .loc = { .stage = 0, .area = 0 } }, .command = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank0Clip1Animation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
AnimationPlayRequest gMistParkingJodieIdleAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank0Clips2To4Animations[3] = {
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
    { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 },
};
AnimationPlayRequest gMistParkingJodieShopExitAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static AnimationPlayRequest _gMistParkingUnusedBank0Clip6Animation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
AnimationPlayRequest gMistParkingJodieGreetingAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
AnimationPlayRequest gMistParkingJodieReplyAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
AnimationPlayRequest gMistParkingJodieReplyEndAnimation = { .source = { .index = 0 }, .animationId = 0, .blend = 0, .blendFrames = 0, .enableWorldCollision = 0 };
static EvsCommand _gMistParkingShopDepartureMenuScript[10] = {
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartShopPlayerHeadAim }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingShopPierceAimHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerDepartureMenuAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPierceDepartureMenuAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingShopStayScript[12] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingRunShopDialogueCommand }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerStayAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingShopPierceDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingControlShopPlayerHeadAim }, .operand1 = { .value = -0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingPlayerStayAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingShopPierceReleaseHeadCommand }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingShopDepartForAcropolisScript[18] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingRunShopDialogueCommand }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingShopPierceDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingQueueDelayedShopDisplayModeExit }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingSelectShopDialogueResource }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingStartShopDepartureMovie }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingQueueAcropolisDeparture }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
static EvsCommand _gMistParkingShopDepartForShootingGalleryScript[12] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingRunShopDialogueCommand }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &_gMistParkingShopPierceDepartureReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingQueueDelayedShopDisplayModeExit }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&_mistParkingReloadShopAcropolisArea }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
#endif
#undef ROOM_IS__gMistParkingShopPierceAimHeadCommand

#define ROOM_IS_gMistParkingJodieShopTalkScript 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
EvsCommand gMistParkingJodieShopTalkScript[8] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerShopTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieIdleAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieGreetingAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieIdleAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingJodieOpenShopScript[10] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .storage = (void*)&shopOpenSession }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerShopTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieReplyEndAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieShopExitAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingJodieShopReplyScript[8] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerShopTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieReplyEndAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieIdleAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
EvsCommand gMistParkingJodieShopFarewellScript[8] = {
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingShopPlayerAnimationBankCopy }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingPlayerShopTalkAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieReplyAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieReplyEndAnimation }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
    { .opcode = 0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .storage = &gMistParkingJodieShopExitAnimation }, .operand4 = { .value = 0 } },
    { .opcode = -0, .operand0 = { .value = 0 }, .operand1 = { .value = 0 }, .operand2 = { .value = 0 }, .operand3 = { .value = 0 }, .operand4 = { .value = 0 } },
};
#endif
#undef ROOM_IS_gMistParkingJodieShopTalkScript
#undef ROOM_PART
