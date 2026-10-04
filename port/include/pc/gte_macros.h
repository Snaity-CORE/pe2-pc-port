/*
 * Support for the generated GTE macro headers (tools/gen_port_headers.py):
 * the software GTE plus the memory accessors the translated MIPS loads and
 * stores use. Addresses are plain host pointers held in uintptr_t.
 */
#ifndef PC_GTE_MACROS_H
#define PC_GTE_MACROS_H

#include <stdint.h>

#include "pc/gte.h"

#define PCMEM_LW(a)  (*(const uint32_t*)(uintptr_t)(a))
#define PCMEM_LH(a)  ((uint32_t)(int32_t)*(const int16_t*)(uintptr_t)(a))
#define PCMEM_LHU(a) ((uint32_t)*(const uint16_t*)(uintptr_t)(a))
#define PCMEM_LB(a)  ((uint32_t)(int32_t)*(const int8_t*)(uintptr_t)(a))
#define PCMEM_LBU(a) ((uint32_t)*(const uint8_t*)(uintptr_t)(a))

#define PCMEM_SW(a, v) (*(uint32_t*)(uintptr_t)(a) = (uint32_t)(v))
#define PCMEM_SH(a, v) (*(uint16_t*)(uintptr_t)(a) = (uint16_t)(v))
#define PCMEM_SB(a, v) (*(uint8_t*)(uintptr_t)(a) = (uint8_t)(v))

#endif /* PC_GTE_MACROS_H */
