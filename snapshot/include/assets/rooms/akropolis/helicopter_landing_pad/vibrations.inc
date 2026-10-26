/* rooms/akropolis/helicopter_landing_pad/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "gameplay/pad_script.h"

#define ROOM_IS__gAkropolisHelicopterLandingPadEncounterVibrationCommands 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
static PadScriptCmd _gAkropolisHelicopterLandingPadEncounterVibrationCommands[4] = {
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
};
static PadScriptVibrationSegment _gAkropolisHelicopterLandingPadEncounterVibrationSegments[2] = {
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
};
#endif
#undef ROOM_IS__gAkropolisHelicopterLandingPadEncounterVibrationCommands

#define ROOM_IS_gAkropolisHelicopterLandingPadLethalStrikeVibrationCommands 1
#if ROOM_CAT(ROOM_IS_, ROOM_PART) == 1
PadScriptCmd gAkropolisHelicopterLandingPadLethalStrikeVibrationCommands[2] = {
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
};
PadScriptVibrationSegment gAkropolisHelicopterLandingPadLethalStrikeVibrationSegment = { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 };
static PadScriptCmd _gAkropolisHelicopterLandingPadDepartureThirdVibrationCommands[2] = {
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
};
static PadScriptVibrationSegment _gAkropolisHelicopterLandingPadDepartureThirdVibrationSegments[2] = {
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
};
static PadScriptCmd _gAkropolisHelicopterLandingPadDepartureSecondVibrationCommands[4] = {
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
};
static PadScriptVibrationSegment _gAkropolisHelicopterLandingPadDepartureSecondVibrationSegments[2] = {
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
};
static PadScriptCmd _gAkropolisHelicopterLandingPadDepartureFirstVibrationCommands[4] = {
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
    { .holdCommand = { .bytes = { .opcode = 0, .operand = 0 } }, .lerpCommand = { .bytes = { .opcode = 0, .operand = 0 } } },
};
static PadScriptVibrationSegment _gAkropolisHelicopterLandingPadDepartureFirstVibrationSegments[2] = {
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
    { .startIntensity = 0, .endIntensity = 0, .durationFrames = 0, .field_3 = 0 },
};
#endif
#undef ROOM_IS_gAkropolisHelicopterLandingPadLethalStrikeVibrationCommands
#undef ROOM_PART
