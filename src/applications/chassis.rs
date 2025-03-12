use crate::modules::motors::dji_motors::rm3508::{Rm3508ReductionRatio, Rm3508};

pub struct Chassis {
    chassis_motors: [Rm3508; 4],
}

impl Default for Chassis {
    fn default() -> Self {
        Chassis::new()
    }
}

impl Chassis {
    pub fn new() -> Self {
        let motor1 = Rm3508::new(Rm3508ReductionRatio::Ratio14, 1).unwrap();
        let motor2 = Rm3508::new(Rm3508ReductionRatio::Ratio14, 2).unwrap();
        let motor3 = Rm3508::new(Rm3508ReductionRatio::Ratio14, 3).unwrap();
        let motor4 = Rm3508::new(Rm3508ReductionRatio::Ratio14, 4).unwrap();
        Self {
            chassis_motors: [motor1, motor2, motor3, motor4],
        }
    }

    pub fn test(&mut self) {
        for motor in self.chassis_motors.iter() {
            motor.id();
            motor.gear_ratio();
        }
    }
}
