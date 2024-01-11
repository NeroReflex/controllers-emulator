use crate::{gamepad::{Gamepad, GamepadStatus}, device::Device};
use crate::message::{InDeviceDataMessage, OutDeviceDataMessage};

#[derive(Clone)]
pub struct DualSenseSettings {
    pub bluetooth: bool,
    pub edge: bool,
    pub mac_addr: [u8; 6],
}

impl Default for DualSenseSettings {
    fn default() -> Self {
        Self {
            bluetooth: true,
            edge: false,
            mac_addr: [ 0x74u8, 0xe7u8, 0xd6u8, 0x3au8, 0x47u8, 0xe8u8 ],
        }
    }
}

pub struct DualSense {
    settings: DualSenseSettings,
    status: GamepadStatus,
}

impl Device for DualSense {
    fn handle_message(&mut self, msg: &InDeviceDataMessage) {
        match msg {
            InDeviceDataMessage::Gamepad(gamepad_msg) => crate::gamepad::process_gamepad_message(self, &gamepad_msg),
            _ => println!("Message type not implemented")
        }
    }

    fn generate_message(&mut self) -> Option<OutDeviceDataMessage> {
        println!("Unimplemented feature!");
        //todo!()
        None
    }
}

impl Gamepad for DualSense {
    fn get_status(&self) -> GamepadStatus {
        self.status
    }

    fn set_status(&mut self, stat: &GamepadStatus) {
        self.status = stat.clone();
    }
}

impl DualSense {
    pub fn new(settings: DualSenseSettings) -> Self {
        Self {
            settings: settings,
            status: GamepadStatus::default()
        }
    }
}

impl Default for DualSense {
    fn default() -> Self {
        DualSense::new(DualSenseSettings::default())
    }
}
