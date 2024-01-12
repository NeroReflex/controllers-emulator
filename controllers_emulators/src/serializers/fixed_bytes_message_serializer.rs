use crate::gamepad::{GamepadStatus, DPadStatus};
use crate::message_serializer::MessageSerializer;

use crate::message::{Message, InDeviceMessage, InMessage, InDeviceDataMessage, InGamepadMessage};

use core::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedBytesMessageSerializerError {
    error_msg: String,
}

impl std::fmt::Display for FixedBytesMessageSerializerError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

impl std::error::Error for FixedBytesMessageSerializerError {

}

impl FixedBytesMessageSerializerError {
    pub fn new(error_msg: String) -> Self {
        Self {
            error_msg
        }
    }
}

pub struct FixedBytesMessageSerializer {

}

const MESSAGE_BYTES: usize = 64;

impl Default for FixedBytesMessageSerializer {
    fn default() -> Self {
        Self {}
    }
}

const IN_MSG_TYPE: u8 = 0x00;
const OUT_MSG_TYPE: u8 = 0x01;

const IN_DEV_MSG_TYPE: u8 = 0x00;

const IN_DEV_PERIPHERIAL_GAMEPAD_MSG_TYPE: u8 = 0x00;
const IN_DEV_PERIPHERIAL_MOUSE_MSG_TYPE: u8 = 0x01;
const IN_DEV_PERIPHERIAL_KEYBOARD_MSG_TYPE: u8 = 0x02;

const IN_DEV_PERIPHERIAL_GAMEPAD_SET_STATUS_MSG_TYPE: u8 = 0x00u8;

impl MessageSerializer for FixedBytesMessageSerializer {
    fn deserialize(&self, bytes: &Vec<u8>) -> Result<Message, Box<dyn std::error::Error>> {
        if bytes.len() != MESSAGE_BYTES {
            return Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
        }

        let mut decoded = 0;

        match bytes[decoded] {
            IN_MSG_TYPE => {
                decoded += 1;
                match bytes[decoded] {
                    IN_DEV_MSG_TYPE => {
                        decoded += 1;
                        // next sizeof(u64): 8 -- are device_hash
                        let hash_bytes = [
                            bytes[decoded+0] as u64,
                            bytes[decoded+1] as u64,
                            bytes[decoded+2] as u64,
                            bytes[decoded+3] as u64,
                            bytes[decoded+4] as u64,
                            bytes[decoded+5] as u64,
                            bytes[decoded+6] as u64,
                            bytes[decoded+7] as u64,
                        ];

                        decoded += 8;

                        let device_hash: u64 = 
                            (hash_bytes[0] << 0) |
                            (hash_bytes[1] << 8) |
                            (hash_bytes[2] << 16) |
                            (hash_bytes[3] << 24) |
                            (hash_bytes[4] << 32) |
                            (hash_bytes[5] << 40) |
                            (hash_bytes[6] << 48) |
                            (hash_bytes[7] << 56);

                        match bytes[decoded] {
                            IN_DEV_PERIPHERIAL_GAMEPAD_MSG_TYPE => {
                                decoded += 1;
                                match bytes[decoded] {
                                    IN_DEV_PERIPHERIAL_GAMEPAD_SET_STATUS_MSG_TYPE => {
                                        decoded += 1;

                                        let buttons_bytes = [
                                            bytes[decoded+0] as u64,
                                            bytes[decoded+1] as u64,
                                            bytes[decoded+2] as u64,
                                            bytes[decoded+3] as u64,
                                            bytes[decoded+4] as u64,
                                            bytes[decoded+5] as u64,
                                            bytes[decoded+6] as u64,
                                            bytes[decoded+7] as u64,
                                        ];

                                        decoded += 8;

                                        let buttons: u64 = 
                                            (buttons_bytes[0] << 0) |
                                            (buttons_bytes[1] << 8) |
                                            (buttons_bytes[2] << 16) |
                                            (buttons_bytes[3] << 24) |
                                            (buttons_bytes[4] << 32) |
                                            (buttons_bytes[5] << 40) |
                                            (buttons_bytes[6] << 48) |
                                            (buttons_bytes[7] << 56);
                                        
                                        let gamepad_status = GamepadStatus {
                                            buttons,
                                            joystick_positions: [[0; 2]; 2],
                                            dpad: DPadStatus::default(),
                                            l2_trigger: 0,
                                            r2_trigger: 0,
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
                                        };

                                        Ok(
                                            Message::In(
                                                InMessage::Device(
                                                    InDeviceMessage {
                                                        device_hash: device_hash,
                                                        data: InDeviceDataMessage::Gamepad(
                                                            InGamepadMessage::SetStatus(
                                                                gamepad_status
                                                            )
                                                        )
                                                    }
                                                )
                                            )
                                        )
                                    },
                                    _ => {Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))}
                                }
                            },
                            IN_DEV_PERIPHERIAL_MOUSE_MSG_TYPE => {
                                decoded += 1;
                                
                                Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
                            },
                            IN_DEV_PERIPHERIAL_KEYBOARD_MSG_TYPE => {
                                decoded += 1;

                                Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
                            },
                            _ => {
                                Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
                            }
                        }
                    },
                    _ => {
                        Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
                    }
                }
            },
            OUT_MSG_TYPE => {
                decoded += 1;
                Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
            },
            _ => {
                Err(Box::<FixedBytesMessageSerializerError>::new(FixedBytesMessageSerializerError::new(String::from(""))))
            }
        }
    }

    fn serialize(&self, message: &Message) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut result = vec![0u8; MESSAGE_BYTES];
        let mut encoded: usize = 0;

        match message {
            Message::In(in_msg) => {
                result[encoded] = IN_MSG_TYPE;
                encoded += 1;

                match &in_msg {
                    InMessage::Device(in_dev_msg) => {
                        result[encoded] = IN_DEV_MSG_TYPE;
                        encoded += 1;
                        
                        // TODO: now encode data: hash + following message

                    }
                }
            },
            Message::Out(out_msg) => {
                result[encoded] = OUT_MSG_TYPE;
                encoded += 1;
            }
        }

        Ok(result)
    }

    fn fixed_size(&self) -> Option<usize> {
        Some(MESSAGE_BYTES)
    }
}