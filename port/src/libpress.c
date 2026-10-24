/*
 * The bitstream half of the data-compression library (libpress) the game
 * calls: decoding MDEC bitstream (BS version 2) frames into the run-length
 * codes the MDEC takes. The MDEC calls themselves are in runtime/libpress.rs.
 *
 * A BS frame starts with four halfwords: the number of 32-bit words of
 * run-length data, the MDEC's 0x3800 command, the quantization scale and the
 * version. Then come 16-bit little-endian words read most significant bit
 * first: for each 8x8 block a 10-bit DC value and AC coefficients in the
 * MPEG-1 variable-length code (ISO 11172-2 table B.14, dct_coeff_next), with
 * an escape of 6 bits of run and 10 of level and "10" ending the block.
 *
 * Version 3 frames code each block's DC instead as a difference from the
 * previous block's of the same component (Cr, Cb, then four Y blocks per
 * macroblock), its size in bits in MPEG-1's dct_dc_size_luminance or
 * _chrominance code (ISO 11172-2 tables B.12, B.13) followed by the value
 * bits, a leading 0 bit marking a negative one; the predictors start at 0
 * with each frame and the DC sent is the predictor times 4. Ten 1 bits in
 * place of a block end such a frame.
 *
 * Output: the frame's first word (size and command), then one halfword per
 * code: (scale << 10) | DC opening each block, (run << 10) | level for each
 * coefficient, 0xFE00 closing it. A DC of 0x1FF ends the frame, and 0xFE00
 * fills the rest of the size.
 *
 * The original builds a 68 KiB lookup table with DecDCTvlcBuild for
 * DecDCTvlc2 to use. This decoder carries its own, so DecDCTvlcBuild leaves
 * the table area alone; nothing else reads it. DecDCTvlcSize2 limits how
 * much a call decodes, so a frame can be spread over several calls; the
 * original splits frames at other points (its count is not in words), the
 * decoded data being the same.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* AC codes without their sign bit: length, code, run, level. */
