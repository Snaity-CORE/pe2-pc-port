/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/helicopter_departure.h"
#include "stages/akropolis/actors/helicopter_landing_pad_no_9.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisHelicopterLandingPadVariant1Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gHelicopterLandingPadNo9TaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisHelicopterLandingPadVariant2Resources[4] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gHelicopterDepartureNo9GolemTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gHelicopterDepartureRupertTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gHelicopterDepartureHelicopterTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisHelicopterLandingPadAreaVariants[13] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStageHelicopterLandingPadVariant1Placements, .resources = _gAkropolisHelicopterLandingPadVariant1Resources },
    { .placements = gAkropolisStageHelicopterLandingPadVariant2Placements, .resources = _gAkropolisHelicopterLandingPadVariant2Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
};
