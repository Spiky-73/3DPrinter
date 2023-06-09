#pragma once
#include "Encoder.h"
#include "defines.h"
class Motor {
public:
    Motor(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, uint8_t slowSpeed, uint8_t fastSpeed, float kp);

    void update(long delta);

    void speed(int16_t speed);
    inline int16_t speed() { return _speed; }

    void home();

    inline bool atTarget() const { return abs(target - position()) <= 1; }

    void hardsetPosition(uint16_t position) {
        _position += position - target;
        target = position;
    }
    inline const uint16_t position() const { return _position; }
    int32_t deltaPosition;

    uint8_t maxSpeed;
    int32_t target;

    float kp;
    byte _homing;

private:
    void updateSpeed();

    void updateHome();
    

    simulate(float _positionF);
    int32_t _position;
    int16_t _speed, _slowSpeed, _fastSpeed;
    unsigned long _wait;
    unsigned long _timer;

    Encoder _encoder;
    uint8_t _direction, _pwm;
};
