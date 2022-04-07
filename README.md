# Arduino-Blinky

A simple LED blink example for the Arduino Nano 33 IoT. This code is the demo from [a talk I gave for UWCS in October 2020](https://www.youtube.com/watch?v=-6nDuX_jMBw), and also accompanies [this blog post](https://joeyh.dev/blog/rust_arduino/).

## Compilation

The code can be compiled using `cargo-objcopy`, which compiles and outputs to a binary in one go

```
cargo objcopy --release -- -O binary blink.bin
```

## Uploading

Double click the button on the arduino to put it into bootloader mode, then use `arduino-cli` to upload the binary

```
arduino-cli upload -i blink.bin -b arduino:samd:nano_33_iot -p COM4
```

This command assumes you're on windows and the Arduino is attatched to the `COM4` port. See the `arduino-cli` docs for full instructions.
