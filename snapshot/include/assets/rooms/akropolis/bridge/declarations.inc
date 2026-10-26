/* rooms/akropolis/bridge/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/area.h"
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/evs.h"
#include "gameplay/light.h"
#include "gameplay/message.h"
#include "gameplay/pad_script.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static AnimationPlayRequest _gAkropolisBridgeArrivalWeaponAnimationRequest;
static EvsSceneKey _gAkropolisBridgeArrivalSceneKey;
static EvsCommand _gAkropolisBridgeArrivalScript[26];
static EvsCommand _gAkropolisBridgeArrivalSkipScript[11];
static SVECTOR _gAkropolisBridgeStarGlowOffsets[7];
static SVECTOR _gAkropolisBridgeGroundGlowOffsets[7];
static SVECTOR _gAkropolisBridgeLampGlowOffsets[12];
static u16 _gAkropolisBridgeStarGlowViewMasks[8];
static u16 _gAkropolisBridgeLampGlowViewMasks[12];
static SVECTOR _gAkropolisBridgePlayerWaterPartPositions[2];
static SVECTOR _gAkropolisBridgeWarmDiscWorldPosition;
static SVECTOR _gAkropolisBridgeCoolDiscWorldPosition;
extern WorldCollisionRoomResources gAkropolisBridgeRoomCollisionResources[2];
static u8 _gAkropolisBridgeRoom2ViewMap[12];
extern u8* gAkropolisBridgeViewMaps[2];
extern ViewCount gAkropolisBridgeViewCounts[2];
extern WorldCoordRoomLighting gAkropolisBridgeRoomLightingTable[2];
extern DirectionWarpEntry gAkropolisBridgeWarpEntries[2];
static SVECTOR _gAkropolisBridgeRoom1CollisionNormals[19];
static SVECTOR _gAkropolisBridgeRoom1CollisionVerts[193];
static WorldCollisionGridFace _gAkropolisBridgeRoom1CollisionFaces[92];
static s16 _gAkropolisBridgeRoom1CollisionCells[274];
static s16* _gAkropolisBridgeRoom1CollisionTable[25];
static WorldCollisionGrid _gAkropolisBridgeRoom1Collision;
static SVECTOR _gAkropolisBridgeRoom2CollisionNormals[17];
static SVECTOR _gAkropolisBridgeRoom2CollisionVerts[205];
static WorldCollisionGridFace _gAkropolisBridgeRoom2CollisionFaces[93];
static s16 _gAkropolisBridgeRoom2CollisionCells[274];
static s16* _gAkropolisBridgeRoom2CollisionTable[25];
static WorldCollisionGrid _gAkropolisBridgeRoom2Collision;
static WorldCollisionTrigger _gAkropolisBridgeViewBoundaryTriggers[4];
static WorldCollisionTrigger _gAkropolisBridgeRoom1ActionTriggers[7];
static WorldCollisionTrigger _gAkropolisBridgeRoom2ActionTriggers[7];
static SpriteBatch _gAkropolisBridgeView1SpriteBatches[2];
static SpriteSource _gAkropolisBridgeView2SpriteSources[104];
static SpriteBatch _gAkropolisBridgeView2SpriteBatches[13];
static SpriteSource _gAkropolisBridgeView3SpriteSources[112];
static SpriteBatch _gAkropolisBridgeView3SpriteBatches[19];
static SpriteSource _gAkropolisBridgeView4SpriteSources[153];
static SpriteBatch _gAkropolisBridgeView4SpriteBatches[17];
static SpriteSource _gAkropolisBridgeView5SpriteSources[112];
static SpriteBatch _gAkropolisBridgeView5SpriteBatches[18];
static SpriteSource _gAkropolisBridgeView6SpriteSources[142];
static SpriteBatch _gAkropolisBridgeView6SpriteBatches[19];
static SpriteSource _gAkropolisBridgeView7SpriteSources[133];
static SpriteBatch _gAkropolisBridgeView7SpriteBatches[18];
static SpriteSource _gAkropolisBridgeKeypadSpriteSources[40];
static SpriteBatch _gAkropolisBridgeKeypadSpriteBatches[37];
static SpriteBatch _gAkropolisBridgeView9SpriteBatches[2];
static SpriteSource _gAkropolisBridgeArrivalSpriteSources[9];
static SpriteBatch _gAkropolisBridgeArrivalSpriteBatches[5];
extern SpriteView gAkropolisBridgeSpriteViews[10];
static AreaResource _gAkropolisBridgeDefaultAreaResources[2];
static AreaResource _gAkropolisBridgeVariant2AreaResources[1];
static AreaResource _gAkropolisBridgeVariant3AreaResources[1];
extern AreaVariant gAkropolisBridgePlacementVariants[12];
static WorldCoordPointLight _gAkropolisBridgePointLights[25];
static WorldCoordRoomLights _gAkropolisBridgeRoomLights;
extern ViewCamera gAkropolisBridgeViewCameras[10];
static PadScriptCmd _gAkropolisBridgeKeypadMovieVibrationCommands[6];
static PadScriptVibrationSegment _gAkropolisBridgeKeypadMovieVibrationSegments[6];
static PadScriptCmd _gAkropolisBridgeArrivalVibrationCommands[6];
static PadScriptVibrationSegment _gAkropolisBridgeArrivalVibrationSegments[5];
static WorldCollisionFootstepSounds _gAkropolisBridgeSurface2FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisBridgeSurface4FootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisBridgeBlockingSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface1Properties;
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface2Properties;
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface4Properties;
static WorldCollisionSurfaceProperties _gAkropolisBridgeSurface7Properties;
extern WorldCollisionSurfaceProperties* gAkropolisBridgeSurfacePropertiesByClass[8];
