use defmt::{error, info};
use embassy_executor;
use embassy_time::Timer;
// use na::{ArrayStorage, Const, Matrix};

use crate::bored::bored_resources::*;
// use crate::modules::algorithms::ekf_imu::ESKF;
use crate::modules::imu::bmi088::*;

#[embassy_executor::task]
pub async fn task_imu(imu_resources: ImuResources) {
    let mut bmi088 = Bmi088::new(imu_resources);
    match bmi088.bmi088_init().await {
        Ok(_) => {
            // let mut eskf = ESKF::default();
            let mut frame: i32 = 1;
            loop {
                // info!("Frame: {}", frame);
                match bmi088.imu_update().await {
                    Ok(_) => {
                        bmi088.format_output_data();
                    }
                    Err(e) => {
                        error!("bmi088 update failed: {}", e);
                    }
                };
                // let accel_data = bmi088.get_accel();
                // let accel_matrix =
                //     Matrix::<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>::from(accel_data);
                // eskf.predict(accel_matrix, 0.001);

                // let gyro_data = bmi088.get_gyro();
                // let gyro_matrix =
                //     Matrix::<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>::from(gyro_data);
                // eskf.update(gyro_matrix);

                // let current_orientation = eskf.get_orientation();
                // let (roll, pitch, yaw) = current_orientation.euler_angles();
                // info!("Orientation - Roll: {}, Pitch: {}, Yaw: {}", roll, pitch, yaw);

                frame += 1;
                Timer::after_millis(1).await;
            }
        }
        Err(e) => {
            error!("bmi088 init failed: {}", e);
        }
    }
}
