use crate::bored_resources::ImuResources;
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

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct Bmi088 {
    pub spi_perh: SpiHandles<2>,
    gyro: [f32; 3],
    accel: [f32; 3],
    temp: f32,
}

impl Bmi088 {
    pub fn new(imu_resources: ImuResources) -> Self {
        let mut config = Config::default();
        config.frequency = Hertz(1_000);

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

        //check communication with BMI088
        let _accel_id: u8 = self.read_accel_single_register(0x00).await?;

        // debug!("accel_id: {:X}", _accel_id);

        // soft reset
        self.write_accel_single_register(&ACCEL_RESET_REGISTER, ACCEL_RESET_MESSAGE)
            .await?;
        Timer::after(Duration::from_millis(80)).await;

        let accel_id = self.read_accel_single_register(0x00).await?;

        debug!("Accel_id: {:X}", accel_id);
        if accel_id != 0x1E {
            return Err(Bmi088Error::Custom("Invalid accelerometer chip ID"));
        }

        self.write_accel_single_register(&ACC_PWR_CTRL_ADDR, ACC_PWR_CTRL_ON)
            .await?;
        Timer::after(Duration::from_millis(50)).await;

        info!("BMI088 Accelerometer initialized successfully");
        Ok(())
    }

    pub async fn bmi088_gyro_init(&mut self) -> Result<(), Bmi088Error> {
        info!("Initialize BMI088 Gyroscope");

        //check communication with BMI088
        let _gyro_id = self.read_gyro_single_register(0x00).await?;

        // soft reset
        self.write_gyro_single_register(&GYRO_RESET_REGISTER, GYRO_RESET_MESSAGE)
            .await?;
        Timer::after(Duration::from_millis(50)).await;

        let gyro_id: u8 = self.read_gyro_single_register(0x00).await?;

        debug!("Gyro_id: {:X}", gyro_id);

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

        Timer::after(Duration::from_millis(50)).await;

        info!("Bmi088 Initialize success");
        Ok(())
    }

    async fn read_accel_single_register(&mut self, read_reg: u8) -> Result<u8, Bmi088Error> {
        self.accel_csb_low();
        Timer::after_micros(1).await;
        // bmi088 accel register needs to be read twice to get the correct message
        let mut buffer = [0u8];

        // For read operations, set the MSB of the register address
        self.spi_perh
            .spi
            .write(&[read_reg | BMI088_SPI_READ_CODE])
            .await?;

        self.spi_perh.spi.read(&mut buffer).await?;

        // debug!("spi_read_once: {}", &buffer[0]);

        self.spi_perh.spi.read(&mut buffer).await?;

        // debug!("spi_read_twice: {}", &buffer[0]);
        self.accel_csb_high();
        Timer::after_micros(1).await;

        Ok(buffer[0])
    }

    async fn read_gyro_single_register(&mut self, read_reg: u8) -> Result<u8, Bmi088Error> {
        self.gyro_csb_low();
        Timer::after_millis(1).await;

        let mut buffer = [0u8];

        // For read operations, set the MSB of the register address

        self.spi_perh
            .spi
            .write(&[read_reg | BMI088_SPI_READ_CODE])
            .await?;

        self.spi_perh.spi.read(&mut buffer).await?;

        // debug!("spi_read_once: {}", &buffer[0]);

        self.gyro_csb_high();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        Ok(buffer[0])
    }

    async fn write_accel_single_register(
        &mut self,
        write_register: &u8,
        write_message: u8,
    ) -> Result<(), Bmi088Error> {
        self.spi_perh
            .spi
            .write(&[write_register & BMI088_SPI_WRITE_CODE])
            .await?;

        self.spi_perh.spi.write(&[write_message]).await?;

        Ok(())
    }

    async fn write_gyro_single_register(
        &mut self,
        write_register: &u8,
        write_message: u8,
    ) -> Result<(), Bmi088Error> {
        // For read operations, set the MSB of the register address
        self.spi_perh
            .spi
            .write(&[write_register & BMI088_SPI_WRITE_CODE])
            .await?;

        self.spi_perh.spi.write(&[write_message]).await?;

        Timer::after(Duration::from_micros(1)).await; // Small delay

        Ok(())
    }

    async fn read_accel_consecutive_register(
        &mut self,
        read_reg_first: u8,
    ) -> Result<[u8; 6], Bmi088Error> {
        // info!("Read accel consecutive register");

        let mut send_data = [read_reg_first | BMI088_SPI_READ_CODE; 8];
        let mut receive_data = [0u8; 8];

        for i in 0..8 {
            send_data[i] += i as u8;
        }

        self.spi_perh
            .spi
            .transfer(&mut receive_data, &mut send_data)
            .await?;

        // Copy the received data (excluding the first byte which is dummy)
        let accel = receive_data[2..]
            .try_into()
            .map_err(|_| Bmi088Error::Custom("Slice to array conversion failed"))?;

        Ok(accel)
    }

