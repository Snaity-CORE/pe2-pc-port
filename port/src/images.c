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
    /* The installed image's first bytes as they were when it was found:
     * its own, or a translation's (`ending`). Another image loaded over its
     * start changes them, though it may leave its code as it was (the
     * title over gameplay's rodata). */
    uint64_t head;
    /* The image that last left the slot (-1: none) and its first bytes then.
     * Its code can stay in RAM after it, the slot reused for data (the map's
     * after the game resets to the title): it comes back by its code alone
     * only with its first bytes as they were, reloaded. */
    int gone;
    uint64_t gone_head;
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

static uint64_t head_hash(const PcImageThunks* img)
{
    return masked_fnv(img, 0, img->size < PREFIX_BYTES ? img->size : PREFIX_BYTES);
}

static int prefix_matches(const PcImageThunks* img)
{
    return head_hash(img) == img->prefix_hash;
}

/* Whether the slot may hold `img`, cheaply: its first bytes, or its code's
 * first bytes (a translation's texts can begin the image: `ending`). */
static int head_matches(const PcImageThunks* img)
{
    if (prefix_matches(img))
        return 1;
    if (img->text_prefix_hash == 0 || img->text_start == 0 || img->text_start >= img->code_size)
        return 0;
    uint32_t end = img->text_start + PREFIX_BYTES;
    if (end > img->code_size)
        end = img->code_size;
    return masked_fnv(img, img->text_start, end) == img->text_prefix_hash;
}

static int fully_matches(const PcImageThunks* img)
{
    return masked_fnv(img, 0, img->code_size) == img->full_hash;
}

/* The code is the image's though its texts are not: another version's
 * disc (a translation) with the same code. Logged once per image. */
static int code_matches(const PcImageThunks* img)
{
    if (img->text_start == 0 || img->text_start >= img->code_size)
        return 0;
    if (masked_fnv(img, img->text_start, img->code_size) != img->text_hash)
        return 0;
    static const PcImageThunks* logged[MAX_SLOTS * 4];
    static unsigned logged_count;
    unsigned k = 0;
    while (k < logged_count && logged[k] != img)
        k++;
    if (k == logged_count && logged_count < sizeof logged / sizeof logged[0]) {
        logged[logged_count++] = img;
        fprintf(stderr, "pe2: images: %s recognized by its code (its texts differ from the USA disc's)\n",
                img->image);
    }
    return 1;
}

static int matches(const PcImageThunks* img)
{
    return fully_matches(img) || code_matches(img);
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
            slots[slot_count].gone = -1;
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
            if (head_hash(cur) == slot->head) {
                if (thunks_intact(cur))
                    continue;
                /* Reloaded over itself: rewrite the thunks once complete. */
                if (matches(cur)) {
                    pc_install_thunks(cur->image);
                    installed++;
                    if (pc_trace_images)
                        fprintf(stderr, "[images] %s reloaded at 0x%08X\n", cur->image, cur->load);
                }
                continue;
            }
        }
        int found = 0;
        /* An image whole as it is first; then one whose code alone is there
         * (a translation's), unless another image's own first bytes are:
         * the title loads over gameplay's rodata and leaves its code, which
         * must not pass for gameplay, complete or not yet. */
        int other_head = -1;
        for (unsigned i = 0; i < pc_image_thunk_count && other_head < 0; i++) {
            const PcImageThunks* img = &pc_image_thunks[i];
            if (img->load == slot->load && fits_in_ram(img) && prefix_matches(img))
                other_head = (int)i;
        }
        for (unsigned pass = 0; pass < 2 && !found; pass++) {
            for (unsigned i = 0; i < pc_image_thunk_count; i++) {
                const PcImageThunks* img = &pc_image_thunks[i];
                if (img->load != slot->load || (int)i == slot->installed || !fits_in_ram(img))
                    continue;
                if (pass == 0 ? !(prefix_matches(img) && fully_matches(img))
                              : !(head_matches(img) && (other_head < 0 || other_head == (int)i) &&
                                  ((int)i != slot->gone || head_hash(img) == slot->gone_head) && code_matches(img)))
                    continue;
                found = 1;
                pc_clear_thunks(img->load, img->size);
                pc_install_thunks(img->image);
                slot->installed = (int)i;
                slot->head = head_hash(img);
                slot->gone = -1;
                installed++;
                if (pc_trace_images)
                    fprintf(stderr, "[images] %s loaded at 0x%08X (%u functions)\n", img->image, img->load,
                            img->count);
                break;
            }
        }
        /* The installed image is gone and nothing known replaced it (yet):
         * its thunks no longer describe the slot's code. */
        if (!found && slot->installed >= 0) {
            const PcImageThunks* cur = &pc_image_thunks[slot->installed];
            pc_clear_thunks(cur->load, cur->size);
            if (pc_trace_images)
                fprintf(stderr, "[images] %s left 0x%08X; slot content not recognized yet\n", cur->image, cur->load);
            slot->gone = slot->installed;
            slot->gone_head = slot->head;
            slot->installed = -1;
        }
    }
    return installed;
}
