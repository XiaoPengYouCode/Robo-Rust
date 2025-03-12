use assign_resources::assign_resources;
use embassy_stm32::peripherals;

// define the resources for DM02 bored
assign_resources! {
    imu: ImuSpiResources {
        spi_perh: SPI2,
        sck: PB13,
        accel_pin: PC0,
        gyro_pin: PC3,
        mosi_pin: PC1,
        miso_pin: PC2,
        dma_tx: DMA1_CH1,
        dma_rx: DMA1_CH0,
    },
    led: LedSpiResources {
        spi_perh: SPI6,
        sck: PA5,
        mosi_pin: PA7,
    },
    user_key: KeyResources {
        pin: PA15,
    },
    fdcan1: Fdcan1Resources {
        can: FDCAN1,
        tx: PD1,
        rx: PD0,
    },
    fdcan2: Fdcan2Resources {
        can: FDCAN2,
        tx: PB6,
        rx: PB5,
    },
    fdcan3: Fdcan3Resources {
        can: FDCAN3,
        tx: PD13,
        rx: PD12,
    },
    usart1: Usart1Resources {
        usart: USART1,
        tx: PA9,
        rx: PA10,
        dma_rx: DMA1_CH3,
        dma_tx: DMA1_CH4,
    },
    usart7: Usart7Resources {
        uart: UART7,
        tx: PE8,
        rx: PE7,
    },
    usart10: Usart10Resources {
        uart: USART10,
        tx: PE3,
        rx: PE2,
    },
    sbus: SBusResources {
        usart: UART5,
        rx: PD2,
        rx_dma: DMA1_CH7
    },
}
