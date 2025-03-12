use crate::bored::bored_resources::LedSpiResources;
use crate::modules::led::Led;

#[embassy_executor::task]
pub async fn task_led(led_resource: LedSpiResources) {
    let mut led = Led::new(led_resource);
    led.water_flow().await;
    // led.flash().await;
}
