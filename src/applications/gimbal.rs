use defmt::{debug, info, todo};

use crate::modules::imu::Bmi088;
use crate::modules::motors::dji_motors::gm6020::Gm6020;

struct ClassicGimbalHandle {
    yaw_motor: Gm6020,
    pitch_motor: Gm6020,
    pitch_imu: Bmi088,
}

struct YPYDoubleYawGimbalHandle {
    yaw_motor: Gm6020,
    pitch_motor: Gm6020,
    small_yaw_motor: Gm6020,
    small_yaw_imu: Bmi088,
}

pub enum Gimbal {
    ClassicGimbal(ClassicGimbalHandle),
    YPYDoubleYawGimbal(YPYDoubleYawGimbalHandle),
    // YYPDoubleYawGimbal {
    //     yaw_motor: Gm6020,
    //     small_yaw_motor: Gm6020,
    //     pitch_motor: Gm6020,
    //     pitch_imu: Bmi088,
    // },
    // // --↑--
    // // b ↑ a
    // // --↑--
    // DoubleClassisGimbal {
    //     yaw_motor: Gm6020,
    //     gimbal_1: ClassicGimbalHandle,
    //     gimbal_2: ClassicGimbalHandle,
    // },
}

impl Gimbal {
    pub fn yaw_rotate(&self, rotate_angle: f32) {
        if let ClassicGimbalHandle { yaw_motor, .. } = self {
            info!("Yaw motor id: {}", yaw_motor.can_id);
            info!("Yaw motor rotate angle: {}", rotate_angle);
            todo!("rotate");
        }
    }
}
