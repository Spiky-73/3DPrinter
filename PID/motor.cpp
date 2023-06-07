#include "motor.h"
#include "defines.h"

Motor::Motor(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, float kp)
    : _position(0), target(0), _speed(0), maxSpeed(180),
      kp(kp),
      _direction(direction), _pwm(pwm),
      _encoder(interrupt, encoder)
{
    pinMode(direction, OUTPUT);
    pinMode(pwm, OUTPUT);
    digitalWrite(direction, LOW);
}

void Motor::update(long deltaTime){
#ifdef SIMULATE
#ifdef DEBUG
    uint16_t npos = target;
#else
    _positionF += speed * delta;
    _positionF = max(0, _positionF);
    uint16_t npos = _positionF;
#endif
#else
    uint16_t npos = _encoder.read();
#endif
    delta = npos - _position;
    _position = npos;
    updateSpeed();
    if(_homing != 0 && (delta == 0 || atTarget())) nextHome();
}

void Motor::speed(int16_t speed) {
    _speed = speed = constrain(speed, -maxSpeed, maxSpeed);

    analogWrite(_pwm, abs(_speed));
    digitalWrite(_direction, (_speed > 0));
}

void Motor::home() {
    _homing = 4;
    nextHome();
}

void Motor::updateSpeed(){
    int16_t error = target - _position;
    float command = kp * error;
    speed(command);
}

void Motor::nextHome() {
#if defined(SIMULATE) && defined(DEBUG)
    _encoder.readAndReset();
    homing = 0;
    _position = 0;
    logPosTime = 0;
    return;
#else
    switch (_homing) {
    case 4: { // fast left
        _encoder.write((1 << 16) - 1);
        maxSpeed = 180;
        target = 0;
        break;
    }
    case 3: { // target 200
        _encoder.write(0);
        target = 200;
        break;
    }
    case 2: { // slow 0
        _encoder.write((1 << 16) - 1);
        target = 0;
        maxSpeed = 120;
        break;
    }
    case 1:  { // done
        _encoder.readAndReset();
        maxSpeed = 180;
        break;
    }
    }
    debug(logPosTime = 0);
    _homing--;
#endif
}