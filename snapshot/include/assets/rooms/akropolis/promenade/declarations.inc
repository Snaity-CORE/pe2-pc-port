/* rooms/akropolis/promenade/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
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

static AnimationPlayRequest _gAkropolisPromenadeBattleFollowupAnimationRequest;
static ActorTransform _gAkropolisPromenadeUnusedTransforms[2];
static EvsCommand _gAkropolisPromenadeBattleFollowupScript[15];
static EvsCommand _gAkropolisPromenadeBattleFollowupSkipScript[9];
static SVECTOR _gAkropolisPromenadeGlowPositions[3];
static SVECTOR _gAkropolisPromenadeGroundGlowPosition;
static SVECTOR _gAkropolisPromenadeLampGlowPositions[12];
static u16 _gAkropolisPromenadeGlowViewMask;
static u16 _gAkropolisPromenadeDripViewMask;
static u16 _gAkropolisPromenadeLampGlowViewMasks[12];
extern WorldCollisionRoomResources gAkropolisPromenadeRoomCollisionResources[2];
static u8 _gAkropolisPromenadeRoom2ViewMap[16];
extern u8* gAkropolisPromenadeViewMaps[2];
extern ViewCount gAkropolisPromenadeViewCounts[2];
extern WorldCoordRoomLighting gAkropolisPromenadeRoomLightingTable[2];
extern DirectionWarpEntry gAkropolisPromenadeWarpEntries[5];
static SVECTOR _gAkropolisPromenadeRoom1CollisionGridNormals[30];
static SVECTOR _gAkropolisPromenadeRoom1CollisionGridVerts[93];
static WorldCollisionGridFace _gAkropolisPromenadeRoom1CollisionGridFaces[38];
static s16 _gAkropolisPromenadeRoom1CollisionGridCells[144];
static s16* _gAkropolisPromenadeRoom1CollisionGridTable[10];
static WorldCollisionGrid _gAkropolisPromenadeRoom1CollisionGrid;
static SVECTOR _gAkropolisPromenadeRoom2CollisionGridNormals[30];
static SVECTOR _gAkropolisPromenadeRoom2CollisionGridVerts[103];
static WorldCollisionGridFace _gAkropolisPromenadeRoom2CollisionGridFaces[46];
static s16 _gAkropolisPromenadeRoom2CollisionGridCells[172];
static s16* _gAkropolisPromenadeRoom2CollisionGridTable[10];
static WorldCollisionGrid _gAkropolisPromenadeRoom2CollisionGrid;
static WorldCollisionTrigger _gAkropolisPromenadeViewBoundaryTriggers[6];
static WorldCollisionTrigger _gAkropolisPromenadeActionTriggers[6];
static AreaResource _gAkropolisPromenadeVariant1Resources[2];
static AreaResource _gAkropolisPromenadeVariant2Resources[3];
static AreaResource _gAkropolisPromenadeVariant3Resources[3];
static AreaResource _gAkropolisPromenadeVariant7Resources[2];
static AreaResource _gAkropolisPromenadeVariant8Resources[3];
extern AreaVariant gAkropolisPromenadeAreaVariants[17];
static WorldCoordPointLight _gAkropolisPromenadePointLights[25];
static WorldCoordRoomLights _gAkropolisPromenadeRoomLights;
static SpriteBatch _gAkropolisPromenadeViews1And13SpriteBatches[2];
static SpriteSource _gAkropolisPromenadeView2SpriteSources[74];
static SpriteBatch _gAkropolisPromenadeView2SpriteBatches[11];
static SpriteSource _gAkropolisPromenadeView3SpriteSources[17];
static SpriteBatch _gAkropolisPromenadeView3SpriteBatches[5];
static SpriteSource _gAkropolisPromenadeView4SpriteSources[56];
static SpriteBatch _gAkropolisPromenadeView4SpriteBatches[9];
static SpriteSource _gAkropolisPromenadeView5SpriteSources[83];
static SpriteBatch _gAkropolisPromenadeView5SpriteBatches[9];
static SpriteSource _gAkropolisPromenadeView6SpriteSources[63];
static SpriteBatch _gAkropolisPromenadeView6SpriteBatches[9];
static SpriteSource _gAkropolisPromenadeView7SpriteSources[32];
static SpriteBatch _gAkropolisPromenadeView7SpriteBatches[8];
static SpriteSource _gAkropolisPromenadeView8SpriteSources[46];
static SpriteBatch _gAkropolisPromenadeView8SpriteBatches[9];
static SpriteSource _gAkropolisPromenadeView9SpriteSources[78];
static SpriteBatch _gAkropolisPromenadeView9SpriteBatches[10];
static SpriteBatch _gAkropolisPromenadeView10SpriteBatches[2];
static SpriteBatch _gAkropolisPromenadeView11SpriteBatches[2];
static SpriteBatch _gAkropolisPromenadeView12SpriteBatches[2];
static SpriteBatch _gAkropolisPromenadeUnusedEmptySpriteBatches[2];
extern SpriteView gAkropolisPromenadeSpriteViews[13];
extern ViewCamera gAkropolisPromenadeViewCameras[13];
static PadScriptCmd _gAkropolisPromenadeArrivalMovieVibrationCommands[6];
static PadScriptVibrationSegment _gAkropolisPromenadeArrivalMovieVibrationSegments[2];
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass1FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass2FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass3FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass4FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisPromenadeSurfaceClass7FootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisPromenadeDefaultSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass1Properties;
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass2Properties;
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass3Properties;
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass4Properties;
static WorldCollisionSurfaceProperties _gAkropolisPromenadeSurfaceClass7Properties;
extern WorldCollisionSurfaceProperties* gAkropolisPromenadeSurfacePropertiesByClass[8];
