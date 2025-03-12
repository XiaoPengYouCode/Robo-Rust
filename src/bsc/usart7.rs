use embassy_stm32::mode::Blocking;
use embassy_stm32::usart::{Config, Uart};

use crate::bored::bored_resources::Usart7Resources;

pub struct UsartDma {
    uart: Uart<'static, Blocking>,
}

impl UsartDma {
    pub fn new(re: Usart7Resources) -> Self {
        let mut usart7_config = Config::default();
        usart7_config.baudrate = 9600;
        Self {
            uart: Uart::new_blocking(re.uart, re.rx, re.tx, usart7_config).unwrap(),
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        self.uart.blocking_write(data).unwrap();
    }

    pub fn receive(&mut self, buffer: &mut [u8]) {
        self.uart.blocking_write(buffer).unwrap()
    }
}
