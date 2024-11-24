#[allow(unused_imports)]
use crate::bored_resources::ImuResources;
use defmt::{debug, info};
use embassy_stm32::gpio::Pin;
use embassy_stm32::spi::Config;
use embassy_stm32::time::Hertz;
use embassy_time::Duration;
use embassy_time::Timer;

use crate::bsp::spi_dma::*;

const BMI088_GYRO_2000_SEN: f32 = 0.00106526443603169529841533860381f32;
const BMI088_ACCEL_3G_SEN: f32 = 0.0008974358974f32;

const BMI088_COM_WAIT_SENSOR_TIME: u8 = 150;

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

    pub async fn bmi088_accel_init(&mut self) -> Result<(), &'static str> {
        info!("Initialize BMI088 Accelerometer");

        //check communication with BMI088
        let _accel_id = self.read_accel_single_register(0x00).await?;
        Timer::after(Duration::from_millis(BMI088_COM_WAIT_SENSOR_TIME as u64)).await;
        let _accel_id: u8 = self.read_accel_single_register(0x00).await?;
        Timer::after(Duration::from_millis(BMI088_COM_WAIT_SENSOR_TIME as u64)).await;

        self.write_accel_single_register(&ACCEL_RESET_REGISTER, ACCEL_RESET_MESSAGE).await?;
        Timer::after(Duration::from_millis(80)).await;

        let _accel_id = self.read_accel_single_register(0x00).await?;
        Timer::after(Duration::from_millis(BMI088_COM_WAIT_SENSOR_TIME as u64)).await;
        let accel_id: u8 = self.read_accel_single_register(0x00).await?;
        Timer::after(Duration::from_millis(BMI088_COM_WAIT_SENSOR_TIME as u64)).await;

        info!("accel_id: {:X}", accel_id);
        if accel_id!= 0x1E {
            return Err("Invalid accelerometer chip ID");
        }

        self.write_accel_single_register(&ACC_PWR_CTRL_ADDR, ACC_PWR_CTRL_ON).await?;
        Timer::after(Duration::from_millis(100)).await;
        Ok(())
    }

    pub async fn bmi088_gyro_init(&mut self) -> Result<(), &'static str> {
        info!("Initialize BMI088 Gyroscope");

        //check communication with BMI088
        let _gyro_id = self.read_gyro_single_register(0x00).await?;
        let _gyro_id: u8 = self.read_gyro_single_register(0x00).await?;

        self.write_gyro_single_register(&GYRO_RESET_REGISTER, GYRO_RESET_MESSAGE).await?;
        Timer::after(Duration::from_millis(100)).await;

        let _gyro_id = self.read_gyro_single_register(0x00).await?;
        let gyro_id: u8 = self.read_gyro_single_register(0x00).await?;

        info!("gyro_id: {:X}", gyro_id);

        if gyro_id!= 0x0F {
            return Err("Invalid gyroscope chip ID");
        }
        Ok(())
    }

    pub async fn bmi088_init(&mut self) -> Result<(), &'static str> {
        info!("Initialize BMI088");

        self.bmi088_accel_init().await?;
        self.bmi088_gyro_init().await?;

        Timer::after(Duration::from_millis(50)).await;

        Ok(())
    }

    async fn read_accel_single_register(&mut self, read_reg: u8) -> Result<u8, &'static str> {
        self.spi_perh.cs[0].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // bmi088 accel register needs to be read twice to get the correct message
        let mut buffer = [0u8];
        let read_message = read_reg | 0x80;

        // For read operations, set the MSB of the register address
        self.spi_perh
                .spi
                .write(&[read_message])
                .await
                .map_err(|_| "SPI write failed").unwrap();

        self.spi_perh
            .spi
            .read(&mut buffer)
            .await
            .map_err(|_| "SPI read failed").unwrap();

        // debug!("spi_read_once: {}", &buffer[0]);

        self.spi_perh
            .spi
            .read(&mut buffer)
            .await
            .map_err(|_| "SPI read failed").unwrap();

        // debug!("spi_read_twice: {}", &buffer[0]);

        self.spi_perh.cs[0].set_high();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        Ok(buffer[0])
    }

    async fn read_gyro_single_register(&mut self, read_reg: u8) -> Result<u8, &'static str> {
        let mut buffer = [0u8];
        self.spi_perh.cs[1].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        let read_message = read_reg | 0x80;

        self.spi_perh
            .spi
            .write(&[read_message])
            .await
            .map_err(|_| "SPI write failed").unwrap();

        self.spi_perh
            .spi
            .read(&mut buffer)
            .await
            .map_err(|_| "SPI read failed").unwrap();

        // debug!("spi_read_once: {}", &buffer[0]);

        self.spi_perh.cs[1].set_high();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        Ok(buffer[0])
    }

    async fn write_accel_single_register(
        &mut self,
        write_register: &u8,
        write_message: u8,
    ) -> Result<(), &'static str> {
        self.spi_perh.cs[0].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        self.spi_perh
            .spi
            .write(&[write_register & 0x7Fu8])
            .await
            .map_err(|_| "SPI write failed")?;

        self.spi_perh
            .spi
            .write(&[write_message])
            .await
            .map_err(|_| "SPI write failed")?;

        Timer::after(Duration::from_micros(1)).await; // Small delay
        self.spi_perh.cs[0].set_high();

        Ok(())
    }

    async fn write_gyro_single_register(
        &mut self,
        write_register: &u8,
        write_message: u8,
    ) -> Result<(), &'static str> {
        self.spi_perh.cs[1].set_low();
        Timer::after(Duration::from_micros(1)).await; // Small delay

        // For read operations, set the MSB of the register address
        self.spi_perh
            .spi
            .write(&[write_register & 0x7Fu8])
            .await
            .map_err(|_| "SPI write failed")?;

        self.spi_perh
            .spi
            .write(&[write_message])
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
        debug!("send buffer in binary format = {:#b}", send_buffer);

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
            debug!("received data after first read = {:#b}", &receive[0]);
            self.spi_perh
                .spi
                .read(&mut receive)
                .await
                .map_err(|_| "->  twice read error")?;
            receive_buffer[i] = receive[0];
            debug!("received data after second read = {:#b}", &receive_buffer[i]);
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
        Ok(temp)
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

    pub async fn imu_update(&mut self) -> Result<(), &'static str> {
        info!("Begin to read imu");
        // match self.read_accel().await {
        //     Ok((x, y, z)) => {
        //         self.accel = [x, y, z];
        //     }
        //     Err(e) => {
        //         error!("Failed to read accelerometer: {}", e);
        //     }
        // }

        // match self.read_gyro().await {
        //     Ok((x, y, z)) => {
        //         self.gyro = [x, y, z];
        //     }
        //     Err(e) => {
        //         error!("Failed to read gyroscope: {}", e);
        //     }
        // }

        let temp = self.read_temp().await?;
        self.temp = temp;
        info!("Temp: {}", temp);

        // debug!("Update imu data");

        Ok(())
    }
}
