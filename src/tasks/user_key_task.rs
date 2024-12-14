use crate::bored_resources::KeyResources;
use defmt::{debug, info};
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn user_key_task(key_resources: KeyResources) {
    info!("User key task started!");
    let key_pin = Input::new(key_resources.pin, Pull::Up);
    loop {
        if key_pin.is_high() {
            debug!("LED is high");
        } else {
            debug!("LED is low");
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}
