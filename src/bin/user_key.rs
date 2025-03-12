#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
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
            // info!("LED is high");
        } else {
            // info!("LED is low");
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}
