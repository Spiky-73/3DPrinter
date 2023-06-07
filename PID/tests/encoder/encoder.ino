#include "Encoder.h"


Encoder encoder(2, 5);


const unsigned int PWM = 3;
const unsigned int DIRECTION = 12;

long startTime, time;
int position;

#define MOTOR

void setup() {
    Serial.begin(9600);
#ifdef MOTOR
    pinMode(PWM, OUTPUT);
    pinMode(DIRECTION, OUTPUT);

    digitalWrite(DIRECTION, LOW); // Pour ne pas laisser les broches de pilotage du moteur dans un état indéterminé
    analogWrite(PWM, 255);

    startTime = millis();
    time = startTime;
    position = 0;
#endif
    Serial.println("Basic Encoder Test");
}

void loop() {
    String info = "";
    long pos = encoder.read();
    long t = millis();
    info += "P" + String(pos) + " ";
#ifdef MOTOR
    if(t >= startTime + 400) analogWrite(PWM, 0);
    else info + "S" + String((float)(position - pos) / (time - t)) + " ";
#endif
    Serial.println(info);
    time = t;
    position = pos;
}