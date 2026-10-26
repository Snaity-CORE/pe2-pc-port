/* rooms/akropolis/stage: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/direction.h"
#include "gameplay/room.h"
#include "gameplay/sprites.h"
#include "gameplay/view.h"
#include "stages/akropolis/rooms/bridge.h"
#include "stages/akropolis/rooms/cafeteria.h"
#include "stages/akropolis/rooms/east_elevator_hall.h"
#include "stages/akropolis/rooms/fire_escape.h"
#include "stages/akropolis/rooms/forked_road.h"
#include "stages/akropolis/rooms/fountain.h"
#include "stages/akropolis/rooms/hallway.h"
#include "stages/akropolis/rooms/helicopter_landing_pad.h"
#include "stages/akropolis/rooms/mist_parking.h"
#include "stages/akropolis/rooms/mist_r18.h"
#include "stages/akropolis/rooms/mist_shooting_gallery.h"
#include "stages/akropolis/rooms/observatory.h"
#include "stages/akropolis/rooms/patio.h"
#include "stages/akropolis/rooms/plaza.h"
#include "stages/akropolis/rooms/promenade.h"
#include "stages/akropolis/rooms/roof_garden.h"
#include "stages/akropolis/rooms/sanctuary.h"
#include "stages/akropolis/rooms/security_room.h"
#include "stages/akropolis/rooms/square.h"
#include "stages/akropolis/rooms/west_elevator_hall.h"

WorldCoordRoomLighting* gMapAkropolisAreaRoomLightingTables[20] = {
    (void*)&gAkropolisSquareRoomLightingTable,
    (void*)&gAkropolisEastElevatorHallRoomLightingTable,
    (void*)&gAkropolisPatioRoomLightingTable,
    (void*)&gAkropolisCafeteriaRoomLightingTable,
    (void*)&gAkropolisPlazaRoomLightingTable,
    (void*)&gAkropolisSecurityRoomRoomLightingTable,
    (void*)&gAkropolisHallwayRoomLightingTable,
    (void*)&gAkropolisFountainRoomLightingTable,
    (void*)&gAkropolisForkedRoadRoomLightingTable,
    (void*)&gAkropolisObservatoryRoomLightingTable,
    (void*)&gAkropolisPromenadeRoomLightingTable,
    (void*)&gAkropolisSanctuaryRoomLightingTable,
    (void*)&gAkropolisRoofGardenRoomLightingTable,
    (void*)&gAkropolisBridgeRoomLightingTable,
    (void*)&gAkropolisFireEscapeRoomLightingTable,
    (void*)&gAkropolisHelicopterLandingPadRoomLightingTable,
    (void*)&gAkropolisWestElevatorHallRoomLightingTable,
    (void*)&gMistR18RoomLightingTable,
    (void*)&gMistParkingRoomLightingTable,
    (void*)&gMistShootingGalleryRoomLightingTable,
};
static WorldCollisionRoomResources* _gMapAkropolisWorldCollisionAreaRooms[20] = {
    (void*)&gAkropolisSquareRoomCollisionResources,
    (void*)&gAkropolisEastElevatorHallRoomCollisionResources,
    (void*)&gAkropolisPatioRoomCollisionResources,
    (void*)&gAkropolisCafeteriaCollisionRooms,
    (void*)&gAkropolisPlazaRoomCollisionResources,
    (void*)&gAkropolisSecurityRoomRoomCollisionResources,
    (void*)&gAkropolisHallwayRoomCollisionResources,
    (void*)&gAkropolisFountainRoomCollisionResources,
    (void*)&gAkropolisForkedRoadRoomCollisionResources,
    (void*)&gAkropolisObservatoryRoomCollisionResources,
    (void*)&gAkropolisPromenadeRoomCollisionResources,
    (void*)&gAkropolisSanctuaryRoomCollisionResources,
    (void*)&gAkropolisRoofGardenCollisionRoomResources,
    (void*)&gAkropolisBridgeRoomCollisionResources,
    (void*)&gAkropolisFireEscapeCollisionRoomResources,
    (void*)&gAkropolisHelicopterLandingPadCollisionRoomResources,
    (void*)&gAkropolisWestElevatorHallCollisionRoomResources,
    (void*)&gMistR18CollisionRoomResources,
    (void*)&gMistParkingRoomCollisionResources,
    (void*)&gMistShootingGalleryRoomCollisionResources,
};
WorldCollisionStageResources gMapAkropolisWorldCollisionResources = { .areaRooms = _gMapAkropolisWorldCollisionAreaRooms };
static SpriteView* _gMapAkropolisAreaSpriteViews[20] = {
    (void*)&gAkropolisSquareSpriteViews,
    (void*)&gAkropolisEastElevatorHallSpriteViews,
    (void*)&gAkropolisPatioSpriteViews,
    (void*)&gAkropolisCafeteriaSpriteViews,
    (void*)&gAkropolisPlazaSpriteViews,
    (void*)&gAkropolisSecurityRoomSpriteViews,
    (void*)&gAkropolisHallwaySpriteViews,
    (void*)&gAkropolisFountainSpriteViews,
    (void*)&gAkropolisForkedRoadSpriteViews,
    (void*)&gAkropolisObservatorySpriteViews,
    (void*)&gAkropolisPromenadeSpriteViews,
    (void*)&gAkropolisSanctuarySpriteViews,
    (void*)&gAkropolisRoofGardenSpriteViews,
    (void*)&gAkropolisBridgeSpriteViews,
    (void*)&gAkropolisFireEscapeSpriteViews,
    (void*)&gAkropolisHelicopterLandingPadSpriteViews,
    (void*)&gAkropolisWestElevatorHallSpriteViews,
    (void*)&gMistR18SpriteViews,
    (void*)&gMistParkingSpriteViews,
    (void*)&gMistShootingGallerySpriteViews,
};
SpriteAreaTable gMapAkropolisSpriteAreaTable = { .areaViews = _gMapAkropolisAreaSpriteViews };
DirectionWarpEntry* gMapAkropolisAreaWarpEntries[20] = {
    (void*)&gAkropolisSquareWarpEntries,
    (void*)&gAkropolisEastElevatorHallWarpEntries,
    (void*)&gAkropolisPatioWarpEntries,
    (void*)&gAkropolisCafeteriaWarps,
    (void*)&gAkropolisPlazaWarpEntries,
    (void*)&gAkropolisSecurityRoomWarpEntries,
    (void*)&gAkropolisHallwayWarpEntries,
    (void*)&gAkropolisFountainWarpEntries,
    (void*)&gAkropolisForkedRoadWarpEntries,
    (void*)&gAkropolisObservatoryWarpEntries,
    (void*)&gAkropolisPromenadeWarpEntries,
    (void*)&gAkropolisSanctuaryWarpEntries,
    (void*)&gAkropolisRoofGardenWarpEntries,
    (void*)&gAkropolisBridgeWarpEntries,
    (void*)&gAkropolisFireEscapeWarpEntries,
    (void*)&gAkropolisHelicopterLandingPadWarps,
    (void*)&gAkropolisWestElevatorHallWarpEntries,
    (void*)&gMistR18WarpEntries,
    (void*)&gMistParkingWarpEntries,
    (void*)&gMistShootingGalleryWarpEntries,
};
static ViewCount* _gMapAkropolisAreaViewCounts[20] = {
    (void*)&gAkropolisSquareViewCounts,
    (void*)&gAkropolisEastElevatorHallViewCounts,
    (void*)&gAkropolisPatioViewCounts,
    (void*)&gAkropolisCafeteriaViewCounts,
    (void*)&gAkropolisPlazaViewCounts,
    (void*)&gAkropolisSecurityRoomViewCounts,
    (void*)&gAkropolisHallwayViewCounts,
    (void*)&gAkropolisFountainViewCounts,
    (void*)&gAkropolisForkedRoadViewCounts,
    (void*)&gAkropolisObservatoryViewCounts,
    (void*)&gAkropolisPromenadeViewCounts,
    (void*)&gAkropolisSanctuaryViewCounts,
    (void*)&gAkropolisRoofGardenViewCounts,
    (void*)&gAkropolisBridgeViewCounts,
    (void*)&gAkropolisFireEscapeViewCounts,
    (void*)&gAkropolisHelicopterLandingPadViewCounts,
    (void*)&gAkropolisWestElevatorHallViewCounts,
    (void*)&gMistR18ViewCounts,
    (void*)&gMistParkingViewCounts,
    (void*)&gMistShootingGalleryViewCounts,
};
ViewCountTable gMapAkropolisViewCountTable = { .viewCounts = _gMapAkropolisAreaViewCounts };
static ViewCamera* _gMapAkropolisAreaCameras[20] = {
    (void*)&gAkropolisSquareViewCameras,
    (void*)&gAkropolisEastElevatorHallViewCameras,
    (void*)&gAkropolisPatioViewCameras,
    (void*)&gAkropolisCafeteriaViewCameras,
    (void*)&gAkropolisPlazaViewCameras,
    (void*)&gAkropolisSecurityRoomViewCameras,
    (void*)&gAkropolisHallwayViewCameras,
    (void*)&gAkropolisFountainViewCameras,
    (void*)&gAkropolisForkedRoadViewCameras,
    (void*)&gAkropolisObservatoryViewCameras,
    (void*)&gAkropolisPromenadeViewCameras,
    (void*)&gAkropolisSanctuaryViewCameras,
    (void*)&gAkropolisRoofGardenViewCameras,
    (void*)&gAkropolisBridgeViewCameras,
    (void*)&gAkropolisFireEscapeViewCameras,
    (void*)&gAkropolisHelicopterLandingPadViewCameras,
    (void*)&gAkropolisWestElevatorHallViewCameras,
    (void*)&gMistR18ViewCameras,
    (void*)&gMistParkingViewCameras,
    (void*)&gMistShootingGalleryViewCameras,
};
ViewCameraTable gMapAkropolisViewCameraTable = { .cameras = _gMapAkropolisAreaCameras };
static u8** _gMapAkropolisAreaViewMaps[20] = {
    (void*)&gAkropolisSquareViewMaps,
    (void*)&gAkropolisEastElevatorHallViewMaps,
    (void*)&gAkropolisPatioViewMaps,
    (void*)&gAkropolisCafeteriaViewMaps,
    (void*)&gAkropolisPlazaViewMaps,
    (void*)&gAkropolisSecurityRoomViewMaps,
    (void*)&gAkropolisHallwayViewMaps,
    (void*)&gAkropolisFountainViewMaps,
    (void*)&gAkropolisForkedRoadViewMaps,
    (void*)&gAkropolisObservatoryViewMaps,
    (void*)&gAkropolisPromenadeViewMaps,
    (void*)&gAkropolisSanctuaryViewMaps,
    (void*)&gAkropolisRoofGardenViewMaps,
    (void*)&gAkropolisBridgeViewMaps,
    (void*)&gAkropolisFireEscapeViewMaps,
    (void*)&gAkropolisHelicopterLandingPadViewMaps,
    (void*)&gAkropolisWestElevatorHallViewMaps,
    (void*)&gMistR18ViewMaps,
    (void*)&gMistParkingViewMaps,
    (void*)&gMistShootingGalleryViewMaps,
};
ViewIndexTable gMapAkropolisViewIndexTable = { .viewMaps = _gMapAkropolisAreaViewMaps };
WorldCollisionSurfaceProperties** gMapAkropolisSurfacePropertiesByArea[20] = {
    (void*)&gAkropolisSquareSurfacePropertiesByClass,
    (void*)&gAkropolisEastElevatorHallSurfacePropertiesByClass,
    (void*)&gAkropolisPatioSurfacePropertiesByClass,
    (void*)&gAkropolisCafeteriaSurfacePropertiesByClass,
    (void*)&gAkropolisPlazaSurfacePropertiesByClass,
    (void*)&gAkropolisSecurityRoomSurfacePropertiesByClass,
    (void*)&gAkropolisHallwaySurfacePropertiesByClass,
    (void*)&gAkropolisFountainSurfacePropertiesByClass,
    (void*)&gAkropolisForkedRoadSurfacePropertiesByClass,
    (void*)&gAkropolisObservatorySurfacePropertiesByClass,
    (void*)&gAkropolisPromenadeSurfacePropertiesByClass,
    (void*)&gAkropolisSanctuarySurfacePropertiesByClass,
    (void*)&gAkropolisRoofGardenSurfacePropertiesByClass,
    (void*)&gAkropolisBridgeSurfacePropertiesByClass,
    (void*)&gAkropolisFireEscapeSurfacePropertiesByClass,
    (void*)&gAkropolisHelicopterLandingPadSurfacePropertiesByClass,
    (void*)&gAkropolisWestElevatorHallSurfacePropertiesByClass,
    (void*)&gMistR18SurfacePropertiesByClass,
    (void*)&gMistParkingSurfacePropertiesByClass,
    (void*)&gMistShootingGallerySurfacePropertiesByClass,
};
