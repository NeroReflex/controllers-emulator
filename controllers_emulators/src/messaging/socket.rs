use crate::message::{InMessage, OutMessage, Message};
use crate::message_serializer::{MessageSerializer, MessageSerializationError};
use crate::messaging_interface::MessagingInterface;

use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::fmt::Formatter;

#[derive(Debug)]
pub struct TcpSocketMessagingInterfaceError {
    error_msg: String
}

impl std::fmt::Display for TcpSocketMessagingInterfaceError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for TcpSocketMessagingInterfaceError {
    
}

impl TcpSocketMessagingInterfaceError {
    pub fn new(error_msg: String) -> Self {
        Self {
            error_msg
        }
    }
}

pub struct TcpSocketMessagingInterface<S>
where
    S: MessageSerializer + Default
{
    socket: TcpStream,
    serializer: S,
    queued_bytes: Vec<u8>,
}

impl<S> TcpSocketMessagingInterface<S>
where
    S: MessageSerializer + Default
{
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            serializer: S::default(),
            queued_bytes: Vec::<u8>::new()
        }
    }
}

impl<S> MessagingInterface for TcpSocketMessagingInterface<S>
where
    S: MessageSerializer + Default
{
    async fn recv(&mut self) -> Result<Option<InMessage>, Box<dyn std::error::Error>> {
        let default_capacity: usize = 1024;

        let mut tmp_buf = Vec::<u8>::with_capacity(default_capacity);

        let read_size = if let Some(exact_size) = self.serializer.fixed_size() {
            if exact_size > default_capacity {
                tmp_buf = Vec::<u8>::with_capacity(exact_size);
            }

            self.socket.read_exact(tmp_buf.as_mut()).await?
        } else {
            self.socket.read(tmp_buf.as_mut()).await?
        };

        if read_size == 0 {
            return Err(Box::new(TcpSocketMessagingInterfaceError::new(String::from("read 0 bytes from stream, probably no more data is coming"))));
        }
        
        // TODO: what if I have read just a part of the message?

        let msg = self.serializer.deserialize(&tmp_buf)?;

        match msg {
            Message::In(msg) => {
                Ok(Some(msg))
            },
            _ => {
                Err(Box::new(MessageSerializationError::new(String::from("Message type not supported"))))
            }
        }
    }

    async fn send(&mut self, msg: &OutMessage) -> Result<(), Box<dyn std::error::Error>> {
        let serialization_result = self.serializer.serialize(&Message::Out(msg.clone()))?;

        let written_bytes = self.socket.write(serialization_result.as_slice()).await?;

        if written_bytes == serialization_result.len() {
            Ok(())
        } else {
            //return Err();
            todo!()
        }
    }
}
