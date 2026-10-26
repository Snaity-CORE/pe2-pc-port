/* rooms/akropolis/fountain/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/area.h"
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/light.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static WorldCollisionTrigger _gAkropolisFountainClimbTrigger;
static SVECTOR _gAkropolisFountainSprayOffset;
extern WorldCollisionRoomResources gAkropolisFountainRoomCollisionResources[2];
static u8 _gAkropolisFountainRoom2ViewMap[24];
extern u8* gAkropolisFountainViewMaps[2];
extern ViewCount gAkropolisFountainViewCounts[2];
extern WorldCoordRoomLighting gAkropolisFountainRoomLightingTable[2];
extern DirectionWarpEntry gAkropolisFountainWarpEntries[5];
static SVECTOR _gAkropolisFountainCollisionGridNormals[32];
static SVECTOR _gAkropolisFountainCollisionGridVerts[181];
static WorldCollisionGridFace _gAkropolisFountainCollisionGridFaces[66];
static s16 _gAkropolisFountainCollisionGridCells[326];
static s16* _gAkropolisFountainCollisionGridTable[16];
static WorldCollisionGrid _gAkropolisFountainCollisionGrid;
static WorldCollisionTrigger _gAkropolisFountainViewBoundaryTriggers[12];
static WorldCollisionTrigger _gAkropolisFountainActionTriggers[9];
static AreaResource _gAkropolisFountainVariant2Resources[2];
static AreaResource _gAkropolisFountainVariant7Resources[2];
extern AreaVariant gAkropolisFountainAreaVariants[11];
static WorldCoordPointLight _gAkropolisFountainPointLights[6];
static WorldCoordRoomLights _gAkropolisFountainRoomLights;
static SpriteBatch _gAkropolisFountainView1SpriteBatches[2];
static SpriteBatch _gAkropolisFountainView2SpriteBatches[2];
static SpriteSource _gAkropolisFountainView3SpriteSources[95];
static SpriteBatch _gAkropolisFountainView3SpriteBatches[9];
static SpriteSource _gAkropolisFountainView4SpriteSources[84];
static SpriteBatch _gAkropolisFountainView4SpriteBatches[15];
static SpriteSource _gAkropolisFountainView5SpriteSources[73];
static SpriteBatch _gAkropolisFountainView5SpriteBatches[8];
static SpriteSource _gAkropolisFountainView6SpriteSources[16];
static SpriteBatch _gAkropolisFountainView6SpriteBatches[3];
static SpriteSource _gAkropolisFountainView7SpriteSources[71];
static SpriteBatch _gAkropolisFountainView7SpriteBatches[9];
static SpriteSource _gAkropolisFountainView8SpriteSources[99];
static SpriteBatch _gAkropolisFountainView8SpriteBatches[18];
static SpriteBatch _gAkropolisFountainView9SpriteBatches[2];
static SpriteBatch _gAkropolisFountainView10SpriteBatches[2];
static SpriteSource _gAkropolisFountainView11SpriteSources[54];
static SpriteBatch _gAkropolisFountainView11SpriteBatches[9];
static SpriteSource _gAkropolisFountainView12SpriteSources[92];
static SpriteBatch _gAkropolisFountainView12SpriteBatches[15];
static SpriteSource _gAkropolisFountainView13SpriteSources[39];
static SpriteBatch _gAkropolisFountainView13SpriteBatches[8];
static SpriteSource _gAkropolisFountainView14SpriteSources[8];
static SpriteBatch _gAkropolisFountainView14SpriteBatches[3];
static SpriteBatch _gAkropolisFountainUnusedEmptySpriteBatches[2];
static SpriteBatch _gAkropolisFountainView16SpriteBatches[2];
static SpriteSource _gAkropolisFountainView17SpriteSources[28];
static SpriteBatch _gAkropolisFountainView17SpriteBatches[4];
static SpriteBatch _gAkropolisFountainView18SpriteBatches[2];
static SpriteBatch _gAkropolisFountainView19SpriteBatches[2];
static SpriteSource _gAkropolisFountainView20SpriteSources[9];
static SpriteBatch _gAkropolisFountainView20SpriteBatches[3];
static SpriteBatch _gAkropolisFountainView21SpriteBatches[2];
static SpriteBatch _gAkropolisFountainView22SpriteBatches[2];
extern SpriteView gAkropolisFountainSpriteViews[22];
extern ViewCamera gAkropolisFountainViewCameras[21];
static WorldCollisionFootstepSounds _gAkropolisFountainClass4FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisFountainClass5FootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisFountainDefaultSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisFountainClass2SurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisFountainClass4SurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisFountainClass5SurfaceProperties;
extern WorldCollisionSurfaceProperties* gAkropolisFountainSurfacePropertiesByClass[8];
