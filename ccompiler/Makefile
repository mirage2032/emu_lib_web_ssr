full: build build-asm

build:
	docker run -v .:/src/ -it z88dk/z88dk zcc +z80 -vn -O3 -startup=31 -o main.rom -create-app -compiler=sdcc ./main.c -lm
custom:
	docker run -v .:/src/ -it z88dk/z88dk zcc +z80 -vn -O3 -startup=0 -o main.out main.c -create-app -lm
build-asm:
	docker run -v .:/src/ -it z88dk/z88dk zcc +z80 -create-app ./main.c -compiler=sdcc --no-crt -S

clean:
	rm -rf *.bin *.rom *.asm *.out
