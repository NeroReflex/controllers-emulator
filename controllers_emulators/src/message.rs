use crate::gamepad::GamepadStatus;


// Messages that are incoming from the input handler to the controller emulator about the emulated gamepad
#[derive(Copy, Clone, PartialEq)]
pub enum InMouseMessage {
    
}

// Messages that are incoming from the input handler to the controller emulator about the emulated gamepad
#[derive(Copy, Clone, PartialEq)]
pub enum InKeyboardMessage {
    
}

// Messages that are incoming from the input handler to the controller emulator about the emulated gamepad
#[derive(Copy, Clone, PartialEq)]
pub enum InGamepadMessage {
    SetStatus(GamepadStatus)
}

#[derive(Copy, Clone, PartialEq)]
pub enum InDeviceDataMessage {
    Gamepad(InGamepadMessage),
    Keyboard(InKeyboardMessage),
    Mouse(InMouseMessage),
}

#[derive(Copy, Clone, PartialEq)]
pub struct InDeviceMessage {
    pub device_hash: u64,
    pub data: InDeviceDataMessage,
}

#[derive(Copy, Clone, PartialEq)]
pub enum OutDeviceDataMessage {
    Gamepad(InGamepadMessage),
    Keyboard(InKeyboardMessage),
    Mouse(InMouseMessage),
}

#[derive(Copy, Clone, PartialEq)]
pub struct OutDeviceMessage {
    pub device_hash: u64,
    pub data: OutDeviceDataMessage,
}

// Messages that are outgoing from the controller emulator to the input handler
#[derive(Copy, Clone, PartialEq)]
pub enum OutMessage {
    Device(OutDeviceMessage)
}

// Messages that are incoming from the input handler to the controller emulator
#[derive(Copy, Clone, PartialEq)]
pub enum InMessage {
    Device(InDeviceMessage),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Message {
    Out(OutMessage),
    In(InMessage),
}