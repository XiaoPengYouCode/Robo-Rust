use defmt::*;

pub struct Gm6020 {
    id: u8,
}

impl Gm6020 {
    pub fn new(id: u8) -> Result<Self, &'static str> {
        info!("Gm6020 init!");
        if id > 31 {
            error!("init Gm6020 Error");
            return Err("init Gm6020 Error");
        }
        Ok(Self { id })
    }

    pub fn id(&self) -> u8 {
        info!("motor id = {}", self.id);
        self.id
    }
}
