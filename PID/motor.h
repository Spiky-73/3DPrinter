#pragma once
#include "Encoder.h"
#include "defines.h"
class Motor {
public:
    Motor(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, float kp);

    void update(long delta);

    void speed(float speed);
    inline float speed() { return _speed; }

    void resethome();

    inline bool atTarget() const { return homing == 0 && abs((int32_t)target - position()) <= 1; }

    void hardsetPosition(uint16_t position) {
        _position += position - target;
        target = position;
    }
    inline const uint16_t position() const { return _position; }

    int16_t delta;
    uint16_t target;

    float kp;
    byte homing;

private:
    void PID();

    void home();

    simulate(float _positionF);
    uint16_t _position;
    float _speed;

    uint32_t _sleep;

    Encoder _encoder;
    uint8_t _direction, _pwm;

};
