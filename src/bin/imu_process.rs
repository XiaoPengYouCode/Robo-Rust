#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;

use roborust::bored::bored_config::dm02_bored_config;
use roborust::bored::bored_resources::*;
use roborust::split_resources;

use roborust::tasks::task_imu::task_imu;
// use roborust::tasks::task_user_key::task_user_key;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(dm02_bored_config());
    info!("Peripherals initialized successfully!");

    let resource = split_resources!(p);
    info!("Split peripherals successfully!");

    spawner.spawn(task_imu(resource.imu)).unwrap();
    // spawner.spawn(task_user_key(resource.user_key)).unwrap();
}
