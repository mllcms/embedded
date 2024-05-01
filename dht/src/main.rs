#![no_std]
#![no_main]

use dht_embedded::{DhtSensor, NoopInterruptControl};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let dht11 = io.pins.gpio15.into_open_drain_output();
    let mut dht11 = dht_embedded::Dht11::new(NoopInterruptControl, delay.clone(), dht11);
    loop {
        match dht11.read() {
            Ok(reading) => log::info!("{}°C, {}% RH", reading.temperature(), reading.humidity()),
            Err(e) => log::error!("Error: {}", e),
        }
        delay.delay(500.millis());
    }
}
