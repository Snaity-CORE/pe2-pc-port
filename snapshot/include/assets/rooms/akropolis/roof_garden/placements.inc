/* rooms/akropolis/roof_garden/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "actors/actor_00700.h"
#include "actors/actor_02600.h"
#include "actors/actor_05500.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/roof_garden_swat_member.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisRoofGardenVariant1Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gRoofGardenSwatMemberViewFigureTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor205500MaggotCaterpillarTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisRoofGardenVariant2Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor105500MaggotCaterpillarTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor00700MothTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisRoofGardenVariant7Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gRoofGardenSwatMemberViewFigureTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor202600MaggotCaterpillarBodyTask },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisRoofGardenVariant8Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor02600MaggotCaterpillarBodyTask },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor00700MothTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisRoofGardenAreaVariants[12] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageRoofGardenVariant1Placements, .resources = _gAkropolisRoofGardenVariant1Resources },
    { .placements = gAkropolisStageRoofGardenVariant2Placements, .resources = _gAkropolisRoofGardenVariant2Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageRoofGardenVariant7Placements, .resources = _gAkropolisRoofGardenVariant7Resources },
    { .placements = gAkropolisStageRoofGardenVariant8Placements, .resources = _gAkropolisRoofGardenVariant8Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