static const struct {
    uint8_t len;
    uint16_t code;
    uint8_t run, level;
} ac_codes[] = {
    {2, 0x3, 0, 1},       {3, 0x3, 1, 1},       {4, 0x4, 0, 2},       {4, 0x5, 2, 1},
    {5, 0x5, 0, 3},       {5, 0x7, 3, 1},       {5, 0x6, 4, 1},       {6, 0x6, 1, 2},
    {6, 0x7, 5, 1},       {6, 0x5, 6, 1},       {6, 0x4, 7, 1},       {7, 0x6, 0, 4},
    {7, 0x4, 2, 2},       {7, 0x7, 8, 1},       {7, 0x5, 9, 1},       {8, 0x26, 0, 5},
    {8, 0x21, 0, 6},      {8, 0x25, 1, 3},      {8, 0x24, 3, 2},      {8, 0x27, 10, 1},
    {8, 0x23, 11, 1},     {8, 0x22, 12, 1},     {8, 0x20, 13, 1},     {10, 0x0A, 0, 7},
    {10, 0x0C, 1, 4},     {10, 0x0B, 2, 3},     {10, 0x0F, 4, 2},     {10, 0x09, 5, 2},
    {10, 0x0E, 14, 1},    {10, 0x0D, 15, 1},    {10, 0x08, 16, 1},    {12, 0x1D, 0, 8},
    {12, 0x18, 0, 9},     {12, 0x13, 0, 10},    {12, 0x10, 0, 11},    {12, 0x1B, 1, 5},
    {12, 0x14, 2, 4},     {12, 0x1C, 3, 3},     {12, 0x12, 4, 3},     {12, 0x1E, 6, 2},
    {12, 0x15, 7, 2},     {12, 0x11, 8, 2},     {12, 0x1F, 17, 1},    {12, 0x1A, 18, 1},
    {12, 0x19, 19, 1},    {12, 0x17, 20, 1},    {12, 0x16, 21, 1},    {13, 0x1A, 0, 12},
    {13, 0x19, 0, 13},    {13, 0x18, 0, 14},    {13, 0x17, 0, 15},    {13, 0x16, 1, 6},
    {13, 0x15, 1, 7},     {13, 0x14, 2, 5},     {13, 0x13, 3, 4},     {13, 0x12, 5, 3},
    {13, 0x11, 9, 2},     {13, 0x10, 10, 2},    {13, 0x1F, 22, 1},    {13, 0x1E, 23, 1},
    {13, 0x1D, 24, 1},    {13, 0x1C, 25, 1},    {13, 0x1B, 26, 1},    {14, 0x1F, 0, 16},
    {14, 0x1E, 0, 17},    {14, 0x1D, 0, 18},    {14, 0x1C, 0, 19},    {14, 0x1B, 0, 20},
    {14, 0x1A, 0, 21},    {14, 0x19, 0, 22},    {14, 0x18, 0, 23},    {14, 0x17, 0, 24},
    {14, 0x16, 0, 25},    {14, 0x15, 0, 26},    {14, 0x14, 0, 27},    {14, 0x13, 0, 28},
    {14, 0x12, 0, 29},    {14, 0x11, 0, 30},    {14, 0x10, 0, 31},    {15, 0x18, 0, 32},
    {15, 0x17, 0, 33},    {15, 0x16, 0, 34},    {15, 0x15, 0, 35},    {15, 0x14, 0, 36},
    {15, 0x13, 0, 37},    {15, 0x12, 0, 38},    {15, 0x11, 0, 39},    {15, 0x10, 0, 40},
    {15, 0x1F, 1, 8},     {15, 0x1E, 1, 9},     {15, 0x1D, 1, 10},    {15, 0x1C, 1, 11},
    {15, 0x1B, 1, 12},    {15, 0x1A, 1, 13},    {15, 0x19, 1, 14},    {16, 0x13, 1, 15},
    {16, 0x12, 1, 16},    {16, 0x11, 1, 17},    {16, 0x10, 1, 18},    {16, 0x14, 6, 3},
    {16, 0x1A, 11, 2},    {16, 0x19, 12, 2},    {16, 0x18, 13, 2},    {16, 0x17, 14, 2},
    {16, 0x16, 15, 2},    {16, 0x15, 16, 2},    {16, 0x1F, 27, 1},    {16, 0x1E, 28, 1},
    {16, 0x1D, 29, 1},    {16, 0x1C, 30, 1},    {16, 0x1B, 31, 1},
};

enum { CODE_EOB = 0xFE, CODE_ESCAPE = 0xFF };

/* A DC value of 0x1FF in place of a block ends the frame. */
#define END_OF_FRAME 0x1FFu

/* Version 3 DC size codes: length, code, value bits. */
static const uint8_t dc_codes[2][9][3] = {
    /* luminance */
    {{2, 0x0, 1}, {2, 0x1, 2}, {3, 0x4, 0}, {3, 0x5, 3}, {3, 0x6, 4},
     {4, 0xE, 5}, {5, 0x1E, 6}, {6, 0x3E, 7}, {7, 0x7E, 8}},
    /* chrominance */
    {{2, 0x0, 0}, {2, 0x1, 1}, {2, 0x2, 2}, {3, 0x6, 3}, {4, 0xE, 4},
     {5, 0x1E, 5}, {6, 0x3E, 6}, {7, 0x7E, 7}, {8, 0xFE, 8}},
};

/* By the next 8 bits: code length, then value bits. Entries left {0, 0}
 * (not a valid code) read nothing, a difference of 0, as in the original's
 * table. */
static uint8_t dc_sizes[2][256][2];

static void init_dc_sizes(void)
{
    for (int t = 0; t < 2; t++) {
        for (int c = 0; c < 9; c++) {
            int len = dc_codes[t][c][0];
            uint32_t first = (uint32_t)dc_codes[t][c][1] << (8 - len);
            for (uint32_t k = 0; k < 1u << (8 - len); k++) {
                dc_sizes[t][first + k][0] = (uint8_t)len;
                dc_sizes[t][first + k][1] = dc_codes[t][c][2];
            }
        }
    }
}

