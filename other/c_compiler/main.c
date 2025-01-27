#include <stdint.h>
typedef uint8_t DISPLAY[192][256];
DISPLAY *display = (DISPLAY *)0x1000;
uint16_t last_index = 0;

int main(void);
void entry(void) {
  main();
  __asm__("halt");
}

// uint16_t current = 0;
uint8_t blue = 0;
int main(void) {
main:
  for (uint8_t x = 0;; x++) {
    for (uint8_t y = 0; y <= 191; y++) {
      uint8_t val = x & 0b11100000;
      val = val | ((y & 0b11100000) >> 3);
      val = val | (blue & 11);
      // uint8_t blue = ((x + y) & 0b01100000) >> 5;
      (*display)[y][x] = val; //| blue;
    }
    if (x == 255)
      break;
  }
  blue++;
  goto main;
  return 0;
}
