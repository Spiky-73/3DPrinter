#include "Encoder.h"


Encoder encoder(2, 5);

const unsigned int PWM = 3;
const unsigned int DIRECTION = 12;

long stopTime, time, deltaTime;
int position, deltaPos;
String info = "";


void setup() {
    Serial.begin(9600);
#ifdef MOTOR
    pinMode(PWM, OUTPUT);
    pinMode(DIRECTION, OUTPUT);
    digitalWrite(DIRECTION, LOW); // Pour ne pas laisser les broches de pilotage du moteur dans un état indéterminé
    analogWrite(PWM, 0);
#endif
    Serial.println("Basic Encoder Test");
}

void loop() {
    long t = millis();
    deltaTime = time - t;
    time = t;
    long pos = encoder.read();
    deltaPos = pos-position;
    position = pos;
#ifdef MOTOR
    updateMotor();
#else
    updateEncoder();
#endif

    if(info.length() > 0){
        Serial.println(info);
        info = "";
    }
}
void updateMotor(){
    if(Serial.available()){
        String message = Serial.readString();
        Serial.println(message);
        int split = message.indexOf(' ');
        String spd;
        if (split != 0){
            spd = message.substring(0, split);
            stopTime = time + message.substring(split+1).toInt();
        }
        else {
            spd = message;
            stopTime = time + 500;
        }
        int speed = spd.toInt();
        Serial.print("Moving motor at ");
        Serial.print(speed);
        Serial.print(" for ");
        Serial.print(stopTime-time);
        Serial.println(" ms");
        digitalWrite(DIRECTION, speed < 0);
        analogWrite(PWM, abs(speed));
    }
    if(stopTime != 0 && time < stopTime) {
        updateEncoder();
        info + "S" + String((float)deltaPos / deltaTime) + " ";
    }else if (stopTime != 0) {
        analogWrite(PWM, 0);
        stopTime = 0;
    } 
}

void updateEncoder(){
    info += "P" + String(position) + " ";
    String info = "";
    long pos = encoder.read();
    long t = millis();
    info += "P" + String(pos) + " ";
    Serial.println(info);
    time = t;
    position = pos;
}