/* For each 16-bit window: code length in the low byte (0: invalid), then
 * run and level (run CODE_EOB / CODE_ESCAPE for those two codes). */
static uint32_t lut[1 << 16];

static void add_code(int len, uint32_t code, uint32_t run, uint32_t level)
{
    uint32_t first = code << (16 - len), count = 1u << (16 - len);
    for (uint32_t i = 0; i < count; i++) {
        lut[first + i] = (uint32_t)len | run << 8 | level << 16;
    }
}

static void init_lut(void)
{
    if (lut[0x8000] != 0) {
        return;
    }
    for (size_t i = 0; i < sizeof ac_codes / sizeof ac_codes[0]; i++) {
        add_code(ac_codes[i].len, ac_codes[i].code, ac_codes[i].run, ac_codes[i].level);
    }
    add_code(2, 0x2, CODE_EOB, 0);    /* 10: end of block */
    add_code(6, 0x1, CODE_ESCAPE, 0); /* 000001: escape */
    init_dc_sizes();
}

/* Decoder state between calls. */
static struct {
    const uint16_t* in;  /* next input halfword */
    uint32_t bits;       /* bit buffer, most significant first */
    int count;           /* bits in the buffer */
    uint16_t* out;       /* next output halfword */
    uint16_t* end;       /* end of the run-length data */
    uint32_t size;       /* the frame's size, from its header (cost model) */
    uint16_t scale;      /* quantization scale << 10 */
    int in_block;        /* inside a block (its DC has been read) */
    int active;
    int version3;        /* DC coded as differences */
    int block;           /* version 3: 1, 2 (Cr, Cb), 3..6 (Y) */
    int32_t pred[3];     /* version 3 DC predictors: Cr, Cb, Y */
} dec;

static int words_per_call;

static uint32_t peek(int n)
{
    while (dec.count < n) {
        dec.bits |= (uint32_t)*dec.in++ << (16 - dec.count);
        dec.count += 16;
    }
    return dec.bits >> (32 - n);
}

static void skip(int n)
{
    dec.bits <<= n;
    dec.count -= n;
}

static uint32_t take(int n)
{
    uint32_t v = peek(n);
    skip(n);
    return v;
}

/* Decodes one code into the output; returns 0 once the data is complete. */
static int decode_code(void)
{
    if (!dec.in_block) {
        uint32_t dc;
        int end;
        if (dec.version3) {
            end = peek(10) == 0x3FF;
            if (!end) {
                const uint8_t* size = dc_sizes[dec.block < 3][peek(8)];
                int len = size[0], bits = size[1];
                int32_t diff = 0;
                skip(len);
                if (bits) {
                    uint32_t v = take(bits);
                    diff = (v >> (bits - 1)) ? (int32_t)v : (int32_t)v - (int32_t)((1u << bits) - 1);
                }
                int32_t* p = &dec.pred[dec.block < 3 ? dec.block - 1 : 2];
                *p += diff;
                dc = ((uint32_t)*p << 2) & 0x3FF;
                dec.block = dec.block == 6 ? 1 : dec.block + 1;
            }
        } else {
            dc = take(10);
            end = dc == END_OF_FRAME;
        }
        if (end) {
            while (dec.out < dec.end) {
                *dec.out++ = 0xFE00;
            }
            return 0;
        }
        *dec.out++ = (uint16_t)(dec.scale | dc);
        dec.in_block = 1;
        return 1;
    }
    uint32_t e = lut[peek(16)];
    int len = (int)(e & 0xFF);
    uint32_t run = (e >> 8) & 0xFF, level = e >> 16;
    if (len == 0) {
        /* Not a valid code: end the block, as on the stream's end. */
        skip(16);
        *dec.out++ = 0xFE00;
        dec.in_block = 0;
        return 1;
    }
    skip(len);
    if (run == CODE_EOB) {
        *dec.out++ = 0xFE00;
        dec.in_block = 0;
    } else if (run == CODE_ESCAPE) {
        uint32_t v = take(16);
        *dec.out++ = (uint16_t)(((v >> 10) << 10) | (v & 0x3FF));
    } else {
        int32_t l = take(1) ? -(int32_t)level : (int32_t)level;
        *dec.out++ = (uint16_t)((run << 10) | ((uint32_t)l & 0x3FF));
    }
    return 1;
}

