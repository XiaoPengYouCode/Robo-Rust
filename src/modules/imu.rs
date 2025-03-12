use na::Vector3;

use crate::bored::bored_resources::ImuSpiResources;
use crate::bsc::spi2_dma_bmi088::{Bmi088, Bmi088Error};
use crate::modules::algorithms::ekf_imu::ESKF;

pub struct ImuData {
    accel: [f32; 3],
    gyro: [f32; 3],
    temp: f32,
    euler_angle: [f32; 3],
}

impl ImuData {
    pub fn new() -> Self {
        Self {
            accel: [0f32; 3],
            gyro: [0f32; 3],
            temp: 0f32,
            euler_angle: [0f32; 3],
        }
    }

    pub fn set(&mut self, accel: [f32; 3], gyro: [f32; 3], temp: f32) {
        self.accel = accel;
        self.euler_angle = gyro;
        self.temp = temp;
    }
}

impl Default for ImuData {
    fn default() -> Self {
        ImuData::new()
    }
}

pub struct Imu {
    bmi088: Bmi088,
    eskf: ESKF,
    data: ImuData,
}

impl Imu {
    pub fn new(re: ImuSpiResources) -> Self {
        Self {
            bmi088: Bmi088::new(re),
            eskf: ESKF::new(),
            data: ImuData::new(),
        }
    }

    pub async fn data_update(&mut self) -> Result<(), Bmi088Error> {
        self.bmi088.update(&mut self.data).await.unwrap();
        Ok(())
    }

    pub async fn init(&mut self) -> Result<(), Bmi088Error> {
        self.bmi088.imu_bmi088_init().await
    }

    pub fn predict(&mut self, gyro: Vector3<f32>, dt: f32) {
        self.eskf.predict(gyro, dt);
    }

    pub fn eskf_update(&mut self, accel: Vector3<f32>) {
        self.eskf.update(accel);
    }

    pub fn get_euler_angles_degrees(&self) -> [f32; 3] {
        self.data.euler_angle
    }

    #[inline(always)]
    pub fn accel(&self) -> [f32; 3] {
        self.data.accel
    }

    #[inline(always)]
    pub fn gyro(&self) -> [f32; 3] {
        self.data.gyro
    }

    #[inline(always)]
    pub fn temp(&self) -> f32 {
        self.data.temp
    }
}
