#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;

use roborust::bored_resources::*;
use roborust::split_resources;

use roborust::tasks::imu_ahrs_task::ahrs_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Peripherals initialized successfully!");

    let res = split_resources!(p);
    info!("split peripherals successfully!");

    spawner.spawn(ahrs_task(res.imu)).unwrap();
}
