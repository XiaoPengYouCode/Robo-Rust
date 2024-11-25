use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::spi::{Config, Spi};
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::peripherals::{DMA1_CH0, DMA1_CH1, PB13, PC1, PC2, SPI2};

pub struct SpiHandles<const N: usize> {
    pub spi: Spi<'static, Async>,
    pub cs: [Output<'static>; N],
}

impl<const N: usize> SpiHandles<N> {
    pub fn new(
        spi_perh: SPI2,
        sck: PB13,
        mosi: PC1,
        miso: PC2,
        dma_tx: DMA1_CH1,
        dma_rx: DMA1_CH0,
        cs_pins: [AnyPin; N],
        config: Config,
    ) -> Self {
        let spi = Spi::<Async>::new(spi_perh, sck, mosi, miso, dma_tx, dma_rx, config);
        let cs_outputs = cs_pins.map(|pin| Output::new(pin, Level::High, Speed::High));
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
}
