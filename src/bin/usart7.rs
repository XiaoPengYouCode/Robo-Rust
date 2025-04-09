#![no_std]
#![no_main]

use defmt::debug;
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

use roborust::bored::bored_config::dm02_bored_config;
use roborust::bored::bored_resources::*;
use roborust::split_resources;

use roborust::tasks::task_led::task_led;
use roborust::tasks::task_usart7::task_usart7;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(dm02_bored_config());
    debug!("Peripherals initialized successfully!");

    let resource = split_resources!(p);
    debug!("Split peripherals successfully!");

    spawner.spawn(task_led(resource.led)).unwrap();
    spawner.spawn(task_usart7(resource.usart7)).unwrap();
}
