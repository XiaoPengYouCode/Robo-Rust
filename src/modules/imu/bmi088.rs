use crate::bored::bored_resources::ImuResources;

use defmt::{debug, info, Format};
use embassy_stm32::gpio::Pin;
use embassy_stm32::spi;
use embassy_stm32::spi::Config;
use embassy_stm32::time::Hertz;
use embassy_time::Duration;
use embassy_time::Timer;

use crate::bsp::spi_dma::*;
use crate::modules::imu::bmi088_registers::*;

#[derive(Debug, Format)]
pub enum Bmi088Error {
    Custom(&'static str),
    SpiError(spi::Error),
}

impl From<spi::Error> for Bmi088Error {
    fn from(err: spi::Error) -> Self {
        Bmi088Error::SpiError(err)
    }
}

pub struct Bmi088 {
    pub spi_perh: SpiHandles<2>,
    gyro: [f32; 3],
    accel: [f32; 3],
    temp: f32,
}

impl Bmi088 {
    pub fn new(imu_resources: ImuResources) -> Self {
        let mut config = Config::default();
        // bmi088 spi最大通信频率为10MHz
        config.frequency = Hertz(10_000_000);

        Self {
            spi_perh: SpiHandles::new(
                imu_resources.spi_channel,
                imu_resources.sck,
                imu_resources.mosi_pin,
                imu_resources.miso_pin,
                imu_resources.dma_tx,
                imu_resources.dma_rx,
                [
                    imu_resources.accel_pin.degrade(),
                    imu_resources.gyro_pin.degrade(),
                ],
                config,
            ),
            gyro: [0.0, 0.0, 0.0],
            accel: [0.0, 0.0, 0.0],
            temp: 0.0,
        }
    }

    pub async fn bmi088_accel_init(&mut self) -> Result<(), Bmi088Error> {
        info!("Initialize BMI088 Accelerometer");
        // soft reset
        self.write_accel_single_register(&ACCEL_RESET_REGISTER, ACCEL_RESET_MESSAGE)
            .await?;
        Timer::after(Duration::from_millis(50)).await;

        // 不知道为什么一次写入不成功，但是两次写入就成功了
        // 打开加速度计电源（要求软重启后必须打开）
        self.write_accel_single_register(&ACC_PWR_CTRL_ADDR, ACC_PWR_CTRL_ON)
            .await?;
        self.write_accel_single_register(&ACC_PWR_CTRL_ADDR, ACC_PWR_CTRL_ON)
            .await?;
        Timer::after(Duration::from_millis(50)).await;

        // 不知道为什么一次写入就成功了
        // 默认为低功耗模式，需要写为active模式
        self.write_accel_single_register(&ACC_PWR_CONF_ADDR, ACC_PWR_CONF_ACT)
            .await?;
        Timer::after(Duration::from_millis(50)).await;

        // 检查加速度计ID
        let accel_id = self.read_accel_single_register(0x00).await?;
        if accel_id != 0x1E {
            return Err(Bmi088Error::Custom("Invalid accelerometer chip ID"));
        }

        info!("BMI088 Accelerometer initialized successfully");
        Ok(())
    }

    pub async fn bmi088_gyro_init(&mut self) -> Result<(), Bmi088Error> {
        info!("Initialize BMI088 Gyroscope");

        // soft reset
        self.write_gyro_single_register(&GYRO_RESET_REGISTER, GYRO_RESET_MESSAGE)
            .await?;
        Timer::after(Duration::from_millis(50)).await;

        // 检查陀螺仪ID
        let gyro_id: u8 = self.read_gyro_single_register(0x00).await?;
        if gyro_id != 0x0F {
            return Err(Bmi088Error::Custom("Invalid gyroscope chip ID"));
        }

        info!("BMI088 Gyroscope initialized successfully");
        Ok(())
    }

    pub async fn bmi088_init(&mut self) -> Result<(), Bmi088Error> {
        info!("Initialize BMI088");

        self.bmi088_accel_init().await?;
        self.bmi088_gyro_init().await?;

        info!("Bmi088 Initialize success");
        Ok(())
    }

    // bmi088 accel register needs to be read twice to get the correct message
    async fn read_accel_single_register(&mut self, read_reg: u8) -> Result<u8, Bmi088Error> {
        self.accel_csb_low().await;
        let mut buffer = [0u8];

        // For read operations, set the MSB of the register address
        self.spi_perh
            .spi
            .write(&[read_reg | BMI088_SPI_READ_CODE])
            .await?;

        self.spi_perh.spi.read(&mut buffer).await?;
        self.spi_perh.spi.read(&mut buffer).await?;

        self.accel_csb_high().await;

        Ok(buffer[0])
    }

    async fn read_gyro_single_register(&mut self, read_reg: u8) -> Result<u8, Bmi088Error> {
        self.gyro_csb_low().await;

        let mut buffer = [0u8];

        self.spi_perh
            .spi
            .write(&[read_reg | BMI088_SPI_READ_CODE])
            .await?;

        self.spi_perh.spi.read(&mut buffer).await?;

        self.gyro_csb_high().await;
        Ok(buffer[0])
    }

