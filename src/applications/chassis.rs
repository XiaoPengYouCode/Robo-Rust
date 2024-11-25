// use defmt::{info, todo};

// use crate::bsp;
// use crate::modules::motors::dji_motors::{gm6020::Gm6020, rm3508::Rm3508};
const MOTOR_DISTANCE_TO_CENTER: f32 = 1.1;
const CHASSIS_WZ_SET_SCALE: f32 = 1.1;
const STEER_ANGLES_SAVE: (f32, f32, f32, f32) = (45.0, 135.0, 45.0, 135.0);

// chassis direction
// b ↑ a
// --↑--
// c ↑ d

// pub struct chassis_move_t
// {
//   const RC_ctrl_t *chassis_RC;               //����ʹ�õ�ң����ָ��, the point to remote control
//   const gimbal_motor_t *chassis_yaw_motor;   //will use the relative angle of yaw gimbal motor to calculate the euler angle.����ʹ�õ�yaw��̨�������ԽǶ���������̵�ŷ����.
//   const gimbal_motor_t *chassis_pitch_motor; //will use the relative angle of pitch gimbal motor to calculate the euler angle.����ʹ�õ�pitch��̨�������ԽǶ���������̵�ŷ����
//   const fp32 *chassis_INS_angle;             //the point to the euler angle of gyro sensor.��ȡ�����ǽ������ŷ����ָ��
//   chassis_mode_e chassis_mode;               //state machine. ���̿���״̬��
//   chassis_mode_e last_chassis_mode;          //last state machine.�����ϴο���״̬��
//   chassis_motor_t motor_chassis[4];          //chassis motor data.���̵������
//   pid_type_def motor_speed_pid[4];             //motor speed PID.���̵���ٶ�pid
//   pid_type_def chassis_angle_pid;              //follow angle PID.���̸���Ƕ�pid

//   first_order_filter_type_t chassis_cmd_slow_set_vx;  //use first order filter to slow set-point.ʹ��һ�׵�ͨ�˲������趨ֵ
//   first_order_filter_type_t chassis_cmd_slow_set_vy;  //use first order filter to slow set-point.ʹ��һ�׵�ͨ�˲������趨ֵ

//   fp32 vx;                          //chassis vertical speed, positive means forward,unit m/s. �����ٶ� ǰ������ ǰΪ������λ m/s
//   fp32 vy;                          //chassis horizontal speed, positive means letf,unit m/s.�����ٶ� ���ҷ��� ��Ϊ��  ��λ m/s
//   fp32 wz;                          //chassis rotation speed, positive means counterclockwise,unit rad/s.������ת���ٶȣ���ʱ��Ϊ�� ��λ rad/s
//   fp32 vx_set;                      //chassis set vertical speed,positive means forward,unit m/s.�����趨�ٶ� ǰ������ ǰΪ������λ m/s
//   fp32 vy_set;                      //chassis set horizontal speed,positive means left,unit m/s.�����趨�ٶ� ���ҷ��� ��Ϊ������λ m/s
//   fp32 wz_set;                      //chassis set rotation speed,positive means counterclockwise,unit rad/s.�����趨��ת���ٶȣ���ʱ��Ϊ�� ��λ rad/s
//   fp32 chassis_relative_angle;      //the relative angle between chassis and gimbal.��������̨����ԽǶȣ���λ rad
//   fp32 chassis_relative_angle_set;  //the set relative angle.���������̨���ƽǶ�
//   fp32 chassis_yaw_set;

//   fp32 vx_max_speed;  //max forward speed, unit m/s.ǰ����������ٶ� ��λm/s
//   fp32 vx_min_speed;  //max backward speed, unit m/s.���˷�������ٶ� ��λm/s
//   fp32 vy_max_speed;  //max letf speed, unit m/s.��������ٶ� ��λm/s
//   fp32 vy_min_speed;  //max right speed, unit m/s.�ҷ�������ٶ� ��λm/s
//   fp32 chassis_yaw;   //the yaw angle calculated by gyro sensor and gimbal motor.�����Ǻ���̨������ӵ�yaw�Ƕ�
//   fp32 chassis_pitch; //the pitch angle calculated by gyro sensor and gimbal motor.�����Ǻ���̨������ӵ�pitch�Ƕ�
//   fp32 chassis_roll;  //the roll angle calculated by gyro sensor and gimbal motor.�����Ǻ���̨������ӵ�roll�Ƕ�
// }

pub enum ChassisControlMode {
    ChassisZeroFroce,
    ChassisNoMove,
    ChassisNoFollow,
    ChassisVectorFollowGimbalYaw,
    ChassisVectorFollowChassisYaw,
    ChiaasisVectorNoFollowChassisYaw,
    ChassisVectorRaw,
}

pub enum ChassisStructureHandle {
    OmniChassis {
        a_wheel_motor: Rm3508,
        b_wheel_motor: Rm3508,
        c_wheel_motor: Rm3508,
        d_wheel_motor: Rm3508,
    },
    MecanumChassis {
        a_wheel_motor: Rm3508,
        b_wheel_motor: Rm3508,
        c_wheel_motor: Rm3508,
        d_wheel_motor: Rm3508,
    },
}

impl ChassisStructureHandle {}

pub struct Chassis {
    chassis_structure: ChassisStructureHandle,
    chassis_mode: ChassisControlMode,
    chassis_speed: (f32, f32, f32),
    chassis_speed_strget: (f32, f32),
    chassis_rotate_speed: (f32, bool),
}

impl Chassis {
    pub async control(&self) -> Result<(), &static str> {
        match self.chassis_mode {

        }
    }
}
