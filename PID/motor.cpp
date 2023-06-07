#include "motor.h"
#include "defines.h"

Motor::Motor(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, float kp)
    : _position(0), target(0), _speed(0),
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
    if(homing == 0) PID();
    else home();
}

void Motor::speed(float speed) {
    _speed = speed = constrain(speed, -0.8f, 0.8f);

    analogWrite(_pwm, 255 * abs(_speed));
    digitalWrite(_direction, (_speed > 0));
}

void Motor::resethome() { // TODO redo
    homing = 4;
    target = 0;
    // _encoder.write
}

void Motor::PID(){
    int16_t error = target - _position;
    float command = kp * error;
    speed(command);
}

void Motor::home() {
    _encoder.readAndReset();
    homing = 0;
    _position = 0;
    logPosTime = 0;
    return;
#if defined(SIMULATE) && defined(DEBUG)
#else
    if (time < _sleep)
        return;
    switch (homing)
    {
    case 4:
    { // fast left
        speed(-1);
        _sleep = time + 20;
        homing--;
        logPosTime = 0;
        break;
    }
    case 3:
    { // target 200
        if (delta == 0)
        {
            speed(0.3f);
            homing--;
            logPosTime = 0;
        }
        break;
    }
    case 2:
    { // slow 0
        if (_position > 200)
        {
            speed(-1);
            _sleep = time + 20;
            homing--;
            logPosTime = 0;
        }
        break;
    }
    case 1:
    { // done
        if (delta == 0)
        {
            _encoder.readAndReset();
            homing = 0;
            logPosTime = 0;
        }
        break;
    }
    }
#endif
}