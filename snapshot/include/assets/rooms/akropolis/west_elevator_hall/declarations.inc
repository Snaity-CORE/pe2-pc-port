/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/evs.h"
#include "gameplay/light.h"
#include "gameplay/message.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static DATA_CONST SVECTOR _gAkropolisWestElevatorHallFirstBeaconPosition;
static DATA_CONST SVECTOR _gAkropolisWestElevatorHallBayLightPosition;
static AnimationBankCopyRequest _gAkropolisWestElevatorHallArrivalAnimationBankCopyRequest;
static AnimationPlayRequest _gAkropolisWestElevatorHallArrivalAnimationRequest;
static ActorTransform _gAkropolisWestElevatorHallPlayerArrivalHiddenTransform;
static ActorTransform _gAkropolisWestElevatorHallPlayerDoorwayTransform;
static ActorTransform _gAkropolisWestElevatorHallUnusedTransform;
static ActorTransform _gAkropolisWestElevatorHallPlayerArrivalEndTransform;
static AnimationPlayRequest _gAkropolisWestElevatorHallPlayerIdleAnimationRequest;
static EvsCommand _gAkropolisWestElevatorHallArrivalScript[26];
static EvsCommand _gAkropolisWestElevatorHallArrivalSkipScript[13];
extern WorldCollisionRoomResources gAkropolisWestElevatorHallCollisionRoomResources[1];
extern u8* gAkropolisWestElevatorHallViewMaps[1];
extern ViewCount gAkropolisWestElevatorHallViewCounts[1];
extern WorldCoordRoomLighting gAkropolisWestElevatorHallRoomLightingTable[1];
extern DirectionWarpEntry gAkropolisWestElevatorHallWarpEntries[2];
static SVECTOR _gAkropolisWestElevatorHallCollisionGridNormals[7];
static SVECTOR _gAkropolisWestElevatorHallCollisionGridVerts[29];
static WorldCollisionGridFace _gAkropolisWestElevatorHallCollisionGridFaces[14];
static s16 _gAkropolisWestElevatorHallCollisionGridCells[52];
static s16* _gAkropolisWestElevatorHallCollisionGridTable[6];
static WorldCollisionGrid _gAkropolisWestElevatorHallCollisionGrid;
static WorldCollisionTrigger _gAkropolisWestElevatorHallViewBoundaryTriggers[4];
static WorldCollisionTrigger _gAkropolisWestElevatorHallActionTriggers[5];
static SpriteBatch _gAkropolisWestElevatorHallView1SpriteBatches[2];
static SpriteSource _gAkropolisWestElevatorHallView2SpriteSources[26];
static SpriteBatch _gAkropolisWestElevatorHallView2SpriteBatches[4];
static SpriteSource _gAkropolisWestElevatorHallView3SpriteSources[63];
static SpriteBatch _gAkropolisWestElevatorHallView3SpriteBatches[5];
static SpriteSource _gAkropolisWestElevatorHallView4SpriteSources[60];
static SpriteBatch _gAkropolisWestElevatorHallView4SpriteBatches[4];
static SpriteSource _gAkropolisWestElevatorHallView5SpriteSources[26];
static SpriteBatch _gAkropolisWestElevatorHallView5SpriteBatches[3];
extern SpriteView gAkropolisWestElevatorHallSpriteViews[5];
static WorldCoordPointLight _gAkropolisWestElevatorHallPointLights[15];
static WorldCoordRoomLights _gAkropolisWestElevatorHallRoomLights;
extern ViewCamera gAkropolisWestElevatorHallViewCameras[5];
static WorldCollisionFootstepSounds _gAkropolisWestElevatorHallFootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisWestElevatorHallSurfaceProperties;
extern WorldCollisionSurfaceProperties* gAkropolisWestElevatorHallSurfacePropertiesByClass[8];
