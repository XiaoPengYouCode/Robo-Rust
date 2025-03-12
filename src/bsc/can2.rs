use defmt::error;
use embassy_stm32::can::{self, Can, CanConfigurator};
use embassy_stm32::can::{IT0InterruptHandler, IT1InterruptHandler};

use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals::FDCAN2;

use crate::bored::bored_resources::Fdcan2Resources;

bind_interrupts!(struct FdCan2Irqs {
    FDCAN2_IT0 => IT0InterruptHandler<FDCAN2>;
    FDCAN2_IT1 => IT1InterruptHandler<FDCAN2>;
});

pub struct Can2Handle {
    can2: Can<'static>,
}

impl Can2Handle {
    pub fn new(re: Fdcan2Resources) -> Self {
        let mut can2 = CanConfigurator::new(re.can, re.rx, re.tx, FdCan2Irqs);
        can2.set_bitrate(1_000_000);
        let can2 = can2.into_normal_mode();
        // debug!("FDCAN 1 into normal mode");
        Self { can2 }
    }

    pub async fn can_send_read_data(&mut self, data: &[u8; 8]) -> Result<[u8; 8], &'static str> {
        let frame = can::frame::Frame::new_extended(0x03, data).unwrap();
        _ = self.can2.write(&frame).await;

        match self.can2.read().await {
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
