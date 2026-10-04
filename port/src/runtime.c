/* PC runtime services. See pc/runtime.h. */
#include "pc/runtime.h"

#include <stdio.h>
#include <string.h>

#define MAX_REPORTED 512

int pc_unimplemented(const char* name)
{
    static const char* reported[MAX_REPORTED];
    static unsigned count;
    for (unsigned i = 0; i < count; i++) {
        if (reported[i] == name || strcmp(reported[i], name) == 0)
            return 0;
    }
    if (count < MAX_REPORTED)
        reported[count++] = name;
    fprintf(stderr, "[pcrt] unimplemented: %s\n", name);
    return 0;
}
