# 3D Printer

## Cross-compilation
Install the [Raspberry Pi toolchain](https://gnutoolchains.com/raspberry/).

Run
```bash
rustup target add armv7-unknown-linux-gnueabihf
```

Add to your .cargo/config.toml
```toml
[build]
target = "armv7-unknown-linux-gnueabihf"
```

## Carte Main - Raspberry Pi

## Carte Moteurs - Arduino Uno / Mega

## Communication Main <-> Moteur
### Packet Main -> Moteurs
1. ControlBuse
    - spdX, posX
    - spdY, posY
    - spdExtrusion
2. NextLayer
 - posZ
### Moteur -> Main
