use controllers_emulators::messaging_interface::MessagingInterface;
use controllers_emulators::serializers::fixed_bytes_message_serializer::FixedBytesMessageSerializer;
use controllers_emulators::{messaging::socket::*, device_manager::DeviceManager};

use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev_mananager = DeviceManager::default();

    // Create a TcpSocket and configure it (you can set options if needed)
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Listen for incoming connections
    while let Ok((socket, _addr)) = listener.accept().await {
        // Handle each connection in a separate task
        tokio::spawn(handle_connection(socket));
    }

    Ok(())
}

async fn handle_connection(socket: TcpStream) {
    let mut messaging_iface = TcpSocketMessagingInterface::<FixedBytesMessageSerializer>::new(socket);

    loop {
        match messaging_iface.recv().await {
            Ok(message) => {
                // TODO: here have the correct device see its message by calling device.handle_message
                // TODO: after the above is done call device.generate_message() to check if a message is required
            },
            Err(err) => {

            }
        }
    }
}