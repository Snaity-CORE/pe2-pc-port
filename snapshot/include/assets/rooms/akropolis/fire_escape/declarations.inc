/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
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

static s16 _gAkropolisFireEscapeGlowCircleSamples[24];
extern WorldCollisionRoomResources gAkropolisFireEscapeCollisionRoomResources[1];
extern u8* gAkropolisFireEscapeViewMaps[1];
extern ViewCount gAkropolisFireEscapeViewCounts[1];
extern WorldCoordRoomLighting gAkropolisFireEscapeRoomLightingTable[1];
extern DirectionWarpEntry gAkropolisFireEscapeWarpEntries[3];
extern SVECTOR gAkropolisFireEscapeCollisionGridNormals[10];
extern SVECTOR gAkropolisFireEscapeCollisionGridVerts[55];
extern WorldCollisionGridFace gAkropolisFireEscapeCollisionGridFaces[23];
static s16 _gAkropolisFireEscapeCollisionGridCells[122];
static s16* _gAkropolisFireEscapeCollisionGridTable[9];
static WorldCollisionGrid _gAkropolisFireEscapeCollisionGrid;
static WorldCollisionTrigger _gAkropolisFireEscapeViewBoundaryTriggers[8];
static WorldCollisionTrigger _gAkropolisFireEscapeActionTriggers[12];
static WorldCollisionOccluder _gAkropolisFireEscapeOccluders[2];
static AreaResource _gAkropolisFireEscapeActorResources[2];
extern AreaVariant gAkropolisFireEscapeAreaVariants[5];
static WorldCoordPointLight _gAkropolisFireEscapePointLights[5];
static WorldCoordRoomLights _gAkropolisFireEscapeRoomLights;
static SpriteBatch _gAkropolisFireEscapeView1SpriteBatches[2];
static SpriteBatch _gAkropolisFireEscapeView2SpriteBatches[2];
static SpriteSource _gAkropolisFireEscapeView3SpriteSources[13];
static SpriteBatch _gAkropolisFireEscapeView3SpriteBatches[3];
static SpriteBatch _gAkropolisFireEscapeView4SpriteBatches[2];
static SpriteSource _gAkropolisFireEscapeView5SpriteSources[5];
static SpriteBatch _gAkropolisFireEscapeView5SpriteBatches[3];
static SpriteBatch _gAkropolisFireEscapeView6SpriteBatches[2];
static SpriteDrawArea _gAkropolisFireEscapeView6SpriteDrawAreas[2];
static SpriteSource _gAkropolisFireEscapeView7SpriteSources[5];
static SpriteBatch _gAkropolisFireEscapeView7SpriteBatches[3];
static SpriteBatch _gAkropolisFireEscapeView8SpriteBatches[2];
static SpriteBatch _gAkropolisFireEscapeViews9And10SpriteBatches[2];
static SpriteDrawArea _gAkropolisFireEscapeView9SpriteDrawAreas[2];
static SpriteBatch _gAkropolisFireEscapeUnusedSpriteBatches[2];
extern SpriteView gAkropolisFireEscapeSpriteViews[10];
extern ViewCamera gAkropolisFireEscapeViewCameras[9];
static WorldCollisionFootstepSounds _gAkropolisFireEscapeSurfaceClass1FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisFireEscapeSurfaceClass3FootstepSounds;
static WorldCollisionFootstepSounds _gAkropolisFireEscapeDefaultFootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeDefaultSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass1Properties;
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass2Properties;
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass3Properties;
static WorldCollisionSurfaceProperties _gAkropolisFireEscapeSurfaceClass6Properties;
extern WorldCollisionSurfaceProperties* gAkropolisFireEscapeSurfacePropertiesByClass[8];
