use controllers_emulators::gamepads::dualsense::{DualSense, DualSenseSettings};
use controllers_emulators::message::{Message, InMessage, OutMessage, OutDeviceMessage};
use controllers_emulators::messaging_interface::MessagingInterface;
use controllers_emulators::serializers::fixed_bytes_message_serializer::FixedBytesMessageSerializer;
use controllers_emulators::{messaging::socket::*, device_manager::DeviceManager};

use std::sync::Arc;
use std::sync::Mutex;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dev_mananager = Arc::new(Mutex::new(DeviceManager::default()));

    let dualsense_hash = dev_mananager.lock().unwrap().alloc(Arc::new(Mutex::new(DualSense::new(DualSenseSettings::default()))));

    println!("Added Dualsense with hash {}", dualsense_hash);

    // Create a TcpSocket and configure it (you can set options if needed)
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // Listen for incoming connections
    while let Ok((socket, _addr)) = listener.accept().await {
        // Handle each connection in a separate task
        let dm = dev_mananager.clone();
        tokio::spawn(async move { handle_connection(dm, socket).await } );
    }

    // you can test the controller by sending:
    // clear gamepad status: echo -n -e "\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00" | nc 127.0.0.1 8080
    Ok(())
}

async fn handle_connection(dev_manager: Arc<Mutex<DeviceManager>>, socket: TcpStream) {
    let mut messaging_iface = TcpSocketMessagingInterface::<FixedBytesMessageSerializer>::new(socket);

    'app_manager_loop: loop {
        let mut out_msg: Option<OutMessage> = None;

        match messaging_iface.recv().await {
            Ok(maybe_message) => {
                match &maybe_message {
                    Some(message) => {
                        match message {
                            &InMessage::Device(im_msg) => {
                                // here have the correct device see its message by calling device.handle_message
                                let dev_manager = dev_manager.lock().unwrap();
                                match dev_manager.fetch(im_msg.device_hash) {
                                    Some(dev) => {
                                        match dev.lock() {
                                            Ok(mut device) => {
                                                device.handle_message(&im_msg.data);
                                                
                                                // after the above is done call device.generate_message() to check if a message is required
                                                match device.generate_message() {
                                                    Some(output_msg) => {
                                                        out_msg = Some(OutMessage::Device(OutDeviceMessage {
                                                            device_hash: im_msg.device_hash,
                                                            data: output_msg,
                                                        }));
                                                    },
                                                    None => {
                                                        out_msg = None
                                                    }
                                                }
                                            },
                                            Err(err) => {
                                                println!("Error locking mutex: {}", err);
                                            }
                                        }
                                    },
                                    None => {
                                        println!("No such device {}", im_msg.device_hash);
                                    }
                                }
                            }
                        }
                    },
                    None => {}
                }
                
                
            },
            Err(err) => {
                println!("TCP socket in a bad state: '{}' -- closing it", err);
                break 'app_manager_loop;
            }
        }

        if let Some(message) = out_msg {
            match message {
                OutMessage::Device(dev_out_msg) => {
                    println!("Generated event for device with hash {}", dev_out_msg.device_hash);

                    match messaging_iface.send(&OutMessage::Device(dev_out_msg)).await {
                        Ok(_) => { },
                        Err(err) => {
                            println!("Error sending output message: {} -- closing TCP socket that is now in a bad state", err);
                            break 'app_manager_loop;
                        }
                    }
                }
            }
            
        }
    }
}