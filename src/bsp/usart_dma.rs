use embassy_stm32::dma::{Channel, Dma};
use embassy_stm32::usart::{self, Config, Uart};
use embassy_time::Duration;

pub struct UsartDma {
    uart: Uart,
    dma_tx: Channel,
    dma_rx: Channel,
}

impl UsartDma {
    pub fn new(
        uart: Uart,
        dma_tx: Channel,
        dma_rx: Channel,
        baud_rate: u32,
    ) -> Self {
        let config = Config {
            baudrate: baud_rate,
            ..Default::default()
        };

        // 初始化UART
        uart.init(config, dma_tx, dma_rx);

        Self { uart, dma_tx, dma_rx }
    }

    pub async fn send(&mut self, data: &[u8]) {
        self.dma_tx.start_transfer(data).await.unwrap();
        self.uart.write(data).await.unwrap();
    }

    pub async fn receive(&mut self, buffer: &mut [u8]) -> usize {
        self.dma_rx.start_transfer(buffer).await.unwrap();
        self.uart.read(buffer).await.unwrap()
    }
}
