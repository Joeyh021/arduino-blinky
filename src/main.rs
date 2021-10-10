//tell rust not to link to link to the std crate, and to not use the normal main interface
#![no_std]
#![no_main]

//import the board support crate
use arduino_nano33iot as bsp;
//the bsp re-exports the hal,
use bsp::hal;

use bsp::hal::gpio::v2::{Pin, PushPullOutput, PB10};
//use panic_halt so it gets linked
use panic_halt as _;

//the entry macro
//have to define our own entry function
use bsp::entry;
//hal prelude with convenient traits
use hal::prelude::*;

//some peripherals we need
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    //take peripherals from hal
    //*we need ownership*
    let mut peripherals = Peripherals::take().unwrap();
    //take peripherals from uarch crate (cortex m)
    let core = CorePeripherals::take().unwrap();

    //initalise the clocks using the internal peripherals
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    //initalise a new delay device with clocks
    let mut delay = Delay::new(core.SYST, &mut clocks);

    //setup pins
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: Pin<PB10, PushPullOutput> = pins.d2.into();

    //loop, blinking indefinitely
    loop {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}
