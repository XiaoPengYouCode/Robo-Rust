#![no_std]
#![no_main]

use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, StandardId, TxInterruptHandler,
};
use {defmt_rtt as _, panic_probe as _};
use defmt::*;
use embassy_executor::Spawner;

use embassy_stm32::peripherals::{CAN1, CAN2};
use embassy_stm32::gpio::{Input, Pull};
// use embassy_time::Instant;

use embassy_stm32::bind_interrupts;

bind_interrupts!(struct Can_Irqs_1 {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

bind_interrupts!(struct Can_Irqs_2 {
    CAN2_RX0 => Rx0InterruptHandler<CAN2>;
    CAN2_RX1 => Rx1InterruptHandler<CAN2>;
    CAN2_SCE => SceInterruptHandler<CAN2>;
    CAN2_TX => TxInterruptHandler<CAN2>;
});

// use embassy_stm32::gpio;
use {defmt_rtt as _, panic_probe as _};

// // The next two lines are a workaround for testing without transceiver.
// // To synchronise to the bus the RX input needs to see a high level.
// // Use `mem::forget()` to release the borrow on the pin but keep the
// // pull-up resistor enabled.
// let rx_pin = Input::new(&mut p.PA11, Pull::Up);
// core::mem::forget(rx_pin);

async fn can1_service_init(p: Peripherals) -> Result<T, E> {
    info!("Can2 service init!");
    let can1 = Can::new(p.CAN1, p.PA11, p.PA12, Irqs).unwrap();

    can1.modify_config()
        .set_loopback(false)
        .set_silent(false)
        .set_bitrate(1_000_000);
    
    can1.modify_filters().enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    
    can1.enable().await;

    loop {
        let tx_frame = Frame::new_data(unwrap!(StandardId::new(i as _)), &[i]).unwrap();
        let tx_ts = Instant::now();
        can.write(&tx_frame).await;

        let envelope = can.read().await.unwrap();

        // We can measure loopback latency by using receive timestamp in the `Envelope`.
        // Our frame is ~55 bits long (exlcuding bit stuffing), so at 1mbps loopback delay is at least 55 us.
        // When measured with `tick-hz-1_000_000` actual latency is 80~83 us, giving a combined hardware and software
        // overhead of ~25 us. Note that CPU frequency can greatly affect the result.
        let latency = envelope.ts.saturating_duration_since(tx_ts);
    }
}

async fn can2_service_init(p: Peripherals) -> Result<T, E> {
    info!("Can2 service init!");
    let can1 = Can::new(p.can2, p.PA11, p.PA12, Irqs).unwrap();

    can2.modify_config()
        .set_loopback(false)
        .set_silent(false)
        .set_bitrate(1_000_000);
    
    can2.modify_filters().enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    
    can2.enable().await;

    loop {
        let tx_frame = Frame::new_data(unwrap!(StandardId::new(i as _)), &[i]).unwrap();
        let tx_ts = Instant::now();
        can.write(&tx_frame).await;

        let envelope = can.read().await.unwrap();

        // We can measure loopback latency by using receive timestamp in the `Envelope`.
        // Our frame is ~55 bits long (exlcuding bit stuffing), so at 1mbps loopback delay is at least 55 us.
        // When measured with `tick-hz-1_000_000` actual latency is 80~83 us, giving a combined hardware and software
        // overhead of ~25 us. Note that CPU frequency can greatly affect the result.
        let latency = envelope.ts.saturating_duration_since(tx_ts);
    }
}

async fn can_service_init(p: Peripherals) -> Result<T, E>{
    match can1_service_init(p).await {
        Ok(_) => {
            info!("Can1 service init success!");
        },
        Err(e) => {
            error!("Can1 service init error: {:?}", e);
            return Err(e);
        }
    }
    match can2_service_init(p).await {
        Ok(_) => {
            info!("Can2 service init success!");
        },
        Err(e) => {
            error!("Can2 service init error: {:?}", e);
            return Err(e);
        }
    }
    info!("Can service init success!")
    Ok(())
}