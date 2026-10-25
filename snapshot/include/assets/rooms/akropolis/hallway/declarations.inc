/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

extern WorldCollisionRoomResources gAcropolisHallwayRoomCollisionResources[1];
extern u8* gAcropolisHallwayViewMaps[1];
extern ViewCount gAcropolisHallwayViewCounts[1];
extern WorldCoordRoomLighting gAcropolisHallwayRoomLightingTable[1];
extern DirectionWarpEntry gAcropolisHallwayWarpEntries[4];
static SVECTOR _gAcropolisHallwayCollisionGridNormals[10];
static SVECTOR _gAcropolisHallwayCollisionGridVerts[35];
static WorldCollisionGridFace _gAcropolisHallwayCollisionGridFaces[17];
static s16 _gAcropolisHallwayCollisionGridCells[30];
static s16* _gAcropolisHallwayCollisionGridTable[2];
static WorldCollisionGrid _gAcropolisHallwayCollisionGrid;
static WorldCollisionTrigger _gAcropolisHallwayViewBoundaryTriggers[4];
static WorldCollisionTrigger _gAcropolisHallwayActionTriggers[9];
static AreaResource _gAcropolisHallwayVariant1Resources[2];
static AreaResource _gAcropolisHallwayVariant2Resources[2];
static AreaResource _gAcropolisHallwayVariant3Resources[2];
static AreaResource _gAcropolisHallwayVariant4Resources[3];
extern AreaVariant gAcropolisHallwayAreaVariants[13];
static WorldCoordPointLight _gAcropolisHallwayPointLights[3];
static WorldCoordRoomLights _gAcropolisHallwayRoomLights;
static SpriteBatch _gAcropolisHallwayView1SpriteBatches[2];
static SpriteBatch _gAcropolisHallwayView2SpriteBatches[2];
static SpriteBatch _gAcropolisHallwayView3SpriteBatches[2];
static SpriteBatch _gAcropolisHallwayView4SpriteBatches[2];
static SpriteBatch _gAcropolisHallwayView5SpriteBatches[2];
extern SpriteView gAcropolisHallwaySpriteViews[5];
extern ViewCamera gAcropolisHallwayViewCameras[5];
static WorldCollisionFootstepSounds _gAcropolisHallwayFootstepSounds;
static WorldCollisionSurfaceProperties _gAcropolisHallwaySilentBlockingSurface;
static WorldCollisionSurfaceProperties _gAcropolisHallwayWalkablePassThroughSurface;
static WorldCollisionSurfaceProperties _gAcropolisHallwayWalkableBlockingSurface;
extern WorldCollisionSurfaceProperties* gAcropolisHallwaySurfacePropertiesByClass[8];
