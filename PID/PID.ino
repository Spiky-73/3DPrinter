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
        _positionF += speed * delta * 3;
        _positionF = max(0, _positionF);
        uint16_t npos = _positionF;
#else
        uint16_t npos = _encoder.read();
#endif
        delta = npos - _position;
        _position = npos;
        if(homing == 0) PID();
        else home();
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

    inline void resethome() {
        homing = 4;
        target = 0;
    }

    inline bool atTarget() const { return homing == 0 && abs((int32_t)target - position()) <= 1; }

    inline const uint16_t position() const { return _position; }

    int16_t delta;
    uint16_t target;
    float speed;

    float kp;
    byte homing;

private:
    void PID() {
        int16_t error = target - _position;
        float command = kp * error;
        setSpeed(command);
    }

    void home(){
        if(time < _sleep) return;
        switch (homing) {
        case 4: {
            setSpeed(-1);
            _sleep = time + 20;
            homing--;
            logPosTime = 0;
            break;
        }
        case 3: { // fast
            if(delta == 0) {
                setSpeed(0.3f);
                homing--;
                logPosTime = 0;
            }
            break;
        }
        case 2: {// target 50
            if(_position > 50){
                setSpeed(-1);
                _sleep = time + 20;
                homing--;
                logPosTime = 0;
            }
            break;
        }
        case 1: {// slow 0
            if(delta == 0){
                _encoder.readAndReset();
                homing = 0;
                logPosTime = 0;
            }
            break;
        }
        }
    }

    uint32_t _sleep;
    simulate(float _positionF);

    uint16_t _position;
    Encoder _encoder;
    uint8_t _direction, _pwm;
};

enum SerialState {
    Buffered = 0,
    WaitingForData = 255
};


MotorPID xPID(0, 0, 0, 0, 0.2f), yPID(0, 0, 0, 0, 0.2f), zPID(0, 0, 0, 0, 0.2f);

// TODO change SERIAL_RX_BUFFER_SIZE
uint8_t bytesToRead = WaitingForData;
byte instructionSize = 0;
byte instruction[16];

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
        Serial.readBytes(instruction, instructionSize);
        bytesToRead = Buffered;
    }
}

void updateIntruction(){
    bool paused = time < pauseUntil;
    if ((working || !paused) && xPID.atTarget() && yPID.atTarget() && zPID.atTarget()) {
        writeHeader('_', 0);
        debug(
            idleStart = time;
            logPosTime = 0;
        );
        working = false;
    }
    if (!working && !paused && bytesToRead == Buffered) {
        debug(
            if (idleStart != time) {
                String s = "Idle (" + String(time - idleStart) + "ms)";
                writeHeader('D', s.length());
                Serial.write(s.c_str(), s.length());
            }
        )
        working = true;
        processInstruction(instruction, instructionSize);
        bytesToRead = WaitingForData;
        debug(logPosTime = 0);
    }
    if(working){
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
                logPosTime = time + 1000;
            }
        )
    }
}


void processInstruction(const byte* const buffer, uint8_t lenght){
    byte n = 0;
    while (n < lenght){
        switch (buffer[n++]) {
        case 'H':{
            simulate(
                writeHeader('I', 41);
                Serial.write("Simulating movement. Motors will not move", 41);
            )
            debug(
                writeHeader('D', 25);
                Serial.write("Motor positions displayed", 25);
            )
            byte data = buffer[n++];
            if(data & 0b001) xPID.resethome();
            if(data & 0b010) yPID.resethome();
            if(data & 0b100) zPID.resethome();
            break;
        }
        case 'X':
            xPID.target = parse_uint16_t(buffer, n);
            break;
        case 'Y':
            yPID.target = parse_uint16_t(buffer, n);
            break;
        case 'Z':
            zPID.target = parse_uint16_t(buffer, n);
            break;
        case 'P': {
            uint32_t pause = parse_uint32_t(buffer, n);
            pauseUntil = pause == 0 ? 0 : (time + parse_uint32_t(buffer, n));
            working = false;
            break;
        }
        default:
            writeHeader('E', 15);
            Serial.write("Unknown param ", 14);
            Serial.write(buffer[n-1]);
            return;
        }
    }
}

inline uint16_t parse_uint16_t(const byte *const buffer, byte &index) { return buffer[index++] + (buffer[index++] << 8); }
inline uint32_t parse_uint32_t(const byte *const buffer, byte &index) { return parse_uint16_t(buffer, index) + (parse_uint16_t(buffer, index) << 16); }