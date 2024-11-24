use embassy_executor;
use embassy_time::Timer;

use crate::bored_resources::*;
use crate::modules::imu::*;

#[embassy_executor::task]
pub async fn ahrs_task(imu_resources: ImuResources) {
    let mut bmi088 = Bmi088::new(imu_resources);
    bmi088.bmi088_init().await.unwrap();

    loop {
        bmi088.imu_update().await.expect("msg: imu update failed");
        Timer::after_millis(1).await;
    }
}