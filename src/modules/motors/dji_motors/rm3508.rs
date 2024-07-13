use defmt::*;

pub enum ReductionGearboxTransmissionRatio {
    Ratio19,
    Ratio168_17,
}

pub struct Rm3508 {
    can_id: u16,
    _can_id_full: u16,
    _gear_ratio: ReductionGearboxTransmissionRatio,
    can_message: [u8; 8],
}

const RM3508_KP: f32 = 0.5;

impl Rm3508 {
    pub async fn new(id: u16, _gear_ratio: ReductionGearboxTransmissionRatio) -> Self {
        info!("Rm3508 init!");
        Self {
            can_id: id,
            _can_id_full: 0x200,
            _gear_ratio,
            can_message: [1, 64, 0, 0, 0, 0, 0, 0],
        }
    }

    pub async fn get_id(&self) -> u16 {
        // info!("motor id = {}", self.can_id);
        self.can_id
    }

    pub async fn get_can_message(&self) -> &[u8; 8] {
        // info!("Get can message = {:?}", self.can_message);
        &self.can_message
    }

    pub async fn speed_control(&mut self, target_speed_rpm: i16, current_speed_rpm: i16, mut current_current: i16) {
        if current_speed_rpm <= 0 {
            current_current += 10;
        } else if current_speed_rpm < target_speed_rpm {
            current_current += ((target_speed_rpm - current_speed_rpm) as f32 * RM3508_KP) as i16; 
        } else if current_speed_rpm > target_speed_rpm {
            current_current -= ((current_speed_rpm - target_speed_rpm) as f32 * RM3508_KP) as i16; 
        }

        info!("{}", (target_speed_rpm - current_speed_rpm) as f32);

        self.can_message[0] = (current_current >> 8) as u8;
        self.can_message[1] = current_current as u8;
    }

    pub async fn protect(&mut self) {
        self.can_message = [0; 8];
        info!("protected!");
    }
}
