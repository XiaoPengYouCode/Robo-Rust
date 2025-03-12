use defmt::{debug, error};

pub enum Rm3508ReductionRatio {
    Ratio19,
    Ratio168_17,
    Ratio14,
    Ratio1,
}

impl Rm3508ReductionRatio {
    fn to_string(&self) -> &str {
        match self {
            Rm3508ReductionRatio::Ratio19 => "19:1",
            Rm3508ReductionRatio::Ratio168_17 => "168.17:1",
            Rm3508ReductionRatio::Ratio14 => "14:1",
            Rm3508ReductionRatio::Ratio1 => "With out reductor",
        }
    }
}

pub struct Rm3508 {
    id: u8,
    frame: [u8; 8],
    gear_ratio: Rm3508ReductionRatio,
}

impl Rm3508 {
    pub fn new(reduction_ratio: Rm3508ReductionRatio, id: u8) -> Result<Self, &'static str> {
        debug!("Rm3508 init!");
        if id > 31 {
            error!("init Rm3508 Error");
            return Err("init Rm3508 Error");
        }
        let gear_ratio = reduction_ratio;
        Ok(Self {
            id,
            frame: [0u8; 8],
            gear_ratio,
        })
    }

    pub fn id(&self) -> u8 {
        debug!("motor id = {}", self.id);
        self.id
    }

    pub fn gear_ratio(&self) -> &Rm3508ReductionRatio {
        debug!("gear ratio = {}", self.gear_ratio.to_string());
        &self.gear_ratio
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
