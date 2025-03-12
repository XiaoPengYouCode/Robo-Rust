use crate::applications::gimbal::Gimbal;
use crate::modules::motors::dji_motors::gm6020::Gm6020;

#[embassy_executor::task]
pub async fn task_gimbal() {
    let gimbal = Gimbal::new(Gm6020::new(1).unwrap(), Gm6020::new(2).unwrap());
    gimbal.id();
}
