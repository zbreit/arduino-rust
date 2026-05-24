// Opt out of standard library since we don't have an OS.
#![no_std]
// We can't use the standard main() interface since it requires
// Rust nightly (see https://internals.rust-lang.org/t/reintroduce-main-functions-to-no-std-targets/23015)
// and https://docs.rust-embedded.org/book/start/qemu.html for more info.
#![no_main]
// Don't allow the use of unsafe code.
#![deny(unsafe_code)]

/// Here's a panic handler that does nothing. Panic handlers are required in no_std
/// environments.
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        avr_device::asm::sleep();
    }
}

/// Busy sleeps for the requested number of milliseconds
pub fn sleep_ms(ms: u32) {
    const CLOCK_FREQ_HZ: u32 = 16_000_000;
    const MS_PER_S: u32 = 1000;
    let cycles = CLOCK_FREQ_HZ / MS_PER_S * ms;

    avr_device::asm::delay_cycles(cycles);
}

#[avr_device::entry]
fn main() -> ! {
    let peripherals = avr_device::atmega328p::Peripherals::take().unwrap();

    // LED_BUILTIN for the Arduino UNO is on PB5.
    let led_port = peripherals.PORTB;

    // Configure LED port as a digital output pin.
    led_port.ddrb().write(|w| w.pb5().set_bit());

    loop {
        // Writing a 1 to PINB5 toggles the state of the pin.
        led_port.pinb().write(|w| w.pb5().set_bit());
        sleep_ms(500);
    }
}
