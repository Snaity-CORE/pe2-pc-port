/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

#include "types.h"
#include <psyq/sys/types.h>
#include <psyq/libgte.h>
#include "gameplay/collision.h"

static SVECTOR _gAkropolisEastElevatorHallCollisionGridNormals[10] = {
#include "assets/akropolis_east_elevator_hall_collision_09278_normals.inc"
};
static SVECTOR _gAkropolisEastElevatorHallCollisionGridVerts[66] = {
#include "assets/akropolis_east_elevator_hall_collision_09278_verts.inc"
};
static WorldCollisionGridFace _gAkropolisEastElevatorHallCollisionGridFaces[33] = {
#include "assets/akropolis_east_elevator_hall_collision_09278_faces.inc"
};
static s16 _gAkropolisEastElevatorHallCollisionGridCells[90] = {
#include "assets/akropolis_east_elevator_hall_collision_09278_cells.inc"
};
#define AKROPOLIS_EAST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAkropolisEastElevatorHallCollisionGridCells[(halfwordOffset)])
static s16* _gAkropolisEastElevatorHallCollisionGridTable[8] = {
#include "assets/akropolis_east_elevator_hall_collision_09278_table.inc"
};
#undef AKROPOLIS_EAST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAkropolisEastElevatorHallCollisionGrid = { .viewCoord = NULL, .normals = _gAkropolisEastElevatorHallCollisionGridNormals, .vertices = _gAkropolisEastElevatorHallCollisionGridVerts, .faces = _gAkropolisEastElevatorHallCollisionGridFaces, .cellFaceIds = _gAkropolisEastElevatorHallCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
