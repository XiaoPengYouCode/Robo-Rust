use embassy_stm32::peripherals::TIM4;
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::Channel;

async fn buzzer_on(mut pwm: SimplePwm<'static, TIM4>, freq: Hertz, duty: u32) {
    pwm.set_frequency(freq);
    pwm.set_duty(Channel::Ch3, duty);
}

async fn buzzer_off(mut pwm: SimplePwm<'static, TIM4>) {
    pwm.set_duty(Channel::Ch3, 0);
}