    pub async fn read_accel(&mut self) -> Result<(f32, f32, f32), Bmi088Error> {
        self.accel_csb_low();
        // info!("Begin to read imu accel");
        let buffer = self.read_accel_consecutive_register(0x12u8).await?;

        let x_l = buffer[0];
        let x_h = buffer[1];
        let y_l = buffer[2];
        let y_h = buffer[3];
        let z_l = buffer[4];
        let z_h = buffer[5];

        // debug!(
        //     "Accel: x_l = {}, x_h = {}, y_l = {}, y_h = {}, z_l = {}, z_h = {}",
        //     x_l, x_h, y_l, y_h, z_l, z_h
        // );

        let x = i16::from_le_bytes([x_l, x_h]) as f32 * BMI088_ACCEL_3G_SEN;
        let y = i16::from_le_bytes([y_l, y_h]) as f32 * BMI088_ACCEL_3G_SEN;
        let z = i16::from_le_bytes([z_l, z_h]) as f32 * BMI088_ACCEL_3G_SEN;

        // debug!("Accel_official: x = {}, y = {}, z = {}", x, y, z);

        self.accel_csb_high();
        Ok((x, y, z))
    }

    async fn read_gyro_consecutive_register(
        &mut self,
        read_reg_first: u8,
    ) -> Result<[u8; 6], Bmi088Error> {
        // info!("Read gyro register");

        let mut send_data = [read_reg_first | BMI088_SPI_WRITE_CODE; 7];
        let mut receive_data = [0u8; 7];

        for i in 0..7 {
            send_data[i] += i as u8;
        }

        self.spi_perh
            .spi
            .transfer(&mut receive_data, &mut send_data)
            .await?;
        // debug!("receive data{:#}", receive_data);

        // Copy the received data (excluding the first byte which is dummy)
        let gyro = receive_data[1..]
            .try_into()
            .map_err(|_| Bmi088Error::Custom("Slice to array conversion failed"))?;

        Ok(gyro)
    }

    pub async fn read_gyro(&mut self) -> Result<(f32, f32, f32), Bmi088Error> {
        self.gyro_csb_low();
        let buffer = self.read_gyro_consecutive_register(0x02u8).await?;

        let x_l = buffer[0];
        let x_h = buffer[1];
        let y_l = buffer[2];
        let y_h = buffer[3];
        let z_l = buffer[4];
        let z_h = buffer[5];

        // debug!("gyro: x_l = {}, x_h = {}, y_l = {}, y_h = {}, z_l = {}, z_h = {}", x_l, x_h, y_l, y_h, z_l, z_h);

        let x = i16::from_le_bytes([x_l, x_h]) as f32 * BMI088_GYRO_2000_SEN;
        let y = i16::from_le_bytes([y_l, y_h]) as f32 * BMI088_GYRO_2000_SEN;
        let z = i16::from_le_bytes([z_l, z_h]) as f32 * BMI088_GYRO_2000_SEN;

        // debug!("Gyro: x={}, y={}, z={}", x, y, z);
        self.gyro_csb_high();

        Ok((x, y, z))
    }

    pub async fn read_temp(&mut self) -> Result<f32, Bmi088Error> {
        let temp_l = self.read_accel_single_register(0x22).await? as i16;
        let temp_h = self.read_accel_single_register(0x23).await? as i16;

        let mut temp = (temp_l << 3) | (temp_h >> 5);
        if temp > 1023 {
            temp = temp - 2048;
        }
        let temp = temp as f32 * 0.125f32 + 23.0f32;
        Ok(temp)
    }

    pub async fn imu_update(&mut self) -> Result<(), Bmi088Error> {
        // info!("Begin to read imu");

        let (accel_x, accel_y, accel_z) = self.read_accel().await?;
        self.accel = [accel_x, accel_y, accel_z];

        let gyro = self.read_gyro().await?;
        self.gyro = [gyro.0, gyro.1, gyro.2];

        let temp = self.read_temp().await?;
        self.temp = temp;

        Ok(())
    }

    pub fn format_data(&self) {
        info!("---------------------------------");
        info!(
            "Accel: x = {}, y = {}, z = {}",
            self.accel[0], self.accel[1], self.accel[2]
        );
        info!(
            "Gyro: x = {}, y = {}, z = {}",
            self.gyro[0], self.gyro[1], self.gyro[2]
        );
        info!("Temp: {}", self.temp);
        info!("---------------------------------");
    }

    #[inline(always)]
    pub fn accel_csb_low(&mut self) {
        self.spi_perh.cs[0].set_low();
    }

    #[inline(always)]
    pub fn accel_csb_high(&mut self) {
        self.spi_perh.cs[0].set_high();
    }

    #[inline(always)]
    pub fn gyro_csb_low(&mut self) {
        self.spi_perh.cs[1].set_low();
    }

    #[inline(always)]
    pub fn gyro_csb_high(&mut self) {
        self.spi_perh.cs[1].set_high();
    }
}
