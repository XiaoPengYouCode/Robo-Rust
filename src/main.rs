#![no_std]
#![no_main]

#![allow(dead_code)]

// close warning on debug mode
// open warning on release mode
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;

mod bsp;
// mod modules;
// mod applications;

use crate::bsp::{pwm::*, led::*};
use crate::bsp::errors;

fn peripherals_init() -> Result<(), errors::PeripheralsError> {
    // peripherals struct init
    let p = embassy_stm32::init(Default::default());

    // LED init
    let blue_led_pin = p.PH10;
    let green_led_pin = p.PH11;
    let red_led_pin = p.PH12;
    led_init(blue_led_pin, green_led_pin, red_led_pin);

    // PWM init
    let pwm1_pin = p.PE9;
    let pwm1_timer = p.TIM1;
    pwm_init(pwm1_pin, pwm1_timer)?;

    Ok(())
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    match peripherals_init() {
        Ok(_) => {
            loop {
                Timer::after_millis(1000).await;
                info!("Hello World!");
            }
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

}