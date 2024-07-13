#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;

// pub mod bsp;
// pub mod modules;
// pub mod applications;

// use crate::bsp::{pwm::*, led::*};
use roborust::bsp::errors;
use roborust::modules::imu::*;

async fn peripherals_init() -> Result<Bmi088, errors::PeripheralsError> {
    // peripherals struct init
    let p = embassy_stm32::init(Default::default());

    // LED init
    // let blue_led_pin = p.PH10;
    // let green_led_pin = p.PH11;
    // let red_led_pin = p.PH12;
    // led_init(blue_led_pin, green_led_pin, red_led_pin);

    // PWM init
    // let pwm1_pin = p.PE9;
    // let pwm1_timer = p.TIM1;
    // pwm_init(pwm1_pin, pwm1_timer)?;

    // IMU init
    let spi_perh = p.SPI1;
    let sck = p.PB3;
    let accel_pin = p.PA4;
    let gyro_pin = p.PB0;
    let mosi_pin = p.PA7;
    let miso_pin = p.PB4;
    let dma_tx = p.DMA2_CH3;
    let dma_rx = p.DMA2_CH2;

    let bmi088 = Bmi088::new(
        spi_perh,
        sck,
        accel_pin,
        gyro_pin,
        mosi_pin,
        miso_pin,
        dma_tx,
        dma_rx,
    );
    Ok(bmi088)
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");
    match peripherals_init().await {
        Ok(bmi088) => {
            info!("Peripherals initialized successfully!");
            match spawner.spawn(imu_task(bmi088)) {
                Ok(_) => {
                    info!("IMU task spawned successfully!");
                },
                Err(e) => {
                    error!("IMU task spawn failed: {:?}", e);
                }
            };
        },
        Err(e) => {
            error!("Peripherals initialization failed: {:?}", e);
        }
    }
}