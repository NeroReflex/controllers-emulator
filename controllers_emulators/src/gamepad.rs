use crate::{device::Device, message::InGamepadMessage};

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct GamepadStatus {
    pub joystick_positions: [[i32; 2]; 2], // [0 left | 1 right][x axis | y axis]

    pub dpad: u8, // 0x00 x - | 0x01 x -> | 0x02 x <- | 0x00 y - | 0x10 y ^ | 0x10 y . |

    pub l2_trigger: u8,
    pub r2_trigger: u8,

    pub triangle: u8,
    pub circle: u8,
    pub cross: u8,
    pub square: u8,

    pub l1: u8,
    pub r1: u8,

    pub r3: u8,
    pub l3: u8,

    pub option: u8,
    pub share: u8,
    pub center: u8,

    pub l4: u8,
    pub r4: u8,

    pub l5: u8,
    pub r5: u8,

    pub touchpad_press: u8,

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

impl Default for GamepadStatus {
    fn default() -> Self {
        Self {
            joystick_positions: [[0; 2]; 2], // [0 left | 1 right][x axis | y axis]
            dpad: 0, // 0x00 x - | 0x01 x -> | 0x02 x <- | 0x00 y - | 0x10 y ^ | 0x10 y . |
            l2_trigger: 0,
            r2_trigger: 0,
            triangle: 0,
            circle: 0,
            cross: 0,
            square: 0,
            l1: 0,
            r1: 0,
            r3: 0,
            l3: 0,
            option: 0,
            share: 0,
            center: 0,
            l4: 0,
            r4: 0,
            l5: 0,
            r5: 0,
            touchpad_press: 0,
            touchpad_touch_num: 0, // touchpad is inactive when this is -1
            touchpad_x: 0, // 0 to 1920
            touchpad_y: 0, // 0 to 1080
            last_gyro_motion_timestamp_ns: 0,
            last_accel_motion_timestamp_ns: 0,
            raw_gyro: [0; 3],
            raw_accel: [0; 3],
            rumble_events_count: 0,
            motors_intensity: [0; 2], // 0 = left, 1 = right
            leds_events_count: 0,
            leds_colors: [0; 3], // r | g | b
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