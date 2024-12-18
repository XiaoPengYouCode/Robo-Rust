use defmt::info;
use embassy_time::{Duration, Instant};

pub struct SimpleMonitor {
    target_period_us: u64,       // 目标周期（微秒）
    start_time: Option<Instant>, // 开始时间
    max_time_us: u64,            // 最大执行时间（微秒）
    min_time_us: Option<u64>,    // 最小执行时间（微秒），初始为 None
    total_time_us: u64,          // 累计总执行时间（微秒）
    total_frames: u32,           // 总帧数
    over_period_count: u32,      // 超时帧数
}

impl SimpleMonitor {
    pub fn new(target_period: Duration) -> Self {
        Self {
            target_period_us: target_period.as_micros(),
            start_time: None,
            max_time_us: 0,
            min_time_us: None, // 使用 Option 表示未初始化
            total_time_us: 0,
            total_frames: 0,
            over_period_count: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed_us = (Instant::now() - start).as_micros();

            self.max_time_us = self.max_time_us.max(elapsed_us);

            // 更新最小值，避免未初始化的情况
            self.min_time_us = Some(
                self.min_time_us
                    .map_or(elapsed_us, |min| min.min(elapsed_us)),
            );

            if elapsed_us > self.target_period_us {
                self.over_period_count += 1;
            }

            self.total_time_us += elapsed_us;
            self.total_frames += 1;

            self.start_time = None; // 重置开始时间
        }
    }

    pub fn print_stats(&self) {
        let avg_frame_rate = if self.total_frames > 0 {
            1_000_000.0 / (self.total_time_us as f32 / self.total_frames as f32)
        } else {
            0.0
        };

        info!(
            "执行统计:\n目标周期: {}us\n最大耗时: {}us\n最小耗时: {}us\n超时次数: {}/{}\n平均帧率: {} Hz",
            self.target_period_us,
            self.max_time_us,
            self.min_time_us.unwrap_or(0), // 如果没有值，返回 0
            self.over_period_count,
            self.total_frames,
            avg_frame_rate
        );
    }
}
