use defmt::{error, info};
use embassy_executor;
use embassy_time::Timer;

use crate::bored_resources::*;
use crate::modules::imu::bmi088::*;

#[embassy_executor::task]
pub async fn ahrs_task(imu_resources: ImuResources) {
    let mut bmi088 = Bmi088::new(imu_resources);
    match bmi088.bmi088_init().await {
        Ok(_) => {
            let mut frame: i32 = 1;
            loop {
                info!("Frame: {}", frame);
                match bmi088.imu_update().await {
                    Ok(_) => {
                        bmi088.format_data();
                    }
                    Err(e) => {
                        error!("bmi088 update failed: {}", e);
                    }
                };

                frame += 1;
                Timer::after_millis(1).await;
            }
        }
        Err(e) => {
            error!("bmi088 init failed: {}", e);
        }
    }
}
