use std::io::{Read, Write};
use std::net::TcpStream;

pub fn start_client() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:9001").expect("Failed to connect to the server");

    let message = "Hello, server!";
    stream.write(message.as_bytes()).expect("Failed to send data to the server");

    let mut buffer = [0; 128];
    let bytes_received = stream.read(&mut buffer).expect("Failed to receive data from the server");

    let received_text = String::from_utf8_lossy(&buffer[..bytes_received]);
    println!("Received: {}", received_text);
}