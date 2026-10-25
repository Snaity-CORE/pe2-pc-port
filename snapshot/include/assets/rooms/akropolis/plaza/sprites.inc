/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SpriteBatch _gAcropolisPlazaHideImageSpriteBatches[1] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAcropolisPlazaShowImageSpriteBatches[1] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
SpriteView gAcropolisPlazaSpriteViews[8] = {
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaHideImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAcropolisPlazaHideImageSpriteBatches, .drawAreas = NULL },
};
