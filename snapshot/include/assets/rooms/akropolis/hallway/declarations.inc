/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
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

extern WorldCollisionRoomResources gAkropolisHallwayRoomCollisionResources[1];
extern u8* gAkropolisHallwayViewMaps[1];
extern ViewCount gAkropolisHallwayViewCounts[1];
extern WorldCoordRoomLighting gAkropolisHallwayRoomLightingTable[1];
extern DirectionWarpEntry gAkropolisHallwayWarpEntries[4];
static SVECTOR _gAkropolisHallwayCollisionGridNormals[10];
static SVECTOR _gAkropolisHallwayCollisionGridVerts[35];
static WorldCollisionGridFace _gAkropolisHallwayCollisionGridFaces[17];
static s16 _gAkropolisHallwayCollisionGridCells[30];
static s16* _gAkropolisHallwayCollisionGridTable[2];
static WorldCollisionGrid _gAkropolisHallwayCollisionGrid;
static WorldCollisionTrigger _gAkropolisHallwayViewBoundaryTriggers[4];
static WorldCollisionTrigger _gAkropolisHallwayActionTriggers[9];
static AreaResource _gAkropolisHallwayVariant1Resources[2];
static AreaResource _gAkropolisHallwayVariant2Resources[2];
static AreaResource _gAkropolisHallwayVariant3Resources[2];
static AreaResource _gAkropolisHallwayVariant4Resources[3];
extern AreaVariant gAkropolisHallwayAreaVariants[13];
static WorldCoordPointLight _gAkropolisHallwayPointLights[3];
static WorldCoordRoomLights _gAkropolisHallwayRoomLights;
static SpriteBatch _gAkropolisHallwayView1SpriteBatches[2];
static SpriteBatch _gAkropolisHallwayView2SpriteBatches[2];
static SpriteBatch _gAkropolisHallwayView3SpriteBatches[2];
static SpriteBatch _gAkropolisHallwayView4SpriteBatches[2];
static SpriteBatch _gAkropolisHallwayView5SpriteBatches[2];
extern SpriteView gAkropolisHallwaySpriteViews[5];
extern ViewCamera gAkropolisHallwayViewCameras[5];
static WorldCollisionFootstepSounds _gAkropolisHallwayFootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisHallwaySilentBlockingSurface;
static WorldCollisionSurfaceProperties _gAkropolisHallwayWalkablePassThroughSurface;
static WorldCollisionSurfaceProperties _gAkropolisHallwayWalkableBlockingSurface;
extern WorldCollisionSurfaceProperties* gAkropolisHallwaySurfacePropertiesByClass[8];
