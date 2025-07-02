char* DISPLAY = (char*)0x4000;
const unsigned char WIDTH = 192;
const unsigned char HEIGHT = 128;

int pixelindex(unsigned char x, unsigned char y){
    return x+WIDTH*y;
}

void main(){
    char color = 0b01001010;
    for (;;){
        for (unsigned char y=0;y<HEIGHT;y++){
            for (unsigned char x=0;x<WIDTH;x++){
                int index = pixelindex(x,y);
                DISPLAY[index] = color;
            }
        }
        color+=3;
    }
    return;
}