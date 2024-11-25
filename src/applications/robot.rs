use crate::applications::chassis;

use crate::chassis::ChassisControlMode;

pub struct ControlMode {
    chassis_contorl_mode: ChassisControlMode,
}

impl ControlMode {
    pub async fn chassis_set_mode() {
        defmt::todo!("");
    }
}
