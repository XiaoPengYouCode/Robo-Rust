use na::{Matrix3, Matrix3x6, Matrix6, Unit, UnitQuaternion, Vector3, Vector6};

pub struct ESKF {
    nominal_q: UnitQuaternion<f32>, // 名义状态 四元数
    error_estimate: Vector6<f32>,
    error_estimate_p: Matrix6<f32>,
    error_prediction: Vector6<f32>,
    error_prediction_p: Matrix6<f32>,
    f: Matrix6<f32>,           // 状态转移矩阵
    q: Matrix6<f32>,           // 过程噪声协方差矩阵
    measurement: Vector6<f32>, // 测量值
    r: Matrix3<f32>,           // 传感器噪声协方差矩阵
    kalman_gain: Matrix6<f32>, // 卡尔曼增益
    dt: f32,
}

impl ESKF {
    pub fn new(default_estimate: Vector6<f32>, default_estimate_p: Matrix6<f32>) -> Self {
        ESKF {
            nominal_q: UnitQuaternion::identity(),
            error_estimate: default_estimate,
            error_estimate_p: default_estimate_p,
            error_prediction: Vector6::<f32>::zeros(),
            error_prediction_p: Matrix6::<f32>::zeros(),
            f: Matrix6::<f32>::zeros(),
            q: Matrix6::<f32>::identity() * 0.01,
            measurement: Vector6::<f32>::zeros(),
            r: Matrix3::<f32>::identity() * 0.1,
            kalman_gain: Matrix6::<f32>::zeros(),
            dt: 0.001,
        }
    }

    pub fn measurement(&mut self, measurement: Vector6<f32>) {
        self.measurement = measurement;
    }

    pub fn update(&mut self, accel: Vector3<f32>) {
        // 测量矩阵
        // let mut h = Matrix3x6::zeros();
        // h.view_mut((0, 3), (3, 3)).copy_from(&Matrix3::identity());

        // 预测加速度
        let predicted_accel = self.nominal_q.to_rotation_matrix() * Vector3::z();

        // 测量残差
        let y = accel - predicted_accel;

        // 计算观测协方差
        // let s: Matrix3<f32> = h * self.error_estimate_p * h.transpose();

        // 求逆
        // let s_inv = s.try_inverse().unwrap_or(Matrix3::<f32>::identity());

        // 1. 计算卡尔曼增益
        // self.kalman_gain = self.error_estimate_p * h.transpose() * s_inv;

        // 2. 更新误差状态
        // self.error_estimate += self.kalman_gain * y;

        // 校正名义状态
        // 使用对数映射将误差状态注入名义状态
        let correction = UnitQuaternion::from_axis_angle(
            &Unit::new_normalize(self.error_estimate.fixed_rows::<3>(3).into_owned()),
            self.error_estimate.fixed_rows::<3>(3).norm(),
        );
        self.nominal_q = correction * self.nominal_q;

        // 更新协方差
        // let i_kh = Matrix6::identity() - self.kalman_gain * h;
        // 使用joseph形式的协方差更新，以保持正定性
        // self.p = i_kh * self.p * i_kh.transpose()
        //     + self.kalman_gain * self.error_estimate_p * self.kalman_gain.transpose();

        // 重置误差状态
        self.error_estimate = Vector6::zeros();
    }

    // 预测步骤
    pub fn predict(&mut self, gyro: Vector3<f32>) {
        // 使用陀螺仪数据更新名义状态
        let rotation_increment = UnitQuaternion::from_scaled_axis(gyro * self.dt);

        // 更新名义四元数
        self.nominal_q = rotation_increment * self.nominal_q;
        self.nominal_q = UnitQuaternion::new_normalize(self.nominal_q.into_inner());

        // 协方差传播, F是状态转移矩阵，通常近似为单位矩阵
        self.error_estimate_p =
            self.f * self.error_prediction_p * self.f.transpose() + self.error_prediction_p;
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
