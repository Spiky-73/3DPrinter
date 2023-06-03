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