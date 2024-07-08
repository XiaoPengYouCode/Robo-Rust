use defmt::*;
use embassy_stm32::gpio::{Level, Output, Speed};

use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::peripherals::{PH10, PH11, PH12};

pub fn led_init(blue_pin: PH10, green_pin: PH11, red_pin: PH12) {
    info!("Led Init!");

    let mut led_blue = Output::new(blue_pin, Level::High, Speed::Low);
    let mut led_green = Output::new(green_pin, Level::High, Speed::Low);
    let mut led_red = Output::new(red_pin, Level::High, Speed::Low);

    led_blue.set_high();
    led_green.set_high();
    led_red.set_high();
}
