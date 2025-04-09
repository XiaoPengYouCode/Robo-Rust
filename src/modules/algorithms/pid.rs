#[allow(unused)]
pub struct Pid {
    kp: f32,
    ki: f32,
    kd: f32,
    max_out: f32,
    max_iout: f32,
    out: f32,
    p_out: f32,
    i_out: f32,
    d_out: f32,
    d_buf: [f32; 3],
    error: [f32; 3],
}

impl Default for Pid {
    fn default() -> Self {
        Pid::new()
    }
}

impl Pid {
    pub fn new() -> Self {
        Self {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
            max_out: 0.0,
            max_iout: 0.0,
            out: 0.0,
            p_out: 0.0,
            i_out: 0.0,
            d_out: 0.0,
            d_buf: [0.0; 3],
            error: [0.0; 3],
        }
    }

    #[allow(unused)]
    pub fn update(&self, error: f32) -> Result<(), &'static str> {
        Ok(())
    }
}
