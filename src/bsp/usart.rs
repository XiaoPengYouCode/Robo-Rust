use embassy_stm32::gpio::{Pin, Output, Pull};
use embassy_stm32::usart::{self, Config, Uart};
use embassy_time::Duration;

pub struct Usart {
    uart: Uart,
}

impl Usart {
    pub fn new(
        uart: Uart,
        tx_pin: impl Pin<Output, Pull>,
        rx_pin: impl Pin<Output, Pull>,
        baud_rate: u32,
    ) -> Self {
        let config = Config {
            baudrate: baud_rate,
            ..Default::default()
        };

        // Initialize the UART with the specified configuration
        uart.init(config, tx_pin, rx_pin);

        Self { uart }
    }

    pub async fn send(&mut self, data: &[u8]) {
        self.uart.write(data).await.unwrap();
    }

    pub async fn receive(&mut self, buffer: &mut [u8]) -> usize {
        self.uart.read(buffer).await.unwrap()
    }
}
