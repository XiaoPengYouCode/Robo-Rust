use embassy_stm32::bind_interrupts;
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::UART5;
use embassy_stm32::usart::{Config, InterruptHandler, UartRx};

use crate::bored::bored_resources::SBusResources;

bind_interrupts!(struct SBusIrqs {
    UART5 => InterruptHandler<UART5>;
});

pub struct UsartDma {
    sbus: UartRx<'static, Async>,
}

impl UsartDma {
    pub fn new(re: SBusResources) -> Self {
        let mut usart7_config = Config::default();
        usart7_config.baudrate = 192_000;
        Self {
            sbus: UartRx::new(re.usart, SBusIrqs, re.rx, re.rx_dma, usart7_config).unwrap(),
        }
    }

    pub async fn receive(&mut self, buffer: &mut [u8]) {
        self.sbus.read(buffer).await.unwrap();
    }
}
