/*
 * PC runtime services shared by the C side of the port.
 */
#ifndef PC_RUNTIME_H
#define PC_RUNTIME_H

#ifdef __cplusplus
extern "C" {
#endif

/* Reports (once per name) a call to a runtime function the port does not
 * implement yet, and returns 0 to the caller. */
int pc_unimplemented(const char* name);

/* Installs a handler that reports native crashes (fault address, registers,
 * stack) on stderr. */
void pc_crash_handler_install(void);

#ifdef __cplusplus
}
#endif

#endif /* PC_RUNTIME_H */
