use na::{Matrix3, Matrix3x6, Matrix6, Unit, UnitQuaternion, Vector3, Vector6};

pub struct ESKF {
    nominal_q: UnitQuaternion<f32>, // 名义状态 四元数
    error_estimate: Vector6<f32>,
    error_estimate_p: Matrix6<f32>,
    error_prediction: Vector6<f32>,
    error_prediction_p: Matrix6<f32>,
    f: Matrix6<f32>, // 状态转移矩阵
    q: Matrix6<f32>, // 过程噪声协方差矩阵
    measurement: Vector6<f32>, // 测量值
    r: Matrix3<f32>, // 传感器噪声协方差矩阵
    kalman_gian: Matrix6<f32>, // 卡尔曼增益
    dt: f32,
}

impl ESKF {
    pub fn new(default_estimate: Vector6<f32>, default_estimate_p: Vector6<f32>) -> Self {
        let eskf = ESKF {
            nominal_q: UnitQuaternion::identity(),
            error_state: default_estimate,
            error_estimate_p: default_estimate_p,
            error_prediction: Vector6<f32>::zeros(),
            error_prediction_p: Matrix6<f32>::zeros(),
            f: Matrix6<f32>::zeros(),
            q: Matrix6<f32>::identity() * 0.01,
            measurement: Vector3<f32>::zeros(),
            r: Matrix3<f32>::identity() * 0.1,
            kalman_gian: Matrix6<f32>::zeros(),
        }

        // init predict
        eskf.predict = eskf.f * eskf.estimate + eskf.b * eskf.u;
        eskf.predict_p = eskf.f * eskf.estimate_p * eskf.f.transpose() + eskf.q;
        eskf
    }

    pub fn measurement(&mut self, measurement: Vector6<f32>) {
        self.measurement = measurement;
    }

    pub fn update(&mut self) {
        // 测量矩阵
        let mut h = Matrix3x6::zeros();
        h.view_mut((0, 3), (3, 3)).copy_from(&Matrix3::identity());

        // 预测加速度
        let predicted_accel = self.nominal_q.to_rotation_matrix() * Vector3::z();

        // 测量残差
        let y = accel - predicted_accel;

        // 计算观测协方差
        let s: Matrix3<f32> = h * self.p * h.transpose() + self.r_noise;

        // 求逆
        let s_inv = s.try_inverse().unwrap_or(Matrix3::<f32>::zeros());

        // 1. 计算卡尔曼增益
        self.kalman_gian = self.p * h.transpose() * s_inv;

        // 2. 更新误差状态
        self.error_state += self.kalman_gian * y;

        // 校正名义状态
        // 使用对数映射将误差状态注入名义状态
        let correction = UnitQuaternion::from_axis_angle(
            &Unit::new_normalize(self.error_state.fixed_rows::<3>(3).into_owned()),
            self.error_state.fixed_rows::<3>(3).norm(),
        );
        self.nominal_q = correction * self.nominal_q;

        // 更新协方差
        let i_kh = Matrix6::identity() - self.kalman_gian * h;
        // 使用joseph形式的协方差更新，以保持正定性
        self.p = i_kh * self.p * i_kh.transpose() + self.kalman_gian * self.r_noise * self.kalman_gian.transpose();

        // 重置误差状态
        self.error_state = Vector6::zeros();
    }

    // 预测步骤
    pub fn predict(&mut self) {
        // 使用陀螺仪数据更新名义状态
        let rotation_increment = UnitQuaternion::from_scaled_axis(gyro * dt);

        // 更新名义四元数
        self.nominal_q = rotation_increment * self.nominal_q;
        self.nominal_q = UnitQuaternion::new_normalize(self.nominal_q.into_inner());

        // 协方差传播, F是状态转移矩阵，通常近似为单位矩阵
        self.p = self.f * self.p * self.f.transpose() + self.q_noise;
    }


    pub fn get_euler_angles_degrees(&self) -> [f32; 3] {
        let euler = self.nominal_q.euler_angles();
        [
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        ]
    }
}
