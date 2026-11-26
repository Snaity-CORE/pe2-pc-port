/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "gameplay/evs.h"
#include "gameplay/message.h"

static ActorTransform _gMistParkingRoom2PlayerPlacement;
static ActorTransform _gMistParkingRoom1PlayerPlacement;
static ActorTransform _gMistParkingCommand1PlayerPlacement;
static AnimationBankCopyRequest _gMistParkingCommand1PlayerBankCopy;
static AnimationPlayRequest _gMistParkingCommand1PlayerAnimation;
static AnimationPlayRequest _gMistParkingPlayerRoomTransitionAnimation;
static EvsCommand _gMistParkingSelectRoom2Script[15];
static EvsCommand _gMistParkingSelectRoom1Script[13];
static EvsCommand _gMistParkingCommand1Script[12];
