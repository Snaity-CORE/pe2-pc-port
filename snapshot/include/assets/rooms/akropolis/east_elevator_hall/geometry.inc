/* rooms/akropolis/east_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisEastElevatorHallCollisionGridNormals[10] = {
#include "assets/acropolis_east_elevator_hall_collision_09278_normals.inc"
};
static SVECTOR _gAcropolisEastElevatorHallCollisionGridVerts[66] = {
#include "assets/acropolis_east_elevator_hall_collision_09278_verts.inc"
};
static WorldCollisionGridFace _gAcropolisEastElevatorHallCollisionGridFaces[33] = {
#include "assets/acropolis_east_elevator_hall_collision_09278_faces.inc"
};
static s16 _gAcropolisEastElevatorHallCollisionGridCells[90] = {
#include "assets/acropolis_east_elevator_hall_collision_09278_cells.inc"
};
#define ACROPOLIS_EAST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisEastElevatorHallCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisEastElevatorHallCollisionGridTable[8] = {
#include "assets/acropolis_east_elevator_hall_collision_09278_table.inc"
};
#undef ACROPOLIS_EAST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisEastElevatorHallCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisEastElevatorHallCollisionGridNormals, .vertices = _gAcropolisEastElevatorHallCollisionGridVerts, .faces = _gAcropolisEastElevatorHallCollisionGridFaces, .cellFaceIds = _gAcropolisEastElevatorHallCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
