#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger

use roborust::bored::bored_config::dm02_bored_config;
use roborust::bored::bored_resources::*;
use roborust::split_resources;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Roborust!");
    let p = embassy_stm32::init(dm02_bored_config());
    let r = split_resources!(p);

    let key_pin = Input::new(r.user_key.pin, Pull::Up);
    loop {
        if key_pin.is_high() {
            info!("LED is high");
        } else {
            info!("LED is low");
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}

#[embassy_executor::task]
async fn led_task(led_resources: LedResources) {
    // TODO: Use spi to fix led task
    let mut led = Output::new(led_resources.pin, Level::Low, Speed::Low);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
