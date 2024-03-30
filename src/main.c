#include<raylib.h>

const int WIDTH = 800;
const int HEIGHT = 450;

int main(void) 
{
    InitWindow(WIDTH, HEIGHT, "Space");

    Camera3D camera = { 0 };
    camera.position = (Vector3) { 10.0f, 10.0f, 10.0f };
    camera.target   = (Vector3) {  0.0f,  0.0f,  0.0f };
    camera.up       = (Vector3) {  0.0f,  4.0f,  0.0f };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;

    Vector3 cubePosition = { 0.0f, 0.0f, 0.0f };

    SetTargetFPS(60);

    while (!WindowShouldClose())
    {
        UpdateCamera(&camera, CAMERA_FREE);

        if (IsKeyPressed('Z')) camera.target = (Vector3){ 0.0f, 0.0f, 0.0f };

        BeginDrawing();
		ClearBackground(RAYWHITE);

		BeginMode3D(camera);

		DrawCube(cubePosition, 2.0f, 2.0f, 2.0f, RED);
		DrawCubeWires(cubePosition, 2.0f, 2.0f, 2.0f, MAROON);

        DrawGrid(10, 1.0f);

        EndMode3D();

        EndDrawing();
    }

    CloseWindow();
    return 0;
}
