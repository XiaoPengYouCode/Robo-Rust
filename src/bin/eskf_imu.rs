#![no_std]
#![no_main]

use defmt::debug;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;
use panic_probe as _;

use roborust::bored::bored_config::dm02_bored_config;
use roborust::bored::bored_resources::*;
use roborust::split_resources;

use roborust::tasks::task_imu::task_imu;
use roborust::tasks::task_led::task_led;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peri = embassy_stm32::init(dm02_bored_config());
    debug!("Peripherals initialized successfully!");

    let re = split_resources!(peri);
    debug!("Split peripherals successfully!");

    spawner.spawn(task_led(re.led)).unwrap();
    spawner.spawn(task_imu(re.imu, re.usart7)).unwrap();
}
