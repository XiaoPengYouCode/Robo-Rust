use embassy_executor;
use embassy_time::Timer;

use crate::bored_resources::*;
use crate::modules::imu::*;

#[embassy_executor::task]
pub async fn ahrs_task(imu_resources: ImuResources) {
    let mut bmi088 = Bmi088::new(imu_resources);

    loop {
        bmi088.imu_update().await;
        Timer::after_micros(500).await;
    }
}