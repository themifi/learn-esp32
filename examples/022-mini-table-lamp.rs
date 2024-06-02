#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();
    let button = io.pins.gpio13.into_pull_up_input();

    esp_println::logger::init_logger_from_env();

    loop {
        if button.is_low() {
            delay.delay(20.millis());
            if button.is_low() {
                led.toggle();
            }

            // Wait for button release
            while button.is_low() {}
            delay.delay(20.millis());
            while button.is_low() {}
        }
    }
}
