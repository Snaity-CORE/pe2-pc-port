/* rooms/mist/r18/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
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
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"

static AnimationBankCopyRequest _gMistR18PlayerBriefingBankCopyRequest;
static AnimationPlayRequest _gMistR18PlayerBaseClip1Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip48Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip49Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip50Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip51Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip52Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip53Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip54Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip55Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip56Request;
static AnimationPlayRequest _gMistR18PlayerBriefingClip57Request;
static AnimationPlayRequest _gMistR18BaldwinResetClip1Request;
static AnimationPlayRequest _gMistR18BaldwinIntroClip1Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip2Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip3Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip4Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip5Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip6Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip7Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip8Request;
static AnimationPlayRequest _gMistR18BaldwinBriefingClip9Request;
static AnimationPlayRequest _gMistR18BaldwinReminderClip10Request;
static AnimationPlayRequest _gMistR18RupertResetClip1Request;
static AnimationPlayRequest _gMistR18RupertIntroClip1Request;
static AnimationPlayRequest _gMistR18RupertBriefingClip2Request;
static AnimationPlayRequest _gMistR18RupertBriefingClip3Request;
static AnimationPlayRequest _gMistR18RupertBriefingClip4Request;
static AnimationPlayRequest _gMistR18RupertBriefingClip5Request;
static AnimationPlayRequest _gMistR18RupertBriefingClip6Request;
static AnimationPlayRequest _gMistR18RupertBriefingClip7Request;
static AnimationPlayRequest _gMistR18RupertReminderClip8Request;
static ActorTransform _gMistR18BriefingPlayerPlacement;
static ActorCommand _gMistR18BaldwinShowFirstHeldModelCommand;
static ActorCommand _gMistR18BaldwinHideFirstHeldModelCommand;
static ActorCommand _gMistR18BaldwinShowSecondHeldModelCommand;
static ActorCommand _gMistR18BaldwinHideSecondHeldModelCommand;
static EvsCommand _gMistR18IntroScript[56];
static EvsCommand _gMistR18FirstHeldModelScript[37];
static EvsCommand _gMistR18DepartureScript[41];
static EvsCommand _gMistR18PlacePropScript[16];
static EvsCommand _gMistR18OpenKeyItemsScript[16];
static EvsCommand _gMistR18DryfieldMapReminderScript[20];
static EvsCommand _gMistR18SkipIntroScript[8];
static EvsCommand _gMistR18SkipFirstHeldModelScript[8];
static EvsCommand _gMistR18SkipDepartureScript[3];
static EvsCommand _gMistR18SkipPlacePropScript[7];
extern WorldCollisionRoomResources gMistR18CollisionRoomResources[1];
extern u8* gMistR18ViewMaps[1];
extern ViewCount gMistR18ViewCounts[2];
extern WorldCoordRoomLighting gMistR18RoomLightingTable[1];
extern DirectionWarpEntry gMistR18WarpEntries[1];
static SVECTOR _gMistR18FloorCollisionGridNormals[1];
static SVECTOR _gMistR18FloorCollisionGridVerts[4];
static WorldCollisionGridFace _gMistR18FloorCollisionGridFaces[1];
static s16 _gMistR18FloorCollisionGridCells[24];
static s16* _gMistR18FloorCollisionGridTable[12];
static WorldCollisionGrid _gMistR18FloorCollisionGrid;
extern ViewCamera gMistR18ViewCameras[10];
static SpriteBatch _gMistR18View1SpriteBatches[2];
static SpriteBatch _gMistR18View2SpriteBatches[4];
static SpriteBatch _gMistR18View3SpriteBatches[2];
static SpriteSource _gMistR18View4SpriteSources[10];
static SpriteBatch _gMistR18View4SpriteBatches[3];
static SpriteBatch _gMistR18View5SpriteBatches[2];
static SpriteSource _gMistR18View6SpriteSources[17];
static SpriteBatch _gMistR18View6SpriteBatches[3];
static SpriteBatch _gMistR18View7SpriteBatches[2];
static SpriteBatch _gMistR18View8SpriteBatches[2];
static SpriteBatch _gMistR18View9SpriteBatches[2];
static SpriteBatch _gMistR18View10SpriteBatches[2];
extern SpriteView gMistR18SpriteViews[10];
static AreaResource _gMistR18BriefingActorResources[3];
extern AreaVariant gMistR18AreaVariants[13];
static WorldCoordPointLight _gMistR18PointLights[5];
static WorldCoordRoomLights _gMistR18RoomLights;
static WorldCollisionFootstepSounds _gMistR18FootstepSounds;
static WorldCollisionSurfaceProperties _gMistR18SurfaceProperties;
extern WorldCollisionSurfaceProperties* gMistR18SurfacePropertiesByClass[8];
