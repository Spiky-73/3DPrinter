#include "Adafruit_MotorShield.h"
#include "Encoder.h"
#include "HardwareSerial.h"

class MotorPID {
public:
    MotorPID(uint8_t interrupt, uint8_t encoder, uint8_t n, float kp)
        : _position(0), target(0), speed(0),
        kp(kp),
        _encoder(interrupt, encoder) {
        _motor = AFMS.getMotor(n);
    }

    void update(){
        _position = _encoder.read();
        int16_t error = target - _position;
        float command = kp * error;

        speed = constrain(command, -255, 255);
        _motor->setSpeed(abs(speed));
        _motor->run(speed > 0 ? FORWARD : BACKWARD);        
    }

    inline const uint8_t position() const { return _position; }

    uint16_t target;
    int16_t speed;

    float kp;

private:
    uint16_t _position;
    Encoder _encoder;
    Adafruit_DCMotor* _motor;
};


Adafruit_MotorShield AFMS = Adafruit_MotorShield();


const uint8_t BUFFERED_INTRUCTIONS = 5;
uint8_t next = 0, buffered = 0;
const uint8_t READ_LENGTH = 64;
uint8_t readBuffer[8][READ_LENGTH];


MotorPID xPID;
MotorPID yPID;
MotorPID zPID;


void setup() {

    xPID = MotorPID(0,8,0,0.05f);
    yPID = MotorPID(1,8,0,0.05f);
    zPID = MotorPID(2,8,0,0.05f);
    Serial.begin(115200);
}

void loop() {
    if (Serial.available() > 0){
        buffered = (buffered + 1) % BUFFERED_INTRUCTIONS;
        Serial.readBytes(readBuffer[buffered], 1);
        Serial.readBytes(readBuffer[buffered], readBuffer[buffered][0]);
    }
    // processInstruction();
    xPID.update();
    yPID.update();
    zPID.update();
}

void processInstruction(){
    const uint8_t *buffer = readBuffer[next];
    uint8_t n = 0;
    while (n < READ_LENGTH){
        switch (buffer[n++]) {
        case 'H':
            xPID.target = 0; // TODO reset the encoders
            yPID.target = 0;
            zPID.target = 0;
            break;
        case 'X':
            xPID.target = parse_uint16_t(buffer, n);
            break;
        case 'Y':
            yPID.target = parse_uint16_t(buffer, n);
            break;
        case 'Z':
            zPID.target = parse_uint16_t(buffer, n);
            break;
        }
    }
    next = (next + 1) % BUFFERED_INTRUCTIONS;
}

inline uint16_t parse_uint16_t(const uint8_t const *buffer, uint8_t &index) { return buffer[index++] + buffer[index++] << 8; }