use defmt::*;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::peripherals::{PE9, TIM1};

use crate::bsp::errors::PeripheralsError;

pub fn pwm_init(
    pin: PE9,
    timer: TIM1,
) -> Result<u32, PeripheralsError> {
    info!("pwm_init");
    let ch1 = PwmPin::new_ch1(pin, OutputType::PushPull);
    let mut pwm = SimplePwm::new(timer, Some(ch1), None, None, None, khz(10), Default::default());
    let max = pwm.get_max_duty();
    pwm.enable(Channel::Ch1);
    let duty = max / 2;
    pwm.set_duty(Channel::Ch1, duty);
    info!("pwm_duty = {}", duty);
    Ok(duty)
}