use embassy_time::Duration;
mod robot_def;



fn shoot_init() {
    let friction_l = DJIMotorInstance::new();
    let friction_r = DJIMotorInstance::new();
    let loader = DJIMotorInstance::new();
    let shoot_pub = Publisher_t::new();
    let shoot_sub = Subscriber_t::new();
    let shoot_cmd_recv = Shoot_Ctrl_Cmd_s::new();
    let shoot_feedback_data = Shoot_Upload_Data_s::new();

    // Implement the initialization logic here
}

fn shoot_task() {
    let shoot_cmd_recv = Shoot_Ctrl_Cmd_s::new();

    // Implement the shoot task logic here
    match shoot_cmd_recv.shoot_mode {
        ShootMode::Off => {
            friction_l.stop();
            friction_r.stop();
            loader.stop();
        }
        _ => {
            friction_l.enable();
            friction_r.enable();
            loader.enable();
        }
    }

    match shoot_cmd_recv.load_mode {
        LoadMode::Stop => {
            loader.outer_loop(SPEED_LOOP);
            loader.set_ref(0.0);
        }
        LoadMode::OneBullet => {
            loader.outer_loop(ANGLE_LOOP);
            loader.set_ref(loader.measure.total_angle + ONE_BULLET_DELTA_ANGLE);
            // Implement the hibernate_time and dead_time logic here
        }
        LoadMode::ThreeBullet => {
            loader.outer_loop(ANGLE_LOOP);
            loader.set_ref(loader.measure.total_angle + 3 * ONE_BULLET_DELTA_ANGLE);
            // Implement the hibernate_time and dead_time logic here
        }
        LoadMode::BurstFire => {
            loader.outer_loop(SPEED_LOOP);
            loader.set_ref(shoot_cmd_recv.shoot_rate * 360.0 * REDUCTION_RATIO_LOADER / 8.0);
        }
        LoadMode::Reverse => {
            loader.outer_loop(SPEED_LOOP);
            // Implement the reverse logic here
        }
    }

    match shoot_cmd_recv.friction_mode {
        FrictionMode::On => {
            // Implement the friction mode logic here
        }
        FrictionMode::Off => {
            friction_l.set_ref(0.0);
            friction_r.set_ref(0.0);
        }
    }

    match shoot_cmd_recv.lid_mode {
        LidMode::Close => {
            // Implement the close lid logic here
        }
        LidMode::Open => {
            // Implement the open lid logic here
        }
    }

    // Implement the feedback data logic here
}
