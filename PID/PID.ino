#include "Encoder.h"
#include "defines.h"

bool working = false;
unsigned long pauseUntil = 0;

long idleStart = 0;
long logPosTime = 0;

unsigned long time, delta;

class MotorPID {
public:
    MotorPID(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, float kp)
        : _position(0), target(0), speed(0),
        kp(kp),
        _direction(direction), _pwm(pwm),
        _encoder(interrupt, encoder) {
        pinMode(direction, OUTPUT);
        pinMode(pwm, OUTPUT);
        digitalWrite(direction, LOW);
    }

    void update(long delta){
#ifdef SIMULATE
        _position += speed * delta * 3;
#else 
        _position = _encoder.read();
#endif
        int16_t error = target - _position;
        float command = kp * error;
        setSpeed(command);
    }

    void setSpeed(float speed) {
        this->speed = constrain(speed, -1, 1);

        analogWrite(_pwm, 255*abs(speed));
        digitalWrite(_direction, (speed > 0));
    }

    void hardsetPosition(uint16_t position) {
        _position += position - target;
        target = position;
    }

    inline bool atTarget() const { return abs((int32_t)target - position()) <= 2; }

    inline const uint16_t position() const { return _position; }

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


MotorPID xPID(0, 0, 0, 0, 1), yPID(0, 0, 0, 0, 1), zPID(0, 0, 0, 0, 1);

// TODO change SERIAL_RX_BUFFER_SIZE
uint8_t bytesToRead = WaitingForData;
byte instructionSize = 0;
byte intruction[16];

void setup() {
    Serial.begin(115200);
}
void loop() {
    long current = millis();
    delta = current - time;
    time = current;
    updateReception();
    updateIntruction();
}

void updateReception(){
    if (bytesToRead == WaitingForData && Serial.available() > 0) { // received count byte
        bytesToRead = Serial.read();
    }
    if (bytesToRead > Buffered && Serial.available() >= bytesToRead) { // received data
        instructionSize = bytesToRead;
        Serial.readBytes(intruction, instructionSize);
        bytesToRead = Buffered;
    }
}

void updateIntruction(){
    if (working && time >= pauseUntil && xPID.atTarget() && yPID.atTarget() && zPID.atTarget()) {
        writeHeader('_', 0);
        debug(idleStart = time);
        working = false;
    }
    if (!working && bytesToRead == Buffered) {
        debug(
            if (idleStart != time) {
                String s = "Idle (" + String(time - idleStart) + "ms)";
                writeHeader('D', s.length());
                Serial.write(s.c_str(), s.length());
            }
        )
        processInstruction(intruction, instructionSize);
        bytesToRead = WaitingForData;
        working = true;
        debug(logPosTime = 0);
    }
    xPID.update(delta);
    yPID.update(delta);
    zPID.update(delta);

    debug(
        if (working && time >= logPosTime) {
            String info = 'X' + String(xPID.position()) + '>' + String(xPID.target);
            info += " Y" + String(yPID.position()) + '>' + String(yPID.target);
            info += " Z" + String(zPID.position()) + '>' + String(zPID.target);
            writeHeader('D', info.length());
            Serial.write(info.c_str(), info.length());
            logPosTime = time + 500;
        }
    )
}


void processInstruction(const byte* const buffer, uint8_t lenght){
    byte n = 0;
    while (n < lenght){
        switch (buffer[n++]) {
        case 'H':
            simulate(
                writeHeader('I', 41);
                Serial.write("Simulating movement. Motors will not move", 41);
            )
            debug(
                writeHeader('D', 25);
                Serial.write("Motor positions displayed", 25);
            )
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
            pauseUntil = time + parse_uint32_t(buffer, n);
            break;
        default:
            writeHeader('E', 15);
            Serial.write("Unknown param ", 14);
            Serial.write(buffer[n-1]);
            return;
        }
    }
}

inline uint16_t parse_uint16_t(const byte *const buffer, byte &index) { return buffer[index++] + (buffer[index++] << 8); }
inline uint32_t parse_uint32_t(const byte *const buffer, byte &index) { return parse_uint16_t(buffer, index) + parse_uint16_t(buffer, index) << 16; }