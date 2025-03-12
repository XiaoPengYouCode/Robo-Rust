use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::USART1;
use embassy_stm32::usart::InterruptHandler;
use embassy_stm32::usart::{Config, Uart};

use embassy_stm32::bind_interrupts;

use crate::bored::bored_resources::Usart1Resources;

pub struct UsartDma {
    uart: Uart<'static, Async>,
}

bind_interrupts!(struct Usart1Irqs {
    USART1 => InterruptHandler<USART1>;
});

impl UsartDma {
    pub fn new(re: Usart1Resources) -> Self {
        let mut usart1_config = Config::default();
        usart1_config.baudrate = 9600;
        Self {
            uart: Uart::new(
                re.usart,
                re.rx,
                re.tx,
                Usart1Irqs,
                re.dma_tx,
                re.dma_rx,
                usart1_config,
            )
            .unwrap(),
        }
    }

    pub async fn send(&mut self, data: &[u8]) {
        self.uart.write(data).await.unwrap();
    }

    pub async fn receive(&mut self, buffer: &mut [u8]) {
        self.uart.read(buffer).await.unwrap()
    }
}
