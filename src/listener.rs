use std::net::TcpListener;
use std::io::{stdin, Read, Write};
use std::str;

pub fn socket_listen(addr: &String, port: &String) {

    // bind to socket
    let listner = TcpListener::bind(format!("{addr}:{port}")).unwrap();
    
    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        loop {
            // write
            let mut write_buf = String::new();
            stdin().read_line(&mut write_buf).unwrap();
            stream.write(write_buf.as_bytes()).unwrap();

            // read
            let mut read_buf= [0u8; 1024];
            let bytes_read = stream.read(&mut read_buf).unwrap();
            let data = &read_buf[..bytes_read];
            println!("{}", str::from_utf8(&data).unwrap());
        }
    }
}
