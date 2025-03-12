use defmt::error;
use embassy_stm32::can::{self, Can, CanConfigurator};
use embassy_stm32::can::{IT0InterruptHandler, IT1InterruptHandler};

use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals::FDCAN1;

use crate::bored::bored_resources::Fdcan1Resources;

bind_interrupts!(struct FdCan1Irqs {
    FDCAN1_IT0 => IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => IT1InterruptHandler<FDCAN1>;
});

pub struct Can1Handle {
    can1: Can<'static>,
}

impl Can1Handle {
    pub fn new(re: Fdcan1Resources) -> Self {
        let mut can1 = CanConfigurator::new(re.can, re.rx, re.tx, FdCan1Irqs);
        can1.set_bitrate(1_000_000);
        let can1 = can1.into_normal_mode();
        // debug!("FDCAN 1 into normal mode");
        Self { can1 }
    }

    pub async fn can_send_read_data(&mut self, data: &[u8; 8]) -> Result<[u8; 8], &'static str> {
        let frame = can::frame::Frame::new_extended(0x03, data).unwrap();
        _ = self.can1.write(&frame).await;

        match self.can1.read().await {
            Ok(envelope) => {
                let (rx_frame, _) = envelope.parts();
                Ok([
                    rx_frame.data()[0],
                    rx_frame.data()[1],
                    rx_frame.data()[2],
                    rx_frame.data()[3],
                    rx_frame.data()[4],
                    rx_frame.data()[5],
                    rx_frame.data()[6],
                    rx_frame.data()[7],
                ])
            }
            Err(_err) => {
                error!("Error in frame");
                Err("Failed to send data")
            }
        }
    }
}
