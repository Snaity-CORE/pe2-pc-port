/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/view.h"
#include "rooms/rooms_shared_8018055c.h"

static DATA_CONST JukeboxTrackLists _gJukeboxTrackLists;
static _MistShootingGalleryTargetScore _gMistShootingGalleryTargetScores[13];
static SVECTOR _gMistShootingGalleryStoryBarrierCollisionTemplateNormals[3];
static SVECTOR _gMistShootingGalleryStoryBarrierCollisionTemplateVerts[8];
static WorldCollisionGridFace _gMistShootingGalleryStoryBarrierCollisionTemplateFaces[3];
static s16 _gMistShootingGalleryStoryBarrierCollisionTemplateCells[4];
static s16* _gMistShootingGalleryStoryBarrierCollisionTemplateTable[1];
static WorldCollisionGrid _gMistShootingGalleryStoryBarrierCollisionTemplate;
static SVECTOR _gMistShootingGalleryTrainingBarrierCollisionTemplateNormals[1];
static SVECTOR _gMistShootingGalleryTrainingBarrierCollisionTemplateVerts[4];
static WorldCollisionGridFace _gMistShootingGalleryTrainingBarrierCollisionTemplateFaces[1];
static s16 _gMistShootingGalleryTrainingBarrierCollisionTemplateCells[2];
static s16* _gMistShootingGalleryTrainingBarrierCollisionTemplateTable[1];
static WorldCollisionGrid _gMistShootingGalleryTrainingBarrierCollisionTemplate;
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingReplayTracks[3];
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingBountyTracks[3];
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingScavengerTracks[3];
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingNightmareTracks[3];
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingUnclearedTracks[3];
static JukeboxTrack _gMistShootingGalleryJukeboxRegularReplayTracks[4];
static JukeboxTrack _gMistShootingGalleryJukeboxRegularBountyTracks[4];
static JukeboxTrack _gMistShootingGalleryJukeboxRegularScavengerTracks[4];
static JukeboxTrack _gMistShootingGalleryJukeboxRegularNightmareTracks[4];
static JukeboxTrack _gMistShootingGalleryJukeboxRegularUnclearedTracks[4];
extern WorldCollisionRoomResources gMistShootingGalleryRoomCollisionResources[1];
extern u8* gMistShootingGalleryViewMaps[1];
extern ViewCount gMistShootingGalleryViewCounts[2];
extern WorldCoordRoomLighting gMistShootingGalleryRoomLightingTable[1];
extern DirectionWarpEntry gMistShootingGalleryWarpEntries[7];
static SVECTOR _gMistShootingGalleryLightGlowPoints[45];
