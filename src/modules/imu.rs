use defmt::*;

use embassy_stm32::spi::Config;
use embassy_stm32::gpio::Pin;
use embassy_time::{Timer, Instant};
use embassy_time::Duration;

use embassy_stm32::peripherals::{DMA2_CH2, DMA2_CH3, PA4, PA7, PB0, PB3, PB4, SPI1};

use crate::bsp::spi_dma::*;

const BMI088_GYRO_2000_SEN: f32 = 0.00106526443603169529841533860381f32;
const BMI088_ACCEL_3G_SEN: f32 = 0.0008974358974f32;

pub struct Bmi088 {
    spi_perh: SpiHandles<2>,
    gyro: [f32; 3],
    accel: [f32; 3],
    temp: f32,
}

impl Bmi088 {
    pub fn new(
        spi_perh: SPI1,
        sck: PB3,
        accel_pin: PA4,
        gyro_pin: PB0,
        mosi_pin: PA7,
        miso_pin: PB4,
        dma_tx: DMA2_CH3,
        dma_rx: DMA2_CH2,
    ) -> Self {
        let config = Config::default();
        // let csb1_accel = Output::new(accel_pin, Level::Low, Speed::High);
        // let csb2_gyro = Output::new(gyro_pin, Level::Low, Speed::High);

        Self {
            spi_perh: SpiHandles::new(
                spi_perh,
                sck,
                mosi_pin,
                miso_pin,
                dma_tx,
                dma_rx,
                [accel_pin.degrade(), gyro_pin.degrade()],
                config),
            gyro: [0.0, 0.0, 0.0],
            accel: [0.0, 0.0, 0.0],
            temp: 0.0,
        }
    }

    // pub async fn init(&mut self) -> Result<(), &'static str> {
    //     // Initialize BMI088
    //     // Check chip ID, configure settings, etc.
    //     let accel_id = self.read_accel_register(0x00).await?;
    //     if accel_id != 0x1E {
    //         return Err("Invalid accelerometer chip ID");
    //     }

    //     let gyro_id = self.read_gyro_register(0x00).await?;
    //     if gyro_id != 0x0F {
    //         return Err("Invalid gyroscope chip ID");
    //     }

    //     Ok(())
    // }

    async fn read_accel_register(&mut self, reg: u8) -> Result<u8, &'static str> {
        self.spi_perh.cs[0].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        let read_reg = reg | 0x80;
        
        self.spi_perh.spi.write(&[read_reg]).await.map_err(|_| "SPI write failed")?;
        let mut buffer = [0u8];
        self.spi_perh.spi.transfer(&mut buffer, &[read_reg]).await.map_err(|_| "SPI transfer failed")?;

        self.spi_perh.cs[0].set_high();
        Ok(buffer[0])
    }

    async fn read_gyro_register(&mut self, reg: u8) -> Result<u8, &'static str> {
        self.spi_perh.cs[1].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        let read_reg = reg | 0x80;
        
        self.spi_perh.spi.write(&[read_reg]).await.map_err(|_| "SPI write failed")?;
        let mut buffer = [0u8];
        self.spi_perh.spi.transfer(&mut buffer, &[read_reg]).await.map_err(|_| "SPI transfer failed")?;

        self.spi_perh.cs[1].set_high();
        Ok(buffer[0])
    }

    pub async fn read_accel(&mut self) -> Result<(f32, f32, f32), &'static str> {
        let x_l = self.read_accel_register(0x12).await? as i16;
        let x_h = self.read_accel_register(0x13).await? as i16;
        let y_l = self.read_accel_register(0x14).await? as i16;
        let y_h = self.read_accel_register(0x15).await? as i16;
        let z_l = self.read_accel_register(0x16).await? as i16;
        let z_h = self.read_accel_register(0x17).await? as i16;

        let x = ((x_h << 8) | x_l) as f32;
        let y = ((y_h << 8) | y_l) as f32;
        let z = ((z_h << 8) | z_l) as f32;

        // info!("x = {}, y = {}, z = {}", x, y, z);

        Ok((x, y, z))
    }

    pub async fn read_gyro(&mut self) -> Result<(f32, f32, f32), &'static str> {
        let x_l = self.read_gyro_register(0x02).await? as i16;
        let x_h = self.read_gyro_register(0x03).await? as i16;
        let y_l = self.read_gyro_register(0x04).await? as i16;
        let y_h = self.read_gyro_register(0x05).await? as i16;
        let z_l = self.read_gyro_register(0x06).await? as i16;
        let z_h = self.read_gyro_register(0x07).await? as i16;

        let x = ((x_h << 8) | x_l) as f32;
        let y = ((y_h << 8) | y_l) as f32;
        let z = ((z_h << 8) | z_l) as f32;

        Ok((x, y, z))
    }

    pub async fn read_temp(&mut self) -> Result<f32, &'static str> {
        let temp_l = self.read_accel_register(0x22).await? as i16;
        let temp_h = self.read_accel_register(0x23).await? as i16;

        let mut temp = (temp_l << 3) | (temp_h >> 5);
        if temp > 1023 {     
            temp = temp - 2048;
        }
        let temp = temp as f32 * 0.125f32 + 23.0f32;
        // defmt::debug!("Raw temperature: {}", temp);
        Ok(temp as f32)
    }
}

// 2kHz imu read
#[embassy_executor::task]
pub async fn imu_task(mut bmi088: Bmi088) {
    let period = Duration::from_hz(2000);
    let mut last_run = Instant::now();
    loop {
        match bmi088.read_accel().await {
            Ok((x, y, z)) => {
                bmi088.accel = [x * BMI088_ACCEL_3G_SEN, y * BMI088_ACCEL_3G_SEN, z * BMI088_ACCEL_3G_SEN];
                // info!("Accel: x={}, y={}, z={}", 
                //     bmi088.accel[0],
                //     bmi088.accel[1], 
                //     bmi088.accel[2]);
            },
            Err(e) => {
                defmt::error!("Failed to read accelerometer: {}", e);
            }
        }

        match bmi088.read_gyro().await {
            Ok((x, y, z)) => {
                bmi088.gyro = [x * BMI088_GYRO_2000_SEN, y * BMI088_GYRO_2000_SEN, z * BMI088_GYRO_2000_SEN];
                info!("Gyro: x={}, y={}, z={}", 
                    bmi088.gyro[0], 
                    bmi088.gyro[1], 
                    bmi088.gyro[2]);
            },
            Err(e) => {
                defmt::error!("Failed to read gyroscope: {}", e);
            }
        }

        match bmi088.read_temp().await {
            Ok(temp) => {
                bmi088.temp = temp;
                // info!("Temp: {}", temp);
            },
            Err(e) => {
                error!("Failed to read temperature: {}", e);
            } 
        }

        last_run += period;
        Timer::at(last_run).await;
    }
}