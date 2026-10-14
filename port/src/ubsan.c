/*
 * Undefined-behaviour reports for sanitized game builds
 * (`python tools/build_game.py --sanitize`).
 *
 * clang's -fsanitize checks call these handlers; this file stands in for
 * the sanitizer runtime, which the port does not link. Each source location
 * is reported once, with the operands, and the game carries on: the point is
 * to find decomp C whose behaviour differs from what the R3000 did (shift
 * counts of 32 or more, reads past the end of an array), not to stop.
 *
 * Handlers can run in interrupt context while the game thread is suspended
 * anywhere, even inside the C runtime, so they take no locks: the output
 * goes straight to the standard error handle, and a location is claimed by
 * swapping its column for ~0 as the sanitizer runtime itself does.
 */
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <windows.h>

typedef struct {
    const char* file;
    uint32_t line;
    volatile LONG column;
} SourceLocation;

typedef struct {
    uint16_t kind;
    uint16_t info;
    char name[1];
} TypeDescriptor;

typedef struct {
    SourceLocation loc;
    const TypeDescriptor* lhs_type;
    const TypeDescriptor* rhs_type;
} ShiftOutOfBoundsData;

typedef struct {
    SourceLocation loc;
    const TypeDescriptor* array_type;
    const TypeDescriptor* index_type;
} OutOfBoundsData;

/* Claims `loc` for reporting; returns its column, or -1 if already reported. */
static long claim(SourceLocation* loc)
{
    LONG column = InterlockedExchange(&loc->column, -1);
    return column == -1 ? -1 : column;
}

static void report(const char* text)
{
    DWORD written;
    WriteFile(GetStdHandle(STD_ERROR_HANDLE), text, (DWORD)strlen(text), &written, NULL);
}

/* Integer operands up to 32 bits are passed by value. */
static long long value(const TypeDescriptor* type, uintptr_t v)
{
    unsigned bits = 1u << (type->info >> 1);
    int is_signed = type->info & 1;
    if (bits >= 32) {
        return is_signed ? (long long)(int32_t)v : (long long)(uint32_t)v;
    }
    uint32_t mask = (1u << bits) - 1;
    uint32_t u = (uint32_t)v & mask;
    if (is_signed && (u >> (bits - 1))) {
        return (long long)u - (1ll << bits);
    }
    return u;
}

void __ubsan_handle_shift_out_of_bounds(ShiftOutOfBoundsData* data, uintptr_t lhs, uintptr_t rhs)
{
    long column = claim(&data->loc);
    if (column < 0) {
        return;
    }
    char text[512];
    snprintf(text, sizeof text, "[ubsan] %s:%u:%ld: shift %lld by %lld (%s by %s)\n", data->loc.file,
             data->loc.line, column, value(data->lhs_type, lhs), value(data->rhs_type, rhs),
             data->lhs_type->name, data->rhs_type->name);
    report(text);
}

void __ubsan_handle_out_of_bounds(OutOfBoundsData* data, uintptr_t index)
{
    long column = claim(&data->loc);
    if (column < 0) {
        return;
    }
    char text[512];
    snprintf(text, sizeof text, "[ubsan] %s:%u:%ld: index %lld out of bounds for %s\n", data->loc.file,
             data->loc.line, column, value(data->index_type, index), data->array_type->name);
    report(text);
}

/* -fno-sanitize-recover variants: report, then carry on all the same. */
void __ubsan_handle_shift_out_of_bounds_abort(ShiftOutOfBoundsData* data, uintptr_t lhs, uintptr_t rhs)
{
    __ubsan_handle_shift_out_of_bounds(data, lhs, rhs);
}

void __ubsan_handle_out_of_bounds_abort(OutOfBoundsData* data, uintptr_t index)
{
    __ubsan_handle_out_of_bounds(data, index);
}
