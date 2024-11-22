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

impl Pid {
    pub async fn new() -> Result<Self, &'static str> {
        todo!("");
    }
}
