#include<raylib.h>

const int WIDTH = 800;
const int HEIGHT = 450;

int main(void) 
{
    InitWindow(WIDTH, HEIGHT, "Space");
    SetTargetFPS(60);

    while (!WindowShouldClose())
    {
        BeginDrawing();
        ClearBackground(RAYWHITE);
        EndDrawing();
    }

    CloseWindow();
    return 0;
}
