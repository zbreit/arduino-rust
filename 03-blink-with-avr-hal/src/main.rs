#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // The Pins struct abstracts pin configuration so you don't need manual register
    // writes :).
    let mut led = pins.d13.into_output();

    let mut tick = 0;

    loop {
        led.toggle();
        ufmt::uwriteln!(&mut serial, "Tick {}", tick).unwrap_infallible();
        arduino_hal::delay_ms(500);
        tick += 1;
    }
}
