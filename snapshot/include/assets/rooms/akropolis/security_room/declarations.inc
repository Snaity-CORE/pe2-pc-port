/* rooms/akropolis/security_room/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/area.h"
#include "gameplay/area_flags.h"
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/light.h"
#include "gameplay/message.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static AnimationPlayRequest _gAkropolisSecurityRoomUnusedBank1Animation1Request;
static SVECTOR _gAkropolisSecurityRoomLockIndicatorGlowOffsets[4];
extern WorldCollisionRoomResources gAkropolisSecurityRoomRoomCollisionResources[1];
extern u8* gAkropolisSecurityRoomViewMaps[1];
extern ViewCount gAkropolisSecurityRoomViewCounts[1];
extern WorldCoordRoomLighting gAkropolisSecurityRoomRoomLightingTable[1];
extern DirectionWarpEntry gAkropolisSecurityRoomWarpEntries[1];
static SVECTOR _gAkropolisSecurityRoomCollisionGridNormals[11];
static SVECTOR _gAkropolisSecurityRoomCollisionGridVerts[50];
static WorldCollisionGridFace _gAkropolisSecurityRoomCollisionGridFaces[24];
static s16 _gAkropolisSecurityRoomCollisionGridCells[46];
static s16* _gAkropolisSecurityRoomCollisionGridTable[2];
static WorldCollisionGrid _gAkropolisSecurityRoomCollisionGrid;
static WorldCollisionTrigger _gAkropolisSecurityRoomViewBoundaryTriggers[4];
static WorldCollisionTrigger _gAkropolisSecurityRoomActionTriggers[5];
static AreaResource _gAkropolisSecurityRoomActorResources[3];
extern AreaVariant gAkropolisSecurityRoomAreaVariants[4];
static WorldCoordPointLight _gAkropolisSecurityRoomPointLights[3];
static WorldCoordRoomLights _gAkropolisSecurityRoomLights;
static SpriteBatch _gAkropolisSecurityRoomView1SpriteBatches[2];
static SpriteSource _gAkropolisSecurityRoomView2SpriteSources[18];
static SpriteBatch _gAkropolisSecurityRoomView2SpriteBatches[3];
static SpriteBatch _gAkropolisSecurityRoomView3SpriteBatches[2];
static SpriteBatch _gAkropolisSecurityRoomView4SpriteBatches[2];
static SpriteSource _gAkropolisSecurityRoomMonitorFrameSpriteSources[10];
static SpriteBatch _gAkropolisSecurityRoomMonitorFrameSpriteBatches[3];
static SpriteSource _gAkropolisSecurityRoomReleasedLockSpriteSources[2];
static SpriteBatch _gAkropolisSecurityRoomReleasedLockSpriteBatches[4];
static SpriteBatch _gAkropolisSecurityRoomView7SpriteBatches[2];
static SpriteBatch _gAkropolisSecurityRoomUnusedEmptySpriteBatchLists[8][2];
static SpriteSource _gAkropolisSecurityRoomRightUnlockSpriteSources[84];
static SpriteBatch _gAkropolisSecurityRoomRightUnlockSpriteBatches[15];
extern SpriteView gAkropolisSecurityRoomSpriteViews[16];
extern ViewCamera gAkropolisSecurityRoomViewCameras[16];
static AreaApplyRec _gAkropolisSecurityRoomRightUnlockAreaUpdates[10];
static AreaApplyRec _gAkropolisSecurityRoomRightUnlockEarlyStoryAreaUpdates[1];
static AreaApplyRec _gAkropolisSecurityRoomRightUnlockLateStoryAreaUpdates[1];
static AreaApplyRec _gAkropolisSecurityRoomLeftUnlockAreaUpdates[1];
static WorldCollisionFootstepSounds _gAkropolisSecurityRoomFloorFootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisSecurityRoomSilentSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisSecurityRoomFloorSurfaceProperties;
extern WorldCollisionSurfaceProperties* gAkropolisSecurityRoomSurfacePropertiesByClass[8];
