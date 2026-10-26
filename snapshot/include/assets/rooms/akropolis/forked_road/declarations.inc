/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
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

static AnimationBankCopyRequest _gAkropolisForkedRoadAmbushPlayerBankCopyRequest;
static AnimationPlayRequest _gAkropolisForkedRoadUnusedBank1Animation1Request;
static AnimationPlayRequest _gAkropolisForkedRoadAmbushPlayerStandRequest;
static AnimationPlayRequest _gAkropolisForkedRoadUnusedBank1AnimationRequests[2];
extern EvsCommand gAkropolisForkedRoadAmbushScript[8];
static SVECTOR _gAkropolisForkedRoadWallLampPositions[14];
static u16 _gAkropolisForkedRoadWallLampViewMasks[14];
extern WorldCollisionRoomResources gAkropolisForkedRoadRoomCollisionResources[3];
static u8 _gAkropolisForkedRoadRoom2ViewMap[12];
static u8 _gAkropolisForkedRoadRoom3ViewMap[12];
extern u8* gAkropolisForkedRoadViewMaps[3];
extern ViewCount gAkropolisForkedRoadViewCounts[3];
extern WorldCoordRoomLighting gAkropolisForkedRoadRoomLightingTable[3];
extern DirectionWarpEntry gAkropolisForkedRoadWarpEntries[5];
static SVECTOR _gAkropolisForkedRoadCollisionGridNormals[18];
static SVECTOR _gAkropolisForkedRoadCollisionGridVerts[118];
static WorldCollisionGridFace _gAkropolisForkedRoadCollisionGridFaces[38];
static s16 _gAkropolisForkedRoadCollisionGridCells[244];
static s16* _gAkropolisForkedRoadCollisionGridTable[24];
static WorldCollisionGrid _gAkropolisForkedRoadCollisionGrid;
static WorldCollisionTrigger _gAkropolisForkedRoadViewBoundaryTriggers[6];
static WorldCollisionTrigger _gAkropolisForkedRoadActionTriggers[7];
static AreaResource _gAkropolisForkedRoadVariant2Resources[2];
static AreaResource _gAkropolisForkedRoadVariant3Resources[3];
static AreaResource _gAkropolisForkedRoadVariant4Resources[2];
static AreaResource _gAkropolisForkedRoadVariant7Resources[3];
static AreaResource _gAkropolisForkedRoadVariant8Resources[2];
static AreaResource _gAkropolisForkedRoadVariant9Resources[2];
static AreaResource _gAkropolisForkedRoadVariant10Resources[3];
static AreaResource _gAkropolisForkedRoadVariant11Resources[2];
static AreaResource _gAkropolisForkedRoadVariant12Resources[2];
static AreaResource _gAkropolisForkedRoadVariant13Resources[2];
static AreaResource _gAkropolisForkedRoadVariant14Resources[2];
static AreaResource _gAkropolisForkedRoadVariant15Resources[2];
static AreaResource _gAkropolisForkedRoadVariant16Resources[2];
static AreaResource _gAkropolisForkedRoadVariant20Resources[2];
static AreaResource _gAkropolisForkedRoadVariant21Resources[3];
static AreaResource _gAkropolisForkedRoadVariant22Resources[5];
extern AreaVariant gAkropolisForkedRoadAreaVariants[24];
static SpriteBatch _gAkropolisForkedRoadView1SpriteBatches[2];
static SpriteSource _gAkropolisForkedRoadView2SpriteSources[85];
static SpriteBatch _gAkropolisForkedRoadView2SpriteBatches[12];
static SpriteSource _gAkropolisForkedRoadViews3And12SpriteSources[6];
static SpriteBatch _gAkropolisForkedRoadViews3And12SpriteBatches[3];
static SpriteBatch _gAkropolisForkedRoadView4SpriteBatches[2];
static SpriteSource _gAkropolisForkedRoadView5SpriteSources[6];
static SpriteBatch _gAkropolisForkedRoadView5SpriteBatches[3];
static SpriteBatch _gAkropolisForkedRoadView6SpriteBatches[2];
static SpriteBatch _gAkropolisForkedRoadUnusedEmptySpriteBatches1[2];
static SpriteSource _gAkropolisForkedRoadViews7And8SpriteSources[58];
static SpriteBatch _gAkropolisForkedRoadViews7And8SpriteBatches[6];
static SpriteBatch _gAkropolisForkedRoadView9SpriteBatches[2];
static SpriteBatch _gAkropolisForkedRoadView10SpriteBatches[2];
static SpriteSource _gAkropolisForkedRoadView11SpriteSources[62];
static SpriteBatch _gAkropolisForkedRoadView11SpriteBatches[7];
static SpriteBatch _gAkropolisForkedRoadUnusedEmptySpriteBatches2[2];
extern SpriteView gAkropolisForkedRoadSpriteViews[12];
static WorldCoordPointLight _gAkropolisForkedRoadPointLights[24];
static WorldCoordRoomLights _gAkropolisForkedRoadRoomLights;
extern ViewCamera gAkropolisForkedRoadViewCameras[12];
static PadScriptCmd _gAkropolisForkedRoadReturnMovieVibrationCommands[6];
static PadScriptVibrationSegment _gAkropolisForkedRoadReturnMovieVibrationSegments[2];
static PadScriptCmd _gAkropolisForkedRoadOutboundMovieVibrationCommands[6];
static PadScriptVibrationSegment _gAkropolisForkedRoadOutboundMovieVibrationSegments[2];
static WorldCollisionFootstepSounds _gAkropolisForkedRoadSurfaceClass4FootstepSounds;
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadDefaultSurfaceProperties;
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadSurfaceClass2Properties;
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadSurfaceClass3Properties;
static WorldCollisionSurfaceProperties _gAkropolisForkedRoadSurfaceClass4Properties;
extern WorldCollisionSurfaceProperties* gAkropolisForkedRoadSurfacePropertiesByClass[8];
