// See https://avr-guide.github.io/timers-on-the-atmega328/ for useful info on
// configuring timers on AVR devices.

#![no_std]
#![no_main]
// Enable experimental AVR interrupt ABI (see https://github.com/rust-lang/rust/issues/69664)
#![feature(abi_avr_interrupt)]

use core::cell::OnceCell;

use avr_device::{
    asm::sleep,
    atmega328p::{PORTB, Peripherals},
    interrupt::{self, Mutex},
};

const MS_PER_S: u32 = 1000;
const CLOCK_FREQ_HZ: u32 = 16_000_000;
const PRESCALAR: u32 = 1024;
const DELAY_MS: u32 = 500;
const TC1_COUNT_VAL: u16 = (CLOCK_FREQ_HZ / PRESCALAR * DELAY_MS / MS_PER_S) as u16;

static LED_GPIO_PORT: Mutex<OnceCell<Option<PORTB>>> = Mutex::new(OnceCell::new());

/// Here's a panic handler that does nothing. Panic handlers are required in no_std
/// environments.
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        sleep();
    }
}

// Set up an interrupt for the TIMER1_COMPA (timer 1 compare A) event.
#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    interrupt::free(|cs| {
        // For more info, see https://docs.rust-embedded.org/book/concurrency/index.html#sharing-peripherals

        // To grab peripherals, we need to perform the access in a critical section.
        // unwrap() is safe here since we know the port was inserted into the mutex
        // before the timer was configured.
        let led_port = LED_GPIO_PORT.borrow(cs).get().unwrap();
        led_port
            .as_ref()
            .unwrap()
            .pinb()
            .write(|w| w.pb5().set_bit()); // Writing a 1 to PINB5 toggles the state of the pin.
    });
}

#[avr_device::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    // LED_BUILTIN for the Arduino UNO is on PB5. Configure it as a
    // digital output.
    peripherals.PORTB.ddrb().write(|w| w.pb5().set_bit());

    // Expose the GPIO port by inserting it into the mutex. After running this line,
    // the Rust borrow checker prevents us from PORTB outside of the mutex. Cool!
    interrupt::free(|cs| {
        LED_GPIO_PORT
            .borrow(cs)
            .set(Some(peripherals.PORTB))
            .unwrap();
    });

    // Use a / 1024 prescalar for the clock frequency. Also enable Clear Timer on Compare
    // Match (CTC) mode by setting wgm1 to 01. CTC mode clears the timer after we reach
    // our target count.
    peripherals
        .TC1
        .tccr1b()
        .write(|w| w.cs1().prescale_1024().wgm1().set(1));
    // Set the Output Compare A register target count.
    peripherals.TC1.ocr1a().write(|w| w.set(TC1_COUNT_VAL));
    // Enable interrupt generation when the counter reaches the value in OCR1A.
    // The interrupt flag gets automatically disabled.
    peripherals.TC1.timsk1().write(|w| w.ocie1a().set_bit());

    // The AVR SREG Global Interrupt Enable is disabled by default, so we have to
    // enable it first.
    unsafe {
        interrupt::enable();
    }

    // Continually wait for new interrupts for power-savings.
    loop {
        sleep();
    }
}
