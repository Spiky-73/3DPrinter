#pragma once
#include "Encoder.h"
#include "defines.h"
class Motor {
public:
    Motor(uint8_t direction, uint8_t pwm, uint8_t interrupt, uint8_t encoder, float kp);

    void update(long delta);

    void speed(int16_t speed);
    inline int16_t speed() { return _speed; }

    void home();

    inline bool atTarget() const { return _homing == 0 && abs((int32_t)target - position()) <= 1; }

    void hardsetPosition(uint16_t position) {
        _position += position - target;
        target = position;
    }
    inline const uint16_t position() const { return _position; }

    uint8_t maxSpeed;
    int64_t delta;
    uint32_t target;

    float kp;

private:
    void updateSpeed();

    void nextHome();
    
    byte _homing;

    simulate(float _positionF);
    uint32_t _position;
    int16_t _speed;

    Encoder _encoder;
    uint8_t _direction, _pwm;
};
