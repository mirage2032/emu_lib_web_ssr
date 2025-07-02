#include <stdint.h>
#include <math.h>

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

void draw_circle_outline(uint16_t cx, uint16_t cy, uint16_t radius, uint8_t color) {
    if (radius == 0) return;

    int16_t x = 0;
    int16_t y = radius;
    int16_t d = 1 - radius;

    while (x <= y) {
        set_pixel(cx + x, cy + y, color);
        set_pixel(cx - x, cy + y, color);
        set_pixel(cx + x, cy - y, color);
        set_pixel(cx - x, cy - y, color);
        set_pixel(cx + y, cy + x, color);
        set_pixel(cx - y, cy + x, color);
        set_pixel(cx + y, cy - x, color);
        set_pixel(cx - y, cy - x, color);

        x++;
        if (d < 0) {
            d += 2 * x + 1;
        } else {
            y--;
            d += 2 * (x - y) + 1;
        }
    }
}

void frame_delay(void) {
    volatile uint16_t i;
    for (i = 0; i < FRAME_DELAY; i++);
}

int main(void) {
    const uint16_t cx = WIDTH / 2;
    const uint16_t cy = HEIGHT / 2;
    const uint8_t bg_color = 0x4F;
    const uint8_t circle_color = 0xAF;

    const float base_radius = 5.0f;
    const float radius_range = 40.0f;
    const float pulsate_speed = 0.3f;

    float phase = 0.0f;
    uint16_t prev_radius = (uint16_t)base_radius;

    clear_display(bg_color);

    while (1) {
        // Calculate new radius with sine wave (0 to 1 scaled)
        float sine_val = (sinf(phase) + 1.0f) * 0.5f;  // from 0 to 1
        uint16_t radius = (uint16_t)(base_radius + sine_val * radius_range);

        // Erase previous circle by drawing it in bg color
        draw_circle_outline(cx, cy, prev_radius, bg_color);

        // Draw new circle in visible color
        draw_circle_outline(cx, cy, radius, circle_color);

        prev_radius = radius;
        phase += pulsate_speed;
        if (phase > 6.2831853f) {  // wrap around 2*pi
            phase -= 6.2831853f;
        }

        frame_delay();
    }

    return 0;
}
