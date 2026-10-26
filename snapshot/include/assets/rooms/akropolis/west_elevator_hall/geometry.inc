/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisWestElevatorHallCollisionGridNormals[7] = {
#include "assets/akropolis_west_elevator_hall_collision_07D3C_normals.inc"
};
static SVECTOR _gAkropolisWestElevatorHallCollisionGridVerts[29] = {
#include "assets/akropolis_west_elevator_hall_collision_07D3C_verts.inc"
};
static WorldCollisionGridFace _gAkropolisWestElevatorHallCollisionGridFaces[14] = {
#include "assets/akropolis_west_elevator_hall_collision_07D3C_faces.inc"
};
static s16 _gAkropolisWestElevatorHallCollisionGridCells[52] = {
#include "assets/akropolis_west_elevator_hall_collision_07D3C_cells.inc"
};
#define AKROPOLIS_WEST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisWestElevatorHallCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisWestElevatorHallCollisionGridTable[6] = {
#include "assets/akropolis_west_elevator_hall_collision_07D3C_table.inc"
};
#undef AKROPOLIS_WEST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisWestElevatorHallCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisWestElevatorHallCollisionGridNormals, .vertices = _gAkropolisWestElevatorHallCollisionGridVerts, .faces = _gAkropolisWestElevatorHallCollisionGridFaces, .cellFaceIds = _gAkropolisWestElevatorHallCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
