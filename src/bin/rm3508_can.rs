#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, StandardId,
    TxInterruptHandler,
};
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::peripherals::CAN1;
use embassy_time::{Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

use roborust::modules::motors::dji_motors::rm3508::{Rm3508, ReductionGearboxTransmissionRatio};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, Roborust!");

    let period = Duration::from_hz(1000);
    let mut last_run = Instant::now();

    let rm3508_1_id: u16 = 0x200;
    let rm3508_gearbox_ration = ReductionGearboxTransmissionRatio::Ratio19;
    let mut rm3508_1 = Rm3508::new(rm3508_1_id, rm3508_gearbox_ration).await;

    let p = embassy_stm32::init(Default::default());

    let button = Input::new(p.PA0, Pull::Up);

    let mut rm3508_can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);
    rm3508_can
        .modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

    rm3508_can
        .modify_config()
        .set_loopback(false) // Receive own frames
        .set_silent(false)
        .set_bitrate(1_000_000);

    rm3508_can.enable().await;

    loop {
        let tx_frame = Frame::new_data(
            unwrap!(StandardId::new(rm3508_1.get_id().await as _)),
            rm3508_1.get_can_message().await,
        )
        .unwrap();
        // info!("send: {:?}", &tx_frame);
        rm3508_can.write(&tx_frame).await;

        let envelope = rm3508_can.read().await.unwrap();

        let receive_data: [u8; 8] = [
            envelope.frame.data()[0],
            envelope.frame.data()[1],
            envelope.frame.data()[2],
            envelope.frame.data()[3],
            envelope.frame.data()[4],
            envelope.frame.data()[5],
            envelope.frame.data()[6],
            envelope.frame.data()[7],
        ];

        let current_speed = receive_data[3] as i16 | (receive_data[2] as i16) << 8;
        info!("motor_speed: {}", current_speed);

        let current_current = receive_data[5] as i16 | (receive_data[4] as i16) << 8;

        // let target_current = (
        //     rm3508_1.get_can_message().await[0] as u16) << 8 
        //     | rm3508_1.get_can_message().await[1] as u16;
        let target_speed = 500;

        rm3508_1.speed_control(target_speed, current_speed, current_current).await;

        if current_speed > 0x384 || current_speed < -0x384 {
            rm3508_1.protect().await;
        }

        last_run += period;
        Timer::at(last_run).await;

        if button.is_low() {
            rm3508_1.protect().await;
        }
    }
}
