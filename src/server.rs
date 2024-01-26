use std::io::{Write, BufRead, BufReader};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::process::Command;
use std::str;

pub fn start_server()  {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream));
    }
}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    loop {

        let mut buf = String::new();
        if stream.read_line(&mut buf).is_err() {
            break;
        }

        println!("Received: {}", buf);

        let my_command = Command::new("bash")
            .arg("-c")
            .arg(buf)
            .output()
            .expect("failed to execute");

        let raw = my_command.stdout;

        let output = match str::from_utf8(&raw) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        
        stream.get_mut().write(output.as_bytes()).unwrap();
        stream.get_mut().flush().unwrap();

    }
}