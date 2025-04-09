use embassy_time::{Duration, Ticker};

use crate::bored::bored_resources::FdCan1Resources;
use crate::bsc::can1::Can1Handler;

#[embassy_executor::task]
pub async fn task_can1(can1_re: FdCan1Resources) {
    let mut can1 = Can1Handler::new(can1_re);
    let mut ticker = Ticker::every(Duration::from_millis(1));
    loop {
        can1.estimate();
        can1.input();
        can1.ctrl();
        can1.write().await;
        can1.read().await;
        ticker.next().await;
    }
}
