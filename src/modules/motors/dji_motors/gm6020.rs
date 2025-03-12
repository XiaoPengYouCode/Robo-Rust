use defmt::*;

pub struct Gm6020 {
    id: u8,
    frame: [u8; 8],
}

impl Gm6020 {
    pub fn new(id: u8) -> Result<Self, &'static str> {
        info!("Gm6020 init!");
        if id > 31 {
            error!("init Gm6020 Error");
            return Err("init Gm6020 Error");
        }
        Ok(Self {
            id,
            frame: [0u8; 8],
        })
    }

    pub fn id(&self) -> u8 {
        info!("motor id = {}", self.id);
        self.id
    }

    pub fn angle_control(&mut self) {
        // 计算对应的数据帧

        // 修改数据帧
        self.frame[0] = 0;
    }

    pub fn speed_control(&mut self) {
        // 计算对应的数据帧

        // 修改数据帧
        self.frame[0] = 0;
    }

    pub fn force_control(&mut self) {
        // 计算对应的数据帧

        // 修改数据帧
        self.frame[0] = 0;
    }
}
