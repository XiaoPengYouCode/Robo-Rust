use embassy_stm32::can::CanConfigurator;
use embassy_stm32::can::{IT0InterruptHandler, IT1InterruptHandler};

use defmt::dbg;
use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals::{FDCAN1, FDCAN2, FDCAN3};
use embassy_stm32::Peripherals;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct FdCan1Irqs {
    FDCAN1_IT0 => IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => IT1InterruptHandler<FDCAN1>;
});

bind_interrupts!(struct FdCan2Irqs {
    FDCAN2_IT0 => IT0InterruptHandler<FDCAN2>;
    FDCAN2_IT1 => IT1InterruptHandler<FDCAN2>;
});

bind_interrupts!(struct FdCan3Irqs {
    FDCAN3_IT0 => IT0InterruptHandler<FDCAN3>;
    FDCAN3_IT1 => IT1InterruptHandler<FDCAN3>;
});

pub fn fdcan1_service_init(p: Peripherals) -> Result<(), &'static str> {
    let mut can1 = CanConfigurator::new(p.FDCAN1, p.PA11, p.PA12, FdCan1Irqs);
    can1.set_bitrate(1_000_000);
    can1.into_normal_mode();
    dbg!("FDCAN 1 into normal mode");
    Ok(())
}

// pub fn fdcan2_service_init(p: Peripherals) -> Result<(), &'static str> {
//     let mut can2 = CanConfigurator::new(p.FDCAN2, p.PA11, p.PA12, FdCan2Irqs);
//     can2.set_bitrate(1_000_000);
//     can2.into_normal_mode();
//     dbg!("FDCAN 2 into normal mode");
//     Ok(())
// }

// pub fn fdcan3_service_init(p: Peripherals) -> Result<(), &'static str> {
//     let mut can3 = CanConfigurator::new(p.FDCAN3, p.PA11, p.PA12, FdCan3Irqs);
//     can3.set_bitrate(1_000_000);
//     can3.into_normal_mode();
//     dbg!("FDCAN 1 into normal mode");
//     Ok(())
// }
