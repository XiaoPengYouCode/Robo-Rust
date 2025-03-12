#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

use roborust::bored::bored_config::dm02_bored_config;
use roborust::bored::bored_resources::*;
use roborust::split_resources;

use roborust::tasks::task_led::task_led;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(dm02_bored_config());
    info!("Peripherals initialized successfully!");

    let resource = split_resources!(p);
    info!("Split peripherals successfully!");

    spawner.spawn(task_led(resource.led)).unwrap();
}
