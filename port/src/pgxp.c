/*
 * Sub-pixel precision for projected vertices (in the manner of PGXP).
 *
 * The GTE rounds every projected vertex to whole pixels of the console's
 * 320x240 screen. At a higher internal resolution that makes polygons
 * wobble and gap. The GTE emulation (gte.c) also keeps each projection's
 * exact position; when the game stores a screen position (SXY) to memory
 * through pcgte_store_sxy, that position is recorded here against the
 * memory word. The renderer, reading a vertex from a primitive, uses the
 * exact position when the word still holds the value stored with it, and
 * the whole-pixel one otherwise (the word was rewritten or copied).
 *
 * Only the higher-resolution layer uses these positions: the console's
 * VRAM keeps being drawn with the whole-pixel ones.
 *
 * A position may also carry the identity of the vertex it projects (a
 * model part and the vertex's address, see pcgte_set_vector_key), which
 * lets the renderer find the same vertex in the previous frame and draw
 * frames in between.
 */
#include <stdint.h>

#include "pc/gte.h"

#define RAM_WORDS (0x200000u / 4)

typedef struct {
    uint32_t word; /* the stored SXY value */
    float x, y, z;
    uint32_t valid;
    uint32_t key; /* the vertex's identity, or 0 */
    uint32_t frame; /* when it was stored (see pc_pgxp_tick) */
} Precise;

static Precise table[RAM_WORDS];

/* Frames (vertical blanks) so far. Primitives are drawn within a frame or
 * two of being built; an older entry may belong to a word rewritten since
 * by code that does not record positions, with the same value by chance. */
static uint32_t frame;
#define MAX_AGE 2

void pc_pgxp_tick(void)
{
    frame++;
}

static Precise* entry(uint32_t addr)
{
    /* Main RAM through KSEG0/KUSEG mirrors only. */
    if ((addr & 0x1FE00000u) != 0 || (addr & 3) != 0) {
        return 0;
    }
    return &table[(addr & 0x1FFFFFu) >> 2];
}

void pc_pgxp_record(uint32_t addr, uint32_t word, float x, float y, float z, int valid, uint32_t key)
{
    Precise* e = entry(addr);
    if (e) {
        e->word = word;
        e->x = x;
        e->y = y;
        e->z = z;
        e->valid = (uint32_t)valid;
        e->key = key;
        e->frame = frame;
    }
}

void pc_pgxp_copy(uint32_t dst, uint32_t src)
{
    Precise* d = entry(dst);
    Precise* s = entry(src);
    if (d && s) {
        *d = *s;
        d->frame = frame;
    } else if (d) {
        d->valid = 0;
    }
}

int pc_pgxp_lookup(uint32_t addr, uint32_t word, float* x, float* y, float* z, uint32_t* key)
{
    Precise* e = entry(addr);
    if (!e || !e->valid || e->word != word || frame - e->frame > MAX_AGE) {
        return 0;
    }
    *x = e->x;
    *y = e->y;
    *z = e->z;
    *key = e->key;
    return 1;
}
