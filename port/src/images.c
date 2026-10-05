/*
 * Recognizing overlays in PSX RAM. The game loads its code packages itself;
 * native code calls into them through their functions' PSX addresses, which
 * need thunks to the native bodies. This finds which image occupies each
 * load address by content hashes generated at build time, and installs that
 * image's thunks when it changes. See pc/thunks.h.
 */
#include <stdio.h>
#include <string.h>

#include "pc/memory.h"
#include "pc/thunks.h"

#define PREFIX_BYTES 16u
#define EDGE_BYTES 4096u
#define FNV_OFFSET 0xCBF29CE484222325ull
#define FNV_PRIME 0x100000001B3ull
#define MAX_SLOTS 128

int pc_trace_images;

typedef struct Slot {
    uint32_t load;
    int installed; /* index into pc_image_thunks, or -1 */
} Slot;

static Slot slots[MAX_SLOTS];
static unsigned slot_count;

/* FNV-1a over RAM[load+start, load+end) with the 5-byte thunk sites of
 * `img` read as zero, matching image_fingerprint in tools/pclink.py. */
static uint64_t masked_fnv(const PcImageThunks* img, uint32_t start, uint32_t end)
{
    const uint8_t* ram = (const uint8_t*)pc_psx_ptr(img->load);
    uint64_t h = FNV_OFFSET;
    unsigned t = 0;
    for (uint32_t i = start; i < end; i++) {
        while (t < img->count && img->thunks[t].psx_addr - img->load + 5 <= i)
            t++;
        uint8_t b = ram[i];
        if (t < img->count) {
            uint32_t site = img->thunks[t].psx_addr - img->load;
            if (site <= i && i < site + 5)
                b = 0;
        }
        h = (h ^ b) * FNV_PRIME;
    }
    return h;
}

static int prefix_matches(const PcImageThunks* img)
{
    uint32_t n = img->size < PREFIX_BYTES ? img->size : PREFIX_BYTES;
    return masked_fnv(img, 0, n) == img->prefix_hash;
}

static int fully_matches(const PcImageThunks* img)
{
    uint32_t head = img->size < EDGE_BYTES ? img->size : EDGE_BYTES;
    uint32_t tail = img->size > EDGE_BYTES ? img->size - EDGE_BYTES : head;
    if (tail < head)
        tail = head;
    uint64_t h = masked_fnv(img, 0, head) ^ (masked_fnv(img, tail, img->size) * 31u);
    return h == img->full_hash;
}

static int fits_in_ram(const PcImageThunks* img)
{
    return img->load >= PC_PSX_RAM_BASE && img->load + img->size <= PC_PSX_RAM_BASE + PC_PSX_RAM_SIZE;
}

static void init_slots(void)
{
    for (unsigned i = 0; i < pc_image_thunk_count; i++) {
        const PcImageThunks* img = &pc_image_thunks[i];
        if (strcmp(img->image, "main") == 0 || !fits_in_ram(img))
            continue;
        unsigned s = 0;
        while (s < slot_count && slots[s].load != img->load)
            s++;
        if (s == slot_count && slot_count < MAX_SLOTS) {
            slots[slot_count].load = img->load;
            slots[slot_count].installed = -1;
            slot_count++;
        }
    }
}

int pc_scan_images(void)
{
    if (slot_count == 0)
        init_slots();
    int installed = 0;
    for (unsigned s = 0; s < slot_count; s++) {
        Slot* slot = &slots[s];
        if (slot->installed >= 0 && prefix_matches(&pc_image_thunks[slot->installed]))
            continue;
        for (unsigned i = 0; i < pc_image_thunk_count; i++) {
            const PcImageThunks* img = &pc_image_thunks[i];
            if (img->load != slot->load || (int)i == slot->installed || !fits_in_ram(img))
                continue;
            if (!prefix_matches(img) || !fully_matches(img))
                continue;
            pc_clear_thunks(img->load, img->size);
            pc_install_thunks(img->image);
            slot->installed = (int)i;
            installed++;
            if (pc_trace_images)
                fprintf(stderr, "[images] %s loaded at 0x%08X (%u functions)\n", img->image, img->load, img->count);
            break;
        }
    }
    return installed;
}
