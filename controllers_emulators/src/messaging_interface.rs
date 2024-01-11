use crate::message::{InMessage, OutMessage};

pub trait MessagingInterface {
    async fn recv(&mut self) -> Result<Option<InMessage>, Box<dyn std::error::Error>>;

    async fn send(&mut self, msg: &OutMessage) -> Result<(), Box<dyn std::error::Error>>;
}