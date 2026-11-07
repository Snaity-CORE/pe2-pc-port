/*
 * Crash reporting: prints where a fault happened, so a native crash can be
 * mapped back to a game function (pcfn_<image>_<ADDR>_<name> symbols) or to
 * a PSX address the runtime does not back yet.
 */
#include <stdio.h>
#include <string.h>
#include <windows.h>

#include "pc/runtime.h"

/* logfile.rs: waits until the log has what was written. */
void pc_log_drain(void);

/* gfx.rs: while the graphics card is being opened with Vulkan, the file a
 * crash then leaves, naming the module it happened in (the next start uses
 * OpenGL instead). Empty otherwise. */
wchar_t pc_gfx_crash_marker[MAX_PATH];

/* The file name of the module `address` is in, and its base; NULL for an
 * address in no module. */
static const char* module_of(DWORD address, char* path, DWORD size, DWORD* base)
{
    HMODULE module;
    if (!GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                            (LPCSTR)(ULONG_PTR)address, &module) ||
        !GetModuleFileNameA(module, path, size))
        return NULL;
    *base = (DWORD)(ULONG_PTR)module;
    const char* name = strrchr(path, '\\');
    return name ? name + 1 : path;
}

/* Prints " (<module>+0x<offset>)" for an address in a loaded module (a
 * driver's DLL, an overlay's): pe2.exe's own addresses map through its
 * .map file, the others only this way. Nothing for one in no module. */
static void print_module(DWORD address)
{
    char path[MAX_PATH];
    DWORD base;
    const char* name = module_of(address, path, sizeof path, &base);
    if (name)
        fprintf(stderr, " (%s+0x%lX)", name, (unsigned long)(address - base));
}

/* A crash while the graphics card opens: leaves pc_gfx_crash_marker. */
static void mark_gfx_crash(DWORD address)
{
    if (!pc_gfx_crash_marker[0])
        return;
    FILE* f = _wfopen(pc_gfx_crash_marker, L"w");
    if (!f)
        return;
    char path[MAX_PATH];
    DWORD base;
    const char* name = module_of(address, path, sizeof path, &base);
    fprintf(f, "%s\n", name ? name : "an unknown module");
    fclose(f);
}

static LONG CALLBACK on_exception(EXCEPTION_POINTERS* info)
{
    const EXCEPTION_RECORD* rec = info->ExceptionRecord;
    switch (rec->ExceptionCode) {
    case EXCEPTION_ACCESS_VIOLATION:
    case EXCEPTION_ILLEGAL_INSTRUCTION:
    case EXCEPTION_INT_DIVIDE_BY_ZERO:
    case EXCEPTION_PRIV_INSTRUCTION:
    case EXCEPTION_STACK_OVERFLOW:
        break;
    default:
        return EXCEPTION_CONTINUE_SEARCH;
    }
    const CONTEXT* ctx = info->ContextRecord;
    fprintf(stderr, "[crash] exception 0x%08lX at eip=0x%08lX", rec->ExceptionCode, ctx->Eip);
    print_module(ctx->Eip);
    if (rec->ExceptionCode == EXCEPTION_ACCESS_VIOLATION && rec->NumberParameters >= 2) {
        static const char* const kinds[] = {"read", "write", "?", "?", "?", "?", "?", "?", "execute"};
        ULONG_PTR kind = rec->ExceptionInformation[0];
        fprintf(stderr, " (%s of 0x%08lX)", kind < 9 ? kinds[kind] : "?",
                (unsigned long)rec->ExceptionInformation[1]);
    }
    fprintf(stderr, "\n[crash] eax=%08lX ebx=%08lX ecx=%08lX edx=%08lX esi=%08lX edi=%08lX esp=%08lX ebp=%08lX\n",
            ctx->Eax, ctx->Ebx, ctx->Ecx, ctx->Edx, ctx->Esi, ctx->Edi, ctx->Esp, ctx->Ebp);
    /* Game structures a register points at, in PSX RAM. */
    const DWORD regs[] = {ctx->Eax, ctx->Ebx, ctx->Ecx, ctx->Edx, ctx->Esi, ctx->Edi};
    const char* const names[] = {"eax", "ebx", "ecx", "edx", "esi", "edi"};
    for (int r = 0; r < 6; r++) {
        if (regs[r] < 0x80000000u || regs[r] >= 0x80200000u)
            continue;
        const DWORD* p = (const DWORD*)(regs[r] & ~3u);
        fprintf(stderr, "[crash] [%s] %08lX:", names[r], (unsigned long)regs[r]);
        for (int i = 0; i < 12; i++)
            fprintf(stderr, " %08lX", p[i]);
        fprintf(stderr, "\n");
    }
    /* Return addresses on the stack help place the fault in game code. */
    const DWORD* sp = (const DWORD*)ctx->Esp;
    fprintf(stderr, "[crash] stack:");
    for (int i = 0; i < 16 && !IsBadReadPtr(sp + i, sizeof(DWORD)); i++)
        fprintf(stderr, " %08lX", sp[i]);
    fprintf(stderr, "\n");
    /* Those that are code of another module than pe2.exe: whose they are. */
    for (int i = 0; i < 16 && !IsBadReadPtr(sp + i, sizeof(DWORD)); i++) {
        HMODULE module;
        if (sp[i] >= 0x10000u && GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS |
                                                        GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                                                    (LPCSTR)(ULONG_PTR)sp[i], &module) &&
            module != GetModuleHandleA(NULL)) {
            fprintf(stderr, "[crash] stack %08lX", (unsigned long)sp[i]);
            print_module(sp[i]);
            fprintf(stderr, "\n");
        }
    }
    mark_gfx_crash(ctx->Eip);
    fflush(stderr);
    /* The log's time stamps go through a pipe (logfile.rs): get it all out
     * before the process ends. */
    pc_log_drain();
    return EXCEPTION_CONTINUE_SEARCH;
}

void pc_crash_handler_install(void)
{
    AddVectoredExceptionHandler(1, on_exception);
}
