#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;

use roborust::bored_resources::*;
use roborust::split_resources;

use roborust::bored_config::dm02_b_config;
use roborust::tasks::imu_ahrs_task::ahrs_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(dm02_b_config());
    info!("Peripherals initialized successfully!");

    let res = split_resources!(p);
    info!("Split peripherals successfully!");

    spawner.spawn(ahrs_task(res.imu)).unwrap();
    // spawner.spawn(user_key_task(res.user_key)).unwrap();
}
