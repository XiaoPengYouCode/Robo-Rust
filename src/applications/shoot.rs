use crate::modules::motors::dji_motors::rm3508::Rm3508;

struct Shoot {
    left_fric_motor: Rm3508,
    right_fric_motor: Rm3508,
    target_bullet_speed: f32,
    last_seven_bullet_speed: [f32; 5],
}

impl Shoot {
    // pub async fn new()
}
