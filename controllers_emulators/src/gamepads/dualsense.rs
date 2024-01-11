use crate::{gamepad::{Gamepad, GamepadStatus}, device::Device};

#[derive(Clone)]
pub struct DualSenseSettings {
    pub bluetooth: bool,
    pub edge: bool,
}

impl Default for DualSenseSettings {
    fn default() -> Self {
        Self {
            bluetooth: true,
            edge: false,
        }
    }
}

pub struct DualSense {
    settings: DualSenseSettings,

}

impl Device for DualSense {
    fn handle_message(&mut self, msg: &crate::message::InDeviceDataMessage) {
        todo!()
    }

    fn generate_message(&mut self) -> Option<crate::message::OutDeviceDataMessage> {
        todo!()
    }
}

impl Gamepad for DualSense {
    fn get_status() -> GamepadStatus { todo!() }
}

impl DualSense {
    pub fn new(settings: DualSenseSettings) -> Self {
        Self {
            settings: settings,

        }
    }
}

impl Default for DualSense {
    fn default() -> Self {
        DualSense::new(DualSenseSettings::default())
    }
}
