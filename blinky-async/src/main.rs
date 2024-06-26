#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, embassy, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup};

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);

    embassy::init(&clocks, timg0);
    esp_println::logger::init_logger_from_env();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();
    spawner.spawn(hello_world()).ok();
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(500)).await
    }
}

#[embassy_executor::task]
async fn hello_world() {
    loop {
        log::info!("Hello World!");
        Timer::after(Duration::from_millis(500)).await
    }
}
