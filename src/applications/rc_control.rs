use crate::system::event;
use crate::system::resources::{RCResourcesLeftJoystick, RCResourcesRightJoystick, RCResourcesLeftToggle, RCResourcesRightToggle, RCResourcesWheel};
use embassy_stm32::gpio::{Input, Level, Pull};
use embassy_time::{Duration, Timer};

/// Joystick X and Y axis handler
#[embassy_executor::task]
pub async fn rc_left_joystick_handle(r: RCResourcesLeftJoystick) {
    let mut x_axis = Input::new(r.x_axis, Pull::Down);
    let mut y_axis = Input::new(r.y_axis, Pull::Down);
    handle_joystick(&mut x_axis, &mut y_axis, true).await;
}

/// Toggle switch handler
#[embassy_executor::task]
pub async fn rc_right_joystick_handle(r: RCResourcesRightJoystick) {
    let mut x_axis = Input::new(r.x_axis, Pull::Down);
    let mut y_axis = Input::new(r.y_axis, Pull::Down);
    handle_joystick(&mut x_axis, &mut y_axis, false).await;
}

/// 左拨杆处理
#[embassy_executor::task]
pub async fn rc_left_toggle_handle(r: RCResourcesLeftToggle) {
    let mut toggle_up = Input::new(r.toggle_up, Pull::Down);
    let mut toggle_mid = Input::new(r.toggle_mid, Pull::Down);
    let mut toggle_down = Input::new(r.toggle_down, Pull::Down);
    handle_three_way_toggle(&mut toggle_up, &mut toggle_mid, &mut toggle_down, true).await;
}

/// 右拨杆处理
#[embassy_executor::task]
pub async fn rc_right_toggle_handle(r: RCResourcesRightToggle) {
    let mut toggle_up = Input::new(r.toggle_up, Pull::Down);
    let mut toggle_mid = Input::new(r.toggle_mid, Pull::Down);
    let mut toggle_down = Input::new(r.toggle_down, Pull::Down);
    handle_three_way_toggle(&mut toggle_up, &mut toggle_mid, &mut toggle_down, false).await;
}

/// 拨轮处理
#[embassy_executor::task]
pub async fn rc_wheel_handle(r: RCResourcesWheel) {
    let mut wheel = Input::new(r.wheel, Pull::Down);
    handle_wheel(&mut wheel).await;
}

/// 处理摇杆输入并生成事件
async fn handle_joystick(x_axis: &mut Input<'static>, y_axis: &mut Input<'static>, is_left: bool) {
    loop {
        let x_value = x_axis.get_level();
        let y_value = y_axis.get_level();
        
        if is_left {
            event::send(event::Events::LeftJoystickMoved(x_value, y_value)).await;
        } else {
            event::send(event::Events::RightJoystickMoved(x_value, y_value)).await;
        }
        
        Timer::after(Duration::from_millis(50)).await;
    }
}

/// 处理三档拨杆输入并生成事件
async fn handle_three_way_toggle(toggle_up: &mut Input<'static>, toggle_down: &mut Input<'static>, is_left: bool) {
    loop {
        let up_level = debounce(toggle_up).await;
        let down_level = debounce(toggle_down).await;
        
        let position = match (up_level, down_level) {
            (Level::High, Level::Low) => event::TogglePosition::Up,
            (Level::Low, Level::Low) => event::TogglePosition::Middle,
            (Level::Low, Level::High) => event::TogglePosition::Down,
            _ => event::TogglePosition::Middle, // 防止异常情况
        };

        if is_left {
            event::send(event::Events::LeftToggleChanged(position)).await;
        } else {
            event::send(event::Events::RightToggleChanged(position)).await;
        }

        Timer::after(Duration::from_millis(50)).await;
    }
}

/// 处理拨轮输入并生成事件
async fn handle_wheel(wheel: &mut Input<'static>) {
    loop {
        let value = wheel.get_level();
        event::send(event::Events::WheelMoved(value)).await;
        Timer::after(Duration::from_millis(50)).await;
    }
}