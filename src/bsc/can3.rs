use defmt::error;
use embassy_stm32::can::{self, Can, CanConfigurator};
use embassy_stm32::can::{IT0InterruptHandler, IT1InterruptHandler};

use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals::FDCAN3;

use crate::bored::bored_resources::Fdcan3Resources;

// CAN 有两个中断是因为它的设计需要分别处理 接收/发送事件（IT0） 和 错误/状态变化事件（IT1）
// 这样可以减少中断处理的复杂度，提高实时性。这种设计源于 CAN 协议的特点：

// IT0（接收/发送相关）：
// // 处理接收消息队列满、消息发送完成等高频、实时性要求高的事件。
// IT1（错误/状态变化相关）：
// // 处理较低频的错误事件（如总线错误、状态变化），避免这些事件干扰高优先级的接收/发送任务。
// // 这种两中断设计在性能和灵活性之间达到了平衡，不需要更多的中断。

bind_interrupts!(struct FdCan3Irqs {
    FDCAN3_IT0 => IT0InterruptHandler<FDCAN3>;
    FDCAN3_IT1 => IT1InterruptHandler<FDCAN3>;
});

pub struct Can3Handle {
    can3: Can<'static>,
}

impl Can3Handle {
    pub fn new(re: Fdcan3Resources) -> Self {
        let mut can3 = CanConfigurator::new(re.can, re.rx, re.tx, FdCan3Irqs);
        can3.set_bitrate(1_000_000);
        let can3 = can3.into_normal_mode();
        // debug!("FDCAN 1 into normal mode");
        Self { can3 }
    }

    pub async fn can_send_read_data(&mut self, data: &[u8; 8]) -> Result<[u8; 8], &'static str> {
        let frame = can::frame::Frame::new_extended(0x03, data).unwrap();
        _ = self.can3.write(&frame).await;

        match self.can3.read().await {
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
