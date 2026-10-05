/* Identity-mapped PSX memory. See pc/memory.h. */
#include "pc/memory.h"

#include <windows.h>

/* KUSEG starts mirroring RAM at 0; Windows never maps the first 64 KiB, so
 * the low mirror begins at the first allocatable address and accesses below
 * it are emulated by the runtime's fault handler. */
#define LOW_MIRROR_START 0x00010000u

unsigned pc_memory_mirrors;

int pc_memory_init(void)
{
    /* One pagefile-backed section, viewed at every address the PSX mirrors
     * RAM at, so all views share the same bytes. */
    HANDLE section = CreateFileMappingW(INVALID_HANDLE_VALUE, NULL, PAGE_EXECUTE_READWRITE, 0,
                                        PC_PSX_RAM_SIZE, NULL);
    if (!section)
        return -1;
    void* kseg0 = MapViewOfFileEx(section, FILE_MAP_ALL_ACCESS | FILE_MAP_EXECUTE, 0, 0,
                                  PC_PSX_RAM_SIZE, (void*)(uintptr_t)PC_PSX_RAM_BASE);
    if (kseg0 != (void*)(uintptr_t)PC_PSX_RAM_BASE)
        return -1;

    /* Optional mirrors: KSEG1 (uncached) and KUSEG. Something else may own
     * the address range; the game rarely needs them. */
    if (MapViewOfFileEx(section, FILE_MAP_ALL_ACCESS, 0, 0, PC_PSX_RAM_SIZE,
                        (void*)(uintptr_t)PC_PSX_KSEG1_BASE))
        pc_memory_mirrors |= PC_MIRROR_KSEG1;
    if (MapViewOfFileEx(section, FILE_MAP_ALL_ACCESS, 0, LOW_MIRROR_START,
                        PC_PSX_RAM_SIZE - LOW_MIRROR_START, (void*)(uintptr_t)LOW_MIRROR_START))
        pc_memory_mirrors |= PC_MIRROR_KUSEG;

    void* spad = VirtualAlloc((void*)(uintptr_t)PC_PSX_SPAD_BASE, PC_PSX_SPAD_SIZE,
                              MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    if (spad != (void*)(uintptr_t)PC_PSX_SPAD_BASE)
        return -2;
    return 0;
}
