use defmt::{dbg, error};

pub struct RobostrideMotors {
    id: u8,
}

impl RobostrideMotors {
    pub fn new(id: u8) -> Result<Self, &'static str> {
        dbg!("RobostrideMotors init!");
        if id > 31 {
            error!("init RobostrideMotors Error");
            return Err("init RobostrideMotors Error");
        }
        Ok(Self { id })
    }

    pub fn id(&self) -> u8 {
        dbg!("motor id = {}", self.id);
        self.id
    }
}
