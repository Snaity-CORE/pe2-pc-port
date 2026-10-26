/* rooms/akropolis/hallway/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/sprites.h"

static SpriteBatch _gAkropolisHallwayView1SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHallwayView2SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHallwayView3SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHallwayView4SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHallwayView5SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
SpriteView gAkropolisHallwaySpriteViews[5] = {
    { .sources = { .empty = _gAkropolisHallwayView1SpriteBatches }, .batches = _gAkropolisHallwayView1SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHallwayView2SpriteBatches }, .batches = _gAkropolisHallwayView2SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHallwayView3SpriteBatches }, .batches = _gAkropolisHallwayView3SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHallwayView4SpriteBatches }, .batches = _gAkropolisHallwayView4SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHallwayView5SpriteBatches }, .batches = _gAkropolisHallwayView5SpriteBatches, .drawAreas = NULL },
};
