use assign_resources::assign_resources;
use embassy_stm32::peripherals;

// define the resources for DM02 bored
assign_resources! {
    imu: ImuResources {
        spi_channel: SPI1,
        sck: PB3,
        accel_pin: PA4,
        gyro_pin: PB0,
        mosi_pin: PA7,
        miso_pin: PB4,
        dma_tx: DMA2_CH3,
        dma_rx: DMA2_CH2,
    },
    fdcan1: Fdcan1Resources {
        can: FDCAN1,
        tx: PA12,
        rx: PA11,
    },
    fdcan2: Fdcan2Resources {
        can: FDCAN2,
        tx: PB13,
        rx: PB12,
    },
    fdcan3: Fdcan3Resources {
        can: FDCAN3,
        tx: PD1,
        rx: PD0,
    },
}
