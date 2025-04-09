use crate::bored::bored_resources::ImuSpi2Resources;
use crate::bsc::spi2_dma_bmi088::{Bmi088Error, Spi2Handler};
use crate::modules::algorithms::eskf_imu::ESKF;

pub mod bmi088_registers;

#[derive(Debug)]
struct RawData {
    accel: [f32; 3],
    gyro: [f32; 3],
    temp: f32,
}

#[derive(Debug)]
pub struct ImuData {
    raw_data: RawData,
    euler_angle: [f32; 3],
}

impl ImuData {
    pub fn zeros() -> Self {
        Self {
            raw_data: RawData {
                accel: [0f32; 3],
                gyro: [0f32; 3],
                temp: 0f32,
            },
            euler_angle: [0f32; 3],
        }
    }

    pub fn update_raw(&mut self, accel: [f32; 4], gyro: [f32; 3]) {
        self.raw_data.accel = [
            accel[0] * 0.9 + 0.1 * self.raw_data.accel[0],
            accel[1] * 0.9 + 0.1 * self.raw_data.accel[1],
            accel[2] * 0.9 + 0.1 * self.raw_data.accel[2],
        ];
        self.raw_data.gyro = [
            gyro[0] * 0.9 + 0.1 * self.raw_data.gyro[0],
            gyro[1] * 0.9 + 0.1 * self.raw_data.gyro[1],
            gyro[2] * 0.9 + 0.1 * self.raw_data.gyro[2],
        ];
        self.raw_data.temp = accel[3] * 0.9 + 0.1 * self.raw_data.temp;
    }

    pub fn update_euler(&mut self, euler_angle: [f32; 3]) {
        self.euler_angle = euler_angle;
    }
}

impl Default for ImuData {
    fn default() -> Self {
        ImuData::zeros()
    }
}

pub struct Imu {
    bmi088: Spi2Handler,
    pub eskf: ESKF,
    data: ImuData,
}

impl Imu {
    pub fn new(re: ImuSpi2Resources) -> Self {
        Self {
            bmi088: Spi2Handler::new(re),
            eskf: ESKF::init(),
            data: ImuData::zeros(),
        }
    }

    pub async fn raw_data_update(&mut self) -> Result<(), Bmi088Error> {
        self.data.update_raw(
            self.bmi088.read_accel_and_temp().await.unwrap(),
            self.bmi088.read_gyro().await.unwrap(),
        );
        Ok(())
    }

    pub async fn init(&mut self) -> Result<(), Bmi088Error> {
        self.bmi088.imu_bmi088_init().await
    }

    pub fn eskf_predict(&mut self) {
        self.eskf
            .predict(self.get_gyro().into(), self.get_accel().into());
    }

    pub fn eskf_update(&mut self) {
        self.eskf.update(self.get_accel().into());
        self.data.update_euler(self.eskf.get_euler_angles_d());
    }

    #[inline(always)]
    pub fn get_euler_angles_d(&self) -> [f32; 3] {
        self.data.euler_angle
    }

    #[inline(always)]
    pub fn get_accel(&self) -> [f32; 3] {
        self.data.raw_data.accel
    }

    #[inline(always)]
    pub fn get_gyro(&self) -> [f32; 3] {
        self.data.raw_data.gyro
    }

    #[inline(always)]
    pub fn get_temp(&self) -> f32 {
        self.data.raw_data.temp
    }
}