    async fn write_accel_single_register(
        &mut self,
        write_register: &u8,
        write_message: u8,
    ) -> Result<(), Bmi088Error> {
        self.accel_csb_low().await;

        self.spi_perh
            .spi
            .write(&[write_register & BMI088_SPI_WRITE_CODE])
            .await?;

        self.spi_perh.spi.write(&[write_message]).await?;

        self.accel_csb_high().await;
        Ok(())
    }

    async fn write_gyro_single_register(
        &mut self,
        write_register: &u8,
        write_message: u8,
    ) -> Result<(), Bmi088Error> {
        self.gyro_csb_low().await;
        self.spi_perh
            .spi
            .write(&[write_register & BMI088_SPI_WRITE_CODE])
            .await?;

        self.spi_perh.spi.write(&[write_message]).await?;

        self.gyro_csb_high().await;

        Ok(())
    }

    pub async fn read_accel(&mut self) -> Result<(f32, f32, f32, f32), Bmi088Error> {
        // info!("Begin to read imu accel");
        self.accel_csb_low().await;

        let send_data = [0x12 | BMI088_SPI_READ_CODE; 0x23 - 0x12 + 3];
        let mut receive_data = [0u8; 0x23 - 0x12 + 3];

        self.spi_perh
            .spi
            .transfer(&mut receive_data, &send_data)
            .await?;

        self.accel_csb_high().await;

        let x_l = receive_data[2];
        let x_h = receive_data[3];
        let y_l = receive_data[4];
        let y_h = receive_data[5];
        let z_l = receive_data[6];
        let z_h = receive_data[7];

        let x = i16::from_le_bytes([x_l, x_h]) as f32 * BMI088_ACCEL_3G_SEN;
        let y = i16::from_le_bytes([y_l, y_h]) as f32 * BMI088_ACCEL_3G_SEN;
        let z = i16::from_le_bytes([z_l, z_h]) as f32 * BMI088_ACCEL_3G_SEN;

        let temp_l = receive_data[receive_data.len() - 2] as i16;
        let temp_h = receive_data[receive_data.len() - 1] as i16;

        let mut temp = (temp_l << 3) | (temp_h >> 5);
        if temp > 1023 {
            temp -= 2048;
        }
        let temp = temp as f32 * 0.125f32 + 23.0f32;

        Ok((x, y, z, temp))
    }

    pub async fn read_gyro(&mut self) -> Result<(f32, f32, f32), Bmi088Error> {
        // info!("Read gyro register");
        self.gyro_csb_low().await;

        let send_data = [0x02 | BMI088_SPI_READ_CODE; 7];
        let mut receive_data = [0u8; 7];

        self.spi_perh
            .spi
            .transfer(&mut receive_data, &send_data)
            .await?;

        self.gyro_csb_high().await;

        let x_l = receive_data[0];
        let x_h = receive_data[1];
        let y_l = receive_data[2];
        let y_h = receive_data[3];
        let z_l = receive_data[4];
        let z_h = receive_data[5];

        let x = i16::from_le_bytes([x_l, x_h]) as f32 * BMI088_GYRO_2000_SEN;
        let y = i16::from_le_bytes([y_l, y_h]) as f32 * BMI088_GYRO_2000_SEN;
        let z = i16::from_le_bytes([z_l, z_h]) as f32 * BMI088_GYRO_2000_SEN;

        Ok((x, y, z))
    }

    pub async fn read_temp(&mut self) -> Result<f32, Bmi088Error> {
        let temp_l = self.read_accel_single_register(0x22).await? as i16;
        let temp_h = self.read_accel_single_register(0x23).await? as i16;

        let mut temp = (temp_l << 3) | (temp_h >> 5);
        if temp > 1023 {
            temp -= 2048;
        }
        let temp = temp as f32 * 0.125f32 + 23.0f32;
        Ok(temp)
    }

    pub async fn imu_update(&mut self) -> Result<(), Bmi088Error> {
        // info!("Begin to read imu");
        let (accel_x, accel_y, accel_z, temp) = self.read_accel().await?;
        self.accel = [accel_x, accel_y, accel_z];
        let (gyro_x, gyro_y, gyro_z) = self.read_gyro().await?;
        self.gyro = [gyro_x, gyro_y, gyro_z];
        self.temp = temp;

        Ok(())
    }

    pub fn format_output_data(&self) {
        debug!("---------------------------------");
        debug!(
            "Accel: x = {}, y = {}, z = {}",
            self.accel[0], self.accel[1], self.accel[2]
        );
        debug!(
            "Gyro: x = {}, y = {}, z = {}",
            self.gyro[0], self.gyro[1], self.gyro[2]
        );
        debug!("Temp: {}", self.temp);
        debug!("---------------------------------");
    }

    pub fn get_accel(&self) -> &[f32; 3] {
        &self.accel
    }

    pub fn get_gyro(&self) -> &[f32; 3] {
        &self.gyro
    }

    #[inline(always)]
    pub async fn accel_csb_low(&mut self) {
        self.spi_perh.cs[0].set_low();
    }

    #[inline(always)]
    pub async fn accel_csb_high(&mut self) {
        self.spi_perh.cs[0].set_high();
    }

    #[inline(always)]
    pub async fn gyro_csb_low(&mut self) {
        self.spi_perh.cs[1].set_low();
    }

    #[inline(always)]
    pub async fn gyro_csb_high(&mut self) {
        self.spi_perh.cs[1].set_high();
    }
}
