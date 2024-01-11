use crate::message::InDeviceDataMessage;
use crate::message::OutDeviceDataMessage;

pub trait Device: Sync + Send {
    // this is used to make the device alter its status based on the received message
    fn handle_message(&mut self, msg: &InDeviceDataMessage);

    // this is used to make the device make whatever is meant to so it can generate a new message
    fn generate_message(&mut self) -> Option<OutDeviceDataMessage>;
}