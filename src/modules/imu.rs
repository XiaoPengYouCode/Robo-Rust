use crate::bored_resources::ImuResources;
use defmt::*;
use embassy_stm32::gpio::Pin;
use embassy_stm32::spi::Config;
use embassy_stm32::time::Hertz;
use embassy_time::Duration;
use embassy_time::Timer;

use crate::bsp::spi_dma::*;

const BMI088_GYRO_2000_SEN: f32 = 0.00106526443603169529841533860381f32;
const BMI088_ACCEL_3G_SEN: f32 = 0.0008974358974f32;

const ACCEL_RESET_REGISTER: u8 = 0x7Eu8;
const ACCEL_RESET_MESSAGE: u8 = 0xB6u8;

const GYRO_RESET_REGISTER: u8 = 0x14u8;
const GYRO_RESET_MESSAGE: u8 = 0xB6u8;

const ACC_PWR_CTRL_ADDR: u8 = 0x7Du8;
const ACC_PWR_CTRL_ON: u8 = 0x04u8;

pub struct Bmi088 {
    pub spi_perh: SpiHandles<2>,
    gyro: [f32; 3],
    accel: [f32; 3],
    temp: f32,
}

impl Bmi088 {
    pub fn new(imu_resources: ImuResources) -> Self {
        let mut config = Config::default();
        config.frequency = Hertz(2_000_000);

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

    pub async fn enable(&mut self) -> Result<(), &'static str> {
        info!("Initialize BMI088");
        self.spi_perh.cs[0].set_high();
        self.spi_perh.cs[1].set_high();

        // software reset
        self.write_accel_single_register(&ACCEL_RESET_REGISTER, ACCEL_RESET_MESSAGE)
            .await?;
        self.write_gyro_single_register(&GYRO_RESET_REGISTER, GYRO_RESET_MESSAGE)
            .await?;

        // Check chip ID
        let accel_id = self.read_accel_single_register(0x00).await?;
        debug!("accel_id: {}", accel_id);
        if accel_id != 0x1E {
            return Err("Invalid accelerometer chip ID");
        }
        let gyro_id = self.read_gyro_single_register(0x00).await?;
        debug!("gyro_id: {}", gyro_id);
        if gyro_id != 0x0F {
            return Err("Invalid gyroscope chip ID");
        }

        // enable accel data
        self.write_accel_single_register(&ACC_PWR_CTRL_ADDR, ACC_PWR_CTRL_ON)
            .await?;

        // software reset
        self.write_accel_single_register(&ACCEL_RESET_REGISTER, ACCEL_RESET_MESSAGE)
            .await?;
        self.write_gyro_single_register(&GYRO_RESET_REGISTER, GYRO_RESET_MESSAGE)
            .await?;

        Ok(())
    }

    async fn read_accel_single_register(&mut self, read_reg: u8) -> Result<u8, &'static str> {
        self.spi_perh.cs[0].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        let read_message = read_reg | 0x80;
        self.spi_perh
            .spi
            .write(&[read_message])
            .await
            .map_err(|_| "SPI write failed")?;

        // bmi088 accel regoster need to read twice to get correct message
        let mut buffer = [0u8];
        self.spi_perh
            .spi
            .read(&mut buffer)
            .await
            .map_err(|_| "SPI read failed")?;
        // debug!("spi_read_once: {}", &buffer[0]);
        self.spi_perh
            .spi
            .read(&mut buffer)
            .await
            .map_err(|_| "SPI read failed")?;
        // debug!("spi_read_twice: {}", &buffer[0]);
        Timer::after(Duration::from_micros(1)).await; // Small delay