int pcrt_DecDCTvlcSize2(int size)
{
    int old = words_per_call;
    words_per_call = size;
    return old;
}

/* irq.rs: the game thread waits as long as the console's CPU would take. */
void pc_console_cpu_time(uint32_t cycles);

/* Console CPU cycles of the original routines, measured running them
 * interpreted (PE2_LIBTEST=vlcfile) on the game's camera images and movie
 * frames: instructions, plus 6 more cycles for each load from main RAM
 * (DuckStation's figure). Scenes switching camera wait on this time; free,
 * their picture ran ahead of their sound. */
enum {
    /* DecDCTvlcBuild: 727k instructions, 142k loads. */
    VLC_BUILD_CYCLES = 727000 + 142000 * 6,
};

/* DecDCTvlc2 for a frame of `size` (its header's first halfword): 35.5
 * instructions and 3.07 loads a unit, 0.9 million cycles (27 ms) for a
 * typical 320x240 camera image. */
static uint32_t vlc_frame_cycles(uint32_t size)
{
    return (355u * size + 307u * 6u * size) / 10u;
}

void pcrt_DecDCTvlcBuild(uint16_t* table)
{
    (void)table;
    pc_console_cpu_time(VLC_BUILD_CYCLES);
}

/* Words of run-length data a frame decodes to (its first header field). */
int pcrt_DecDCTBufSize(uint32_t* bs)
{
    return (int)*(const uint16_t*)bs;
}

/* Decodes the BS frame at bs into buf (or, with bs NULL, carries on with
 * the frame in progress). Returns 0 when the frame is complete, 1 when
 * DecDCTvlcSize2's limit stopped it first. */
/* PE2_VLCDUMP=<dir>: saves the frames decoded (64 KiB from each frame's
 * start) as <dir>/bs_NNN.bin, the latest 200 (bs_000 to bs_199, then over
 * again), test input for PE2_LIBTEST=vlcfile. */
static void dump_frame(const uint32_t* bs)
{
    static int count;
    const char* dir = getenv("PE2_VLCDUMP");
    if (!dir) {
        return;
    }
    char path[512];
    snprintf(path, sizeof path, "%s/bs_%03d.bin", dir, count++ % 200);
    FILE* f = fopen(path, "wb");
    if (f) {
        uint32_t start = (uint32_t)(uintptr_t)bs, end = 0x80200000u;
        fwrite(bs, 1, end - start < 0x10000u ? end - start : 0x10000u, f);
        fclose(f);
    }
}

int pcrt_DecDCTvlc2(uint32_t* bs, uint32_t* buf, uint16_t* table)
{
    (void)table;
    init_lut();
    if (bs) {
        dump_frame(bs);
        const uint16_t* h = (const uint16_t*)bs;
        buf[0] = bs[0];
        memset(&dec, 0, sizeof dec);
        dec.in = h + 4;
        dec.out = (uint16_t*)(buf + 1);
        dec.end = dec.out + 2u * h[0];
        dec.size = h[0];
        dec.scale = (uint16_t)(h[2] << 10);
        dec.version3 = h[3] >= 3;
        dec.block = 1;
        dec.active = 1;
    }
    if (!dec.active) {
        return 0;
    }
    uint16_t* limit = words_per_call > 0 ? dec.out + 2 * words_per_call : dec.end;
    if (limit > dec.end) {
        limit = dec.end;
    }
    uint16_t* from = dec.out;
    while (dec.out < limit) {
        decode_code();
    }
    /* This call's share of the frame's cost. */
    if (dec.size > 0) {
        uint64_t share = (uint64_t)vlc_frame_cycles(dec.size) * (uint64_t)(dec.out - from) / (2u * dec.size);
        pc_console_cpu_time((uint32_t)share);
    }
    if (dec.out < dec.end) {
        return 1;
    }
    dec.active = 0;
    return 0;
}
