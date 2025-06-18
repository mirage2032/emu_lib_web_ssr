#include <math.h>
#include <float.h>
#include <stdio.h>
#include <input.h>
#define WIDTH 192
#define HEIGHT 128
void infinite_loop(void) {
    while (1) {
        // Do nothing, just loop forever
    }
}

typedef uint8_t DISPLAY[HEIGHT][WIDTH];
DISPLAY *display = (DISPLAY *)0x4000;

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
//#pragma output CRT_ORG_CODE = 0x120// this is equivalent to your -zorg=40000 compile line option so another way to do it
//#pragma output CRT_ENABLE_CLOSE = 0 // do not close open files on exit (at this time this has no effect)
//#pragma output CLIB_EXIT_STACK_SIZE = 0 // get rid of the exit stack (no functions can be registered with atexit() )
//#pragma output CLIB_MALLOC_HEAP_SIZE = 0 // malloc's heap will not exist
//#pragma output CLIB_STDIO_HEAP_SIZE = 0 // stdio's heap will not exist (you will not be able to open files)
//#pragma output CLIB_FOPEN_MAX = -1 // don't even create the open files list
//int BOO(){
//    *(int*)0xFF = 0x33;
//    return 1;
//}
//
//int main(){
//    *(int*)20 = 10;
//    return 1;
//}
//#include <intrinsic.h>
//
//void infinite_loop(void) {
//    while (1) {
//        // Do nothing, just loop forever
//    }
//}
//
//void main(void) {
//    intrinsic_halt();
//    // The program will never reach this point
//    // but it's good practice to have a return statement
//    return;
//}