        self.spi_perh.cs[0].set_high();
        Ok(buffer[0])
    }

    async fn read_gyro_single_register(&mut self, read_reg: u8) -> Result<u8, &'static str> {
        self.spi_perh.cs[1].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        let read_message = read_reg | 0x80;

        self.spi_perh
            .spi
            .write(&[read_message])
            .await
            .map_err(|_| "SPI write failed")?;
        let mut buffer = [0u8];
        self.spi_perh
            .spi
            .read(&mut buffer)
            .await
            .map_err(|_| "SPI read failed")?;

        self.spi_perh.cs[1].set_high();
        Ok(buffer[0])
    }

    async fn write_accel_single_register(
        &mut self,
        write_reg: &u8,
        write_messge: u8,
    ) -> Result<(), &'static str> {
        self.spi_perh.cs[0].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        self.spi_perh
            .spi
            .write(&[write_reg & 0x7Fu8])
            .await
            .map_err(|_| "SPI write failed")?;

        self.spi_perh
            .spi
            .write(&[write_messge])
            .await
            .map_err(|_| "SPI write failed")?;

        Timer::after(Duration::from_micros(1)).await; // Small delay
        self.spi_perh.cs[0].set_high();

        Ok(())
    }

    async fn write_gyro_single_register(
        &mut self,
        write_reg: &u8,
        write_messge: u8,
    ) -> Result<(), &'static str> {
        self.spi_perh.cs[1].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        self.spi_perh
            .spi
            .write(&[write_reg & 0x7Fu8])
            .await
            .map_err(|_| "SPI write failed")?;

        self.spi_perh
            .spi
            .write(&[write_messge])
            .await
            .map_err(|_| "SPI write failed")?;

        Timer::after(Duration::from_micros(1)).await; // Small delay

        self.spi_perh.cs[1].set_high();

        Ok(())
    }

    async fn read_accel_consecutive_register(
        &mut self,
        read_reg_first: u8,
    ) -> Result<[u8; 6], &'static str> {
        info!("Read Accel Register consecutive");

        // Set CS low
        self.spi_perh.cs[0].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        let mut send_buffer = [read_reg_first; 7];
        for i in 0..7 {
            send_buffer[i] = (send_buffer[i] | 0b_1000_0000) + (i as u8);
        }

        let mut receive_buffer = [0u8; 7];

        debug!("send buffer = {:#}", send_buffer);
        debug!("send buffer = {:#b}", send_buffer);

        for i in 0..7 {
            let send = send_buffer[i];
            let mut receive = [0u8];
            self.spi_perh
                .spi
                .write(&[send])
                .await
                .map_err(|_| "->  first send error")?;
            self.spi_perh
                .spi
                .read(&mut receive)
                .await
                .map_err(|_| "->  once read error")?;
            debug!("receive data once = {:#b}", &receive[0]);
            self.spi_perh
                .spi
                .read(&mut receive)
                .await
                .map_err(|_| "->  twice read error")?;
            receive_buffer[i] = receive[0];
            debug!("receive data twice = {:#b}", &receive_buffer[i]);
        }

        debug!("receive_buffer = {:#}", receive_buffer);
        // Set CS high
        self.spi_perh.cs[0].set_high();

        // Return the received data (excluding the first byte which is dummy)
        Ok(receive_buffer[1..]
            .try_into()
            .map_err(|_| "->  Slice to array conversion failed")?)
    }

    async fn read_gyro_consecutive_register(
        &mut self,
        read_reg_first: u8,
    ) -> Result<[u8; 6], &'static str> {
        info!("Read gyro register");

        self.spi_perh.cs[1].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        let mut send_data = [read_reg_first | 0b_1000_0000; 7];
        let mut receive_data = [0u8; 7];

        for i in 0..7 {
            send_data[i] += i as u8;
        }

        // debug!("send data{:#}", send_data);
        // debug!("send data{:#b}", send_data);

        self.spi_perh
            .spi
            .transfer(&mut receive_data, &mut send_data)
            .await
            .map_err(|_| "->  Read gyro register error")?;
        // debug!("receive data{:#}", receive_data);

        // Copy the received data (excluding the first byte which is dummy)
        Ok(receive_data[1..]
            .try_into()
            .map_err(|_| "->  Slice to array conversion failed")?)
    }

    pub async fn read_accel(&mut self) -> Result<(f32, f32, f32), &'static str> {
        info!("Begin to read imu accel");
        let buffer = self.read_accel_consecutive_register(0x12u8).await?;

        let x_l = buffer[0];
        let x_h = buffer[1];
        let y_l = buffer[2];
        let y_h = buffer[3];
        let z_l = buffer[4];
        let z_h = buffer[5];

        // debug!("Accel: x_l = {}, x_h = {}, y_l = {}, y_h = {}, z_l = {}, z_h = {}", x_l, x_h, y_l, y_h, z_l, z_h);

        let x = i16::from_le_bytes([x_l, x_h]) as f32 * BMI088_ACCEL_3G_SEN;
        let y = i16::from_le_bytes([y_l, y_h]) as f32 * BMI088_ACCEL_3G_SEN;
        let z = i16::from_le_bytes([z_l, z_h]) as f32 * BMI088_ACCEL_3G_SEN;

        // debug!("Accel: x = {}, y = {}, z = {}", x, y, z);

        Ok((x, y, z))
    }

    pub async fn read_gyro(&mut self) -> Result<(f32, f32, f32), &'static str> {
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

        Ok((x, y, z))
    }

    pub async fn read_temp(&mut self) -> Result<f32, &'static str> {
        let temp_l = self.read_accel_single_register(0x22).await? as i16;
        let temp_h = self.read_accel_single_register(0x23).await? as i16;

        let mut temp = (temp_l << 3) | (temp_h >> 5);
        if temp > 1023 {
            temp = temp - 2048;
        }
        let temp = temp as f32 * 0.125f32 + 23.0f32;
        Ok(temp as f32)
    }

    pub async fn get_data(&self) -> (f32, f32, f32, f32, f32, f32) {
        (
            self.gyro[0],
            self.gyro[1],
            self.gyro[2],
            self.accel[0],
            self.accel[1],
            self.accel[2],
        )
    }

    pub async fn imu_update(&mut self) {
        info!("Begin to read imu");
        match self.read_accel().await {
            Ok((x, y, z)) => {
                self.accel = [x, y, z];
            }
            Err(e) => {
                error!("Failed to read accelerometer: {}", e);
            }
        }

        match self.read_gyro().await {
            Ok((x, y, z)) => {
                self.gyro = [x, y, z];
            }
            Err(e) => {
                error!("Failed to read gyroscope: {}", e);
            }
        }

        match self.read_temp().await {
            Ok(temp) => {
                self.temp = temp;
            }
            Err(e) => {
                error!("Failed to read temperature: {}", e);
            }
        }
    }
}
