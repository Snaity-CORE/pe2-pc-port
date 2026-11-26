/* rooms/mist/parking/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/area.h"
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/evs.h"
#include "gameplay/light.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

extern EvsCommand gMistParkingJodieShopTalkScript[8];
extern EvsCommand gMistParkingJodieOpenShopScript[10];
extern EvsCommand gMistParkingJodieShopReplyScript[8];
extern EvsCommand gMistParkingJodieShopFarewellScript[8];
extern WorldCollisionRoomResources gMistParkingRoomCollisionResources[4];
static u8 _gMistParkingEarlyChapterViewMap[20];
extern u8* gMistParkingViewMaps[4];
extern ViewCount gMistParkingViewCounts[4];
extern WorldCoordRoomLighting gMistParkingRoomLightingTable[4];
extern DirectionWarpEntry gMistParkingWarpEntries[4];
static SVECTOR _gMistParkingCollisionGridNormals[13];
static SVECTOR _gMistParkingCollisionGridVerts[125];
static WorldCollisionGridFace _gMistParkingCollisionGridFaces[66];
static s16 _gMistParkingCollisionGridCells[434];
static s16* _gMistParkingCollisionGridTable[28];
extern WorldCollisionGrid gMistParkingCollisionGrid;
extern ViewCamera gMistParkingViewCameras[20];
static SpriteBatch _gMistParkingView1SpriteBatches[2];
static SpriteSource _gMistParkingView2SpriteSources[21];
static SpriteBatch _gMistParkingView2SpriteBatches[3];
static SpriteBatch _gMistParkingView3SpriteBatches[2];
static SpriteSource _gMistParkingView4SpriteSources[26];
static SpriteBatch _gMistParkingView4SpriteBatches[5];
static SpriteSource _gMistParkingView5SpriteSources[39];
static SpriteBatch _gMistParkingView5SpriteBatches[5];
static SpriteSource _gMistParkingView6SpriteSources[37];
static SpriteBatch _gMistParkingView6SpriteBatches[7];
static SpriteSource _gMistParkingView7SpriteSources[9];
static SpriteBatch _gMistParkingView7SpriteBatches[3];
static SpriteBatch _gMistParkingView8SpriteBatches[2];
static SpriteBatch _gMistParkingView9SpriteBatches[2];
static SpriteSource _gMistParkingView10SpriteSources[25];
static SpriteBatch _gMistParkingView10SpriteBatches[3];
static SpriteSource _gMistParkingView11SpriteSources[4];
static SpriteBatch _gMistParkingView11SpriteBatches[3];
static SpriteBatch _gMistParkingView12SpriteBatches[2];
static SpriteBatch _gMistParkingView13SpriteBatches[2];
static SpriteBatch _gMistParkingView14SpriteBatches[2];
static SpriteBatch _gMistParkingView15SpriteBatches[2];
static SpriteBatch _gMistParkingView16SpriteBatches[2];
static SpriteBatch _gMistParkingView17SpriteBatches[2];
static SpriteSource _gMistParkingView18SpriteSources[35];
static SpriteBatch _gMistParkingView18SpriteBatches[5];
static SpriteSource _gMistParkingView19SpriteSources[43];
static SpriteBatch _gMistParkingView19SpriteBatches[7];
static SpriteBatch _gMistParkingView20SpriteBatches[2];
extern SpriteView gMistParkingSpriteViews[20];
static WorldCollisionTrigger _gMistParkingViewBoundaryTriggers[12];
static WorldCollisionTrigger _gMistParkingEarlyChapterActionTriggers[13];
static WorldCollisionTrigger _gMistParkingLaterChapterActionTriggers[14];
static WorldCoordPointLight _gMistParkingRoom1PointLights[28];
static WorldCoordRoomLights _gMistParkingRoom1Lights;
static WorldCoordPointLight _gMistParkingRoom2PointLights[2];
static WorldCoordRoomLights _gMistParkingRoom2Lights;
static AreaResource _gMistParkingActorResources[3];
extern AreaVariant gMistParkingAreaVariants[13];
static WorldCoordRoomAmbientEntry _gMistParkingRoom2AmbientByView[21];
static WorldCollisionFootstepSounds _gMistParkingFootstepSounds;
static WorldCollisionSurfaceProperties _gMistParkingDefaultSurfaceProperties;
static WorldCollisionSurfaceProperties _gMistParkingSurfaceClass1Properties;
static WorldCollisionSurfaceProperties _gMistParkingSurfaceClass2Properties;
static WorldCollisionSurfaceProperties _gMistParkingSurfaceClass3Properties;
extern WorldCollisionSurfaceProperties* gMistParkingSurfacePropertiesByClass[8];
