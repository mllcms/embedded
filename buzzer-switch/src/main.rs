#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy,
    gpio::{GpioPin, Input, Output, PullDown, PushPull, IO},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};

#[main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    esp_println::logger::init_logger_from_env();
    embassy::init(&clocks, timg0);

    let buzzer = io.pins.gpio13.into_push_pull_output();
    let switch = io.pins.gpio12.into_pull_down_input();
    let mut led = io.pins.gpio2.into_push_pull_output();
    let mut ticker = Ticker::every(Duration::from_millis(1000));

    spawner.spawn(listen(buzzer, switch)).ok();
    loop {
        led.toggle();
        log::info!("Hello World!");
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn listen(mut buzzer: GpioPin<Output<PushPull>, 13>, mut switch: GpioPin<Input<PullDown>, 12>) {
    loop {
        switch.wait_for_rising_edge().await;
        buzzer.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}
