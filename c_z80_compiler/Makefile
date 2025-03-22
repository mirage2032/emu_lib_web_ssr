full: build build-asm

build:
	docker run -v .:/src/ -it z88dk/z88dk zcc +z80 -o main.bin -create-app ./main.c -compiler=sdcc --no-crt

build-asm:
	docker run -v .:/src/ -it z88dk/z88dk zcc +z80 -create-app ./main.c -compiler=sdcc --no-crt -S

clean:
	rm -rf *.bin *.rom *.asm
