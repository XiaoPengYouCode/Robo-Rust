pub const BMI088_GYRO_2000_SEN: f32 = 0.001_065_264_5_f32;
pub const BMI088_ACCEL_3G_SEN: f32 = 0.001_794_433_6_f32;

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

pub const ACC_CONF_ADDR: u8 = 0x40u8;

// pub const ACC_CONF_BWP_OSR4: u8 = 0x08;
// pub const ACC_CONF_BWP_OSR2: u8 = 0x09;
pub const ACC_CONF_BWP_NORM: u8 = 0x0A;

// pub const ACC_CONF_ODR_12_5_HZ: u8 = 0x05;
// pub const ACC_CONF_ODR_25_HZ: u8 = 0x06;
// pub const ACC_CONF_ODR_50_HZ: u8 = 0x07;
// pub const ACC_CONF_ODR_100_HZ: u8 = 0x08;
// pub const ACC_CONF_ODR_200_HZ: u8 = 0x09;
// pub const ACC_CONF_ODR_400_HZ: u8 = 0x0A;
// pub const ACC_CONF_ODR_800_HZ: u8 = 0x0B;
pub const ACC_CONF_ODR_1600_HZ: u8 = 0x0C;

pub const GYRO_RANGE_ADDR: u8 = 0x0F;
// pub const GYRO_RANGE_2000_DEG_S: u8 = 0x00;
// pub const GYRO_RANGE_1000_DEG_S: u8 = 0x01;
pub const GYRO_RANGE_500_DEG_S: u8 = 0x02;
// pub const GYRO_RANGE_250_DEG_S: u8 = 0x03;
// pub const GYRO_RANGE_125_DEG_S: u8 = 0x04;

pub const GYRO_BANDWIDTH_ADDR: u8 = 0x10;
pub const GYRO_ODR_2000HZ_BANDWIDTH_532HZ: u8 = 0x00;
// pub const GYRO_ODR_2000HZ_BANDWIDTH_230HZ: u8 = 0x01;
// pub const GYRO_ODR_1000HZ_BANDWIDTH_116HZ: u8 = 0x02;
// pub const GYRO_ODR_400HZ_BANDWIDTH_47HZ: u8 = 0x03;
// pub const GYRO_ODR_200HZ_BANDWIDTH_23HZ: u8 = 0x04;
// pub const GYRO_ODR_100HZ_BANDWIDTH_12HZ: u8 = 0x05;
// pub const GYRO_ODR_200HZ_BANDWIDTH_64HZ: u8 = 0x06;
// pub const GYRO_ODR_100HZ_BANDWIDTH_32HZ: u8 = 0x07;
