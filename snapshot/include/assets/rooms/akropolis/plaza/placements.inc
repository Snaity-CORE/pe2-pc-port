/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area.h"
#include "stages/akropolis/actors/police_officers.h"
#include "stages/akropolis/stage.h"

static AreaResource _gAkropolisPlazaVariant1Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gPoliceOfficersSergeantTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gPoliceOfficersRookieTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisPlazaAreaVariants[3] = {
    { .placements = NULL, .resources = NULL },
    { .placements = gAkropolisStagePlazaVariant1Placements, .resources = _gAkropolisPlazaVariant1Resources },
    { .placements = NULL, .resources = NULL },
};
