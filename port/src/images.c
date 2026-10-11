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
    return masked_fnv(img, 0, img->code_size) == img->full_hash;
}

/* Whether every thunk of `img` is still written: reloading an image over
 * itself restores its original code and erases them. */
static int thunks_intact(const PcImageThunks* img)
{
    for (unsigned t = 0; t < img->count; t++) {
        if (*(const uint8_t*)pc_psx_ptr(img->thunks[t].psx_addr) != 0xE9)
            return 0;
    }
    return 1;
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
        if (slot->installed >= 0) {
            const PcImageThunks* cur = &pc_image_thunks[slot->installed];
            if (prefix_matches(cur)) {
                if (thunks_intact(cur))
                    continue;
                /* Reloaded over itself: rewrite the thunks once complete. */
                if (fully_matches(cur)) {
                    pc_install_thunks(cur->image);
                    installed++;
                    if (pc_trace_images)
                        fprintf(stderr, "[images] %s reloaded at 0x%08X\n", cur->image, cur->load);
                }
                continue;
            }
        }
        int found = 0;
        for (unsigned i = 0; i < pc_image_thunk_count; i++) {
            const PcImageThunks* img = &pc_image_thunks[i];
            if (img->load != slot->load || (int)i == slot->installed || !fits_in_ram(img))
                continue;
            if (!prefix_matches(img) || !fully_matches(img))
                continue;
            found = 1;
            pc_clear_thunks(img->load, img->size);
            pc_install_thunks(img->image);
            slot->installed = (int)i;
            installed++;
            if (pc_trace_images)
                fprintf(stderr, "[images] %s loaded at 0x%08X (%u functions)\n", img->image, img->load, img->count);
            break;
        }
        /* The installed image is gone and nothing known replaced it (yet):
         * its thunks no longer describe the slot's code. */
        if (!found && slot->installed >= 0) {
            const PcImageThunks* cur = &pc_image_thunks[slot->installed];
            pc_clear_thunks(cur->load, cur->size);
            if (pc_trace_images)
                fprintf(stderr, "[images] %s left 0x%08X; slot content not recognized yet\n", cur->image, cur->load);
            slot->installed = -1;
        }
    }
    return installed;
}
