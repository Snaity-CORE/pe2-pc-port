/* Identity-mapped PSX memory. See pc/memory.h. */
#include "pc/memory.h"

#include <windows.h>

int pc_memory_init(void)
{
    void* ram = VirtualAlloc((void*)(uintptr_t)PC_PSX_RAM_BASE, PC_PSX_RAM_SIZE,
                             MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    if (ram != (void*)(uintptr_t)PC_PSX_RAM_BASE)
        return -1;
    void* spad = VirtualAlloc((void*)(uintptr_t)PC_PSX_SPAD_BASE, PC_PSX_SPAD_SIZE,
                              MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
    if (spad != (void*)(uintptr_t)PC_PSX_SPAD_BASE)
        return -2;
    return 0;
}
