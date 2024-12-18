use defmt::debug;
use na::{Matrix3, Matrix3x6, Matrix6, Unit, UnitQuaternion, Vector3, Vector6};

pub struct ESKF {
    // 名义状态：四元数
    nominal_q: UnitQuaternion<f32>,

    // 误差状态：
    // 前3维：位置误差
    // 后3维：姿态误差
    error_state: Vector6<f32>,

    // 协方差矩阵
    p: Matrix6<f32>,

    // 过程噪声
    q_noise: Matrix6<f32>,

    // 测量噪声
    r_noise: Matrix3<f32>,
}

impl Default for ESKF {
    fn default() -> Self {
        Self::new()
    }
}

impl ESKF {
    pub fn new() -> Self {
        ESKF {
            nominal_q: UnitQuaternion::identity(),
            error_state: Vector6::zeros(),
            p: Matrix6::identity() * 0.1,                  // 初始协方差
            q_noise: Matrix6::from_diagonal_element(0.01), // 过程噪声
            r_noise: Matrix3::from_diagonal_element(0.1),  // 测量噪声
        }
    }

    // 预测步骤
    pub fn predict(&mut self, gyro: Vector3<f32>, dt: f32) {
        // 使用陀螺仪数据更新名义状态
        let omega = Vector3::new(gyro.x, gyro.y, gyro.z);
        let rotation_increment = UnitQuaternion::from_scaled_axis(omega * dt);

        // 更新名义四元数
        self.nominal_q = rotation_increment * self.nominal_q;
        self.nominal_q = UnitQuaternion::new_normalize(self.nominal_q.into_inner());

        // 协方差传播
        // F是状态转移矩阵，通常近似为单位矩阵
        let f: Matrix6<f32> = Matrix6::identity();
        self.p = f * self.p * f.transpose() + self.q_noise;
    }

    // 测量更新步骤
    pub fn update(&mut self, accel: Vector3<f32>) {
        // 测量矩阵
        let mut h = Matrix3x6::zeros();
        h.view_mut((0, 3), (3, 3)).copy_from(&Matrix3::identity());

        // 预测加速度
        let predicted_accel = self.nominal_q.to_rotation_matrix() * Vector3::z();

        // 测量残差
        let y = accel - predicted_accel;

        // 计算观测协方差
        let s: Matrix3<f32> = h * self.p * h.transpose() + self.r_noise;

        // 安全求逆
        let s_inv = match s.try_inverse() {
            Some(inv) => inv,
            None => {
                debug!("Matrix inversion failed");
                return;
            }
        };

        // Kalman增益
        let k = self.p * h.transpose() * s_inv;

        // 更新误差状态
        self.error_state += k * y;

        // 校正名义状态
        // 使用对数映射将误差状态注入名义状态
        let correction = UnitQuaternion::from_axis_angle(
            &Unit::new_normalize(self.error_state.fixed_rows::<3>(3).into_owned()),
            self.error_state.fixed_rows::<3>(3).norm(),
        );
        self.nominal_q = correction * self.nominal_q;

        // 更新协方差
        let i_kh = Matrix6::identity() - k * h;
        // 使用joseph形式的协方差更新，以保持正定性
        self.p = i_kh * self.p * i_kh.transpose() + k * self.r_noise * k.transpose();

        // 重置误差状态
        self.error_state = Vector6::zeros();
    }

    pub fn get_euler_angles_degrees(&self) -> (f32, f32, f32) {
        let euler = self.nominal_q.euler_angles();
        (
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        )
    }
}
