#include "Encoder.h"
#include "defines.h"
#include "motor.h"

bool idle = true, pids = false;
unsigned long pauseUntil = 0;
debug(long idleStart = 0);

// X: RED+, ORANGE inter / Y: RED+, invert +- GREEN inter / Z: RED+, RED STRIPES inter
Motor xPID(12, 3, 2, 5, 255, 255, -200), yPID(13, 11, 18, 20, 170, 200, -200), zPID(12, 3, 2, 5, 200, 200, 5);

// TODO change SERIAL_RX_BUFFER_SIZE
uint8_t bytesToRead = 0;
uint8_t instructionSize = 0;
byte instruction[16];

void setup() {
    Serial.begin(115200);
}

void loop() {
    updateTime();
    updatePIDs();
    updateReception();
    updateIntruction();
    debug(
        if (info.length() != 0) {
            writeHeader('D', info.length());
            Serial.write(info.c_str(), info.length());
            info = "";
            logTime = time + 200;
        }
    )
}

void updateTime(){
    long current = millis();
    deltaTime = current - time;
    time = current;
}

void updatePIDs(){
    if (!pids) return;
    xPID.update(deltaTime);
    yPID.update(deltaTime);
    // zPID.update(deltaTime);
    debug(
        if (time >= logTime) {
            info += " X" + String(xPID.position()) + '>' + String(xPID.target);
            info += " Y" + String(yPID.position()) + '>' + String(yPID.target);
            // info += " Z" + String(zPID.position()) + '>' + String(zPID.target) + " H" + String(zPID._homing);
        })
}

void updateReception(){
    int available = Serial.available();
    if (available == 0 || available < bytesToRead) return;
    if (bytesToRead == 0) {
        bytesToRead = Serial.read();
        updateReception(); // Tries to read data immediatly
    } else if (instructionSize == 0) {
        instructionSize = bytesToRead;
        Serial.readBytes(instruction, instructionSize);
        bytesToRead = 0;
    }
}

void updateIntruction(){
    if (time < pauseUntil) return;
    if (!xPID.atTarget() || !yPID.atTarget() || !zPID.atTarget()) return;

    if(!idle){
        idle = true;
        writeHeader('_', 0);
        debugOnly(idleStart = time);
    }

    if(instructionSize != 0){
        processInstruction(instruction, instructionSize);
        instructionSize = 0;
        idle = false;
        debugOnly(
            if (idleStart != time) info += "Idle (" + String(time - idleStart) + "ms) ";
        )
    }
    
}

void processInstruction(const byte* const buffer, uint8_t lenght){
    byte n = 0;
    while (n < lenght){
        switch (buffer[n++]) {
        case 'H':{
            pids = true;
            simulate(
                writeHeader('I', 41);
                Serial.write("Simulating movement. Motors will not move", 41);
            )
            debug(
                writeHeader('D', 37);
                Serial.write("Motor positions displayed periodicaly", 37);
            )
            byte data = buffer[n++];
            if(data & 0b001) xPID.home();
            if(data & 0b010) yPID.home();
            // if(data & 0b100) zPID.home();
            break;
        }
        case 'h':
            pids = false;
            xPID.speed(0);
            yPID.speed(0);
            // zPID.speed(0);
            break;
        case 'X':
            xPID.target = parse_uint16_t(buffer, n);
            break;
        case 'Y':
            yPID.target = parse_uint16_t(buffer, n);
            break;
        case 'Z':
            /* zPID.target = */  parse_uint16_t(buffer, n);
            break;
        case 'P': {
            // uint32_t pause = parse_uint32_t(buffer, n);
            pauseUntil = time + parse_uint32_t(buffer, n);;
            info += "T" + String(time) + ">" + String(pauseUntil);
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