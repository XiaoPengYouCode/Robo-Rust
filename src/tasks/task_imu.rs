use defmt::info;
use defmt_rtt as _;
use embassy_executor;
use embassy_time::{Duration, Ticker, Timer};

use crate::bored::bored_resources::*;
use crate::bsc::usart7::{Usart7, VOFA_PLUS_END_FRAME};
use crate::modules::imu::Imu;

#[embassy_executor::task]
pub async fn task_imu(imu_resources: ImuSpi2Resources, uart_resource: Usart7Resources) {
    let mut imu = Imu::new(imu_resources);
    let mut _debug_serial = Usart7::new(uart_resource);
    if let Ok(()) = imu.init().await {
        Timer::after(Duration::from_secs(5)).await;
        info!("Ok");
        let mut _frame_count = 0u32;
        let mut ticker = Ticker::every(Duration::from_millis(1));
        loop {
            imu.raw_data_update()
                .await
                .expect("Failed to update IMU raw data");
            imu.eskf_predict();
            imu.eskf_update();

            // debug
            if _frame_count % 5 == 0 {
                vofa_show(&imu, &mut _debug_serial);
            }

            _frame_count += 1;
            ticker.next().await;
        }
    }
}

#[allow(unused)]
fn vofa_show(imu: &Imu, _debug_serial: &mut Usart7) {
    _debug_serial.send(&imu.get_euler_angles_d()[0].to_le_bytes());
    _debug_serial.send(&imu.get_euler_angles_d()[1].to_le_bytes());
    _debug_serial.send(&imu.get_euler_angles_d()[2].to_le_bytes());

    _debug_serial.send(&imu.get_accel()[0].to_le_bytes());
    _debug_serial.send(&imu.get_accel()[1].to_le_bytes());
    _debug_serial.send(&imu.get_accel()[2].to_le_bytes());

    _debug_serial.send(&imu.get_gyro()[0].to_le_bytes());
    _debug_serial.send(&imu.get_gyro()[1].to_le_bytes());
    _debug_serial.send(&imu.get_gyro()[2].to_le_bytes());

    _debug_serial.send(&VOFA_PLUS_END_FRAME);
}
