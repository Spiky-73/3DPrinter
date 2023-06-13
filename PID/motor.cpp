#include "motor.h"
#include "defines.h"

Motor::Motor(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, uint8_t slowSpeed, uint8_t fastSpeed, float kp)
    : _position(0), target(0), _speed(0), _slowSpeed(slowSpeed), _fastSpeed(fastSpeed), maxSpeed(fastSpeed),
      kp(kp),
      _direction(direction), _pwm(pwm),
      _encoder(interrupt, encoder) {
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
    int32_t npos = _encoder.read();
#endif
    deltaPosition = npos - _position;
    _position += deltaPosition;
    updateSpeed();
    if(_homing != 0) updateHome();
}

void Motor::speed(int16_t speed) {
    _speed = speed = constrain(speed, -maxSpeed, maxSpeed);
    analogWrite(_pwm, abs(_speed));
    digitalWrite(_direction, (_speed > 0));
}

void Motor::home() {
    _homing = 1;
    updateHome();
}

void Motor::updateSpeed(){
    int32_t error = target - _position;
    float command = kp * error;
    speed(constrain(command, -255, 255));
}

void Motor::updateHome() {
#if defined(SIMULATE) && defined(DEBUG)
    _encoder.readAndReset();
    homing = 0;
    _position = 0;
    logPosTime = 0;
    return;
#else
    switch (_homing) {
    case 1: { // fast left
        _encoder.write((1 << 15) - 1);
        maxSpeed = _fastSpeed;
        target = 0;
        _timer = time;
        _homing++;
        info += "Fast left";
        debug(logTime = 0);
        break;
    }
    case 2: // delay
        if(time >= _timer+250){
            _homing++;
            _timer = time;
        }
        break;
    case 3: { // target 200
        if(deltaPosition != 0) _timer = time;
        if(time > _timer + 250){
            _encoder.write(0);
            target = 1000;
            maxSpeed = _fastSpeed;
            info += "Right";
            debug(logTime = 0);
            _homing++;
        }
        break;
    }
    case 4: { // slow 0
        if(atTarget()){
            _encoder.write((1 << 15) - 1);
            target = 0;
            maxSpeed = _slowSpeed;
            _timer = time;
            _homing++;
            info += "Slow left";
            debug(logTime = 0);
        }
        break;
    }
    case 5:
        if(time >= _timer+250){
            _homing++;
            _timer = time;
        }
        break;
    case 6: {
        if(deltaPosition != 0) _timer = time;
        if(time > _timer + 250){
            _encoder.write(0);
            target = 0;
            maxSpeed = _slowSpeed;
            _homing = 0;
            info += "Done";
            debug(logTime = 0);
        }
        break;
    }
    }
#endif
}