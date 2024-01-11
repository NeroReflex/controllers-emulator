use crate::message::Message;

use std::fmt::Formatter;

#[derive(Debug)]
pub struct MessageSerializationError {
    error_msg: String
}

impl std::fmt::Display for MessageSerializationError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for MessageSerializationError {
    
}

impl MessageSerializationError {
    pub fn new(error_msg: String) -> Self {
        Self {
            error_msg
        }
    }
}

pub trait MessageSerializer {
    fn deserialize(&self, bytes: &Vec<u8>) -> Result<Message, Box<dyn std::error::Error>>;

    fn serialize(&self, bytes: &Message) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    fn fixed_size(&self) -> Option<usize>;
}