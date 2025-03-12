use defmt::{error, info};
use defmt_rtt as _;
use embassy_executor;
use embassy_time::{Duration, Ticker};

use crate::bored::bored_resources::*;
use crate::modules::imu::Imu;

#[embassy_executor::task]
pub async fn task_imu(imu_resources: ImuSpiResources) {
    let mut imu = Imu::new(imu_resources);
    if let Ok(()) = imu.init().await {
        let mut _frame = 0;
        let mut ticker = Ticker::every(Duration::from_millis(1));
        loop {
            // debug!("frame = {}", &frame);
            let _prev_time = embassy_time::Instant::now();
            if let Err(e) = imu.data_update().await {
                error!("IMU update error: {:?}", e);
            };

            imu.predict();
            imu.eskf_update();

            let [roll, pitch, yaw] = imu.get_euler_angles_degrees();
            info!("Roll: {}, Pitch: {}, Yaw: {}", roll, pitch, yaw);

            _frame += 1;
            let _current_time = embassy_time::Instant::now();
            // 每100帧输出一次性能统计
            // if frame % 100 == 0 {
            //     let duration = current_time.duration_since(prev_time);
            //     info!("Frame: {}, duration: {} us", frame, duration.as_micros());
            // }
            ticker.next().await;
        }
    }
}
