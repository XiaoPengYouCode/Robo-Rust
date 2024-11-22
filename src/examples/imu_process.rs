#![no_std]
#![no_main]

use defmt::debug;
use embassy_executor::Spawner;
use embassy_time::Timer;

use roborust::bored_resources::*;
use roborust::modules::imu::*;
use roborust::split_resources;

#[embassy_executor::task]
async fn ahrs(imu_resoureces: ImuResources) {
    let mut bmi088 = Bmi088::new(imu_resoureces);

    loop {
        bmi088.imu_update().await;
        Timer::after_micros(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let res = split_resources!(p);
    spawner.spawn(ahrs(res.imu)).unwrap();

    debug!("Peripherals initialized successfully!");
    // spawner.spawn(ahrs_task(bmi088)).unwrap();
}
