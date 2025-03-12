use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;

use crate::bored::bored_resources::LedSpiResources;

use embassy_stm32::mode::Blocking;

pub struct Ws2812 {
    spi: Spi<'static, Blocking>,
}

impl Ws2812 {
    pub fn new(re: LedSpiResources) -> Self {
        let mut config = Config::default();
        config.frequency = Hertz(6_400_000); // 0.15625 us
        Self {
            spi: Spi::<Blocking>::new_blocking_txonly(re.spi_perh, re.sck, re.mosi_pin, config),
        }
    }

    pub fn led_show_brg(&mut self, color: &[u8; 3]) {
        for &byte in color.iter() {
            for bit in (0..8).rev() {
                let spi_byte = if (byte & (1 << bit)) != 0 {
                    0b1111100u8 // 11111000 表示1: 0.78125 + 0.46875 us
                } else {
                    0b1110000u8 // 11000000 表示0: 0.46875 + 0.78125 us
                };
                self.spi.blocking_write(&[spi_byte]).unwrap();
            }
        }

        // 只控制单个Ws2812，所以每次发送完24字节之后直接发送复位信号
        // 发送复位信号（至少50us的低电平）0.15625 * 40 * 8 = 50 us
        self.spi.blocking_write(&[0x00u8; 40]).unwrap();
    }
}
