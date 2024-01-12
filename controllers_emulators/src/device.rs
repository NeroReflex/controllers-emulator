use crate::message::InDeviceDataMessage;
use crate::message::OutDeviceDataMessage;

use std::time::Duration;

pub trait Device: Sync + Send {
    // this is used to make the device alter its status based on the received message
    fn handle_message(&mut self, msg: &InDeviceDataMessage);

    // this is used to make the device make whatever is meant to so it can generate a new message
    fn generate_message(&mut self) -> Option<OutDeviceDataMessage>;

    // this function is meant to be executed when the internal state of the device is meant to be synchronized with the OS/SDL/games/steam
    fn write_report(&self);

    // this function is meant to be executed in a parallel task to answer OS/SDL/games/steam requests changing the internal device state possibly.
    fn read_requests(&mut self);

    // this function is meant to return the rate for reports that has to be written to the OS
    fn write_timing(&self) -> Option<Duration>;
}
