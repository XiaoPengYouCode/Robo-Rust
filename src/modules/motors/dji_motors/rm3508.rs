use defmt::{dbg, error};

// const CAN_ID_IDENFITY_1: u16 = 0x200;
// const CAN_ID_IDENFITY_2: u16 = 0x1ff;

pub enum ReductionGearboxTransmissionRatio {
    Ratio19,
    Ratio168_17,
    Ratio14,
}

impl ReductionGearboxTransmissionRatio {
    pub fn to_string(&self) -> &str {
        match self {
            ReductionGearboxTransmissionRatio::Ratio19 => "19:1",
            ReductionGearboxTransmissionRatio::Ratio168_17 => "168.17:1",
            ReductionGearboxTransmissionRatio::Ratio14 => "14:1",
        }
    }
}

pub struct Rm3508 {
    pub gear_ratio: ReductionGearboxTransmissionRatio,
    id: u8,
}

impl Rm3508 {
    pub fn new(
        gear_ratio: ReductionGearboxTransmissionRatio,
        id: u8,
    ) -> Result<Self, &'static str> {
        dbg!("Rm3508 init!");
        if id > 31 {
            error!("init Rm3508 Error");
            return Err("init Rm3508 Error");
        }
        Ok(Self { gear_ratio, id })
    }

    pub fn id(&self) -> u8 {
        dbg!("motor id = {}", self.id);
        self.id
    }

    pub fn gear_ratio(&self) -> &ReductionGearboxTransmissionRatio {
        dbg!("gear ratio = {}", self.gear_ratio.to_string());
        &self.gear_ratio
    }
}
