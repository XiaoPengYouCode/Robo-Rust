use defmt::{debug, error};
use embassy_stm32::{
    can::{CanRx, CanTx, Frame},
    peripherals::FDCAN1,
};

use crate::applications::shoot::{self, *};
use crate::bored::bored_resources::FdCan1Resources;
use embassy_stm32::{bind_interrupts, can};

bind_interrupts!(struct FdCan1Irqs {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

pub struct Can1Handler {
    pub tx: CanTx<'static>,
    pub rx: CanRx<'static>,
    pub rec_frame: Frame,
    pub write_frame: Frame,
    pub shoot: Shoot,
}

impl Can1Handler {
    pub fn new(re: FdCan1Resources) -> Self {
        let (tx, rx, _) = can::CanConfigurator::new(re.can, re.rx, re.tx, FdCan1Irqs)
            .into_normal_mode()
            .split();
        Self {
            tx,
            rx,
            rec_frame: Frame::new_standard(0x03, &[0u8; 8]).unwrap(),
            write_frame: Frame::new_standard(0x03, &[0u8; 8]).unwrap(),
            shoot: shoot::Shoot::new(),
        }
    }

    pub async fn read(&mut self) {
        match self.rx.read().await {
            Ok(e) => {
                self.rec_frame = e.frame;
                debug!("Received frame: {:?}", self.rec_frame);
            }
            Err(e) => {
                error!("Failed to read frame: {:?}", e);
            }
        }
    }

    pub async fn write(&mut self) {
        match self.tx.write(&self.write_frame).await {
            Some(frame) => {
                error!("Failed to write frame: {:?}", frame);
            }
            None => {
                debug!("Frame written successfully");
            }
        }
    }

    pub fn estimate(&mut self) {
        if let Some(data) = self.rec_frame.data().get(..8) {
            self.shoot
                .estimate(data.try_into().expect("Failed to convert data"));
        } else {
            error!("Invalid data length");
        }
    }

    pub fn input(&mut self) {
        self.shoot.input();
    }

    pub fn ctrl(&mut self) {
        self.shoot.ctrl();
    }
}
