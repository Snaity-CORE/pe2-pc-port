/*
 * Identity-mapped PlayStation memory.
 *
 * PSX RAM (2 MiB at 0x80000000, KSEG0) and the 1 KiB scratchpad (0x1F800000)
 * are mapped at their console addresses, so game pointers, hard-coded
 * addresses and pointers stored in disc data are all valid host pointers.
 * The process must be 32-bit and large-address-aware.
 */
#ifndef PC_MEMORY_H
#define PC_MEMORY_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define PC_PSX_RAM_BASE   0x80000000u
#define PC_PSX_RAM_SIZE   0x00200000u
#define PC_PSX_KSEG1_BASE 0xA0000000u
#define PC_PSX_SPAD_BASE  0x1F800000u
#define PC_PSX_SPAD_SIZE  0x00000400u

/* Bits of pc_memory_mirrors: which RAM mirrors could be mapped. */
#define PC_MIRROR_KSEG1 0x1u /* 0xA0000000 */
#define PC_MIRROR_KUSEG 0x2u /* 0x00010000 and up (below 64 KiB is emulated) */
extern unsigned pc_memory_mirrors;

/* Maps RAM (read/write/execute, for thunks) with its mirrors, and the
 * scratchpad. Returns 0 on success, or a negative value if a required
 * address range is unavailable. */
int pc_memory_init(void);

static inline void* pc_psx_ptr(uint32_t addr) { return (void*)(uintptr_t)addr; }

#ifdef __cplusplus
}
#endif

#endif /* PC_MEMORY_H */
