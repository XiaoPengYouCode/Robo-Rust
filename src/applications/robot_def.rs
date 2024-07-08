// 云台参数
const YAW_CHASSIS_ALIGN_ECD: u16 = 2711;  // 云台和底盘对齐指向相同方向时的电机编码器值,若对云台有机械改动需要修改
const YAW_ECD_GREATER_THAN_4096: u16 = 0; // ALIGN_ECD值是否大于4096,是为1,否为0;用于计算云台偏转角度
const PITCH_HORIZON_ECD: u16 = 3412;      // 云台处于水平位置时编码器值,若对云台有机械改动需要修改
const PITCH_MAX_ANGLE: u8 = 0;           // 云台竖直方向最大角度 (注意反馈如果是陀螺仪，则填写陀螺仪的角度)
const PITCH_MIN_ANGLE: u16 = 0;           // 云台竖直方向最小角度 (注意反馈如果是陀螺仪，则填写陀螺仪的角度)

// 发射参数
enum bullet_power_source {
    fric_wheel {
        motor_type: u8, // 电机类型
        nunmber: u8, // 摩擦轮个数
    }, // 摩擦轮
    compound_bow, // 复合弓
}

const BULLET_SPEED_MAX: f32 = 29.8; // 子弹速度上限MAX
const BULLET_SPEED_MIN: f32 = 26.8; // 子弹速度下限MIN

const REDUCTION_RATIO_LOADER: f32 = 49.0; // 拨盘电机的减速比,英雄需要修改为3508的19.0
const NUM_PER_CIRCLE: u8 = 10;            // 拨盘一圈的装载量
const ONE_BULLET_DELTA_ANGLE: u8 = 360 / NUM_PER_CIRCLE;    // 发射一发弹丸拨盘转动的角度

// 机器人底盘修改的参数,单位为mm(毫米)
const WHEEL_BASE: u16 = 350;              // 纵向轴距(前进后退方向)
const TRACK_WIDTH: u16 = 300;             // 横向轮距(左右平移方向)
const CENTER_GIMBAL_OFFSET_X: u16 = 0;    // 云台旋转中心距底盘几何中心的距离,前后方向,云台位于正中心时默认设为0
const CENTER_GIMBAL_OFFSET_Y: u16 = 0;    // 云台旋转中心距底盘几何中心的距离,左右方向,云台位于正中心时默认设为0
const RADIUS_WHEEL: u16 = 60;             // 轮子半径
const REDUCTION_RATIO_WHEEL: f32 = 19.0;  // 电机减速比,因为编码器量测的是转子的速度而不是输出轴的速度故需进行转换
const GYRO2GIMBAL_DIR_YAW: i8 = 1;   // 陀螺仪数据相较于云台的yaw的方向,1为相同,-1为相反
const PITCH_HORIZON_ECD: u16 = 3412;      // 云台处于水平位置时编码器值,若对云台有机械改动需要修改
const PITCH_MAX_ANGLE: u8 = 0;           // 云台竖直方向最大角度 (注意反馈如果是陀螺仪，则填写陀螺仪的角度)
const PITCH_MIN_ANGLE: u16 = 0;           // 云台竖直方向最小角度 (注意反馈如果是陀螺仪，则填写陀螺仪的角度)

// 检查是否出现主控板定义冲突,只允许一个开发板定义存在,否则编译会自动报错
#[cfg(all(feature = "ONE_BOARD", feature = "CHASSIS_BOARD"))]
compile_error!("Conflict board definition! You can only define one board type.");
#[cfg(all(feature = "ONE_BOARD", feature = "GIMBAL_BOARD"))]
compile_error!("Conflict board definition! You can only define one board type.");
#[cfg(all(feature = "CHASSIS_BOARD", feature = "GIMBAL_BOARD"))]
compile_error!("Conflict board definition! You can only define one board type.");

enum RobotStatus {
    Stop,
    Ready,
}

// 应用状态
enum AppStatus {
    Offline,
    Online,
    Error,
}

enum ChassisMode {
    ZeroForce,              // 电流零输入
    Rotate,                 // 小陀螺模式
    NoFollow,               // 不跟随，允许全向平移
    FollowGimbalYaw,        // 跟随模式，底盘叠加角度环控制
}

// 云台模式设置
enum GimbalMode {
    ZeroForce,              // 电流零输入
    FreeMode,               // 云台自由运动模式,即与底盘分离(底盘此时应为NO_FOLLOW)反馈值为电机total_angle;似乎可以改为全部用IMU数据?
    GyroMode,               // 云台陀螺仪反馈模式,反馈值为陀螺仪pitch,total_yaw_angle,底盘可以为小陀螺和跟随模式
}

// 发射模式设置
enum ShootMode {
    Off,
    On,
}

enum FrictionMode {
    Off,
    On {
        friction_wheel_speed: i16,
    },
}

// 目前硬件未搭载,暂时保留
// enum LidMode {
//     Open, // 弹舱盖打开
//     Close,    // 弹舱盖关闭
// }

enum LoaderMode {
    Stop,         // 停止发射
    Reverse,      // 反转
    OneBullet,    // 点射
    ThreeBullet,  // 三发
    BurstFire {   // 连发
        bullet_speed: u8, // 连发速率，单位为发/秒，上限为20发/秒
    },
}

// 功率限制,从裁判系统获取
struct ChassisPowerData {
    chassis_power_mx: f32,
}

// 机器人状态数据
struct ChassisCtrlCmd {
    // 控制部分
    vx: f32,           // 前进方向速度
    vy: f32,           // 横移方向速度
    wz: f32,           // 旋转速度
    offset_angle: f32, // 底盘和归中位置的夹角
    chassis_mode: ChassisMode,
    chassis_speed_buff: i32,
}

// 云台控制数据
struct GimbalCtrlCmd {
    // 云台角度控制
    yaw: f32,
    pitch: f32,
    chassis_rotate_wz: f32,
    gimbal_mode: GimbalMode,
}

// cmd发布的发射控制数据,由shoot订阅
struct ShootCtrlCmd {
    shoot_mode: ShootMode,
    load_mode: LoaderMode,
    lid_mode: LidMode,
    friction_mode: FrictionMode,
    bullet_speed: BulletSpeed,
    rest_heat: u8,
    shoot_rate: f32, // 连续发射的射频,unit per s,发/秒
}

/* ----------------gimbal/shoot/chassis发布的反馈数据----------------*/
/**
 * @brief 由cmd订阅,其他应用也可以根据需要获取.
 *
 */

struct ChassisUploadData {
    // 非单板的时候底盘还将imu数据回传(若有必要)
    // attitude_t chassis_imu_data;
    // 后续增加底盘的真实速度
    // real_vx: f32,
    // real_vy: f32,
    // real_wz: f32,
    rest_heat: u8,           // 剩余枪口热量
    bullet_speed: BulletSpeed, // 弹速限制
    enemy_color: EnemyColor,   // 0 for blue, 1 for red
}

struct GimbalUploadData {
    gimbal_imu_data: attitude_t,
    yaw_motor_single_round_angle: u16,
}

struct ShootUploadData {
    // code to go here
    // ...
}
