pub mod gamepad;
pub mod message;
pub mod gamepads;
pub mod messaging;
pub mod messaging_interface;
pub mod device;
pub mod device_manager;
pub mod message_serializer;
pub mod serializers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
