/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"
#include "stages/akropolis/actors/police_officers.h"

static AreaResource _gAkropolisPlazaVariant1Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gPoliceOfficersSergeantTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gPoliceOfficersRookieTaskDescriptors },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisPlazaAreaVariants[3] = {
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisPlazaVariant1Placements, .resources = _gAkropolisPlazaVariant1Resources },
    { .placements = NULL, .resources = NULL },
};
