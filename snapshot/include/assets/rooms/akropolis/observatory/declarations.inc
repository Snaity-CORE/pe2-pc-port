/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/area.h"
#include "gameplay/areaplace.h"
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/light.h"
#include "gameplay/pad_script.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static SVECTOR _gAkropolisObservatoryLensFlareAnchors[8];
static u16 _gAkropolisObservatoryLensFlareViewMasks[8];
extern WorldCollisionRoomResources gAkropolisObservatoryRoomCollisionResources[2];
static u8 _gAkropolisObservatoryRoom2ViewMap[8];
extern u8* gAkropolisObservatoryViewMaps[2];
extern ViewCount gAkropolisObservatoryViewCounts[2];
extern WorldCoordRoomLighting gAkropolisObservatoryRoomLightingTable[2];
extern DirectionWarpEntry gAkropolisObservatoryWarpEntries[4];
static SVECTOR _gAkropolisObservatoryCollisionGridNormals[22];
static SVECTOR _gAkropolisObservatoryCollisionGridVerts[155];
static WorldCollisionGridFace _gAkropolisObservatoryCollisionGridFaces[62];
static s16 _gAkropolisObservatoryCollisionGridCells[232];
static s16* _gAkropolisObservatoryCollisionGridTable[9];
static WorldCollisionGrid _gAkropolisObservatoryCollisionGrid;
static WorldCollisionTrigger _gAkropolisObservatoryViewBoundaryTriggers[10];
static WorldCollisionTrigger _gAkropolisObservatoryActionTriggers[9];
static AreaResource _gAkropolisObservatoryVariant1Resources[2];
static AreaResource _gAkropolisObservatoryVariant2Resources[2];
static AreaResource _gAkropolisObservatoryVariant3Resources[2];
static AreaResource _gAkropolisObservatoryVariant7Resources[2];
static AreaResource _gAkropolisObservatoryVariant10Resources[2];
static AreaResource _gAkropolisObservatoryVariant11Resources[2];
static AreaPlacement _gAkropolisObservatoryVariant11Placements[2];
static AreaResource _gAkropolisObservatoryVariants12And13Resources[2];
static AreaPlacement _gAkropolisObservatoryVariant12Placements[2];
static AreaPlacement _gAkropolisObservatoryVariant13Placements[7];
static AreaResource _gAkropolisObservatoryVariant14Resources[2];
static AreaPlacement _gAkropolisObservatoryVariant14Placements[3];
static AreaResource _gAkropolisObservatoryVariant15Resources[1];
static AreaPlacement _gAkropolisObservatoryVariant15Placements[3];
static AreaResource _gAkropolisObservatoryVariant16Resources[2];
static AreaPlacement _gAkropolisObservatoryVariant16Placements[2];
static AreaResource _gAkropolisObservatoryVariant17Resources[2];
static AreaPlacement _gAkropolisObservatoryVariant17Placements[2];
extern AreaVariant gAkropolisObservatoryAreaVariants[19];
static WorldCoordPointLight _gAkropolisObservatoryPointLights[12];
static WorldCoordRoomLights _gAkropolisObservatoryRoomLights;
static SpriteBatch _gAkropolisObservatoryView1SpriteBatches[2];
static SpriteSource _gAkropolisObservatoryView2SpriteSources[49];
static SpriteBatch _gAkropolisObservatoryView2SpriteBatches[7];
static SpriteSource _gAkropolisObservatoryView3SpriteSources[61];
static SpriteBatch _gAkropolisObservatoryView3SpriteBatches[22];
static SpriteSource _gAkropolisObservatoryView4SpriteSources[67];
static SpriteBatch _gAkropolisObservatoryView4SpriteBatches[12];
static SpriteBatch _gAkropolisObservatoryView5SpriteBatches[2];
static SpriteSource _gAkropolisObservatoryView6SpriteSources[29];
static SpriteBatch _gAkropolisObservatoryView6SpriteBatches[6];
static SpriteSource _gAkropolisObservatoryView7SpriteSources[36];
static SpriteBatch _gAkropolisObservatoryView7SpriteBatches[8];
static SpriteSource _gAkropolisObservatoryView8SpriteSources[81];
static SpriteBatch _gAkropolisObservatoryView8SpriteBatches[10];
extern SpriteView gAkropolisObservatorySpriteViews[8];
extern ViewCamera gAkropolisObservatoryViewCameras[8];
extern PadScriptCmd gAkropolisObservatoryForkedRoadArrivalVibrationCommands[6];
extern PadScriptVibrationSegment gAkropolisObservatoryForkedRoadArrivalVibrationSegments[2];
extern PadScriptCmd gAkropolisObservatoryPromenadeArrivalVibrationCommands[6];
extern PadScriptVibrationSegment gAkropolisObservatoryPromenadeArrivalVibrationSegments[2];
static WorldCollisionFootstepSounds _gAkropolisObservatoryFootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisObservatorySilentSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisObservatoryFootstepSurfaceProperties;
extern WorldCollisionSurfaceProperties* gAkropolisObservatorySurfacePropertiesByClass[8];
