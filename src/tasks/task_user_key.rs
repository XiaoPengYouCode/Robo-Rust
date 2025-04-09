use crate::bored::bored_resources::KeyResources;
use defmt::debug;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn task_user_key(key_resources: KeyResources) {
    debug!("User key task started!");
    let key_pin = Input::new(key_resources.pin, Pull::Up);
    let mut last_key_state = false;
    loop {
        let current_key_state = key_pin.is_high();
        if current_key_state != last_key_state {
            if current_key_state {
                debug!("LED is high");
            } else {
                debug!("LED is low");
            }
            last_key_state = current_key_state;
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}
