/* rooms/akropolis/fire_escape/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/sprites.h"

static SpriteBatch _gAkropolisFireEscapeView1SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisFireEscapeView2SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisFireEscapeView3SpriteSources[13] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisFireEscapeView3SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisFireEscapeView4SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisFireEscapeView5SpriteSources[5] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisFireEscapeView5SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisFireEscapeView6SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisFireEscapeView6SpriteDrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisFireEscapeView7SpriteSources[5] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisFireEscapeView7SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisFireEscapeView8SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisFireEscapeViews9And10SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisFireEscapeView9SpriteDrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteBatch _gAkropolisFireEscapeUnusedSpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
SpriteView gAkropolisFireEscapeSpriteViews[10] = {
    { .sources = { .empty = _gAkropolisFireEscapeView1SpriteBatches }, .batches = _gAkropolisFireEscapeView1SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisFireEscapeView2SpriteBatches }, .batches = _gAkropolisFireEscapeView2SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisFireEscapeView3SpriteSources }, .batches = _gAkropolisFireEscapeView3SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisFireEscapeView4SpriteBatches }, .batches = _gAkropolisFireEscapeView4SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisFireEscapeView5SpriteSources }, .batches = _gAkropolisFireEscapeView5SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisFireEscapeView6SpriteBatches }, .batches = _gAkropolisFireEscapeView6SpriteBatches, .drawAreas = _gAkropolisFireEscapeView6SpriteDrawAreas },
    { .sources = { .elements = _gAkropolisFireEscapeView7SpriteSources }, .batches = _gAkropolisFireEscapeView7SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisFireEscapeView8SpriteBatches }, .batches = _gAkropolisFireEscapeView8SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisFireEscapeViews9And10SpriteBatches }, .batches = _gAkropolisFireEscapeViews9And10SpriteBatches, .drawAreas = _gAkropolisFireEscapeView9SpriteDrawAreas },
    { .sources = { .empty = _gAkropolisFireEscapeViews9And10SpriteBatches }, .batches = _gAkropolisFireEscapeViews9And10SpriteBatches, .drawAreas = NULL },
};
