use crate::{device::Device, message::InGamepadMessage};

const BTN_CROSS: u64 = 0x0000000000000001u64;
const BTN_TRIANGLE: u64 = 0x0000000000000002u64;
const BTN_CIRCLE: u64 = 0x0000000000000004u64;
const BTN_SQUARE: u64 = 0x0000000000000008u64;
const BTN_L1: u64 = 0x0000000000000010u64;
const BTN_R1: u64 = 0x0000000000000020u64;
const BTN_L3: u64 = 0x0000000000000040u64;
const BTN_R3: u64 = 0x0000000000000080u64;
const BTN_OPTION: u64 = 0x0000000000000100u64;
const BTN_SHARE: u64 = 0x0000000000000200u64;
const BTN_CENTER: u64 = 0x0000000000000400u64;
const BTN_TOUCHPAD: u64 = 0x0000000000000800u64;
const BTN_L4: u64 = 0x0000000000001000u64;
const BTN_R4: u64 = 0x0000000000002000u64;
const BTN_L5: u64 = 0x0000000000004000u64;
const BTN_R5: u64 = 0x0000000000008000u64;

#[derive(Copy, Clone, PartialEq)]
pub enum DPadXStatus {
    Neutral,
    Up,
    Down,
}

impl Default for DPadXStatus {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum DPadYStatus {
    Neutral,
    Up,
    Down,
}

impl Default for DPadYStatus {
    fn default() -> Self {
        Self::Neutral
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct DPadStatus {
    pub x_status: DPadXStatus,
    pub y_status: DPadYStatus,
}

impl Default for DPadStatus {
    fn default() -> Self {
        Self {
            x_status: DPadXStatus::default(),
            y_status: DPadYStatus::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct GamepadStatus {
    pub joystick_positions: [[i32; 2]; 2], // [0 left | 1 right][x axis | y axis]

    pub dpad: DPadStatus,

    pub l2_trigger: u8,
    pub r2_trigger: u8,

    pub buttons: u64,

    pub touchpad_touch_num: i16, // touchpad is inactive when this is -1
    pub touchpad_x: i16, // 0 to 1920
    pub touchpad_y: i16, // 0 to 1080

    pub last_gyro_motion_timestamp_ns: i64,
    pub last_accel_motion_timestamp_ns: i64,

    pub raw_gyro: [i16; 3],
    pub raw_accel: [i16; 3],

    pub rumble_events_count: u64,
    pub motors_intensity: [u8; 2], // 0 = left, 1 = right

    pub leds_events_count: u64,
    pub leds_colors: [u8; 3], // r | g | b

    pub join_left_analog_and_gyroscope: u8,
    pub join_right_analog_and_gyroscope: u8,
}

impl GamepadStatus {

    pub fn btn_cross_pressed(&self) -> bool {
        (self.buttons & BTN_CROSS) != 0
    }

    pub fn btn_triangle_pressed(&self) -> bool {
        (self.buttons & BTN_TRIANGLE) != 0
    }

    pub fn btn_circle_pressed(&self) -> bool {
        (self.buttons & BTN_CIRCLE) != 0
    }

    pub fn btn_square_pressed(&self) -> bool {
        (self.buttons & BTN_SQUARE) != 0
    }

    pub fn btn_l1_pressed(&self) -> bool {
        (self.buttons & BTN_L1) != 0
    }

    pub fn btn_r1_pressed(&self) -> bool {
        (self.buttons & BTN_R1) != 0
    }

    pub fn btn_l3_pressed(&self) -> bool {
        (self.buttons & BTN_L3) != 0
    }

    pub fn btn_r3_pressed(&self) -> bool {
        (self.buttons & BTN_R3) != 0
    }

    pub fn btn_option_pressed(&self) -> bool {
        (self.buttons & BTN_OPTION) != 0
    }

    pub fn btn_share_pressed(&self) -> bool {
        (self.buttons & BTN_SHARE) != 0
    }

    pub fn btn_center_pressed(&self) -> bool {
        (self.buttons & BTN_CENTER) != 0
    }

    pub fn btn_touchpad_pressed(&self) -> bool {
        (self.buttons & BTN_TOUCHPAD) != 0
    }

    pub fn btn_l4_pressed(&self) -> bool {
        (self.buttons & BTN_L4) != 0
    }

    pub fn btn_r4_pressed(&self) -> bool {
        (self.buttons & BTN_R4) != 0
    }

    pub fn btn_l5_pressed(&self) -> bool {
        (self.buttons & BTN_L5) != 0
    }

    pub fn btn_r5_pressed(&self) -> bool {
        (self.buttons & BTN_R5) != 0
    }

}

impl Default for GamepadStatus {
    fn default() -> Self {
        Self {
            joystick_positions: [[0; 2]; 2],
            dpad: DPadStatus::default(),
            l2_trigger: 0,
            r2_trigger: 0,
            buttons: 0,
            touchpad_touch_num: 0,
            touchpad_x: 0,
            touchpad_y: 0,
            last_gyro_motion_timestamp_ns: 0,
            last_accel_motion_timestamp_ns: 0,
            raw_gyro: [0; 3],
            raw_accel: [0; 3],
            rumble_events_count: 0,
            motors_intensity: [0; 2],
            leds_events_count: 0,
            leds_colors: [0; 3],
            join_left_analog_and_gyroscope: 0,
            join_right_analog_and_gyroscope: 0,
        }
    }
}

pub trait Gamepad: Device {
    fn get_status(&self) -> GamepadStatus;

    fn set_status(&mut self, stat: &GamepadStatus);
}

pub(crate) fn process_gamepad_message(gamepad: &mut dyn Gamepad, msg: &InGamepadMessage) {
    match msg {
        &InGamepadMessage::SetStatus(stat) => gamepad.set_status(&stat)
    }
}