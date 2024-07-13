use defmt::*;
// use embassy_stm32::bind_interrupts;
// use embassy_stm32::can::filter::Mask32;
// use embassy_stm32::can::{
//     Can, Fifo, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, StandardId,
//     TxInterruptHandler,
// };
// use embassy_stm32::peripherals::CAN1;
// use {defmt_rtt as _, panic_probe as _};

// bind_interrupts!(struct Irqs {
//     CAN1_RX0 => Rx0InterruptHandler<CAN1>;
//     CAN1_RX1 => Rx1InterruptHandler<CAN1>;
//     CAN1_SCE => SceInterruptHandler<CAN1>;
//     CAN1_TX => TxInterruptHandler<CAN1>;
// });

// use crate::modules::motors::lk_motors::*;

enum LkMotorsControlState {
    SpeedControl {
        speed: i32
    },
    AngleControl { angle: i32 },
}

pub struct LkMotors {
    can_message: [u8; 8],
    can_id: u8,
    state: LkMotorsControlState,
}

impl LkMotors {
    pub async fn new(can_message: u64, can_id: u8) -> Self {
        info!("LkMotors initialized!");
        if can_id > 32 {
            error!("CAN ID must be less than 32");
        }
        let can_message = can_message.to_be_bytes();
        let can_instance = Self {
            can_message,
            can_id,
            state: LkMotorsControlState::SpeedControl { speed: 0 },
        };
        info!("LkMotors initialized success!");
        can_instance
    }

    pub async fn speed_control(&mut self, speed: i32) {
        match self.state {
            LkMotorsControlState::SpeedControl {
                speed: current_speed,
            } => {
                if speed == current_speed {
                    return;
                }
            }
            _ => {
                self.state = LkMotorsControlState::SpeedControl { speed: speed };
            }
        }
        self.can_message[0] = 0xA2;
        self.can_message[1] = 0x00;
        self.can_message[2] = 0x00;
        self.can_message[3] = 0x00;
        self.can_message[4] = speed as u8;
        self.can_message[5] = (speed >> 8) as u8;
        self.can_message[6] = (speed >> 16) as u8;
        self.can_message[7] = (speed >> 24) as u8;
    }

    pub fn angle_control(&mut self, set_angle: i32) {
        match self.state {
            LkMotorsControlState::AngleControl { angle } => {
                if set_angle == angle {
                    return;
                }
            }
            _ => {
                self.state = LkMotorsControlState::AngleControl { angle: set_angle };
            }
        }
        self.can_message[0] = 0xA3;
        self.can_message[1] = 0x00;
        self.can_message[2] = 0x00;
        self.can_message[3] = 0x00;
        self.can_message[4] = set_angle as u8;
        self.can_message[5] = (set_angle >> 8) as u8;
        self.can_message[6] = (set_angle >> 16) as u8;
        self.can_message[7] = (set_angle >> 24) as u8;
    }

    pub async fn get_id(&self) -> u8 {
        self.can_id
    }

    pub async fn get_can_message(&self) -> &[u8; 8] {
        &self.can_message
    }
}

// #[embassy_executor::task]
// async fn lk_tasks() {
//     // The next two lines are a workaround for testing without transceiver.
//     // To synchronise to the bus the RX input needs to see a high level.
//     // Use `mem::forget()` to release the borrow on the pin but keep the
//     // pull-up resistor enabled.

//     let lk_can_message: u64 = 0xA2_00_00_00_00_00_00_00;
//     let lk_can_id: u8 = 0x01;
//     let mut lk_motors_1 = LkMotors::new(lk_can_message, lk_can_id).await;
//     info!("Hello World!");

//     let p = embassy_stm32::init(Default::default());

//     let mut lk_can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);
//     lk_can
//         .modify_filters()
//         .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

//     lk_can
//         .modify_config()
//         .set_loopback(false) // Receive own frames
//         .set_silent(false)
//         .set_bitrate(1_000_000);

//     lk_can.enable().await;

//     let can_id: u16 = lk_motors_1.get_id().await as u16 + 0x140;

//     lk_motors_1.speed_control(1440_000).await;

//     loop {
//         let tx_frame = Frame::new_data(
//             unwrap!(StandardId::new(can_id as _)),
//             lk_motors_1.get_can_message().await,
//         )
//         .unwrap();
//         // info!("send: {:?}", &tx_frame);
//         lk_can.write(&tx_frame).await;

//         let envelope = lk_can.read().await.unwrap();

//         let receive_data: [u8; 8] = [
//             envelope.frame.data()[0],
//             envelope.frame.data()[1],
//             envelope.frame.data()[2],
//             envelope.frame.data()[3],
//             envelope.frame.data()[4],
//             envelope.frame.data()[5],
//             envelope.frame.data()[6],
//             envelope.frame.data()[7],
//         ];

//         // let motors_speed = (receive_data[4] as i16 | (receive_data[5] as i16) << 8) as f32 / 10.0;
//         // let motors_angle = ((receive_data[6] as u16 | (receive_data[7] as u16) << 8) >> 2) as f32
//         //     / 16384.0
//         //     * 360.0;
//         // info!("receive: {}", motors_angle);

//         let motor_tempearture = receive_data[1];
//         info!("motor temperature = {}", motor_tempearture)
//     }
// }