use crate::modules::motors::dji_motors::gm6020::Gm6020;
use defmt::*;

pub struct Gimbal {
    pitch_motor: Gm6020,
    yaw_motor: Gm6020,
}

impl Gimbal {
    pub fn new(pitch_motor: Gm6020, yaw_motor: Gm6020) -> Self {
        Gimbal {
            pitch_motor,
            yaw_motor,
        }
    }

    pub fn id(&self) {
        info!("pitch motor id = {}", self.pitch_motor.id());
        info!("yaw motor id = {}", self.yaw_motor.id());
    }
}

impl Default for Gimbal {
    fn default() -> Self {
        Gimbal::new(Gm6020::new(1).unwrap(), Gm6020::new(2).unwrap())
    }
}
