use defmt::*;

pub struct Rm6020 {
    pub can_id: u8,

}

impl Rm6020 {
    pub async fn init(&mut self, id: u8) {
        info!("Rm2006 init!");
        self.can_id = id;
    }
}