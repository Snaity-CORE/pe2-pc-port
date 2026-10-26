/* rooms/akropolis/square/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/area.h"
#include "gameplay/area_flags.h"
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/evs.h"
#include "gameplay/light.h"
#include "gameplay/message.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static AnimationPlayRequest _gAkropolisSquareArrivalWeaponAnimation;
static EvsCommand _gAkropolisSquareTelephoneSceneScript[7];
static EvsCommand _gAkropolisSquareTelephoneSceneSkipScript[8];
static EvsCommand _gAkropolisSquareArrivalEventScript[8];
static EvsCommand _gAkropolisSquareArrivalSkipScript[9];
static s16 _gAkropolisSquareGlowSineSamples[24];
extern WorldCollisionRoomResources gAkropolisSquareRoomCollisionResources[1];
extern u8* gAkropolisSquareViewMaps[1];
extern ViewCount gAkropolisSquareViewCounts[1];
extern WorldCoordRoomLighting gAkropolisSquareRoomLightingTable[1];
extern DirectionWarpEntry gAkropolisSquareWarpEntries[7];
static SVECTOR _gAkropolisSquareCollisionGridNormals[82];
static SVECTOR _gAkropolisSquareCollisionGridVerts[262];
static WorldCollisionGridFace _gAkropolisSquareCollisionGridFaces[115];
static s16 _gAkropolisSquareCollisionGridCells[506];
static s16* _gAkropolisSquareCollisionGridTable[16];
static WorldCollisionGrid _gAkropolisSquareCollisionGrid;
static WorldCollisionTrigger _gAkropolisSquareViewBoundaryTriggers[16];
static WorldCollisionTrigger _gAkropolisSquareActionTriggers[26];
static AreaResource _gAkropolisSquareVariant1Resources[2];
extern AreaVariant gAkropolisSquareAreaVariants[3];
static WorldCoordPointLight _gAkropolisSquarePointLights[16];
static WorldCoordRoomLights _gAkropolisSquareRoomLights;
static WorldCoordRoomAmbientEntry _gAkropolisSquareAmbientTable[16];
static SpriteBatch _gAkropolisSquareView1SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView2SpriteBatches[2];
static SpriteSource _gAkropolisSquareView3SpriteSources[73];
static SpriteBatch _gAkropolisSquareView3SpriteBatches[11];
static SpriteSource _gAkropolisSquareView4SpriteSources[41];
static SpriteBatch _gAkropolisSquareView4SpriteBatches[5];
static SpriteSource _gAkropolisSquareView5SpriteSources[31];
static SpriteBatch _gAkropolisSquareView5SpriteBatches[5];
static SpriteSource _gAkropolisSquareView6SpriteSources[91];
static SpriteBatch _gAkropolisSquareView6SpriteBatches[12];
static SpriteSource _gAkropolisSquareView7SpriteSources[94];
static SpriteBatch _gAkropolisSquareView7SpriteBatches[18];
static SpriteSource _gAkropolisSquareView8SpriteSources[53];
static SpriteBatch _gAkropolisSquareView8SpriteBatches[13];
static SpriteBatch _gAkropolisSquareView9SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView10SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView11SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView12SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView13SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView14SpriteBatches[2];
static SpriteBatch _gAkropolisSquareView15SpriteBatches[2];
extern SpriteView gAkropolisSquareSpriteViews[15];
extern ViewCamera gAkropolisSquareViewCameras[15];
static WorldCollisionFootstepSounds _gAkropolisSquareFootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisSquareDefaultSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisSquareClass4SurfaceProperties;
extern WorldCollisionSurfaceProperties* gAkropolisSquareSurfacePropertiesByClass[8];
extern AreaApplyRec gAkropolisSquarePostCutsceneAreaUpdates[4];
