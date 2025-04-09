use embassy_time::{Duration, Ticker};

use crate::bored::bored_resources::Usart7Resources;
use crate::bsc::usart7::Usart7;

#[embassy_executor::task]
pub async fn task_usart7(usart7_re: Usart7Resources) {
    let mut usart7 = Usart7::new(usart7_re);
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        usart7.send(b"Hello, World!");
        // Send the message
        ticker.next().await;
    }
}
