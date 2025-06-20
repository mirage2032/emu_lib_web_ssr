#include <math.h>
#include <stdint.h>

#define WIDTH 192
#define HEIGHT 128
#define FRAME_DELAY 5000

typedef uint8_t DISPLAY[HEIGHT][WIDTH];
volatile DISPLAY *display = (DISPLAY *)0x4000;

void set_pixel(uint16_t x, uint16_t y, uint8_t color) {
    if (x < WIDTH && y < HEIGHT) {
        (*display)[y][x] = color;
    }
}

void clear_display(uint8_t color) {
    for (uint16_t y = 0; y < HEIGHT; y++) {
        for (uint16_t x = 0; x < WIDTH; x++) {
            (*display)[y][x] = color;
        }
    }
}

void draw_circle(uint16_t cx, uint16_t cy, uint16_t radius, uint8_t color) {
    int16_t x0 = cx - radius;
    int16_t x1 = cx + radius;
    int16_t y0 = cy - radius;
    int16_t y1 = cy + radius;

    for (int16_t y = y0; y <= y1; y++) {
        for (int16_t x = x0; x <= x1; x++) {
            int16_t dx = x - cx;
            int16_t dy = y - cy;
            if (dx * dx + dy * dy <= radius * radius) {
                set_pixel(x, y, color);
            }
        }
    }
}

void frame_delay(void) {
    volatile uint16_t i;
    for (i = 0; i < FRAME_DELAY; i++);
}

void draw_loop(void) {
    const uint16_t center_x = WIDTH / 2;
    const uint16_t center_y = HEIGHT / 2;
    const uint8_t bg_color = 0x4F;
    const uint8_t circle_color = 0xAF;

    const uint8_t base_radius = 20;
    const uint8_t radius_range = 10;    // How much the radius pulsates
    const float pulsate_step = 0.1f;    // Pulsation speed

    float pulsate_phase = 0.0f;
    uint16_t prev_radius = 0;

    clear_display(bg_color);
    while (1) {
        float radius_sin = (sinf(pulsate_phase) + 1.0f) * 0.5f;
        uint16_t radius = base_radius + (uint16_t)(radius_sin * radius_range);
        draw_circle(center_x, center_y, prev_radius, bg_color);
        draw_circle(center_x, center_y, radius, circle_color);
        pulsate_phase += pulsate_step;
        frame_delay();
    }
}

int main(void) {
    draw_loop();
    return 0;
}