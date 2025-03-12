use crate::applications::chassis::Chassis;

#[embassy_executor::task]
pub async fn task_chassis() {
    let mut chassis = Chassis::new();
    chassis.test();
}
