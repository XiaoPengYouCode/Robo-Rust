use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::spi::{Config, Spi};
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::peripherals::{DMA2_CH2, DMA2_CH3, PA7, PB3, PB4, SPI1};

pub struct SpiHandles<const N: usize> {
    pub spi: Spi<'static, Async>,
    pub cs: [Output<'static>; N],
}

impl<const N: usize> SpiHandles<N> {
    pub fn new(
        spi_perh: SPI1,
        sck: PB3,
        mosi: PA7,
        miso: PB4,
        dma_tx: DMA2_CH3,
        dma_rx: DMA2_CH2,
        cs_pins: [AnyPin; N],
        config: Config,
    ) -> Self {
        let spi = Spi::<Async>::new(spi_perh, sck, mosi, miso, dma_tx, dma_rx, config);
        let cs_outputs = cs_pins.map(|pin| Output::new(pin, Level::Low, Speed::High));
        Self {
            spi,
            cs: cs_outputs,
        }
    }

    pub fn set_cs(&mut self, index: usize, state: Level) {
        if index < N {
            self.cs[index].set_level(state);
        }
    }

    pub async fn write(&mut self, index: usize, data: &[u8]) {
        self.set_cs(index, Level::Low);
        self.spi.write(data).await.unwrap();
        self.set_cs(index, Level::High);
    }

    pub async fn read(&mut self, index: usize, data: &mut [u8]) {
        self.set_cs(index, Level::Low);
        self.spi.read(data).await.unwrap();
        self.set_cs(index, Level::High);
    }
}
