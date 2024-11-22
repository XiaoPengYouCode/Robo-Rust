use defmt::*;

enum LkMotorsControlState {
    SpeedControl { speed: i32 },
    AngleControl { angle: i32 },
}

pub struct LkMotors {
    can_message: [u8; 8],
    can_id: u8,
    state: LkMotorsControlState,
}

impl LkMotors {
    pub async fn new(can_message: u64, can_id: u8) -> Self {
        info!("LkMotors 初始化中...");
        if can_id > 32 {
            error!("CAN ID 必须小于 32");
        }
        let can_message = can_message.to_be_bytes();
        let can_instance = Self {
            can_message,
            can_id,
            state: LkMotorsControlState::SpeedControl { speed: 0 },
        };
        info!("LkMotors 初始化成功!");
        can_instance
    }

    pub async fn speed_control(&mut self, speed: i32) {
        if let LkMotorsControlState::SpeedControl {
            speed: current_speed,
        } = &self.state
        {
            if speed == *current_speed {
                return;
            }
        }
        self.state = LkMotorsControlState::SpeedControl { speed };
        self.update_can_message(0xA2, speed);
    }

    pub fn angle_control(&mut self, set_angle: i32) {
        if let LkMotorsControlState::AngleControl { angle } = &self.state {
            if set_angle == *angle {
                return;
            }
        }
        self.state = LkMotorsControlState::AngleControl { angle: set_angle };
        self.update_can_message(0xA3, set_angle);
    }

    fn update_can_message(&mut self, command: u8, value: i32) {
        self.can_message[0] = command;
        self.can_message[1] = 0x00;
        self.can_message[2] = 0x00;
        self.can_message[3] = 0x00;
        self.can_message[4] = value as u8;
        self.can_message[5] = (value >> 8) as u8;
        self.can_message[6] = (value >> 16) as u8;
        self.can_message[7] = (value >> 24) as u8;
    }

    pub async fn get_id(&self) -> u8 {
        self.can_id
    }

    pub async fn get_can_message(&self) -> &[u8; 8] {
        &self.can_message
    }
}
