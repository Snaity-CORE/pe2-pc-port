/*
 * Booting the game from its PS-X EXE, the way the BIOS and Psy-Q's crt0 do.
 */
#ifndef PC_BOOT_H
#define PC_BOOT_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

enum {
    PC_BOOT_OK = 0,
    PC_BOOT_ERR_MEMORY = -1,  /* PSX address ranges could not be mapped */
    PC_BOOT_ERR_EXE = -2,     /* not a PS-X EXE, or it does not fit in RAM */
    PC_BOOT_ERR_THUNKS = -3,  /* no thunk table for the main image */
};

/* Maps memory, loads the PS-X EXE image `exe` (the player's SLUS_010.42)
 * at its load address, clears BSS and installs the main image's thunks.
 * Does not run the game. */
int pc_boot_load(const uint8_t* exe, size_t size);

/* Runs the game's main(). Returns only if main() does. */
int pc_boot_run(void);

#ifdef __cplusplus
}
#endif

#endif /* PC_BOOT_H */
