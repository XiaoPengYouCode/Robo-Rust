pub const BMI088_GYRO_2000_SEN: f32 = 0.00106526443603169529841533860381f32;
pub const BMI088_ACCEL_3G_SEN: f32 = 0.00179443359375f32;

pub const ACCEL_RESET_REGISTER: u8 = 0x7Eu8;
pub const ACCEL_RESET_MESSAGE: u8 = 0xB6u8;

pub const GYRO_RESET_REGISTER: u8 = 0x14u8;
pub const GYRO_RESET_MESSAGE: u8 = 0xB6u8;

pub const ACC_PWR_CTRL_ADDR: u8 = 0x7Du8;
pub const ACC_PWR_CTRL_ON: u8 = 0x04u8;

pub const ACC_PWR_CONF_ADDR: u8 = 0x7Cu8;
pub const ACC_PWR_CONF_ACT: u8 = 0x00u8;

pub const BMI088_SPI_WRITE_CODE: u8 = 0x7Fu8;
pub const BMI088_SPI_READ_CODE: u8 = 0x80u8;
