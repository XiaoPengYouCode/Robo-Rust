use embassy_time::Timer;

use crate::bored::bored_resources::LedSpiResources;
use crate::bsc::spi6_ws2812::WS2812;

struct GrbColor {
    pub color_data: [u8; 3],
    color_step: u16,
}

impl GrbColor {
    fn new() -> Self {
        Self {
            color_data: [255u8, 0u8, 0u8],
            color_step: 0,
        }
    }

    fn color_gradient(&mut self) {
        self.color_step %= 765; // 循环步数

        if self.color_step <= 255 {
            // 从绿色到红色
            self.color_data[0] = 255 - self.color_step as u8;
            self.color_data[1] = self.color_step as u8;
            self.color_data[2] = 0u8;
        } else if self.color_step <= 510 {
            // 从红色到蓝色
            self.color_data[0] = 0u8;
            self.color_data[1] = 255 - (self.color_step - 255) as u8;
            self.color_data[2] = (self.color_step - 255) as u8;
        } else {
            // 从蓝色回到绿色
            self.color_data[0] = (self.color_step - 510) as u8;
            self.color_data[1] = 0u8;
            self.color_data[2] = 255 - (self.color_step - 510) as u8;
        };

        self.color_step = self.color_step.wrapping_add(1);
    }

    fn white(&mut self) {
        self.color_data = [255u8; 3]
    }

    fn dark(&mut self) {
        self.color_data = [0; 3]
    }
}

pub struct Led {
    led: WS2812,
    grb_color: GrbColor,
}

impl Led {
    pub fn new(led_resource: LedSpiResources) -> Led {
        Led {
            led: WS2812::new(led_resource),
            grb_color: GrbColor::new(),
        }
    }

    pub async fn water_flow(&mut self) {
        loop {
            self.led.led_show_brg(&self.grb_color.color_data);
            self.grb_color.color_gradient();
            Timer::after_millis(10).await;
        }
    }

    pub async fn flash(&mut self) {
        loop {
            self.grb_color.white();
            self.led.led_show_brg(&self.grb_color.color_data);
            Timer::after_millis(500).await;
            self.grb_color.dark();
            self.led.led_show_brg(&self.grb_color.color_data);
            Timer::after_millis(500).await;
        }
    }
}
