# Arduino-Blinky

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

## Notes

The package manifest currently references a local version of the `arduino_nano33iot` BSP, which I've patched to fix issues with it. When the crate is updated on crates.io to fix the same issues, I'll update this repo to reflect that.
