use defmt::*;

pub struct Gm6020 {
    pub can_id: u8,

}

impl Gm6020 {
    pub async fn init(&mut self, id: u8) {
        info!("Gm6020 init!");
        self.can_id = id;
    }
}