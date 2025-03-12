use crate::modules::motors::dji_motors::rm3508::{Rm3508, Rm3508ReductionRatio};

pub const WARNING_BULLET_SPEED: f32 = 24.8;

struct Shoot {
    left_fric_motor: Rm3508,
    right_fric_motor: Rm3508,
    target_bullet_speed: f32,
    last_seven_bullet_speed: [f32; 7],
}

impl Shoot {
    pub fn new() -> Self {
        Shoot {
            left_fric_motor: Rm3508::new(Rm3508ReductionRatio::Ratio1, 1).unwrap(),
            right_fric_motor: Rm3508::new(Rm3508ReductionRatio::Ratio1, 1).unwrap(),
            target_bullet_speed: 24f32,
            last_seven_bullet_speed: [0f32; 7],
        }
    }

    fn is_bullet_speed_suitable(&self) -> bool {
        if self.last_seven_bullet_speed.iter().last() >= Some(&WARNING_BULLET_SPEED) {
            false
        }
        true
    }
}
