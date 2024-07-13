// use defmt::*;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::mode::Async;
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32::peripherals::{DMA2_CH2, DMA2_CH3, PA7, PB3, PB4, SPI1};

pub struct SpiHandles<const N: usize> {
    pub(crate) spi: Spi<'static, Async>,
    pub(crate) cs: [Output<'static>; N],
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
        Self { spi, cs: cs_pins.map(|pin| Output::new(pin, Level::Low, Speed::High)) }
    }
}
