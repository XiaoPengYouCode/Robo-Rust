// use crate::modules::algorithms::pid::*;
use defmt::*;

// DATA[0] 命令字节 0x9C
// DATA[1] 电机温度 DATA[1] = *(uint8_t *)(&temperature)
// DATA[2] 转矩电流低字节 DATA[2] = *(uint8_t *)(&iq)
// DATA[3] 转矩电流高字节 DATA[3] = *((uint8_t *)(&iq)+1)
// DATA[4] 电机速度低字节 DATA[4] = *(uint8_t *)(&speed)
// DATA[5] 电机速度高字节 DATA[5] = *((uint8_t *)(&speed)+1)
// DATA[6] 编码器位置低字节 DATA[6] = *(uint8_t *)(&encoder)
// DATA[7] 编码器位置高字节 DATA[7] = *((uint8_t *)(&encoder)+1)

pub struct LkMotorsState {
    _command: u8,
    temp: u8,
    teque: u16,
    speed: u16,
    encoder: u16,
}

impl LkMotorsState {
    pub fn from_raw(&mut self, value: [u8; 8]) -> Self {
        let command = value[0];
        let temp = value[1];
        let teque = ((value[2] as u16) << 8) + (value[3] as u16);
        let speed = ((value[4] as u16) << 8) + (value[5] as u16);
        let encoder = ((value[6] as u16) << 8) + (value[7] as u16);

        LkMotorsState {
            _command: command,
            temp,
            teque,
            speed,
            encoder,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LkMotorsCtrlMode {
    // SpeedControl { speed: i32, speed_pid: Pid },
    // AngleControl { angle: i32, angle_pid: Pid },
    IqCtrl,
}

pub struct LkMotors {
    can_message: [u8; 8],
    pub state: LkMotorsState,
    can_id: u8,
    ctrl_mode: LkMotorsCtrlMode,
}

const IQ_CTRL_COMMAND: u8 = 0xA1;

impl LkMotors {
    pub fn new(can_id: u8) -> Self {
        debug!("LkMotors 初始化中...");
        if can_id > 32 {
            error!("CAN ID 必须小于 32");
        }
        let can_message = 0u64.to_be_bytes();
        let _motor_state = 0u64.to_be_bytes();
        let can_instance = Self {
            can_message,
            state: LkMotorsState {
                _command: 0x9c,
                temp: 0,
                teque: 0,
                speed: 0,
                encoder: 0,
            },
            can_id,
            ctrl_mode: LkMotorsCtrlMode::IqCtrl,
        };
        debug!("LkMotors 初始化成功!");
        can_instance
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

    pub fn get_id(&self) -> u8 {
        self.can_id
    }

    pub fn get_can_message(&self) -> &[u8; 8] {
        &self.can_message
    }

    pub fn get_ctrl_mode(&self) -> &LkMotorsCtrlMode {
        &self.ctrl_mode
    }

    pub fn get_speed(&self) -> u16 {
        self.state.speed
    }
    pub fn get_teque(&self) -> u16 {
        self.state.teque
    }
    pub fn get_temp(&self) -> u8 {
        self.state.temp
    }
    pub fn get_encoder(&self) -> u16 {
        self.state.encoder
    }

    pub async fn iq_ctrl(&mut self, target_iq: i32) {
        if *self.get_ctrl_mode() == LkMotorsCtrlMode::IqCtrl {
            self.update_can_message(IQ_CTRL_COMMAND, target_iq);
        } else {
            error!("current control mode isn't IqCtrl");
        }
    }
}
