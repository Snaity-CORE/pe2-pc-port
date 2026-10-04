/* Booting from the PS-X EXE. See pc/boot.h. */
#include "pc/boot.h"

#include <string.h>

#include "pc/memory.h"
#include "pc/runtime.h"
#include "pc/thunks.h"

/* PS-X EXE header (2 KiB): "PS-X EXE" magic, then load parameters. */
#define EXE_HEADER_SIZE 0x800
#define EXE_OFF_PC0     0x10
#define EXE_OFF_T_ADDR  0x18
#define EXE_OFF_T_SIZE  0x1C

static uint32_t read_u32(const uint8_t* p)
{
    return (uint32_t)p[0] | ((uint32_t)p[1] << 8) | ((uint32_t)p[2] << 16) | ((uint32_t)p[3] << 24);
}

int pc_install_thunks(const char* image)
{
    for (unsigned i = 0; i < pc_image_thunk_count; i++) {
        const PcImageThunks* t = &pc_image_thunks[i];
        if (strcmp(t->image, image) != 0)
            continue;
        for (unsigned j = 0; j < t->count; j++) {
            uint8_t* at = (uint8_t*)pc_psx_ptr(t->thunks[j].psx_addr);
            int32_t rel = (int32_t)((uintptr_t)t->thunks[j].native - ((uintptr_t)at + 5));
            at[0] = 0xE9; /* jmp rel32 */
            memcpy(at + 1, &rel, sizeof(rel));
        }
        return (int)t->count;
    }
    return -1;
}

int pc_boot_load(const uint8_t* exe, size_t size)
{
    pc_crash_handler_install();
    if (pc_memory_init() != 0)
        return PC_BOOT_ERR_MEMORY;
    if (size < EXE_HEADER_SIZE || memcmp(exe, "PS-X EXE", 8) != 0)
        return PC_BOOT_ERR_EXE;

    uint32_t t_addr = read_u32(exe + EXE_OFF_T_ADDR);
    uint32_t t_size = read_u32(exe + EXE_OFF_T_SIZE);
    if (t_addr < PC_PSX_RAM_BASE || t_size > size - EXE_HEADER_SIZE ||
        t_addr + t_size > PC_PSX_RAM_BASE + PC_PSX_RAM_SIZE)
        return PC_BOOT_ERR_EXE;

    memcpy(pc_psx_ptr(t_addr), exe + EXE_HEADER_SIZE, t_size);
    memset(pc_psx_ptr(pc_main_image.bss_start), 0, pc_main_image.bss_end - pc_main_image.bss_start);

    if (pc_install_thunks("main") < 0)
        return PC_BOOT_ERR_THUNKS;
    return PC_BOOT_OK;
}

int pc_boot_run(void)
{
    return pc_main_image.entry();
}
