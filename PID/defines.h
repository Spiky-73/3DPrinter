#pragma once

#define writeHeader(type, length) Serial.write(1 + length); Serial.write(type)

#ifdef DEBUG
#define debug(code) code
#else
#define debug(code)
#endif

#ifdef SIMULATE
#define simulate(code) code
#else
#define simulate(code)
#endif

#if !defined(SIMULATE) && defined(DEBUG)
#define debugOnly(code) code
#else
#define debugOnly(code)
#endif


inline long time;
inline long deltaTime;

debug(
    inline String info = "";
    inline long logTime;
)