#include "Encoder.h"
Encoder myEnc(2, 5);

long commande = 0;
long Position;
int erreur;
int consigne = 180; // consigne en position à atteindre
float kp = 0.3f;

void setup() {
    // initialisation et acquisition de la valeur du correcteur proportionnel Kp à travers le moniteur série
    Serial.begin(115200);
    pinMode(12, OUTPUT); // pin Direction
    pinMode(3, OUTPUT);  // pin PWM
    Serial.print("temps (ms)");
    Serial.print(" ; ");
    Serial.print("position");
    Serial.print(" ; ");
    Serial.print("consigne");
    Serial.print(" ; ");
    Serial.println("commande");
}

void loop() {
    // Attention, mettre le moniteur serie a 115200 baud pour pouvoir lire l'affichage
    Position = myEnc.read();      // lecture de la position du codeur
    erreur = consigne - Position; // calcul de la valeur de l'erreur
    commande = kp * erreur;       // calcul de la valeur de commande moteur en fonction de kp et de erreur;
    moteur(commande);
    if(commande != 0){
        Serial.print(millis());
        Serial.print(" ; ");
        Serial.print(Position);
        Serial.print(" ; ");
        Serial.print(consigne);
        Serial.print(" ; ");
        Serial.println(commande);
    }
}

// Commande du moteur
void moteur(int vit) {
    vit = constrain(vit, -255, 255);

    analogWrite(3, abs(vit));
    digitalWrite(12, (vit > 0));
}
