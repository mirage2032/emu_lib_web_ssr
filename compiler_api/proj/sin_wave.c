#include <stdint.h>
#include <math.h>
#define WIDTH 256
#define HEIGHT 192
//OPTIMIZE by SIN LUT
typedef uint8_t DISPLAY[HEIGHT][WIDTH];
DISPLAY *display = (DISPLAY *)0x3000;

void set_pixel(uint16_t x, uint16_t y, uint8_t color){
    if (x < WIDTH && y < HEIGHT) {
            (*display)[y][x] = color;
        }
}

void clear_display(uint8_t color){
    for (uint16_t y = 0; y < HEIGHT; y++) {
        for (uint16_t x = 0; x < WIDTH; x++) {
            (*display)[y][x] = color;
        }
    }
}

void draw_wave(uint8_t *y_cache,uint8_t color){
    uint8_t x = 0;
    do {
        uint16_t y = (uint16_t)y_cache[x];
        if (x!=0xff){
            uint16_t next_y = (uint16_t)y_cache[x+1];
            if (y < next_y){
                for (uint16_t i = y; i < next_y; i++){
                    set_pixel(x, i, color);
                }
            }
            else{
                for (uint16_t i = next_y; i < y; i++){
                    set_pixel(x, i, color);
                }
            }
        }
        set_pixel(x, y, color);
        x++;
    } while (x != 0);
}

void calc_wave(uint8_t amplitude, float frequency, uint8_t *y_cache){
    uint8_t x = 0;
    do {
        y_cache[x] = (uint8_t)(amplitude * sinf((float)x * frequency / 10.0f) + (HEIGHT / 2));
        x++;
    } while (x != 0);
}

void draw_loop(void){
    static const uint8_t wave_color = 0xAF;
    static const uint8_t background_color = 0x4F;
    clear_display(background_color);
    uint8_t amplitude = 20;
    float frequency = 1.0f;
    uint8_t y_cache[WIDTH];
    uint8_t y_cache_clone[WIDTH];
    uint8_t* y_cache_ptr = y_cache;
    uint8_t* y_cache_clone_ptr = y_cache_clone;
    for (;;) {
        calc_wave(amplitude, frequency, y_cache_ptr);
        draw_wave(y_cache_clone_ptr, background_color);
        draw_wave(y_cache_ptr, wave_color);
        frequency += 0.2f;
        //switch ptrs
        uint8_t* temp = y_cache_ptr;
        y_cache_ptr = y_cache_clone_ptr;
        y_cache_clone_ptr = temp;
    }
}

int main(void){
    draw_loop();
    return 0;
}