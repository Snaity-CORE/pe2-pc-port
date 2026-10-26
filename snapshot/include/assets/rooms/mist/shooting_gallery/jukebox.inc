/* rooms/mist/shooting_gallery/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "rooms/rooms_shared_8018055c.h"

#define ROOM_IS__gJukeboxTrackLists 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static DATA_CONST JukeboxTrackLists _gJukeboxTrackLists = { .lists = { _gMistShootingGalleryJukeboxTrainingReplayTracks, _gMistShootingGalleryJukeboxTrainingBountyTracks, _gMistShootingGalleryJukeboxTrainingScavengerTracks, _gMistShootingGalleryJukeboxTrainingNightmareTracks, _gMistShootingGalleryJukeboxTrainingUnclearedTracks, _gMistShootingGalleryJukeboxRegularReplayTracks, _gMistShootingGalleryJukeboxRegularBountyTracks, _gMistShootingGalleryJukeboxRegularScavengerTracks, _gMistShootingGalleryJukeboxRegularNightmareTracks, _gMistShootingGalleryJukeboxRegularUnclearedTracks } };
#endif
#undef ROOM_IS__gJukeboxTrackLists

#define ROOM_IS__gMistShootingGalleryJukeboxTrainingReplayTracks 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingReplayTracks[3] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1CrazyKingLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2EagerForBloodLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3HeavenSentKillerLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingBountyTracks[3] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1CrazyKingLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2PickUpTheGauntletLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3CrawlingWasteEmperorLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingScavengerTracks[3] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1CrazyKingLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2QuadrumanousLeaderLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3HuntersMoonLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingNightmareTracks[3] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1YellowRainLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2RebelForcesLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3GenicReactorLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxTrainingUnclearedTracks[3] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1CrazyKingLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2AmbushLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3PickUpTheGauntletLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxRegularReplayTracks[4] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1GhostTownLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2RockAndFireLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3RequiemLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow4DarkVoiceOfTheHeartLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxRegularBountyTracks[4] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1LAMazeLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2LightningOperationLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3WildHuntLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow4SnooperLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxRegularScavengerTracks[4] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1CurseYourFateLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2WishwashLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3MentalAgonyLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow4GenicReactorLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxRegularNightmareTracks[4] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1ManMadeNatureLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2GazingIntoTheVoidLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3FoolsParadiseLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow4KillingFieldLabel },
};
static JukeboxTrack _gMistShootingGalleryJukeboxRegularUnclearedTracks[4] = {
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow1TowerRendezvousLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow2TheVagrantsLabel },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow3OutOfPhase2Label },
    { .sequenceId = 0, .name = (void*)&_gMistShootingGalleryJukeboxRow4LazingAwayTheMorningLabel },
};
#endif
#undef ROOM_IS__gMistShootingGalleryJukeboxTrainingReplayTracks
#undef ROOM_PART
