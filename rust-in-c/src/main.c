#include <stdint.h>
#include <stdio.h>

struct Point {
    int* x;
    int* y;
};

extern int32_t slength_i32(struct Point* p);

int main() {
    int x = 4, y = 5;
    struct Point input = (struct Point) { .x = &x, .y = &y };
    int32_t output = slength_i32(&input);
    printf("length of Point { .x = %d, .y = %d } is %d\n", *input.x, *input.y, output);
    return 0;
}