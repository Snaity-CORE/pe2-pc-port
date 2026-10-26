/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include "gameplay/sprites.h"

static SpriteBatch _gAkropolisHelicopterLandingPadView1SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisHelicopterLandingPadView2SpriteSources[17] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView2SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView3SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView3DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisHelicopterLandingPadView4SpriteSources[5] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView4SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisHelicopterLandingPadView5SpriteSources[8] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView5SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView5DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisHelicopterLandingPadView6SpriteSources[10] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView6SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView6DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisHelicopterLandingPadView7SpriteSources[29] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView7SpriteBatches[5] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView7DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisHelicopterLandingPadView8SpriteSources[10] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView8SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView8DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisHelicopterLandingPadView9SpriteSources[12] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView9SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView9DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteSource _gAkropolisHelicopterLandingPadView10SpriteSources[27] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView10SpriteBatches[5] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadView10DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView11SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView12SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView13SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView14SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisHelicopterLandingPadViews15And27SpriteSources[1] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadViews15And27SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadViews15And27DrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView16SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisHelicopterLandingPadView17SpriteSources[3] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = -0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView17SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisHelicopterLandingPadView18SpriteSources[4] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView18SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView19SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView20SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteSource _gAkropolisHelicopterLandingPadView21SpriteSources[6] = {
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = -0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
    { .tpage = 0, .clut = 0, .size = { .fields = { .w = 0, .h = 0 } }, .x0 = 0, .y0 = 0, .depth = 0, .uv = { .fields = { .u0 = 0, .v0 = 0 } }, .r0 = 0, .g0 = 0, .b0 = 0, .codeFlags = 0 },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView21SpriteBatches[3] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView22SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView23SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView24SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView25SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteBatch _gAkropolisHelicopterLandingPadView26SpriteBatches[2] = {
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
    { .firstSprite = 0, .spriteCount = 0, .hidden = 0, .skipCachedPackets = 0, .field_6 = { 0, 0 } },
};
static SpriteDrawArea _gAkropolisHelicopterLandingPadUnselectedDrawAreas[2] = {
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
    { .clipRect = { .x = 0, .y = 0, .w = 0, .h = 0 }, .restoreDepth = 0 },
};
SpriteView gAkropolisHelicopterLandingPadSpriteViews[27] = {
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView1SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView1SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView2SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView2SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView3SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView3SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView3DrawAreas },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView4SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView4SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView5SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView5SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView5DrawAreas },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView6SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView6SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView6DrawAreas },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView7SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView7SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView7DrawAreas },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView8SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView8SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView8DrawAreas },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView9SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView9SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView9DrawAreas },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView10SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView10SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadView10DrawAreas },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView11SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView11SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView12SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView12SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView13SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView13SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView14SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView14SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadViews15And27SpriteSources }, .batches = _gAkropolisHelicopterLandingPadViews15And27SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadViews15And27DrawAreas },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView16SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView16SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView17SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView17SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView18SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView18SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView19SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView19SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView20SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView20SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadView21SpriteSources }, .batches = _gAkropolisHelicopterLandingPadView21SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView22SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView22SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView23SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView23SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView24SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView24SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView25SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView25SpriteBatches, .drawAreas = NULL },
    { .sources = { .empty = _gAkropolisHelicopterLandingPadView26SpriteBatches }, .batches = _gAkropolisHelicopterLandingPadView26SpriteBatches, .drawAreas = NULL },
    { .sources = { .elements = _gAkropolisHelicopterLandingPadViews15And27SpriteSources }, .batches = _gAkropolisHelicopterLandingPadViews15And27SpriteBatches, .drawAreas = _gAkropolisHelicopterLandingPadViews15And27DrawAreas },
};
