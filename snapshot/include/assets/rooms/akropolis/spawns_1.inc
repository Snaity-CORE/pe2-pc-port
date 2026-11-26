/* rooms/akropolis/stage: written from its room instance by tools/buildsteps/gen_room_inc.py. */

#include "types.h"
#include "gameplay/area_flags.h"
#include "gameplay/item_menu.h"
#include "stages/akropolis/rooms/cafeteria.h"
#include "stages/akropolis/rooms/hallway.h"
#include "stages/akropolis/rooms/roof_garden.h"
#include "stages/akropolis/rooms/sanctuary.h"
#include "stages/akropolis/rooms/security_room.h"

static AreaObjectSpawn _gAkropolisStageSquareObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageEastElevatorHallObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageCafeteriaObjectSpawns[3] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisCafeteriaStimPickupTask, .data = { .model = &gAkropolisCafeteriaStimPickupModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisCafeteriaMendelPickupTask, .data = { .model = &gAkropolisCafeteriaMendelJournalModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageSecurityRoomObjectSpawns[2] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = itemPickupContainerLidTask, .data = { .model = &gAkropolisSecurityRoomItemBoxModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageHallwayObjectSpawns[2] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisHallwayBlueKeyPickupTask, .data = { .model = &gAkropolisHallwayBlueKeyPickupModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageFountainObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageForkedRoadObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageObservatoryObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStagePromenadeObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageSanctuaryObjectSpawns[3] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisSanctuaryItemVisibilityTask, .data = { .model = &gAkropolisSanctuaryRedKeyPickupModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = itemPickupContainerLidTask, .data = { .model = &gAkropolisSanctuaryItemBoxModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageRoofGardenObjectSpawns[2] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = akropolisRoofGardenPickupModelTask, .data = { .model = &gAkropolisRoofGardenArmoryCardkeyPickupModel } } },
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageFireEscapeObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageWestElevatorHallObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
static AreaObjectSpawn _gAkropolisStageMistParkingObjectSpawns[1] = {
    { .kind = 0, .taskDesc = { .header = { .fields = { .flags = 0, .priority = 0 } }, .callback = NULL, .data = { .value = 0 } } },
};
