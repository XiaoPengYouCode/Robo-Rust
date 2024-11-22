use defmt::*;

pub struct Rm2006 {
    id: u8,
}

impl Rm2006 {
    pub fn new(id: u8) -> Result<Self, &'static str> {
        info!("Rm2006 init!");
        if id > 31 {
            error!("init Rm2006 Error");
            return Err("init Rm2006 Error");
        }
        Ok(Self { id })
    }

    pub fn id(&self) -> u8 {
        info!("motor id = {}", self.id);
        self.id
    }
}
