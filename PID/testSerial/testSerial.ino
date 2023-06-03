#include "../defines.h"

bool up = false;
byte data[3] = {'s', 'p', 'y'};

void setup(){
    Serial.begin(115200);
}

void loop() {
    if(up){
        Serial.write(7);
        Serial.write('D');
        for (size_t i = 0; i < 2; i++) Serial.write(data, 3);
        delay(500);
    } else {
        if(Serial.available() > 0){
            int size = Serial.available();
            byte bytes[size];
            Serial.readBytes(bytes, size);
            up = true;
            debug(
                String s = String("Waiting for ") + size + String(" bytes");
                writeHeader('D', s.length());
                Serial.write(s.c_str(), s.length());
            )
        }
    }

}
