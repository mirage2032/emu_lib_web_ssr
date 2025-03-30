#include <stdint.h>
#include <math.h>
#define WIDTH 256
#define HEIGHT 192

typedef uint8_t DISPLAY[HEIGHT][WIDTH];
DISPLAY *display = (DISPLAY *)0x2000;
uint16_t last_index = 0;

void set_pixel(uint16_t x, uint16_t y, uint8_t color){
    if (x < WIDTH && y < HEIGHT) {
            (*display)[y][x] = color;
        }
}

void set_circle(int16_t x0, int16_t y0, int16_t radius, uint8_t color) {
    int16_t f = 1 - radius;
    int16_t ddF_x = 0;
    int16_t ddF_y = -2 * radius;
    int16_t x = 0;
    int16_t y = radius;

    set_pixel(x0, y0 + radius, color);
    set_pixel(x0, y0 - radius, color);
    set_pixel(x0 + radius, y0, color);
    set_pixel(x0 - radius, y0, color);

    while (x < y) {
        if (f >= 0) {
            y--;
            ddF_y += 2;
            f += ddF_y;
        }
        x++;
        ddF_x += 2;
        f += ddF_x + 1;

        set_pixel(x0 + x, y0 + y, color);
        set_pixel(x0 - x, y0 + y, color);
        set_pixel(x0 + x, y0 - y, color);
        set_pixel(x0 - x, y0 - y, color);
        set_pixel(x0 + y, y0 + x, color);
        set_pixel(x0 - y, y0 + x, color);
        set_pixel(x0 + y, y0 - x, color);
        set_pixel(x0 - y, y0 - x, color);
    }
}

void clear_screen(uint8_t color){
    for(uint16_t y = 0; y < HEIGHT; y++){
        for(uint16_t x = 0; x < WIDTH; x++){
            set_pixel(x, y, color);
        }
    }
}

int main(void) {
    clear_screen(0x74);
    uint8_t x = 0;
    uint8_t y = 0;
    uint8_t z = 3;
    for(;;){
        uint8_t color = (x&0xF0 | (y>>4))+z;
        set_circle(x, y, color>>1, color);
        x= (x+5)%WIDTH;
        y= (y+7)%HEIGHT;
        z+=3;
    }
    return 0;
}
