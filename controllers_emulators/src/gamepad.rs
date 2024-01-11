use crate::device::Device;

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

pub trait Gamepad: Device {
    fn get_status() -> GamepadStatus;
}

