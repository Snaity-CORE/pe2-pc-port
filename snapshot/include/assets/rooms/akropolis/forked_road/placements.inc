/* rooms/akropolis/forked_road/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "actors/actor_00700.h"
#include "actors/actor_01500.h"
#include "actors/actor_01600.h"
#include "actors/actor_01900.h"
#include "actors/actor_02000.h"
#include "actors/actor_02400.h"
#include "actors/actor_02500.h"
#include "actors/actor_02600.h"
#include "actors/actor_03700.h"
#include "actors/actor_03800.h"
#include "actors/actor_04600.h"
#include "actors/actor_05500.h"
#include "actors/actor_07000.h"
#include "actors/actor_207200.h"
#include "actors/actor_300700.h"
#include "actors/actor_401000.h"
#include "actors/actor_401800.h"
#include "gameplay/area.h"
#include "mapui/map_akropolis.h"

static AreaResource _gAkropolisForkedRoadVariant2Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor301900GrinningStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant3Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401000StrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor300700RatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant4Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor105500MaggotCaterpillarTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant7Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor102000PawnGolemTasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor200700RatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant8Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor02600MaggotCaterpillarBodyTask },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant9Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor401800OddStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant10Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor00700RatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor00700MothTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant11Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor102400Tasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant12Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor102500Tasks },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant13Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor02600MaggotCaterpillarBodyTask },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant14Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor103700BatTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant15Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor01500MindSucklerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant16Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor103800BlackBeetleTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant20Resources[2] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor101600ScavengerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant21Resources[3] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor104600SucklercephTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor104600SkullStalkerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
static AreaResource _gAkropolisForkedRoadVariant22Resources[5] = {
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor107000SlouchTaskDescs },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor107000SucklercephTaskDescs },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor207200CreepingStrangerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = &gActor207200SkullStalkerTaskDesc },
    { .entryId = 0, .fileNumber = 0, .fileGroupIndex = 0, .taskIndex = 0, .unknown_6 = { 0, 0 }, .taskTable = NULL },
};
AreaVariant gAkropolisForkedRoadAreaVariants[24] = {
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant2Placements, .resources = _gAkropolisForkedRoadVariant2Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant3Placements, .resources = _gAkropolisForkedRoadVariant3Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant4Placements, .resources = _gAkropolisForkedRoadVariant4Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant7Placements, .resources = _gAkropolisForkedRoadVariant7Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant8Placements, .resources = _gAkropolisForkedRoadVariant8Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant9Placements, .resources = _gAkropolisForkedRoadVariant9Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant10Placements, .resources = _gAkropolisForkedRoadVariant10Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant11Placements, .resources = _gAkropolisForkedRoadVariant11Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant12Placements, .resources = _gAkropolisForkedRoadVariant12Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant13Placements, .resources = _gAkropolisForkedRoadVariant13Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant14Placements, .resources = _gAkropolisForkedRoadVariant14Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant15Placements, .resources = _gAkropolisForkedRoadVariant15Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant16Placements, .resources = _gAkropolisForkedRoadVariant16Resources },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = NULL, .resources = NULL },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant20Placements, .resources = _gAkropolisForkedRoadVariant20Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant21Placements, .resources = _gAkropolisForkedRoadVariant21Resources },
    { .placements = (void*)&gMapAkropolisForkedRoadVariant22Placements, .resources = _gAkropolisForkedRoadVariant22Resources },
    { .placements = NULL, .resources = NULL },
};
