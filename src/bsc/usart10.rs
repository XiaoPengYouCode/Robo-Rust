use embassy_stm32::mode::Blocking;
use embassy_stm32::usart::{Config, Uart};

use crate::bored::bored_resources::Usart10Resources;

pub struct UsartDma {
    uart: Uart<'static, Blocking>,
}

impl UsartDma {
    pub fn new(re: Usart10Resources) -> Self {
        let mut uart10_config = Config::default();
        uart10_config.baudrate = 9600;
        Self {
            uart: Uart::new_blocking(re.uart, re.rx, re.tx, uart10_config).unwrap(),
        }
    }

    pub async fn send(&mut self, data: &[u8]) {
        self.uart.blocking_write(data).unwrap();
    }

    pub async fn receive(&mut self, buffer: &mut [u8]) {
        self.uart.blocking_read(buffer).unwrap()
    }
}
