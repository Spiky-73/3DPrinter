#include "Encoder.h"
#include "HardwareSerial.h"

class MotorPID {
public:
    MotorPID(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, float kp)
        : _position(0), target(0), speed(0),
        kp(kp),
        _direction(direction), _pwm(pwm),
        _encoder(interrupt, encoder) {
        pinMode(direction, OUTPUT);
    }

    void update(){
        _position = _encoder.read();
        int16_t error = target - _position;
        float command = kp * error;

        setSpeed(command);
    }

    void setSpeed(int16_t speed) {
        this->speed = constrain(speed, -255, 255);

        analogWrite(_pwm, abs(speed));
        digitalWrite(_direction, (speed > 0));
    }

    inline bool atTarget() const { return abs(target - position()) < 2; }

    inline const uint8_t position() const { return _position; }

    uint16_t target;
    int16_t speed;

    float kp;

private:
    uint16_t _position;
    Encoder _encoder;
    uint8_t _direction, _pwm;
};

enum SerialState {
    Buffered = 0,
    WaitingForData = 255
};


MotorPID xPID, yPID, zPID;
bool working = false;
unsigned long pauseUntil = 0;

// TODO change SERIAL_RX_BUFFER_SIZE
uint8_t bytesToRead = WaitingForData;
byte instructionSize = 0;
byte intruction[16];
const byte instrDone[2] = {1, 'I'};


void setup() {
    xPID = MotorPID(0,0,0,0,1);
    yPID = MotorPID(0,0,0,0,1);
    zPID = MotorPID(0,0,0,0,1);
    Serial.begin(115200);
}

void loop() {
    updateReception();
    updateIntruction();
}

void updateReception(){
    if (bytesToRead == WaitingForData && Serial.available()) // received count byte
        bytesToRead = Serial.read();
    if (bytesToRead > Buffered && Serial.available() >= bytesToRead) { // received data
        instructionSize = bytesToRead;
        Serial.readBytes(intruction, instructionSize);
        bytesToRead = Buffered;
    }
}

void updateIntruction(){
    if (working && millis() >= pauseUntil && xPID.atTarget() && yPID.atTarget() && yPID.atTarget()) {
        Serial.write(instrDone, instrDone[0]+1);
        working = false;
    }
    if (!working && bytesToRead == Buffered) {
        processInstruction(intruction, instructionSize);
        bytesToRead = WaitingForData;
        working = true;
    }

    xPID.update();
    yPID.update();
    zPID.update();
}


void processInstruction(const byte* const buffer, uint8_t lenght){
    byte n = 0;
    while (n < lenght){
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
        case 'B':
            pauseUntil = millis() + parse_uint32_t(buffer, n);
            break;
        }
    }
}

inline uint16_t parse_uint16_t(const byte *const buffer, byte &index) { return buffer[index++] + buffer[index++] << 8; }
inline uint32_t parse_uint32_t(const byte *const buffer, byte &index) { return buffer[index++] + buffer[index++] << 8 + buffer[index++] << 16 + buffer[index++] << 24; }
