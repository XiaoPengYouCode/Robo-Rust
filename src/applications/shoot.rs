#![allow(unused)]

// use crate::modules::motors::dji_motors::rm3508::{Rm3508, Rm3508ReductionRatio};
use crate::bored::bored_resources::FdCan1Resources;
use crate::modules::motors::lktech_motors::*;

pub const WARNING_BULLET_SPEED: f32 = 24.8;

#[derive(PartialEq)]
pub enum ShootMode {
    Single,
    Continuous,
    Stop,
}

pub struct Shoot {
    stir_motor: LkMotors,
    shoot_mode: ShootMode,
    block_time: u16,
    block_flag: u8,
    reversal_cnt: u8,
}

impl Default for Shoot {
    fn default() -> Self {
        Shoot::new()
    }
}

impl Shoot {
    pub fn new() -> Self {
        Self {
            stir_motor: LkMotors::new(0x01),
            shoot_mode: ShootMode::Stop,
            block_time: 0,
            block_flag: 0,
            reversal_cnt: 0,
        }
    }

    pub fn estimate(&mut self, lk_state: [u8; 8]) {
        self.stir_motor.state.from_raw(lk_state);
        // 将拨盘机械角度转化成360度制
        // shootControl.ShootEstimate.stir_position_d =
        // AngleLimit((float)_stirMotorRec->encoder / 65536.0 * 360.0, -180, 180);

        // 检测堵转
        if self.stir_motor.get_speed() < 1 && self.stir_motor.get_teque() > 1500 {
            self.block_time += 1;
        } else {
            self.block_time = 0;
            self.block_flag = 0;
        }

        if self.block_time > 250 {
            self.block_flag = 1;
        }

        // 判断退出反转
        if self.reversal_cnt > 50 {
            self.block_flag = 0;
            self.block_time = 0;
            self.reversal_cnt = 0;
        }
    }

    pub fn input(&mut self) {
        // 拨盘
        if self.shoot_mode == ShootMode::Continuous {
            // 退出单发后，刷新固定角度的计算
            // shootControl.ShootTargetInput.calculated_position_flag = 0;
            //     if (STIR_SPEED_CONTROL == _robotState->stir_mode &&
            //     shootControl.ShootEstimate.heat_flag &&
            //     fabs(shootControl.ShootTargetInput.fric_speed_rpm[LEFT] - _fricMotorRec[LEFT].mechanical_speed_rpm) < 50)
            //                 {
            // if (shootControl.ShootEstimate.blocked_flag == 0)
            // {
            //     shootControl.ShootTargetInput.stir_speed_rps = -1.8;
            //     shootControl.ShootTargetInput.stir_position_d = shootControl.ShootEstimate.stir_position_d;
            // }
            // else
            // {
            //     shootControl.ShootTargetInput.stir_speed_rps = 1;
            //     shootControl.ShootTargetInput.stir_position_d = shootControl.ShootEstimate.stir_position_d;
            //     shootControl.ShootTargetInput.reversal_cnt++; // 恢复反转计数器
            // }
            // 	}
            // 	else if (STIR_OFF == _robotState->stir_mode) // 拨盘停转
            // 	{
            //         shootControl.ShootTargetInput.stir_speed_rps = 0;
            // 		shootControl.ShootTargetInput.stir_position_d = shootControl.ShootEstimate.stir_position_d;
            // 	}
        } else { // 单发 
            //     shootControl.ShootTargetInput.stir_speed_rps = 0;
            // 	// 保证进入单发时，只计算一次到达固定角度位置
            // 	if (fmod(shootControl.ShootEstimate.stir_position_d, STIR_STEP) != 0 &&
            //     shootControl.ShootTargetInput.calculated_position_flag == 0)
            // 	{
            //         shootControl.ShootTargetInput.stir_position_d =
            //         -fmod(shootControl.ShootEstimate.stir_position_d, STIR_STEP) +
            //         shootControl.ShootEstimate.stir_position_d;
            // 		shootControl.ShootTargetInput.stir_position_d =
            //         AngleLimit(shootControl.ShootTargetInput.stir_position_d - SINGLE_SHOOT_POSITION, -180, 180);
            // 		shootControl.ShootTargetInput.calculated_position_flag = 1;
            // 	}

            // 	if (_robotState->stir_mode == STIR_ANGLE_CONTROL &&
            // 		shootControl.ShootTargetInput.single_shoot_reset_flag &&
            // 		shootControl.ShootTargetInput.calculated_position_flag &&
            // 		shootControl.ShootEstimate.heat_flag)
            // 	{
            // 		shootControl.ShootTargetInput.stir_position_d -= STIR_STEP;
            // 		shootControl.ShootTargetInput.stir_position_d =
            // 			AngleLimit(shootControl.ShootTargetInput.stir_position_d, -180, 180);
            //             shootControl.ShootTargetInput.single_shoot_reset_flag = 0;
            // 		shootControl.ShootTargetInput.single_shoot_position_cnt = 0;
        }
    }

    pub fn ctrl(&mut self) {
        // 拨盘
        // shootControl.ShootMotorControl.stir_angle_target_output =
        // PIDUpdate(&shootControl.ShootMotorControl.stir_angle_control_pid,
        //     AngleLimit(shootControl.ShootTargetInput.stir_position_d - shootControl.ShootEstimate.stir_position_d, -180, 180));
        //     shootControl.ShootMotorControl.stir_angle_target_output += shootControl.ShootTargetInput.stir_speed_rps;
        //     shootControl.ShootMotorControl.stir_speed_target_output =
        //     PIDUpdate(&shootControl.ShootMotorControl.stir_speed_control_pid,
        //         shootControl.ShootMotorControl.stir_angle_target_output - _stirMotorRec->speed_rps);

        //         // 保护模式下拨盘和摩擦轮的电流值为0
        //         if (CONTROL_STOP == _robotState->ctrl_terminal)
        //         {
        //             shootControl.ShootMotorControl.stir_speed_target_output = 0;
        //             shootControl.ShootMotorControl.fric_target_output[LEFT] = 0;
        //     shootControl.ShootMotorControl.fric_target_output[RIGHT] = 0;
        // }
    }

    // 针对掉电的LK电机进行重启
    pub fn motors_restart() {}
}
