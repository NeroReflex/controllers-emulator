use crate::message_serializer::MessageSerializer;

pub struct FixedBytesMessageSerializer {

}

impl Default for FixedBytesMessageSerializer {
    fn default() -> Self {
        Self {}
    }
}

impl MessageSerializer for FixedBytesMessageSerializer {
    fn deserialize(&self, bytes: &Vec<u8>) -> Result<crate::message::Message, Box<dyn std::error::Error>> {
        todo!()
    }

    fn serialize(&self, bytes: &crate::message::Message) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn fixed_size(&self) -> Option<usize> {
        todo!()
    }
}