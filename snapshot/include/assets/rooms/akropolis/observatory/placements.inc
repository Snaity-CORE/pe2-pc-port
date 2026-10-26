/* rooms/akropolis/observatory/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "actors/actor_01100.h"
#include "actors/actor_01200.h"
#include "actors/actor_01900.h"
#include "actors/actor_03700.h"
#include "actors/actor_04000.h"
#include "actors/actor_401000.h"
#include "actors/actor_401300.h"
#include "actors/actor_401800.h"
#include "gameplay/area.h"
#include "gameplay/areaplace.h"
#include "mapui/map_akropolis.h"

static AreaResource _gAkropolisObservatoryVariant1Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor301900GrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisObservatoryVariant2Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101100Tasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisObservatoryVariant3Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor103700BatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisObservatoryVariant7Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor01100Tasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisObservatoryVariant10Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401300HornedStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisObservatoryVariant11Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401000StrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaPlacement _gAkropolisObservatoryVariant11Placements[2] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
static AreaResource _gAkropolisObservatoryVariants12And13Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101200BoneSucklerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaPlacement _gAkropolisObservatoryVariant12Placements[2] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
static AreaPlacement _gAkropolisObservatoryVariant13Placements[7] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
static AreaResource _gAkropolisObservatoryVariant14Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor04000BloodSucklerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaPlacement _gAkropolisObservatoryVariant14Placements[3] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
static AreaResource _gAkropolisObservatoryVariant15Resources[1] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaPlacement _gAkropolisObservatoryVariant15Placements[3] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
static AreaResource _gAkropolisObservatoryVariant16Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaPlacement _gAkropolisObservatoryVariant16Placements[2] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
static AreaResource _gAkropolisObservatoryVariant17Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101900GrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaPlacement _gAkropolisObservatoryVariant17Placements[2] = {
    { .entryId = 0, .variant = 0, .mode = 0, .x = -0, .y = -0, .z = -0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
    { .entryId = 0, .variant = 0, .mode = 0, .x = 0, .y = 0, .z = 0, .yaw = 0, .fileIdLow = 0, .texturePageOffset = 0, .clutRowOffset = 0, .rowIndex = 0 },
};
AreaVariant gAkropolisObservatoryAreaVariants[19] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisObservatoryVariant1Placements, .resources = _gAkropolisObservatoryVariant1Resources },
    { .placements = (void*)&gMapAkropolisObservatoryVariant2Placements, .resources = _gAkropolisObservatoryVariant2Resources },
    { .placements = (void*)&gMapAkropolisObservatoryVariant3Placements, .resources = _gAkropolisObservatoryVariant3Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisObservatoryVariant7Placements, .resources = _gAkropolisObservatoryVariant7Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisObservatoryVariant10Placements, .resources = _gAkropolisObservatoryVariant10Resources },
    { .placements = _gAkropolisObservatoryVariant11Placements, .resources = _gAkropolisObservatoryVariant11Resources },
    { .placements = _gAkropolisObservatoryVariant12Placements, .resources = _gAkropolisObservatoryVariants12And13Resources },
    { .placements = _gAkropolisObservatoryVariant13Placements, .resources = _gAkropolisObservatoryVariants12And13Resources },
    { .placements = _gAkropolisObservatoryVariant14Placements, .resources = _gAkropolisObservatoryVariant14Resources },
    { .placements = _gAkropolisObservatoryVariant15Placements, .resources = _gAkropolisObservatoryVariant15Resources },
    { .placements = _gAkropolisObservatoryVariant16Placements, .resources = _gAkropolisObservatoryVariant16Resources },
    { .placements = _gAkropolisObservatoryVariant17Placements, .resources = _gAkropolisObservatoryVariant17Resources },
    { .placements = NULL, .resources = NULL },
};
