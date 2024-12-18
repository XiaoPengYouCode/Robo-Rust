use defmt::{error, info};
use defmt_rtt as _;
use embassy_executor;
use embassy_time::{Duration, Ticker};
use na::{ArrayStorage, Const, Matrix};

use crate::bored::bored_resources::*;
use crate::modules::algorithms::ekf_imu::ESKF;
use crate::modules::imu::bmi088::*;

#[embassy_executor::task]
pub async fn task_imu(imu_resources: ImuResources) {
    let mut bmi088 = Bmi088::new(imu_resources);
    if let Ok(()) = bmi088.bmi088_init().await {
        let mut frame = 0;
        let mut ticker = Ticker::every(Duration::from_millis(1));
        let mut eskf = ESKF::default();
        loop {
            // debug!("frame = {}", &frame);
            let prev_time = embassy_time::Instant::now();
            if let Err(e) = bmi088.imu_update().await {
                error!("IMU update error: {:?}", e);
            };

            let accel_matrix = Matrix::<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>::from(
                *bmi088.get_accel(),
            );

            let gyro_matrix = Matrix::<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>::from(
                *bmi088.get_gyro(),
            );

            eskf.predict(accel_matrix, 0.001);
            eskf.update(gyro_matrix);

            let (roll, pitch, yaw) = eskf.get_euler_angles_degrees();
            info!("Roll: {}, Pitch: {}, Yaw: {}", roll, pitch, yaw);

            frame += 1;
            let current_time = embassy_time::Instant::now();
            // 每100帧输出一次性能统计
            if frame % 100 == 0 {
                let duration = current_time.duration_since(prev_time);
                info!("Frame: {}, duration: {} us", frame, duration.as_micros());
            }
            ticker.next().await;
        }
    }
}
