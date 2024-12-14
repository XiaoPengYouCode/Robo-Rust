pub struct ExtendedKalmanFilter {
    x: [f32; 2],
    p: [[f32; 2]; 2],
    f: [[f32; 2]; 2],
    h: [[f32; 2]; 2],
    r: [[f32; 2]; 2],
    q: [[f32; 2]; 2],
}

impl ExtendedKalmanFilter {
    pub fn new() -> Self {
        ExtendedKalmanFilter {
            x: [0.0, 0.0],
            p: [[1.0, 0.0], [0.0, 1.0]],
            f: [[1.0, 0.0], [0.0, 1.0]],
            h: [[1.0, 0.0], [0.0, 1.0]],
            r: [[0.1, 0.0], [0.0, 0.1]],
            q: [[0.001, 0.0], [0.0, 0.001]],
        }
    }

    pub fn predict(&mut self) {
        // x = f * x
        self.x = [
            self.f[0][0] * self.x[0] + self.f[0][1] * self.x[1],
            self.f[1][0] * self.x[0] + self.f[1][1] * self.x[1],
        ];

        // p = f * p * f^T + Q
        let mut temp = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                temp[i][j] = self.f[i][0] * self.p[0][j] + self.f[i][1] * self.p[1][j];
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                self.p[i][j] = temp[i][0] * self.f[j][0] + temp[i][1] * self.f[j][1] + self.q[i][j];
            }
        }
    }

    pub fn update(&mut self, z: [f32; 2]) {
        // y = z - h * x
        let y = [
            z[0] - (self.h[0][0] * self.x[0] + self.h[0][1] * self.x[1]),
            z[1] - (self.h[1][0] * self.x[0] + self.h[1][1] * self.x[1]),
        ];

        // s = h * p * h^T + r
        let mut s = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                s[i][j] = self.h[i][0] * self.p[0][j] * self.h[j][0]
                    + self.h[i][1] * self.p[1][j] * self.h[j][1]
                    + self.r[i][j];
            }
        }

        // k = p * h^T * s^-1
        let det = s[0][0] * s[1][1] - s[0][1] * s[1][0];
        let s_inv = [
            [s[1][1] / det, -s[0][1] / det],
            [-s[1][0] / det, s[0][0] / det],
        ];

        let mut k = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                k[i][j] = self.p[i][0] * self.h[0][j] * s_inv[j][0]
                    + self.p[i][1] * self.h[1][j] * s_inv[j][1];
            }
        }

        // x = x + k * y
        self.x[0] += k[0][0] * y[0] + k[0][1] * y[1];
        self.x[1] += k[1][0] * y[0] + k[1][1] * y[1];

        // p = (I - k * h) * p
        let mut temp = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                temp[i][j] = k[i][0] * self.h[0][j] + k[i][1] * self.h[1][j];
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                self.p[i][j] =
                    self.p[i][j] - (temp[i][0] * self.p[0][j] + temp[i][1] * self.p[1][j]);
            }
        }
    }
}
