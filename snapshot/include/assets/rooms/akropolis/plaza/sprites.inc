/* rooms/akropolis/plaza/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/sprites.h"

static SpriteBatch _gAkropolisPlazaHideImageSpriteBatches[1] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisPlazaShowImageSpriteBatches[1] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
SpriteView gAkropolisPlazaSpriteViews[8] = {
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaHideImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaShowImageSpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = NULL }, .batches = _gAkropolisPlazaHideImageSpriteBatches, .drawAreas = NULL },
};
