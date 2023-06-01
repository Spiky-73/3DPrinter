bool up = false;
byte data[3] = {73, 42, 0};

void setup(){
    Serial.begin(115200);
}

void loop() {
    if(up){
        Serial.write(6);
        delay(30);
        for (size_t i = 0; i < 2; i++) Serial.write(data, 3);
        delay(200);
    } else {
        if(Serial.available() > 0){
            int size = Serial.available();
            byte bytes[size];
            Serial.readBytes(bytes, size);
            up = true;
        }
    }

}
