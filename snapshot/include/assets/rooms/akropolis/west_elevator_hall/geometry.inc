/* rooms/akropolis/west_elevator_hall/package: written from its room instance by tools/buildsteps/gen_room_inc.py. */
#define ROOM_CAT_(a, b) a##b
#define ROOM_CAT(a, b) ROOM_CAT_(a, b)

static SVECTOR _gAcropolisWestElevatorHallCollisionGridNormals[7] = {
#include "assets/acropolis_west_elevator_hall_collision_07D3C_normals.inc"
};
static SVECTOR _gAcropolisWestElevatorHallCollisionGridVerts[29] = {
#include "assets/acropolis_west_elevator_hall_collision_07D3C_verts.inc"
};
static WorldCollisionGridFace _gAcropolisWestElevatorHallCollisionGridFaces[14] = {
#include "assets/acropolis_west_elevator_hall_collision_07D3C_faces.inc"
};
static s16 _gAcropolisWestElevatorHallCollisionGridCells[52] = {
#include "assets/acropolis_west_elevator_hall_collision_07D3C_cells.inc"
};
#define ACROPOLIS_WEST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET(halfwordOffset) (&_gAcropolisWestElevatorHallCollisionGridCells[(halfwordOffset)])
static s16* _gAcropolisWestElevatorHallCollisionGridTable[6] = {
#include "assets/acropolis_west_elevator_hall_collision_07D3C_table.inc"
};
#undef ACROPOLIS_WEST_ELEVATOR_HALL_CELL_FACE_LIST_AT_OFFSET
static WorldCollisionGrid _gAcropolisWestElevatorHallCollisionGrid = { .viewCoord = NULL, .normals = _gAcropolisWestElevatorHallCollisionGridNormals, .vertices = _gAcropolisWestElevatorHallCollisionGridVerts, .faces = _gAcropolisWestElevatorHallCollisionGridFaces, .cellFaceIds = _gAcropolisWestElevatorHallCollisionGridTable, .xBias = 0, .zBias = 0, .cellCountX = 0, .cellCountZ = 0, .cellSize = 0, .faceCount = 0 };
