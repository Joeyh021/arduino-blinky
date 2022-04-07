# Arduino-Blinky

A simple arduino blinky example for the Arduino Nano 33 IoT. This code is the demo from [a talk I gave for UWCS in October 2020](https://www.youtube.com/watch?v=-6nDuX_jMBw), and also accompanies [this blog post]().

## Compilation

The code can be compiled to a using `cargo-objcopy`, which compiles and outputs to a binary in one go

```
cargo objcopy --release -- -O binary blink.bin
```

## Uploading

Double click the button on the arduino to put it into bootloader mode, then use `arduino-cli` to upload the binary

```
arduino-cli upload -i blink.bin -b arduino:samd:nano_33_iot -p COM4
```
