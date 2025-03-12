#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _; // global logger
use embassy_executor::Spawner;
use embassy_time::Instant;
use libm::sinf;
use micromath::F32Ext;
use roborust::bored::bored_config::dm02_bored_config;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(dm02_bored_config());
    info!("Peripherals initialized successfully!");

    // 测试代码
    let start = Instant::now();
    for i in 0..1000 {
        let x = (i as f32) / 100.0;
        let _ = x.sin(); // 使用 micro math

        if i % 100 == 0 {
            info!("MICRO MATH: sin({}) = {}", x, x.sin());
        }
    }
    let micro_time = start.elapsed().as_micros();

    let start = Instant::now();
    for i in 0..1000 {
        let x = (i as f32) / 100.0;
        let _ = sinf(x); // 使用 libm

        if i % 100 == 0 {
            info!("libm: sin({}) = {}", x, sinf(x));
        }
    }
    let libm_time = start.elapsed().as_micros();

    info!(
        "micromath time: {} us, libm time: {} us",
        micro_time, libm_time
    );
}
