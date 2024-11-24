use assign_resources::assign_resources;
use embassy_stm32::peripherals;

// define the resources for DM02 bored
assign_resources! {
    imu: ImuResources {
        spi_channel: SPI2,
        sck: PB13,
        accel_pin: PC0,
        gyro_pin: PC3,
        mosi_pin: PC1,
        miso_pin: PC2,
        dma_tx: DMA1_CH1,
        dma_rx: DMA1_CH0,
    },
    led: LedResources {
        pin: PA7,
    },
    key: KeyResources {
        pin: PA15,
    },
    // fdcan1: Fdcan1Resources {
    //     can: FDCAN1,
    //     tx: PA12,
    //     rx: PA11,
    // },
    // fdcan2: Fdcan2Resources {
    //     can: FDCAN2,
    //     tx: PB13,
    //     rx: PB12,
    // },
    // fdcan3: Fdcan3Resources {
    //     can: FDCAN3,
    //     tx: PD1,
    //     rx: PD0,
    // },
    // usart1: Usart1Resources {
    //     usart: USART1,
    //     tx: PA9,
    //     rx: PA10,
    // },
    // usart2: Usart2Resources {
    //     usart: USART2,
    //     tx: PA2,
    //     rx: PA3,
    // },
    // dbus: DbusResources {
    //     usart: USART3,
    //     tx: PB10,
    //     rx: PB11,
    // },
}
